//! Helpers that serialise an `AgentChunk` into a chat-completion
//! `data: ...` SSE frame matching the OpenAI streaming spec.
//!
//! Kept separate from `sse.rs` so the SSE primitives stay generic
//! across chat-completions / responses / runs surfaces.

use super::types::Usage;
use axum::response::sse::Event;
use serde_json::json;

/// `data: { delta: { content } }` chunk shape used while streaming.
pub fn content_chunk(comp_id: &str, model: &str, created: u64, text: &str) -> Event {
    let body = json!({
        "id": comp_id,
        "object": "chat.completion.chunk",
        "created": created,
        "model": model,
        "choices": [{
            "index": 0,
            "delta": { "content": text },
            "finish_reason": null,
        }],
    });
    Event::default().data(body.to_string())
}

/// Final chunk: empty delta + `finish_reason: stop` + usage block.
pub fn finish_chunk(comp_id: &str, model: &str, created: u64, usage: Usage) -> Event {
    let body = json!({
        "id": comp_id,
        "object": "chat.completion.chunk",
        "created": created,
        "model": model,
        "choices": [{
            "index": 0,
            "delta": {},
            "finish_reason": "stop",
        }],
        "usage": {
            "prompt_tokens": usage.prompt_tokens,
            "completion_tokens": usage.completion_tokens,
            "total_tokens": usage.total_tokens,
        },
    });
    Event::default().data(body.to_string())
}

/// `data: [DONE]` sentinel — emitted after the finish chunk so OpenAI
/// SDK clients release the stream.
pub fn done_sentinel() -> Event {
    Event::default().data("[DONE]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_chunk_compiles() {
        let _ = content_chunk("c1", "kei-cortex", 0, "hello");
    }

    #[test]
    fn finish_chunk_compiles() {
        let _ = finish_chunk("c1", "kei-cortex", 0, Usage::default());
    }

    #[test]
    fn done_sentinel_compiles() {
        let _ = done_sentinel();
    }
}
