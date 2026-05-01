//! Agent runner — bridges the OpenAI-compat surface (`/v1/*`) to the
//! real kei-cortex agent loop in `tool::run_with_tools`.
//!
//! Replaces the Phase-1.1 `stub_agent_reply` with two adaptors:
//!
//!   * `collect_reply`  — sync (non-stream) drain. Runs the loop to
//!     completion and concatenates every `LoopEvent::AssistantText`
//!     into a single string. Used by the JSON handlers.
//!   * `stream_events`  — async streaming. Spawns the loop on a
//!     tokio task and returns the raw `LoopEvent` receiver so the
//!     stream-forwarder can translate per-event into SSE.
//!
//! Constructor Pattern: this cube owns ONE responsibility — wiring
//! the agent loop. SSE serialisation lives in `stream_forwarder.rs`;
//! the loop itself lives in `tool::loop_driver`.

use super::error::OpenAiError;
use super::types::{OpenAiTool, Usage};
use crate::anthropic::default_model;
use crate::anthropic_invoker;
use crate::handlers::{chat_cost, chat_token};
use crate::state::AppState;
use crate::tool;
use crate::tool::loop_driver::TokenUsage;
use futures::StreamExt;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// Channel capacity for the loop → forwarder pipe.
pub const EVENT_CHANNEL_CAPACITY: usize = 64;

/// Synchronous (non-stream) drain: run the agent loop to completion
/// and return the concatenated assistant text + REAL TokenUsage.
/// H-2 fix: invoker is wrapped with `chat_cost::wrap_invoker_with_usage_capture`
/// so per-turn TokenUsage accumulates into a shared cell; on Done we
/// snapshot it and translate to the OpenAI `Usage` shape. Tool-call
/// envelopes are not surfaced to OpenAI clients here (they ride inside
/// the assistant text via the loop's normal flow); the second tuple slot
/// is reserved for a future `Vec<ToolCall>` once the wire format supports it.
///
/// Phase 2 token-tracker wiring: after the loop emits Done, this fires
/// a fire-and-forget [`record_post_turn_token_event`] so the tracker
/// captures one row per surface call. `surface` is the
/// caller-supplied source-kind label ("chat-completions" / "responses"
/// / "runs"); `agent_id` correlates the row with downstream artefacts.
pub async fn collect_reply(
    state: &AppState,
    system: String,
    user_prompt: String,
    _tools: Vec<OpenAiTool>,
    surface: &str,
    agent_id: String,
    conversation_id: Option<String>,
) -> Result<(String, Vec<tool::ToolCall>, Usage), OpenAiError> {
    let accum = Arc::new(Mutex::new(TokenUsage::default()));
    let stream = build_loop_stream_with_accum(
        state,
        system,
        user_prompt,
        new_conv_id(),
        CancellationToken::new(),
        accum.clone(),
    );
    futures::pin_mut!(stream);
    let mut text = String::new();
    let mut last_error: Option<String> = None;
    while let Some(ev) = stream.next().await {
        match ev {
            tool::LoopEvent::AssistantText(t) => text.push_str(&t),
            tool::LoopEvent::Error(m) => last_error = Some(m),
            tool::LoopEvent::Done { .. } => break,
            _ => {}
        }
    }
    if text.is_empty() {
        if let Some(m) = last_error {
            return Err(OpenAiError::Upstream(m));
        }
    }
    let snap = chat_cost::snapshot(&accum);
    record_post_turn_token_event(state, &accum, conversation_id, agent_id, surface);
    Ok((text, Vec::new(), translate_usage(&snap)))
}

/// Translate kei-cortex's TokenUsage into OpenAI Usage shape.
pub(super) fn translate_usage(u: &TokenUsage) -> Usage {
    Usage {
        prompt_tokens: u.input_tokens,
        completion_tokens: u.output_tokens,
        total_tokens: u.input_tokens.saturating_add(u.output_tokens),
    }
}

/// Snapshot the streaming TokenUsage accumulator and translate to the
/// OpenAI `Usage` shape. Used by `stream_forwarder` on `LoopEvent::Done`
/// to attach real token totals to the SSE finish-chunk (N-1 stream fix).
pub fn snapshot_usage(accum: &Arc<Mutex<TokenUsage>>) -> Usage {
    translate_usage(&chat_cost::snapshot(accum))
}

/// Streaming variant with token-event recording. Spawns the agent loop on a
/// tokio task and returns a receiver of raw `LoopEvent`s plus a shared
/// TokenUsage accumulator the forwarder snapshots on `Done` (N-1 stream fix).
/// Also fires a fire-and-forget tracker write when the loop forwarder closes
/// (reaches Done or the loop drops). `cancel` propagates client-disconnect
/// (chat-completions) or `/v1/runs/:id/stop` (runs) into the loop.
pub fn stream_events_with_tracking(
    state: &AppState,
    system: String,
    user_prompt: String,
    conv_id: String,
    cancel: CancellationToken,
    surface: &'static str,
    agent_id: String,
    conversation_id: Option<String>,
) -> (mpsc::Receiver<tool::LoopEvent>, Arc<Mutex<TokenUsage>>) {
    let accum = Arc::new(Mutex::new(TokenUsage::default()));
    let (tx, rx) = mpsc::channel::<tool::LoopEvent>(EVENT_CHANNEL_CAPACITY);
    let stream = build_loop_stream_with_accum(
        state,
        system,
        user_prompt,
        conv_id,
        cancel,
        accum.clone(),
    );
    let state_for_post = state.clone();
    let accum_for_post = accum.clone();
    tokio::spawn(async move {
        forward_loop(stream, tx).await;
        // Forwarder closed (loop reached Done or dropped). Snapshot the
        // accumulator and fire the tracker write — fire-and-forget, any
        // failure logs to stderr inside the spawn_blocking helper.
        record_post_turn_token_event(
            &state_for_post,
            &accum_for_post,
            conversation_id,
            agent_id,
            surface,
        );
    });
    (rx, accum)
}

