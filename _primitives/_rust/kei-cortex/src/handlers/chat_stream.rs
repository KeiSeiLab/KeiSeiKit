//! Chat SSE-stream wiring — extracted from `chat.rs` so each cube stays
//! under the 200-LOC ceiling.
//!
//! Responsibilities:
//!   * `run_loop_stream` — assemble invoker (with usage capture),
//!     registry, cancel token, and downstream cost context.
//!   * `build_event_stream` — translate `LoopEvent`s into SSE frames
//!     and fire the post-Done cost-recording task.
//!   * `loop_event_to_sse` — single-event mapper used both here and by
//!     `chat_test.rs` (kept `pub(super)` for that reason).
//!
//! Wave 44c (F-HIGH-5): cancel migrated from `oneshot::Sender<()>` to
//! `CancellationToken`. The previous `_hold = cancel_tx` guard relied
//! on the sender being dropped to fire the receiver — now we use a
//! `CancelOnDrop` RAII struct that calls `token.cancel()` on Drop so
//! the loop short-circuits when the SSE client disconnects.

use super::chat_cost;
use super::chat_events::{
    done_event, error_event, sentiment_event, token_event, tool_use_result_event,
    tool_use_start_event,
};
use crate::anthropic::MODEL;
use crate::anthropic_invoker;
use crate::state::AppState;
use crate::tool;
use crate::tool::loop_driver::TokenUsage;
use async_stream::stream;
use axum::response::sse::Event;
use futures::stream::Stream;
use futures::StreamExt;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use tokio_util::sync::CancellationToken;

/// Spawn the agent loop and translate events into SSE frames.
///
/// `cost_ctx` carries everything the post-Done cost-recording step needs;
/// owning all of it here keeps `build_event_stream` ignorant of `AppState`.
#[allow(clippy::too_many_arguments)]
pub(super) fn run_loop_stream(
    system: String,
    message: String,
    conv_id: String,
    state: AppState,
    user_id: String,
    provider_name: String,
    raw_conversation_id: Option<String>,
) -> impl Stream<Item = Result<Event, Infallible>> + Send + 'static {
    let accum: Arc<Mutex<TokenUsage>> = Arc::new(Mutex::new(TokenUsage::default()));
    let raw_invoker = anthropic_invoker::build_invoker(system);
    let invoker = chat_cost::wrap_invoker_with_usage_capture(raw_invoker, accum.clone());
    let registry = Arc::new(tool::ToolRegistry::default());
    let cancel = CancellationToken::new();
    let conv = conv_id.clone();
    let loop_stream = tool::run_with_tools(
        invoker, registry, tool::tool_definitions(),
        message, conv_id, cancel.clone(),
    );
    let cost_ctx = build_cost_ctx(&state, &user_id, raw_conversation_id, &provider_name, accum);
    build_event_stream(loop_stream, conv, cancel, cost_ctx)
}

/// Compose the post-loop cost-recording context. Extracted from
/// `run_loop_stream` to keep the entry point ≤30 LOC.
fn build_cost_ctx(
    state: &AppState,
    user_id: &str,
    raw_conversation_id: Option<String>,
    provider_name: &str,
    accum: Arc<Mutex<TokenUsage>>,
) -> ChatCostCtx {
    ChatCostCtx {
        accum,
        ledger_path: state.config().ledger_path.clone(),
        agent_id: chat_cost::build_agent_id(raw_conversation_id.as_deref(), user_id),
        provider: provider_name.to_string(),
        model: MODEL.to_string(),
        rates: chat_cost::provider_rates(state.router().as_ref(), provider_name),
    }
}

/// Captures all post-loop side-channel state so `build_event_stream`
/// has a single owned bundle to consume on `Done`.
#[derive(Clone)]
struct ChatCostCtx {
    accum: Arc<Mutex<TokenUsage>>,
    ledger_path: std::path::PathBuf,
    agent_id: String,
    provider: String,
    model: String,
    rates: (u32, u32),
}

/// RAII guard that fires `cancel.cancel()` on Drop. Replaces the
/// pre-Wave-44c `_hold = cancel_tx` pattern so SSE-client disconnect
/// (stream dropped before completion) cancels the agent loop. The
/// CancellationToken's clone-share semantics let us hold one instance
/// in the guard and another inside the loop.
struct CancelOnDrop {
    token: CancellationToken,
}

impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        self.token.cancel();
    }
}

/// Translate `LoopEvent`s into axum SSE events. Client disconnect is
/// handled by the stream's natural backpressure plus `CancelOnDrop`.
///
/// On the trailing `Done` event we snapshot the usage accumulator and
/// dispatch a `spawn_blocking` task to write the row. We do NOT await
/// the write — the SSE client has already received `done`.
fn build_event_stream<S>(
    upstream: S,
    conv_id: String,
    cancel: CancellationToken,
    cost_ctx: ChatCostCtx,
) -> impl Stream<Item = Result<Event, Infallible>> + Send + 'static
where
    S: Stream<Item = tool::LoopEvent> + Send + 'static,
{
    stream! {
        let _hold = CancelOnDrop { token: cancel };
        let mut acc = String::new();
        let mut saw_done = false;
        futures::pin_mut!(upstream);
        while let Some(ev) = upstream.next().await {
            if matches!(&ev, tool::LoopEvent::Done { .. }) {
                saw_done = true;
            }
            for sse in loop_event_to_sse(ev, &mut acc, &conv_id) {
                yield Ok::<Event, Infallible>(sse);
            }
        }
        if saw_done {
            spawn_cost_record(&cost_ctx);
        }
    }
}

/// Fire-and-forget cost record. Errors logged inside `record_chat_cost`;
/// this helper never panics.
fn spawn_cost_record(ctx: &ChatCostCtx) {
    let usage = chat_cost::snapshot(&ctx.accum);
    let (in_rate, out_rate) = ctx.rates;
    let micro_cents = chat_cost::compute_micro_cents(&usage, in_rate, out_rate);
    let cents = chat_cost::display_cents(micro_cents);
    let write = chat_cost::CostWrite {
        ledger_path: ctx.ledger_path.clone(),
        agent_id: ctx.agent_id.clone(),
        provider: ctx.provider.clone(),
        model: ctx.model.clone(),
        cents,
        micro_cents,
    };
    tokio::task::spawn_blocking(move || chat_cost::record_chat_cost(write));
}

/// Map one `LoopEvent` to ≥0 SSE events. `Done` emits sentiment + done.
/// `pub(super)` so `chat_test.rs` can drive this directly with a fake
/// `LoopEvent` while bypassing the rest of the stream wiring.
pub(super) fn loop_event_to_sse(
    ev: tool::LoopEvent,
    acc: &mut String,
    conv: &str,
) -> Vec<Event> {
    use tool::LoopEvent;
    match ev {
        LoopEvent::AssistantText(t) => {
            acc.push_str(&t);
            vec![token_event(&t)]
        }
        LoopEvent::ToolUseStart { tool: name, input } => {
            vec![tool_use_start_event(&name, &input)]
        }
        LoopEvent::ToolUseResult { tool_use_id, is_error } => {
            vec![tool_use_result_event(&tool_use_id, is_error)]
        }
        LoopEvent::Error(m) => vec![error_event(&m)],
        LoopEvent::Done { .. } => vec![sentiment_event(acc.as_str()), done_event(conv)],
    }
}
