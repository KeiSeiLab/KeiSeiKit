//! Translate `tool::LoopEvent`s into SSE frames per `/v1/*` surface.
//!
//! Three surfaces:
//!   * chat-completions — `data: { delta.content }` chunks +
//!     `kei.tool.progress` events (Hermes #6972).
//!   * responses        — `response.output_text.delta` + `response.completed`.
//!   * runs (P1.1.d)    — per-event `Vec<AgentChunk>` translated by
//!     `run_agent::run_real`, then mapped to `run.message.delta` /
//!     `run.completed` by `runs::run_event_for`.

use super::agent_runner;
use super::stream_chunks;
use super::sse::{AgentChunk, ToolProgress};
use super::types::{ResponseObject, Usage};
use crate::tool;
use crate::tool::loop_driver::TokenUsage;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use futures::stream::Stream;
use serde_json::json;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

const KEEPALIVE_SECS: u64 = 30;

/// Forward a stream of `LoopEvent`s as OpenAI chat-completion chunks.
/// `accum` is the TokenUsage accumulator paired with `rx` by
/// `agent_runner::stream_events`; on `Done` it is snapshot + translated
/// into the real `Usage` carried by the finish-chunk (N-1 stream fix).
pub fn forward_chat_completions(
    rx: mpsc::Receiver<tool::LoopEvent>,
    comp_id: String,
    model: String,
    created: u64,
    accum: Arc<Mutex<TokenUsage>>,
) -> Response {
    let mut up = ReceiverStream::new(rx);
    let stream = async_stream::stream! {
        while let Some(ev) = up.next().await {
            let final_usage = if matches!(ev, tool::LoopEvent::Done { .. }) {
                agent_runner::snapshot_usage(&accum)
            } else {
                Usage::default()
            };
            for sse in chat_event_to_sse(ev, &comp_id, &model, created, final_usage) {
                yield Ok::<Event, Infallible>(sse);
            }
        }
    };
    sse_response(stream)
}

/// Map ONE `LoopEvent` to ≥0 SSE chat-completion frames. `final_usage`
/// is consulted only on `Done` — caller passes a real snapshot for that
/// branch and `Usage::default()` otherwise.
fn chat_event_to_sse(
    ev: tool::LoopEvent,
    comp_id: &str,
    model: &str,
    created: u64,
    final_usage: Usage,
) -> Vec<Event> {
    match ev {
        tool::LoopEvent::AssistantText(t) => {
            vec![stream_chunks::content_chunk(comp_id, model, created, &t)]
        }
        tool::LoopEvent::ToolUseStart { tool: name, .. } => {
            vec![tool_progress_event(&name, "start")]
        }
        tool::LoopEvent::ToolUseResult { tool_use_id, .. } => {
            vec![tool_progress_event(&tool_use_id, "done")]
        }
        tool::LoopEvent::Error(m) => vec![error_chunk(comp_id, model, created, &m)],
        tool::LoopEvent::Done { .. } => vec![
            stream_chunks::finish_chunk(comp_id, model, created, final_usage),
            stream_chunks::done_sentinel(),
        ],
    }
}

/// Forward the loop stream as `/v1/responses` SSE frames.
pub fn forward_responses(rx: mpsc::Receiver<tool::LoopEvent>, obj: ResponseObject) -> Response {
    let mut up = ReceiverStream::new(rx);
    let stream = async_stream::stream! {
        while let Some(ev) = up.next().await {
            for sse in responses_event_to_sse(ev, &obj) {
                yield Ok::<Event, Infallible>(sse);
            }
        }
    };
    sse_response(stream)
}

