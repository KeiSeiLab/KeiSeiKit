//! Validation + session-id parsing + response-building helpers for
//! `chat_completions.rs`. Split out so that handler file stays under
//! the 200-LOC Constructor-Pattern ceiling.

use super::error::OpenAiError;
use super::ids::chat_completion_id;
use super::session::SessionStore;
use super::types::{
    ChatCompletionChoice, ChatCompletionRequest, ChatCompletionResponse, ChatMessage, Usage,
};
use axum::http::HeaderMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub const SESSION_HEADER: &str = "X-Kei-Session-Id";

/// Reject empty model / empty messages with a 400.
pub fn validate(req: &ChatCompletionRequest) -> Result<(), OpenAiError> {
    if req.messages.is_empty() {
        return Err(OpenAiError::BadRequest("messages must not be empty".into()));
    }
    if req.model.trim().is_empty() {
        return Err(OpenAiError::BadRequest("model is required".into()));
    }
    Ok(())
}

/// Pull the optional `X-Kei-Session-Id` header. Empty / whitespace
/// values are treated as absent.
pub fn session_id_from_headers(h: &HeaderMap) -> Option<String> {
    h.get(SESSION_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Concatenate prior session messages + the request's messages.
pub fn collect_session_messages(
    store: &SessionStore,
    session_id: &Option<String>,
    req: &ChatCompletionRequest,
) -> Vec<ChatMessage> {
    let mut msgs = Vec::new();
    if let Some(id) = session_id {
        if let Some(rec) = store.get(id) {
            msgs.extend(rec.messages);
        }
    }
    msgs.extend(req.messages.clone());
    msgs
}

/// Persist {request_messages, assistant_reply} into the session if a
/// session id was supplied. Stateless calls are a no-op.
pub fn persist_turn(
    store: &SessionStore,
    session_id: &Option<String>,
    request_msgs: &[ChatMessage],
    assistant: &ChatMessage,
) {
    if let Some(id) = session_id {
        let mut to_append = request_msgs.to_vec();
        to_append.push(assistant.clone());
        store.append(id, to_append);
    }
}

/// Build the non-stream `ChatCompletionResponse` envelope. `usage`
/// flows in from `agent_runner::collect_reply`'s captured TokenUsage
/// (N-1 sync fix); pass `Usage::default()` only for stub paths.
pub fn build_completion_response(
    model: String,
    msg: ChatMessage,
    usage: Usage,
) -> ChatCompletionResponse {
    ChatCompletionResponse {
        id: chat_completion_id(),
        object: "chat.completion",
        created: unix_secs(),
        model,
        choices: vec![ChatCompletionChoice {
            index: 0,
            message: msg,
            finish_reason: "stop".into(),
        }],
        usage,
    }
}

/// Phase-1.1 placeholder agent reply. Phase 1.1.b/c/d replaced this with
/// `agent_runner::collect_reply` / `agent_runner::stream_events` wired
/// into `tool::run_with_tools`. Kept around (deprecated, private,
/// dead-code-allowed) only for the unit test below that pins the legacy
/// envelope's shape — production paths no longer call this.
#[deprecated(note = "use openai::agent_runner::collect_reply (real loop)")]
#[allow(dead_code)]
fn stub_agent_reply(prompt: &str) -> String {
    let head: String = prompt.chars().take(120).collect();
    format!("[kei-cortex stub] echo: {head}")
}

pub fn unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_req() -> ChatCompletionRequest {
        ChatCompletionRequest {
            model: "kei-cortex".into(),
            messages: vec![ChatMessage {
                role: "user".into(),
                content: Some("hi".into()),
                name: None,
                tool_call_id: None,
                tool_calls: None,
            }],
            stream: false,
            temperature: None,
            max_tokens: None,
            tools: None,
            tool_choice: None,
        }
    }

    #[test]
    fn validate_rejects_empty_messages() {
        let mut r = base_req();
        r.messages.clear();
        assert!(validate(&r).is_err());
    }

    #[test]
    fn session_header_parsed_when_present() {
        let mut h = HeaderMap::new();
        h.insert(SESSION_HEADER, "sess_42".parse().unwrap());
        assert_eq!(session_id_from_headers(&h).as_deref(), Some("sess_42"));
    }
}
