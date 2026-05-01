//! Anthropic-backed implementation of the memory-review `Invoker` trait.
//!
//! Constructor Pattern: this cube owns ONE responsibility — execute the
//! review prompt against Anthropic Messages and return the assistant
//! reply as a single `String`. No tool-use, no streaming. Tools are
//! intentionally absent: the review prompt forbids further actions and
//! either emits `Nothing to save.` or a short paragraph. Adding tool
//! support here would re-introduce the loop driver and break the
//! ≤200-LOC budget. The persistence step happens in `memory_persist`.
//!
//! Wiring contract:
//!   * Caller provides a `system` string (typically `REVIEW_PROMPT`).
//!   * Each `invoke()` call snapshot-renders the conversation as
//!     `Message` rows and POSTs once to `/v1/messages`.
//!   * Errors (network, 4xx/5xx, missing API key) collapse to a stable
//!     short string so the scheduler can log without panicking.
//!
//! Tradeoff: one HTTP call per review (~500ms tail). The scheduler
//! cool-down (60s) caps cost; reviews fire ≤1/min/session.

use std::pin::Pin;
use std::time::Duration;

use serde_json::{json, Value};
use tokio::time::timeout;

use crate::anthropic::{default_model, endpoint, API_VERSION};

use super::memory_nudge::Turn;
use super::memory_review_task::Invoker;

/// HTTP budget for a single review call. Mirrors the streaming-handshake
/// budget — review responses are short (one short paragraph) so 60s is
/// generous; we still keep the same tail-of-distribution cap as
/// `anthropic_invoker.rs` for predictability.
const REVIEW_BUDGET: Duration = Duration::from_secs(60);

/// Concrete Invoker that POSTs to Anthropic Messages. Captures the
/// system prompt (review template) by value so a fresh struct can be
/// reused across sessions without re-reading the template each time.
pub struct AnthropicMemoryInvoker {
    system: String,
}

impl AnthropicMemoryInvoker {
    /// Build a new invoker bound to a fixed system/review prompt.
    pub fn new(system: String) -> Self {
        Self { system }
    }
}

impl Invoker for AnthropicMemoryInvoker {
    fn invoke(
        &self,
        snapshot: Vec<Turn>,
        prompt: String,
    ) -> Pin<Box<dyn std::future::Future<Output = String> + Send + '_>> {
        let system = self.system.clone();
        Box::pin(async move { run_review_call(&system, snapshot, prompt).await })
    }
}

/// Single round-trip to Anthropic. Errors collapse to a stable
/// short-circuit string so callers don't need to discriminate kinds —
/// the scheduler treats any non-`Nothing to save.` reply as a
/// memory-write candidate, and treats explicit error-prefixed strings
/// as no-ops via `is_error_reply`.
pub async fn run_review_call(system: &str, snapshot: Vec<Turn>, prompt: String) -> String {
    let api_key = match std::env::var("ANTHROPIC_API_KEY") {
        Ok(k) if !k.is_empty() => k,
        _ => return error_reply("missing ANTHROPIC_API_KEY"),
    };
    let body = build_review_body(system, &snapshot, &prompt);
    match timeout(REVIEW_BUDGET, send_review(&api_key, &body)).await {
        Ok(Ok(text)) => text,
        Ok(Err(e)) => error_reply(&e),
        Err(_) => error_reply("timeout"),
    }
}

/// JSON body for the review request. Snapshot turns are mapped to
/// `user`/`assistant` content; the review prompt becomes the trailing
/// `user` message that asks the model to summarise. The `system` slot
/// holds the parent agent's persona — kept stable across reviews so
/// the model knows the voice it's reviewing.
fn build_review_body(system: &str, snapshot: &[Turn], prompt: &str) -> Value {
    let mut messages: Vec<Value> = snapshot
        .iter()
        .filter(|t| matches!(t.role.as_str(), "user" | "assistant"))
        .map(|t| json!({"role": t.role, "content": t.content}))
        .collect();
    messages.push(json!({"role": "user", "content": prompt}));
    json!({
        "model": default_model().as_ref(),
        "max_tokens": 256,
        "system": system,
        "messages": messages,
    })
}

/// POST and extract the first text block from the response. Any HTTP
/// error or shape mismatch surfaces as `Err(message)` so the caller
/// converts into an error reply.
async fn send_review(api_key: &str, body: &Value) -> Result<String, String> {
    let resp = reqwest::Client::new()
        .post(endpoint().as_ref())
        .header("x-api-key", api_key)
        .header("anthropic-version", API_VERSION)
        .header("content-type", "application/json")
        .json(body)
        .send()
        .await
        .map_err(|e| format!("http: {e}"))?;
    let resp = resp
        .error_for_status()
        .map_err(|e| format!("status: {e}"))?;
    let raw: Value = resp
        .json()
        .await
        .map_err(|e| format!("json: {e}"))?;
    extract_first_text(&raw).ok_or_else(|| "missing text block".to_string())
}

/// Walk `content` looking for the first `{"type":"text","text":...}`
/// block. Anthropic guarantees `content` is a non-empty array on a
/// successful Messages call; we still handle the absent case so a
/// future API change downgrades to an error reply, not a panic.
fn extract_first_text(raw: &Value) -> Option<String> {
    let arr = raw.get("content")?.as_array()?;
    for block in arr {
        if block.get("type").and_then(|v| v.as_str()) == Some("text") {
            if let Some(t) = block.get("text").and_then(|v| v.as_str()) {
                return Some(t.to_string());
            }
        }
    }
    None
}

/// Stable prefix the persist step can detect via `is_error_reply` so a
/// transport failure doesn't accidentally land in the memory store as
/// a faux user fact.
pub const ERROR_PREFIX: &str = "[memory-review-error] ";

fn error_reply(msg: &str) -> String {
    format!("{ERROR_PREFIX}{msg}")
}

/// True when the reply is a transport-error placeholder rather than
/// real model output. Persist step uses this to skip writes.
pub fn is_error_reply(reply: &str) -> bool {
    reply.starts_with(ERROR_PREFIX)
}

#[cfg(test)]
#[path = "anthropic_memory_invoker_test.rs"]
mod tests;
