//! Ollama HTTP API wire types.
//!
//! Schema source: <https://github.com/ollama/ollama/blob/main/docs/api.md>
//! Pinned against Ollama v0.x — schema is stable across patch releases.
//! Verified live against running daemon v0.21.2 at 127.0.0.1:11434.

use serde::{Deserialize, Serialize};

/// Chat message — matches `/api/chat` and `/api/generate` schema.
/// `role` is one of `"system"` | `"user"` | `"assistant"`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// `/api/tags` GET response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TagsResp {
    pub models: Vec<ModelEntry>,
}

/// One installed model in the tags list.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelEntry {
    pub name: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub modified_at: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub digest: String,
    #[serde(default)]
    pub details: Option<ModelDetails>,
}

/// Optional details block emitted by Ollama for installed models.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelDetails {
    #[serde(default)]
    pub parent_model: String,
    #[serde(default)]
    pub format: String,
    #[serde(default)]
    pub family: String,
    #[serde(default)]
    pub families: Vec<String>,
    #[serde(default)]
    pub parameter_size: String,
    #[serde(default)]
    pub quantization_level: String,
}

/// `/api/generate` POST request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateReq {
    pub model: String,
    pub prompt: String,
    #[serde(default = "default_false")]
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}

/// `/api/generate` non-stream response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GenerateResp {
    pub model: String,
    #[serde(default)]
    pub created_at: String,
    pub response: String,
    pub done: bool,
    #[serde(default)]
    pub eval_count: Option<u64>,
    #[serde(default)]
    pub eval_duration: Option<u64>,
}

/// `/api/chat` POST request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatReq {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(default = "default_false")]
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}

/// `/api/chat` non-stream response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatResp {
    pub model: String,
    #[serde(default)]
    pub created_at: String,
    pub message: Message,
    pub done: bool,
    #[serde(default)]
    pub eval_count: Option<u64>,
    #[serde(default)]
    pub eval_duration: Option<u64>,
}

/// `/api/pull` POST progress line.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullResp {
    pub status: String,
    #[serde(default)]
    pub digest: Option<String>,
    #[serde(default)]
    pub total: Option<u64>,
    #[serde(default)]
    pub completed: Option<u64>,
}

/// `/api/version` GET response (used by health probe).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VersionResp {
    pub version: String,
}

fn default_false() -> bool {
    false
}

/// Build options object for generate/chat from CLI flags.
pub fn build_options(temperature: Option<f32>, max_tokens: Option<u32>) -> Option<serde_json::Value> {
    if temperature.is_none() && max_tokens.is_none() {
        return None;
    }
    let mut map = serde_json::Map::new();
    if let Some(t) = temperature {
        map.insert("temperature".into(), serde_json::json!(t));
    }
    if let Some(n) = max_tokens {
        map.insert("num_predict".into(), serde_json::json!(n));
    }
    Some(serde_json::Value::Object(map))
}
