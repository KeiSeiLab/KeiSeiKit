//! JSON-RPC 2.0 envelope + MCP method enum + per-request server context.
//!
//! MCP method names are dotted/slash-delimited per spec
//! (`tools/list`, `tools/call`, `resources/list`, `resources/read`,
//! `prompts/list`, `prompts/get`, `initialize`).

use kei_skills::SkillRegistry;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

/// JSON-RPC 2.0 request envelope. `id` is `Value` (number or string per spec).
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    #[serde(default)]
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Option<Value>,
}

/// JSON-RPC 2.0 response envelope. Either `result` or `error`, never both.
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC 2.0 error object — `code` + `message` + optional `data`.
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// MCP method enum — strings normalised to spec wire names.
#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    Initialize,
    ToolsList,
    ToolsCall,
    ResourcesList,
    ResourcesRead,
    PromptsList,
    PromptsGet,
    Other(String),
}

impl Method {
    /// Map a wire string to the typed enum. Unknown methods become `Other(s)`.
    pub fn parse(s: &str) -> Self {
        match s {
            "initialize" => Self::Initialize,
            "tools/list" => Self::ToolsList,
            "tools/call" => Self::ToolsCall,
            "resources/list" => Self::ResourcesList,
            "resources/read" => Self::ResourcesRead,
            "prompts/list" => Self::PromptsList,
            "prompts/get" => Self::PromptsGet,
            other => Self::Other(other.to_string()),
        }
    }
}

/// Per-server context shared across requests. Holds the atom-registry root
/// and a kei-skills `SkillRegistry` (canonical SSoT for skills — replaces
/// the prior raw walkdir scan). Built once at server boot. Re-walks happen
/// only via `SkillRegistry::reload()`; `initialize` does not re-scan.
///
/// HERMES-MIGRATION-PLAN P3.1.b — kei-mcp is the FIRST production consumer
/// of kei-skills. Until this wire-up landed, the crate was a leaf with no
/// callers (audit verdict 30% functional). Now `resources/list` and
/// `resources/read` flow through the validated registry.
pub struct ServerContext {
    pub atoms_root: PathBuf,
    pub skills_root: PathBuf,
    pub skills_registry: SkillRegistry,
    pub server_name: String,
    pub server_version: String,
}

impl ServerContext {
    pub fn new(atoms_root: PathBuf, skills_root: PathBuf) -> Self {
        let skills_registry = SkillRegistry::new(&skills_root);
        Self {
            atoms_root,
            skills_root,
            skills_registry,
            server_name: "kei-mcp".into(),
            server_version: env!("CARGO_PKG_VERSION").into(),
        }
    }
}

/// Standard JSON-RPC error codes per spec.
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
pub const INTERNAL_ERROR: i32 = -32603;

/// Build a `result`-shaped response for a known id.
pub fn ok(id: Option<Value>, result: Value) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".into(),
        id,
        result: Some(result),
        error: None,
    }
}

/// Build an `error`-shaped response.
pub fn err(id: Option<Value>, code: i32, message: impl Into<String>) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".into(),
        id,
        result: None,
        error: Some(JsonRpcError {
            code,
            message: message.into(),
            data: None,
        }),
    }
}
