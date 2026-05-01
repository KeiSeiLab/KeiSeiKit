//! Wire types shared between the client and the UI.
//!
//! `ChatEvent` mirrors the SSE frames emitted by
//! `kei-cortex::handlers::chat`. The cortex daemon currently emits four kinds
//! of events: `token`, `sentiment`, `error`, `done`. Two additional variants
//! (`ToolUseStart`, `ToolResult`) are reserved here for forward-compat with
//! the upcoming tool-use streaming pipeline; an unknown event tag yields
//! `ChatEvent::Other` rather than a parse error so the UI keeps draining.

use serde::Deserialize;

/// One parsed SSE event from `/api/v1/cortex/pet/:user_id/chat`.
#[derive(Debug, Clone, PartialEq)]
pub enum ChatEvent {
    /// Incremental text delta from the model.
    Token(String),
    /// Final sentiment classification (post-stream).
    Sentiment { tag: String, confidence: f32 },
    /// Server-side tool invocation has started (forward-compat).
    ToolUseStart { name: String, id: String },
    /// Server-side tool invocation completed (forward-compat).
    ToolResult { id: String, output: String },
    /// Mid-stream error frame; UI surfaces it red and clears `in_flight`.
    Error(String),
    /// Stream finished cleanly. Carries the conversation_id for resume.
    Done { conversation_id: String },
    /// Unknown event tag (forward-compat); UI logs but does not panic.
    Other(String),
}

/// Parse a single `data: { ... }` payload (without the `data:` prefix and
/// without the trailing `\n\n`). Unknown tags become [`ChatEvent::Other`].
///
/// Returns `None` when `payload` is empty (SSE keep-alive comments are
/// filtered earlier; this is a defence in depth).
pub fn parse_event(payload: &str) -> Option<ChatEvent> {
    let trimmed = payload.trim();
    if trimmed.is_empty() {
        return None;
    }
    let v: serde_json::Value = serde_json::from_str(trimmed).ok()?;
    let tag = v.get("type")?.as_str()?;
    Some(match tag {
        "token" => ChatEvent::Token(v.get("text")?.as_str()?.to_string()),
        "sentiment" => ChatEvent::Sentiment {
            tag: v.get("tag")?.as_str().unwrap_or("neutral").to_string(),
            confidence: v.get("confidence").and_then(|c| c.as_f64()).unwrap_or(0.0) as f32,
        },
        "tool_use_start" => ChatEvent::ToolUseStart {
            name: v.get("name").and_then(|n| n.as_str()).unwrap_or("?").to_string(),
            id: v.get("id").and_then(|i| i.as_str()).unwrap_or("").to_string(),
        },
        "tool_result" => ChatEvent::ToolResult {
            id: v.get("id").and_then(|i| i.as_str()).unwrap_or("").to_string(),
            output: v.get("output").and_then(|o| o.as_str()).unwrap_or("").to_string(),
        },
        "error" => ChatEvent::Error(v.get("message")?.as_str()?.to_string()),
        "done" => ChatEvent::Done {
            conversation_id: v
                .get("conversation_id")
                .and_then(|c| c.as_str())
                .unwrap_or("")
                .to_string(),
        },
        other => ChatEvent::Other(other.to_string()),
    })
}

/// Outgoing chat request body (`POST .../chat`).
#[derive(Debug, serde::Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_token_event() {
        let ev = parse_event(r#"{"type":"token","text":"hi"}"#).unwrap();
        assert_eq!(ev, ChatEvent::Token("hi".into()));
    }

    #[test]
    fn parse_sentiment_event() {
        let ev = parse_event(r#"{"type":"sentiment","tag":"happy","confidence":0.93}"#).unwrap();
        match ev {
            ChatEvent::Sentiment { tag, confidence } => {
                assert_eq!(tag, "happy");
                assert!((confidence - 0.93).abs() < 1e-6);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn parse_done_event() {
        let ev = parse_event(r#"{"type":"done","conversation_id":"abc"}"#).unwrap();
        assert_eq!(
            ev,
            ChatEvent::Done {
                conversation_id: "abc".into()
            }
        );
    }

    #[test]
    fn parse_error_event() {
        let ev = parse_event(r#"{"type":"error","message":"boom"}"#).unwrap();
        assert_eq!(ev, ChatEvent::Error("boom".into()));
    }

    #[test]
    fn parse_unknown_tag_falls_back_to_other() {
        let ev = parse_event(r#"{"type":"future_event","payload":42}"#).unwrap();
        assert_eq!(ev, ChatEvent::Other("future_event".into()));
    }

    #[test]
    fn parse_empty_returns_none() {
        assert!(parse_event("").is_none());
        assert!(parse_event("   ").is_none());
    }

    #[test]
    fn parse_invalid_json_returns_none() {
        assert!(parse_event("not json").is_none());
    }
}