fn responses_event_to_sse(ev: tool::LoopEvent, obj: &ResponseObject) -> Vec<Event> {
    match ev {
        tool::LoopEvent::AssistantText(t) => vec![Event::default()
            .event("response.output_text.delta")
            .data(json!({ "delta": t }).to_string())],
        tool::LoopEvent::ToolUseStart { tool: name, .. } => {
            vec![tool_progress_event(&name, "start")]
        }
        tool::LoopEvent::ToolUseResult { tool_use_id, .. } => {
            vec![tool_progress_event(&tool_use_id, "done")]
        }
        tool::LoopEvent::Error(m) => vec![Event::default()
            .event("response.error")
            .data(json!({ "error": m }).to_string())],
        tool::LoopEvent::Done { .. } => vec![Event::default()
            .event("response.completed")
            .data(json!({ "response": obj }).to_string())],
    }
}

/// P1.1.d: per-event translator for `/v1/runs`. The runs surface
/// drains the loop in `run_agent::run_real` (so the registry can mark
/// `in_progress` on the first AssistantText), so we return
/// `Vec<AgentChunk>` rather than a Stream. `runs::run_event_for`
/// then maps each chunk into a `run.message.delta` / `run.completed`
/// SSE shape.
pub fn forward_runs(ev: tool::LoopEvent) -> Vec<AgentChunk> {
    match ev {
        tool::LoopEvent::AssistantText(t) => vec![AgentChunk::Delta(t)],
        tool::LoopEvent::ToolUseStart { tool: name, .. } => {
            vec![AgentChunk::ToolProgress(progress(&name, "start"))]
        }
        tool::LoopEvent::ToolUseResult { tool_use_id, .. } => {
            vec![AgentChunk::ToolProgress(progress(&tool_use_id, "done"))]
        }
        tool::LoopEvent::Error(m) => vec![AgentChunk::Delta(format!("[error] {m}"))],
        tool::LoopEvent::Done { .. } => vec![AgentChunk::Done(Usage::default())],
    }
}

fn sse_response<S>(stream: S) -> Response
where
    S: Stream<Item = Result<Event, Infallible>> + Send + 'static,
{
    Sse::new(stream)
        .keep_alive(KeepAlive::new().interval(Duration::from_secs(KEEPALIVE_SECS)))
        .into_response()
}

fn progress(tool: &str, phase: &'static str) -> ToolProgress {
    ToolProgress { tool: tool.to_string(), phase, ts: now_ms() }
}

fn error_chunk(comp_id: &str, model: &str, created: u64, msg: &str) -> Event {
    let body = json!({
        "id": comp_id,
        "object": "chat.completion.chunk",
        "created": created,
        "model": model,
        "choices": [{
            "index": 0,
            "delta": { "content": format!("[error] {msg}") },
            "finish_reason": "stop",
        }],
    });
    Event::default().data(body.to_string())
}

fn tool_progress_event(tool: &str, phase: &'static str) -> Event {
    let data = serde_json::to_string(&progress(tool, phase)).unwrap_or_else(|_| "{}".to_string());
    Event::default().event("kei.tool.progress").data(data)
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_assistant_text_maps_to_one_chunk() {
        let v = chat_event_to_sse(
            tool::LoopEvent::AssistantText("hi".into()),
            "c1",
            "m",
            0,
            Usage::default(),
        );
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn chat_done_emits_finish_plus_sentinel() {
        let v = chat_event_to_sse(
            tool::LoopEvent::Done { conversation_id: "c".into(), turns: 1 },
            "c1",
            "m",
            0,
            Usage::default(),
        );
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn runs_assistant_text_maps_to_delta() {
        let v = forward_runs(tool::LoopEvent::AssistantText("hi".into()));
        assert!(matches!(v.as_slice(), [AgentChunk::Delta(s)] if s == "hi"));
    }

    #[test]
    fn runs_done_maps_to_done_chunk() {
        let v = forward_runs(tool::LoopEvent::Done {
            conversation_id: "c".into(),
            turns: 1,
        });
        assert!(matches!(v.as_slice(), [AgentChunk::Done(_)]));
    }

    #[test]
    fn runs_tool_start_maps_to_progress() {
        let v = forward_runs(tool::LoopEvent::ToolUseStart {
            tool: "read".into(),
            input: serde_json::json!({}),
        });
        assert!(matches!(v.as_slice(), [AgentChunk::ToolProgress(_)]));
    }
}
