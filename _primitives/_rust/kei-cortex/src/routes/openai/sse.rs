//! SSE plumbing for /v1/chat/completions (stream=true), /v1/responses
//! (stream=true), and /v1/runs/{id}/events.
//!
//! Implements Hermes' `kei.tool.progress` custom event (Hermes #6972)
//! so frontends can render tool execution UI without inferring it from
//! `delta.content` text. Keepalive is 30 s — short enough for nginx /
//! Cloudflare default 60 s read-timeouts, long enough to not spam
//! quiet streams.
//!
//! Channel capacity is 64 — back-pressures the agent loop if the
//! client is slow without dropping events.

use super::types::Usage;
use async_stream::stream;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::IntoResponse;
use futures::stream::Stream;
use serde::Serialize;
use std::convert::Infallible;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

/// Channel capacity for the agent → SSE writer pipe.
pub const CHANNEL_CAPACITY: usize = 64;

/// Keepalive interval. Half a typical 60 s reverse-proxy read-timeout.
pub const KEEPALIVE_SECS: u64 = 30;

/// One event the agent loop wants to push. The variant determines
/// whether we emit a default-event `data: ...` frame or a custom
/// `event: kei.tool.progress` frame.
#[derive(Debug, Clone)]
pub enum AgentChunk {
    /// Streamed `delta.content` text — sent as the default chat-completion
    /// chunk shape.
    Delta(String),
    /// Tool-progress notification — sent as a `kei.tool.progress` event
    /// so the frontend doesn't render it as model output.
    ToolProgress(ToolProgress),
    /// Final usage block — caller flushes after this and sends [DONE].
    Done(Usage),
}

/// Payload for `kei.tool.progress` events.
#[derive(Debug, Clone, Serialize)]
pub struct ToolProgress {
    pub tool: String,
    /// "start" | "delta" | "done"
    pub phase: &'static str,
    pub ts: u64,
}

/// Build a paired `(sender, sse-response)` for streaming.
///
/// `make_chunk_event` lets the caller serialise an `AgentChunk::Delta`
/// into the right shape (chat-completions vs responses); the `Done`
/// variant lets the caller emit the final usage frame and `[DONE]`
/// sentinel.
pub fn build_sse<F>(
    make_chunk_event: F,
) -> (mpsc::Sender<AgentChunk>, impl IntoResponse)
where
    F: Fn(&AgentChunk) -> Option<Event> + Send + Sync + 'static,
{
    let (tx, rx) = mpsc::channel::<AgentChunk>(CHANNEL_CAPACITY);
    let sse = sse_from_rx(rx, make_chunk_event);
    (tx, sse)
}

/// Build an SSE response from an existing `Receiver<AgentChunk>`.
/// Used by `/v1/runs/{id}/events` where the channel was created at
/// run-spawn time and stored in the registry.
pub fn sse_from_rx<F>(
    rx: mpsc::Receiver<AgentChunk>,
    make_chunk_event: F,
) -> impl IntoResponse
where
    F: Fn(&AgentChunk) -> Option<Event> + Send + Sync + 'static,
{
    let stream = chunk_stream(ReceiverStream::new(rx), make_chunk_event);
    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(KEEPALIVE_SECS)))
}

/// Translate the channel of `AgentChunk` into a stream of SSE `Event`s.
fn chunk_stream<S, F>(
    mut rx: S,
    make_chunk_event: F,
) -> impl Stream<Item = Result<Event, Infallible>> + Send + 'static
where
    S: Stream<Item = AgentChunk> + Unpin + Send + 'static,
    F: Fn(&AgentChunk) -> Option<Event> + Send + Sync + 'static,
{
    stream! {
        while let Some(chunk) = rx.next().await {
            match &chunk {
                AgentChunk::ToolProgress(p) => {
                    if let Ok(ev) = tool_progress_event(p) {
                        yield Ok::<_, Infallible>(ev);
                    }
                }
                _ => {
                    if let Some(ev) = make_chunk_event(&chunk) {
                        yield Ok::<_, Infallible>(ev);
                    }
                }
            }
        }
    }
}

/// Serialise a `kei.tool.progress` event with custom event-name.
fn tool_progress_event(p: &ToolProgress) -> Result<Event, serde_json::Error> {
    let data = serde_json::to_string(p)?;
    Ok(Event::default().event("kei.tool.progress").data(data))
}

/// Convenience constructor for tool-progress events. `phase` ∈
/// {"start", "delta", "done"}; ts is unix-ms.
pub fn tool_progress(tool: impl Into<String>, phase: &'static str) -> ToolProgress {
    ToolProgress {
        tool: tool.into(),
        phase,
        ts: now_ms(),
    }
}

/// Unix-time milliseconds without panicking. Falls back to 0 if the
/// system clock is somehow before 1970 (it isn't, but `unwrap()` is
/// banned outside tests).
fn now_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_progress_event_carries_event_name() {
        let p = tool_progress("read", "start");
        let ev = tool_progress_event(&p).unwrap();
        // axum's Event has no public getter for `event`; serialise via Display
        // is also private. Smoke-test by checking the helper compiled and
        // the payload roundtrips.
        let s = serde_json::to_string(&p).unwrap();
        assert!(s.contains("\"tool\":\"read\""));
        assert!(s.contains("\"phase\":\"start\""));
        let _ = ev;
    }

    #[test]
    fn now_ms_is_nonzero() {
        assert!(now_ms() > 1_700_000_000_000);
    }
}