/// Construct the agent-loop event stream with caller-provided usage
/// accumulator. The invoker is wrapped via
/// `chat_cost::wrap_invoker_with_usage_capture` so per-turn TokenUsage
/// flows into `accum`; caller snapshots after Done. Captures the
/// project-root from `AppState` so file-touching tools chroot correctly.
fn build_loop_stream_with_accum(
    state: &AppState,
    system: String,
    user_prompt: String,
    conv_id: String,
    cancel: CancellationToken,
    accum: Arc<Mutex<TokenUsage>>,
) -> impl futures::Stream<Item = tool::LoopEvent> + Send + 'static {
    let raw_invoker = anthropic_invoker::build_invoker(system);
    let invoker = chat_cost::wrap_invoker_with_usage_capture(raw_invoker, accum);
    let registry = Arc::new(tool::ToolRegistry::with_project_root(
        state.config().project_root.clone(),
    ));
    tool::run_with_tools(
        invoker,
        registry,
        tool::tool_definitions(),
        user_prompt,
        conv_id,
        cancel,
    )
}

/// Drain the loop stream into the bounded mpsc. Stops on first send
/// error (forwarder dropped) or when the loop finishes.
async fn forward_loop<S>(stream: S, tx: mpsc::Sender<tool::LoopEvent>)
where
    S: futures::Stream<Item = tool::LoopEvent> + Send + 'static,
{
    futures::pin_mut!(stream);
    while let Some(ev) = stream.next().await {
        if tx.send(ev).await.is_err() {
            return;
        }
    }
}

/// Fire one [`TokenWrite`] into the AppState's token-tracker after a
/// completed turn. Helper centralises the conversion from the runtime
/// snapshot (TokenUsage + provider rates) to a tracker row, so each
/// OpenAI handler (`chat_completions::handle_sync`, `responses`,
/// `run_agent::run_real`) just calls this once with its surface name.
///
/// `provider_name` selects the rate row from `state.router()`; matches
/// what `chat_cost::provider_rates` would return on the streaming side.
/// `source_kind` is the surface label ("chat" / "responses" / "runs").
pub fn record_post_turn_token_event(
    state: &AppState,
    accum: &Arc<Mutex<TokenUsage>>,
    conversation_id: Option<String>,
    agent_id: String,
    source_kind: &str,
) {
    let usage = chat_cost::snapshot(accum);
    let provider_name = state.config().default_provider.clone();
    let (in_rate, out_rate) = chat_cost::provider_rates(state.router().as_ref(), &provider_name);
    let micro_cents = chat_cost::compute_micro_cents(&usage, in_rate, out_rate);
    let write = chat_token::TokenWrite {
        agent_id,
        conversation_id,
        model: default_model().into_owned(),
        role: "kei-cortex-chat".into(),
        source_kind: source_kind.to_string(),
        usage,
        micro_cents,
    };
    chat_token::spawn_record_token_event(state.token_tracker(), write);
}

/// Local `conversation_id` for one-shot chat-completions calls. The
/// loop requires a non-empty id; OpenAI clients do not pass one. Kept
/// short and prefixed so it's distinguishable from the native
/// `/api/v1/cortex/pet/:user_id/chat` ids in logs.
fn new_conv_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("oai-{nanos:x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_conv_id_is_prefixed() {
        let a = new_conv_id();
        assert!(a.starts_with("oai-"));
    }

    #[test]
    fn new_conv_id_is_unique_under_repeated_calls() {
        let a = new_conv_id();
        std::thread::sleep(std::time::Duration::from_nanos(1));
        let b = new_conv_id();
        assert_ne!(a, b);
    }

    /// H-2 contract: TokenUsage with input/output_tokens translates to
    /// OpenAI Usage shape with matching prompt/completion + summed total.
    #[test]
    fn translate_usage_maps_token_counts() {
        let snap = TokenUsage { input_tokens: 10, output_tokens: 5 };
        let u = translate_usage(&snap);
        assert_eq!(u.prompt_tokens, 10);
        assert_eq!(u.completion_tokens, 5);
        assert_eq!(u.total_tokens, 15);
    }

    /// H-2 contract: snapshot_usage round-trips through Arc<Mutex<TokenUsage>>.
    #[test]
    fn snapshot_usage_reads_accumulator() {
        let accum = Arc::new(Mutex::new(TokenUsage {
            input_tokens: 100,
            output_tokens: 250,
        }));
        let u = snapshot_usage(&accum);
        assert_eq!(u.prompt_tokens, 100);
        assert_eq!(u.completion_tokens, 250);
        assert_eq!(u.total_tokens, 350);
    }
}
