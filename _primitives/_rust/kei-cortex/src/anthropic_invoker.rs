//! Non-streaming Anthropic Messages call with tool-use support.
//! `tool::run_with_tools` needs a `ModelInvoker` returning a full `ModelTurn`
//! (content blocks + stop_reason); the streaming `anthropic::open_stream` is
//! text-only. This module POSTs once per turn and parses Anthropic's
//! multi-block content (`text` + `tool_use`) into `Vec<ContentBlock>`.

use crate::anthropic::{default_model, endpoint, API_VERSION};
use crate::tool::loop_driver::{
    ContentBlock, ConversationMessage, ModelInvoker, ModelTurn, TokenUsage,
};
use crate::tool::types::ToolCall;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

/// Overall HTTP budget per turn. Anthropic non-streaming responses can be
/// chatty when tools fire; 120s mirrors the streaming-handshake budget.
const REQUEST_BUDGET: Duration = Duration::from_secs(120);

/// Build a `ModelInvoker` that POSTs to Anthropic Messages with tools.
/// `system` is captured by value so it is reused across every turn.
pub fn build_invoker(system: String) -> ModelInvoker {
    Arc::new(move |messages, tool_defs| {
        let system = system.clone();
        Box::pin(async move { invoke(&system, messages, tool_defs).await })
    })
}

/// Single round-trip to Anthropic Messages with tools. Errors stringify so
/// they fit the `Result<ModelTurn, String>` invoker contract.
async fn invoke(
    system: &str,
    messages: Vec<ConversationMessage>,
    tool_defs: Vec<Value>,
) -> Result<ModelTurn, String> {
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .map_err(|_| "ANTHROPIC_API_KEY not set".to_string())?;
    let body = build_body(system, &messages, &tool_defs);
    let resp = match timeout(REQUEST_BUDGET, send(&api_key, &body)).await {
        Ok(Ok(r)) => r,
        Ok(Err(e)) => return Err(format!("anthropic request: {e}")),
        Err(_) => return Err("anthropic request: timeout".into()),
    };
    let raw: Value = resp
        .json()
        .await
        .map_err(|e| format!("anthropic body json: {e}"))?;
    parse_turn(&raw)
}

/// Build the JSON body. Local messages map to API messages: User→user,
/// Assistant→assistant w/ blocks, Tool→user w/ tool_result blocks.
fn build_body(system: &str, messages: &[ConversationMessage], tool_defs: &[Value]) -> Value {
    json!({
        "model": default_model().as_ref(),
        "max_tokens": 1024,
        "system": system,
        "tools": tool_defs,
        "messages": render_messages(messages),
    })
}

/// Translate the local conversation history into the Anthropic API shape.
fn render_messages(messages: &[ConversationMessage]) -> Vec<Value> {
    messages.iter().map(render_one).collect()
}

fn render_one(m: &ConversationMessage) -> Value {
    match m {
        ConversationMessage::User(text) => json!({"role": "user", "content": text}),
        ConversationMessage::Assistant(blocks) => {
            json!({"role": "assistant", "content": render_assistant_blocks(blocks)})
        }
        ConversationMessage::Tool(results) => {
            let content: Vec<Value> = results
                .iter()
                .map(|r| {
                    json!({
                        "type": "tool_result",
                        "tool_use_id": r.tool_use_id,
                        "content": r.content,
                        "is_error": r.is_error,
                    })
                })
                .collect();
            json!({"role": "user", "content": content})
        }
    }
}

fn render_assistant_blocks(blocks: &[ContentBlock]) -> Vec<Value> {
    blocks
        .iter()
        .map(|b| match b {
            ContentBlock::Text(t) => json!({"type": "text", "text": t}),
            ContentBlock::ToolUse(c) => {
                json!({"type": "tool_use", "id": c.id, "name": c.name, "input": c.input})
            }
        })
        .collect()
}

/// Fire the POST request and surface 4xx/5xx as a string error.
async fn send(api_key: &str, body: &Value) -> Result<reqwest::Response, reqwest::Error> {
    let resp = reqwest::Client::new()
        .post(endpoint().as_ref())
        .header("x-api-key", api_key)
        .header("anthropic-version", API_VERSION)
        .header("content-type", "application/json")
        .json(body)
        .send()
        .await?;
    resp.error_for_status()
}

/// Parse the response body into a `ModelTurn`. Unknown content-block types
/// are ignored (forward-compat with new Anthropic block kinds). The `usage`
/// block is optional; when absent, downstream cost recording falls back to
/// a zero-cents row rather than failing the turn.
fn parse_turn(raw: &Value) -> Result<ModelTurn, String> {
    let stop_reason = raw
        .get("stop_reason")
        .and_then(|v| v.as_str())
        .unwrap_or("end_turn")
        .to_string();
    let content_arr = raw
        .get("content")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "anthropic response: missing content array".to_string())?;
    let blocks: Vec<ContentBlock> = content_arr.iter().filter_map(parse_block).collect();
    let usage = parse_usage(raw);
    Ok(ModelTurn { content: blocks, stop_reason, usage })
}

/// Pull `usage.input_tokens` + `usage.output_tokens` out of the Anthropic
/// response body. Returns `None` when the block is absent or both fields
/// are missing — keeps the existing `Default` impl from leaking into a
/// "real, but zero" usage row that would mis-classify the cost.
fn parse_usage(raw: &Value) -> Option<TokenUsage> {
    let u = raw.get("usage")?;
    let input = u.get("input_tokens").and_then(|v| v.as_u64()).unwrap_or(0);
    let output = u.get("output_tokens").and_then(|v| v.as_u64()).unwrap_or(0);
    if input == 0 && output == 0 {
        return None;
    }
    Some(TokenUsage { input_tokens: input, output_tokens: output })
}

fn parse_block(b: &Value) -> Option<ContentBlock> {
    let kind = b.get("type")?.as_str()?;
    match kind {
        "text" => Some(ContentBlock::Text(b.get("text")?.as_str()?.to_string())),
        "tool_use" => Some(ContentBlock::ToolUse(ToolCall {
            id: b.get("id")?.as_str()?.to_string(),
            name: b.get("name")?.as_str()?.to_string(),
            input: b.get("input").cloned().unwrap_or(Value::Null),
        })),
        _ => None,
    }
}

#[cfg(test)]
#[path = "anthropic_invoker_test.rs"]
mod tests;
