//! Thin reqwest client for Anthropic Messages API (streaming mode).
//!
//! `open_stream` performs the HTTP handshake synchronously then returns a
//! `Stream<Item = Result<String, Error>>` of text deltas extracted from
//! `content_block_delta` frames. Non-text events are skipped.
//! API key is read from `ANTHROPIC_API_KEY` at call time (env rotation-friendly).
//!
//! Reliability envelope:
//!   - `BUDGET` (120 s) caps the HTTP handshake (`open_stream`).
//!   - `IDLE` (30 s) caps the gap between individual SSE chunks; silent
//!     streams are surfaced as `Error::Timeout` so the handler can emit
//!     an SSE error event rather than hanging the client.

use crate::anthropic_sse::SseParser;
use async_stream::try_stream;
use futures::stream::Stream;
use futures::StreamExt;
use serde::Serialize;
use std::time::Duration;
use tokio::time::timeout;

pub use crate::anthropic_config::{
    default_model, endpoint, API_VERSION, ENDPOINT, MODEL_FALLBACK,
};

/// Overall HTTP-handshake budget. Past this point we give up even if the
/// upstream accepted the TCP connection but never sent headers.
pub const BUDGET: Duration = Duration::from_secs(120);

/// Per-chunk idle budget. If the stream goes silent for this long we bail
/// instead of holding the SSE client open forever.
pub const IDLE: Duration = Duration::from_secs(30);

/// Cap on upstream error bodies we propagate. Prevents Anthropic echoing a
/// large error page into our logs or client.
const BODY_PREVIEW_CAP: usize = 512;

/// A single turn in the conversation.
#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Client errors surfaced to the caller.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("ANTHROPIC_API_KEY not set")]
    MissingKey,
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("upstream status {status}: {body}")]
    Upstream { status: u16, body: String },
    #[error("upstream rate limit")]
    RateLimit,
    #[error("upstream service unavailable")]
    ServiceUnavailable,
    #[error("upstream timeout")]
    Timeout,
}

/// Open a streaming Messages request. Returns the async stream of text deltas
/// AFTER the upstream HTTP handshake completes successfully — so callers can
/// map 429/5xx to proper status codes before any SSE framing is emitted.
pub async fn open_stream(
    system: &str,
    messages: &[Message],
) -> Result<impl Stream<Item = Result<String, Error>> + Send + 'static, Error> {
    let api_key = std::env::var("ANTHROPIC_API_KEY").map_err(|_| Error::MissingKey)?;
    let body = build_body(system, messages);
    let fut = send_request(&api_key, &body);
    let resp = match timeout(BUDGET, fut).await {
        Ok(r) => r?,
        Err(_) => return Err(Error::Timeout),
    };
    Ok(body_to_text_stream(resp))
}

/// Turn a validated streaming response into a text-delta stream.
fn body_to_text_stream(
    resp: reqwest::Response,
) -> impl Stream<Item = Result<String, Error>> + Send + 'static {
    try_stream! {
        let mut parser = SseParser::new();
        let mut bytes_stream = resp.bytes_stream();
        loop {
            let next = timeout(IDLE, bytes_stream.next()).await;
            let chunk_opt = match next {
                Ok(x) => x,
                Err(_) => Err(Error::Timeout)?,
            };
            let Some(chunk) = chunk_opt else { break };
            let chunk = chunk.map_err(Error::Http)?;
            for text in parser.push(&chunk) {
                yield text;
            }
        }
    }
}

/// Build the JSON request body.
fn build_body(system: &str, messages: &[Message]) -> serde_json::Value {
    serde_json::json!({
        "model": default_model().as_ref(),
        "max_tokens": 1024,
        "system": system,
        "stream": true,
        "messages": messages,
    })
}

/// Fire the POST request with the right headers; map HTTP errors to `Error`.
async fn send_request(
    api_key: &str,
    body: &serde_json::Value,
) -> Result<reqwest::Response, Error> {
    let client = reqwest::Client::new();
    let resp = client
        .post(endpoint().as_ref())
        .header("x-api-key", api_key)
        .header("anthropic-version", API_VERSION)
        .header("content-type", "application/json")
        .json(body)
        .send()
        .await?;
    check_status(resp).await
}

/// Turn non-2xx responses into structured `Error` values. 429 → RateLimit,
/// 503/529 → ServiceUnavailable, remaining 4xx/5xx → Upstream with body
/// truncated at `BODY_PREVIEW_CAP` bytes so we never propagate a megabyte
/// of upstream HTML.
async fn check_status(resp: reqwest::Response) -> Result<reqwest::Response, Error> {
    let status = resp.status();
    if status.is_success() {
        return Ok(resp);
    }
    let code = status.as_u16();
    if code == 429 {
        return Err(Error::RateLimit);
    }
    if code == 503 || code == 529 {
        return Err(Error::ServiceUnavailable);
    }
    let body = resp.text().await.unwrap_or_default();
    Err(Error::Upstream {
        status: code,
        body: truncate(&body, BODY_PREVIEW_CAP),
    })
}

/// Cap a string at `max` bytes on a char boundary. Used for error previews
/// so we never log unbounded upstream content.
fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut end = max;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    s[..end].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_caps_long_strings() {
        let long = "a".repeat(10_000);
        assert_eq!(truncate(&long, 256).len(), 256);
    }

    #[test]
    fn truncate_leaves_short_strings() {
        assert_eq!(truncate("hi", 256), "hi");
    }

    #[test]
    fn truncate_respects_char_boundary() {
        // "я" is 2 bytes; ensure we don't slice mid-char.
        let s = "я".repeat(10);
        let out = truncate(&s, 5);
        assert!(out.len() <= 5);
        assert!(out.chars().all(|c| c == 'я'));
    }
}
