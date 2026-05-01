//! OpenAI 2024-10-01 wire-format DTOs for /v1/* endpoints.
//!
//! Constructor Pattern: ONE responsibility — serde shapes. No business
//! logic, no IO. Names match the OpenAI JSON keys verbatim so frontends
//! (Open WebUI, LobeChat, LibreChat, …) deserialise without translation.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// POST /v1/chat/completions request body.
#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(default)]
    pub stream: bool,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub max_tokens: Option<u32>,
    #[serde(default)]
    pub tools: Option<Vec<OpenAiTool>>,
    #[serde(default)]
    pub tool_choice: Option<Value>,
}

/// One chat-message turn. `role` ∈ {system, user, assistant, tool}.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

/// OpenAI tool descriptor (function-calling schema).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiTool {
    #[serde(rename = "type")]
    pub kind: String,
    pub function: OpenAiToolFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiToolFunction {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
}

/// Tool-call as emitted in `assistant.tool_calls[]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub function: ToolCallFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallFunction {
    pub name: String,
    /// JSON-serialised arguments string (OpenAI quirk — not an object).
    pub arguments: String,
}

/// Non-stream response body.
#[derive(Debug, Clone, Serialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: &'static str,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: Usage,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatCompletionChoice {
    pub index: u32,
    pub message: ChatMessage,
    pub finish_reason: String,
}

/// Token-usage block. Names match OpenAI exactly.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

/// POST /v1/responses request body. Stateful via `previous_response_id`.
#[derive(Debug, Clone, Deserialize)]
pub struct ResponsesRequest {
    pub model: String,
    pub input: Value,
    #[serde(default)]
    pub previous_response_id: Option<String>,
    #[serde(default)]
    pub stream: bool,
    #[serde(default)]
    pub instructions: Option<String>,
}

/// Stored / returned response object.
#[derive(Debug, Clone, Serialize)]
pub struct ResponseObject {
    pub id: String,
    pub object: &'static str,
    pub created_at: u64,
    pub model: String,
    pub status: String,
    pub output: Vec<Value>,
    pub previous_response_id: Option<String>,
    pub usage: Usage,
}

/// POST /v1/runs request — accepted (202) and processed asynchronously.
#[derive(Debug, Clone, Deserialize)]
pub struct RunRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(default)]
    pub metadata: Option<Value>,
}

/// 202 body returned from POST /v1/runs.
#[derive(Debug, Clone, Serialize)]
pub struct RunObject {
    pub id: String,
    pub object: &'static str,
    pub created_at: u64,
    pub status: String,
    pub model: String,
}

/// GET /v1/models response item.
#[derive(Debug, Clone, Serialize)]
pub struct ModelObject {
    pub id: String,
    pub object: &'static str,
    pub created: u64,
    pub owned_by: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct ModelList {
    pub object: &'static str,
    pub data: Vec<ModelObject>,
}

/// OpenAI-style error envelope `{ "error": { message, type, code } }`.
#[derive(Debug, Clone, Serialize)]
pub struct OpenAiErrorBody {
    pub error: OpenAiErrorInner,
}

#[derive(Debug, Clone, Serialize)]
pub struct OpenAiErrorInner {
    pub message: String,
    #[serde(rename = "type")]
    pub kind: &'static str,
    pub code: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_request_round_trips() {
        let raw = r#"{"model":"kei-cortex","messages":[{"role":"user","content":"hi"}]}"#;
        let req: ChatCompletionRequest = serde_json::from_str(raw).unwrap();
        assert_eq!(req.model, "kei-cortex");
        assert_eq!(req.messages.len(), 1);
        assert_eq!(req.messages[0].role, "user");
    }

    #[test]
    fn usage_serializes_with_openai_keys() {
        let u = Usage { prompt_tokens: 1, completion_tokens: 2, total_tokens: 3 };
        let s = serde_json::to_string(&u).unwrap();
        assert!(s.contains("\"prompt_tokens\":1"));
        assert!(s.contains("\"completion_tokens\":2"));
    }
}
