//! Chat SSE-stream wiring — extracted from `chat.rs` so each cube stays
//! under the 200-LOC ceiling.
//!
//! Responsibilities: `run_loop_stream` (assemble invoker + ctxs);
//! `build_event_stream` (translate LoopEvents to SSE + fire post-Done
//! cost + memory-nudge tasks); `loop_event_to_sse` (single-event
//! mapper, `pub(super)` so `chat_test.rs` can drive it directly).
//!
//! Wave 44c (F-HIGH-5): cancel via `CancellationToken` + `CancelOnDrop`
//! so SSE-client disconnect cancels the agent loop.
//!
//! Hermes P2.2.b: post-`Done` fires `chat_memory_nudge::spawn_nudge` to
//! register the (user, assistant) turn pair with the scheduler.

use super::chat_cost;
use super::chat_events::{
    done_event, error_event, sentiment_event, token_event, tool_use_result_event,
    tool_use_start_event,
};
use super::chat_memory_nudge;
use super::chat_stream_ctx::{ChatCostCtx, MemoryNudgeCtx};
use super::chat_token;
use crate::anthropic::default_model;
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
/// `nudge_ctx` carries the AppState handle + the user message for the
/// memory-review nudge. Owning both here keeps `build_event_stream`
/// ignorant of `AppState`.
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
    let user_msg_for_nudge = message.clone();
    let loop_stream = tool::run_with_tools(
        invoker, registry, tool::tool_definitions(),
        message, conv_id, cancel.clone(),
    );
    let cost_ctx = build_cost_ctx(&state, &user_id, raw_conversation_id, &provider_name, accum);
    let nudge_ctx = MemoryNudgeCtx {
        state,
        user_id,
        conversation_id: conv.clone(),
        user_msg: user_msg_for_nudge,
    };
    build_event_stream(loop_stream, conv, cancel, cost_ctx, nudge_ctx)
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
    // W55 Stage 2/3 — `default_model()` = env → registry → literal. Audit
    // Risk #2 (cross-provider id) parked for Stage-4: pulls anthropic id
    // even when provider != anthropic (router doesn't yet expose its pick).
    ChatCostCtx {
        accum,
        ledger_path: state.config().ledger_path.clone(),
        agent_id: chat_cost::build_agent_id(raw_conversation_id.as_deref(), user_id),
        provider: provider_name.to_string(),
        model: default_model().into_owned(),
        rates: chat_cost::provider_rates(state.router().as_ref(), provider_name),
        conversation_id: raw_conversation_id,
        token_tracker: state.token_tracker(),
    }
}

/// RAII guard that fires `cancel.cancel()` on Drop. Replaces the
/// pre-Wave-44c `_hold = cancel_tx` pattern so SSE-client disconnect
/// (stream dropped before completion) cancels the agent loop.
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
/// dispatch a `spawn_blocking` task to write the row, then fire the
/// memory-nudge so the scheduler sees the new turn. We do NOT await
/// either side-task — the SSE client has already received `done`.
fn build_event_stream<S>(
    upstream: S,
    conv_id: String,
    cancel: CancellationToken,
    cost_ctx: ChatCostCtx,
    nudge_ctx: MemoryNudgeCtx,
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
            fire_post_done(&cost_ctx, nudge_ctx, acc);
        }
    }
}

/// Side-effects fired after the trailing `Done` event: cost record +
/// memory-nudge spawn. Extracted from `build_event_stream` to keep
/// the stream body ≤30 LOC.
fn fire_post_done(cost_ctx: &ChatCostCtx, nudge_ctx: MemoryNudgeCtx, assistant_msg: String) {
    spawn_cost_record(cost_ctx);
    chat_memory_nudge::spawn_nudge(
        nudge_ctx.state,
        nudge_ctx.user_id,
        nudge_ctx.conversation_id,
        nudge_ctx.user_msg,
        assistant_msg,
    );
}

/// Fire-and-forget cost record. Errors logged inside `record_chat_cost`;
/// this helper never panics. Also fires a sibling token-tracker write
/// on the same snapshot so sleep-report sees per-turn telemetry.
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
    spawn_token_record(ctx, &usage, micro_cents);
}

/// Sibling of `spawn_cost_record` — populates the token-event row for
/// the same turn. The two helpers run in parallel so a tracker IO
/// failure cannot delay the ledger write or vice versa.
fn spawn_token_record(ctx: &ChatCostCtx, usage: &TokenUsage, micro_cents: u64) {
    let token_write = chat_token::TokenWrite {
        agent_id: ctx.agent_id.clone(),
        conversation_id: ctx.conversation_id.clone(),
        model: ctx.model.clone(),
        role: "kei-cortex-chat".into(),
        source_kind: "chat".into(),
        usage: usage.clone(),
        micro_cents,
    };
    chat_token::spawn_record_token_event(ctx.token_tracker.clone(), token_write);
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
