//! Real agent loop driving `/v1/runs` via `agent_runner::stream_events`.
//!
//! P1.1.d (2026-04-28): replaces the Phase-1.1 `run_stub` with a wired
//! loop that drains `tool::LoopEvent`s into `AgentChunk`s the SSE
//! handler streams to the client.
//!
//! Lifecycle marks on the `RunRegistry`:
//!   * `in_progress` on the FIRST `LoopEvent::AssistantText` (or first
//!     `Delta` chunk that escapes the translator).
//!   * `cancelled` if the cancel token fires before the loop completes.
//!   * `completed` when `LoopEvent::Done` lands (loop's natural exit).
//!
//! Cancel is honoured by passing the same `CancellationToken` into
//! `agent_runner::stream_events` — the loop's own `select!` against
//! the in-flight invoker (Wave 44c) terminates the run within
//! milliseconds of `/v1/runs/{id}/stop`.

use super::agent_runner;
use super::run_registry::RunRegistry;
use super::sse::AgentChunk;
use super::stream_forwarder;
use super::types::{ChatMessage, Usage};
use crate::state::AppState;
use crate::tool;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// Default system prompt used when the request does not include a
/// `role: system` message. Kept in sync with chat_completions.rs so
/// the two surfaces produce comparable replies for the same input.
const DEFAULT_SYSTEM: &str =
    "You are kei-cortex, a helpful assistant. Use <think> for reasoning and tool calls when appropriate.";

/// Mutable per-run drain bookkeeping. Tracks whether the loop already
/// emitted a Done chunk (so finalize doesn't duplicate it) and whether
/// the registry was marked `in_progress` (so we only mark once).
struct DrainState {
    marked_in_progress: bool,
    saw_done: bool,
}

impl DrainState {
    fn new() -> Self {
        Self { marked_in_progress: false, saw_done: false }
    }
}

/// The real agent. Spawned by `runs::create_run` immediately after the
/// 202 response is queued. Drains the loop into the SSE channel.
/// Phase 2: `stream_events_with_tracking` fires a fire-and-forget
/// token-event after the loop closes (Done or cancelled) so sleep-
/// report sees runs alongside chat-completions / responses.
pub async fn run_real(
    state: AppState,
    registry: RunRegistry,
    id: String,
    messages: Vec<ChatMessage>,
    tx: mpsc::Sender<AgentChunk>,
    cancel: CancellationToken,
) {
    let system = system_prompt_from_messages(&messages);
    let prompt = flatten_user_prompt(&messages);
    let agent_id = format!("openai-run-{}", id);
    let (event_rx, _accum) = agent_runner::stream_events_with_tracking(
        &state,
        system,
        prompt,
        id.clone(),
        cancel.clone(),
        "runs",
        agent_id,
        Some(id.clone()),
    );
    drain_loop(registry, id, event_rx, tx, cancel).await;
}

/// Drain the loop's `LoopEvent` stream into `AgentChunk` SSE frames,
/// updating the registry status as the loop progresses.
async fn drain_loop(
    registry: RunRegistry,
    id: String,
    mut event_rx: mpsc::Receiver<tool::LoopEvent>,
    tx: mpsc::Sender<AgentChunk>,
    cancel: CancellationToken,
) {
    let mut state = DrainState::new();
    while let Some(evt) = event_rx.recv().await {
        update_progress(&registry, &id, &evt, &mut state);
        let chunks = stream_forwarder::forward_runs(evt);
        if forward_chunks(&tx, chunks).await.is_err() {
            break;
        }
    }
    finalize(&registry, &id, &cancel, &tx, &state).await;
}

/// Update the registry / drain bookkeeping based on the next loop event.
fn update_progress(registry: &RunRegistry, id: &str, ev: &tool::LoopEvent, state: &mut DrainState) {
    if !state.marked_in_progress && is_assistant_text(ev) {
        registry.mark(id, "in_progress");
        state.marked_in_progress = true;
    }
    if matches!(ev, tool::LoopEvent::Done { .. }) {
        state.saw_done = true;
    }
}

/// Forward translated chunks to the SSE channel. Returns `Err` if the
/// receiver dropped (client disconnected) so the caller can stop pulling
/// from the loop.
async fn forward_chunks(
    tx: &mpsc::Sender<AgentChunk>,
    chunks: Vec<AgentChunk>,
) -> Result<(), ()> {
    for c in chunks {
        if tx.send(c).await.is_err() {
            return Err(());
        }
    }
    Ok(())
}

/// Mark the registry final state. If the loop never reached `Done`
/// (cancel-fired or client-dropped), emit a terminal `AgentChunk::Done`
/// so the SSE handler can close the stream.
async fn finalize(
    registry: &RunRegistry,
    id: &str,
    cancel: &CancellationToken,
    tx: &mpsc::Sender<AgentChunk>,
    state: &DrainState,
) {
    if cancel.is_cancelled() {
        registry.mark(id, "cancelled");
    } else {
        registry.mark(id, "completed");
    }
    if !state.saw_done {
        let _ = tx.send(AgentChunk::Done(Usage::default())).await;
    }
}

/// Pull the first `role: system` message out, falling back to the
/// shared `DEFAULT_SYSTEM` prompt when none is present.
fn system_prompt_from_messages(messages: &[ChatMessage]) -> String {
    messages
        .iter()
        .find(|m| m.role == "system")
        .and_then(|m| m.content.clone())
        .unwrap_or_else(|| DEFAULT_SYSTEM.to_string())
}

/// Concatenate `system` + `user` content into a single prompt for the
/// agent loop. Mirrors `translation::flatten_user_prompt` to avoid a
/// cross-module call from the spawn path.
fn flatten_user_prompt(messages: &[ChatMessage]) -> String {
    let mut out = String::new();
    for m in messages {
        if (m.role == "system" || m.role == "user") && m.content.is_some() {
            if !out.is_empty() {
                out.push_str("\n\n");
            }
            out.push_str(m.content.as_deref().unwrap_or(""));
        }
    }
    out
}

fn is_assistant_text(ev: &tool::LoopEvent) -> bool {
    matches!(ev, tool::LoopEvent::AssistantText(_))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn msg(role: &str, content: &str) -> ChatMessage {
        ChatMessage {
            role: role.into(),
            content: Some(content.into()),
            name: None,
            tool_call_id: None,
            tool_calls: None,
        }
    }

    #[test]
    fn system_prompt_picks_role_system() {
        let m = vec![msg("system", "you are X"), msg("user", "hi")];
        assert_eq!(system_prompt_from_messages(&m), "you are X");
    }

    #[test]
    fn system_prompt_falls_back_when_absent() {
        let m = vec![msg("user", "hi")];
        assert_eq!(system_prompt_from_messages(&m), DEFAULT_SYSTEM);
    }

    #[test]
    fn flatten_concatenates_system_and_user() {
        let m = vec![msg("system", "be terse"), msg("user", "hi")];
        let p = flatten_user_prompt(&m);
        assert!(p.contains("be terse"));
        assert!(p.contains("hi"));
    }

    #[test]
    fn assistant_text_predicate() {
        assert!(is_assistant_text(&tool::LoopEvent::AssistantText("x".into())));
        assert!(!is_assistant_text(&tool::LoopEvent::Error("e".into())));
    }
}
