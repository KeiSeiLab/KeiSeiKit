//! Async HTTP/SSE client for the cortex daemon.
//!
//! `chat_stream` opens `POST /api/v1/cortex/pet/:user_id/chat`, drains the
//! SSE response, and invokes a callback for every parsed [`ChatEvent`].
//!
//! The SSE parser is intentionally minimal — frames are split on `\n\n`
//! (event terminator) and each frame has its `data:` lines concatenated.
//! Comment lines (starting with `:`) and `event:` / `id:` / `retry:` lines
//! are ignored, matching the W3C EventSource specification subset that
//! axum's `Sse` writer emits.

use crate::types::{parse_event, ChatEvent, ChatRequest};
use anyhow::{Context, Result};
use futures::StreamExt;

/// Dispatch a chat request and stream events to `on_event` as they arrive.
///
/// `url` is the daemon base (e.g. `http://127.0.0.1:9797`); the path
/// `/api/v1/cortex/pet/{user_id}/chat` is appended internally so callers
/// only configure the host once.
pub async fn chat_stream<F>(
    url: &str,
    token: &str,
    user_id: &str,
    message: &str,
    conversation_id: Option<String>,
    mut on_event: F,
) -> Result<()>
where
    F: FnMut(ChatEvent),
{
    let endpoint = format!("{}/api/v1/cortex/pet/{}/chat", url.trim_end_matches('/'), user_id);
    let body = ChatRequest {
        message: message.to_string(),
        conversation_id,
    };
    let resp = reqwest::Client::new()
        .post(&endpoint)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await
        .with_context(|| format!("connect {endpoint}"))?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        anyhow::bail!("daemon {status}: {body}");
    }
    drain_sse(resp, &mut on_event).await
}

/// Drain a `reqwest::Response` body as SSE frames.
async fn drain_sse<F>(resp: reqwest::Response, on_event: &mut F) -> Result<()>
where
    F: FnMut(ChatEvent),
{
    let mut buf = String::new();
    let mut bytes = resp.bytes_stream();
    while let Some(chunk) = bytes.next().await {
        let chunk = chunk.context("read SSE chunk")?;
        buf.push_str(&String::from_utf8_lossy(&chunk));
        flush_complete_frames(&mut buf, on_event);
    }
    flush_complete_frames(&mut buf, on_event);
    Ok(())
}

/// Pull every complete `\n\n`-terminated frame out of `buf`, parse it, and
/// dispatch resulting events. Incomplete trailing bytes stay in `buf` for
/// the next chunk.
pub fn flush_complete_frames<F>(buf: &mut String, on_event: &mut F)
where
    F: FnMut(ChatEvent),
{
    while let Some(idx) = buf.find("\n\n") {
        let frame: String = buf.drain(..idx + 2).collect();
        if let Some(ev) = parse_frame(&frame) {
            on_event(ev);
        }
    }
}

/// Extract the `ChatEvent` carried by a single SSE frame.
fn parse_frame(frame: &str) -> Option<ChatEvent> {
    let mut data = String::new();
    for line in frame.lines() {
        if let Some(rest) = line.strip_prefix("data:") {
            if !data.is_empty() {
                data.push('\n');
            }
            data.push_str(rest.trim_start_matches(' '));
        }
    }
    parse_event(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_frame_extracts_data() {
        let frame = "data: {\"type\":\"token\",\"text\":\"hi\"}\n\n";
        let ev = parse_frame(frame).unwrap();
        assert_eq!(ev, ChatEvent::Token("hi".into()));
    }

    #[test]
    fn parse_frame_ignores_comments_and_event_id() {
        let frame = ": keepalive\nevent: chat\nid: 7\ndata: {\"type\":\"done\",\"conversation_id\":\"x\"}\n\n";
        let ev = parse_frame(frame).unwrap();
        assert_eq!(ev, ChatEvent::Done { conversation_id: "x".into() });
    }

    #[test]
    fn flush_handles_partial_buffer() {
        let mut buf = String::from("data: {\"type\":\"token\",\"text\":\"a\"}\n\ndata: {\"typ");
        let mut got = Vec::new();
        flush_complete_frames(&mut buf, &mut |e| got.push(e));
        assert_eq!(got.len(), 1);
        assert_eq!(buf, "data: {\"typ");
    }

    #[test]
    fn flush_drains_multiple_frames() {
        let mut buf = String::from(
            "data: {\"type\":\"token\",\"text\":\"a\"}\n\ndata: {\"type\":\"token\",\"text\":\"b\"}\n\n",
        );
        let mut got = Vec::new();
        flush_complete_frames(&mut buf, &mut |e| got.push(e));
        assert_eq!(got.len(), 2);
        assert!(buf.is_empty());
    }
}
