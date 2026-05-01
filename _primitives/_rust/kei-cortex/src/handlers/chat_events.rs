//! Single-event SSE constructors for the chat handler.
//!
//! Constructor Pattern: extracted from `chat.rs` so the parent file
//! stays under the 200-LOC ceiling now that Wave 40 added the cost
//! recording side-channel. Each function maps one shape of internal
//! event into the JSON payload axum's `Sse` will frame.

use crate::sentiment;
use axum::response::sse::Event;
use serde_json::json;

pub fn token_event(text: &str) -> Event {
    Event::default().data(json!({"type": "token", "text": text}).to_string())
}

pub fn tool_use_start_event(name: &str, input: &serde_json::Value) -> Event {
    Event::default()
        .data(json!({"type": "tool_use_start", "name": name, "input": input}).to_string())
}

pub fn tool_use_result_event(tool_use_id: &str, is_error: bool) -> Event {
    Event::default().data(
        json!({"type": "tool_result", "id": tool_use_id, "is_error": is_error}).to_string(),
    )
}

pub fn error_event(message: &str) -> Event {
    Event::default().data(json!({"type": "error", "message": message}).to_string())
}

pub fn sentiment_event(accumulated: &str) -> Event {
    let s = sentiment::classify(accumulated);
    Event::default()
        .data(json!({"type": "sentiment", "tag": s.tag, "confidence": s.confidence}).to_string())
}

pub fn done_event(conversation_id: &str) -> Event {
    Event::default()
        .data(json!({"type": "done", "conversation_id": conversation_id}).to_string())
}
