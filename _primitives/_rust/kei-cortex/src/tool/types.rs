//! Tool primitive types — `ToolCall`, `ToolResult`, `ToolError`.
//!
//! These mirror the Anthropic Messages API `tool_use` / `tool_result`
//! content-block shape. A `ToolCall` is what the model emits and what we
//! dispatch on; a `ToolResult` is what we hand back in the next turn.
//!
//! Constructor Pattern: type-only module, no I/O, no state. The actual
//! executor functions live in sibling cubes (`read.rs`, `write.rs`, etc.).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// One tool invocation requested by the model. Mirrors the
/// `content_block.type = "tool_use"` payload from Anthropic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Anthropic-side block id, echoed back verbatim in `tool_result`.
    pub id: String,
    /// The tool name (matches a registry key).
    pub name: String,
    /// Tool-specific input arguments (validated per tool).
    pub input: Value,
}

/// One tool result we hand back to the model. Goes into the next
/// `messages.create` body as a `tool_result` content block under a `user`
/// message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Echo of the originating `ToolCall.id`.
    pub tool_use_id: String,
    /// Plain text content. Anthropic accepts a string OR a content-block
    /// list; we always emit a string for simplicity.
    pub content: String,
    /// True when the executor errored. Tells the model "your call failed,
    /// recover" rather than "here is data".
    pub is_error: bool,
}

impl ToolResult {
    /// Build a successful result.
    pub fn ok(tool_use_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            tool_use_id: tool_use_id.into(),
            content: content.into(),
            is_error: false,
        }
    }

    /// Build an error result.
    pub fn err(tool_use_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            tool_use_id: tool_use_id.into(),
            content: message.into(),
            is_error: true,
        }
    }
}

/// Errors produced by tool executors. Distinguished from `AppError` so
/// they never escape the tool loop — they always become a `ToolResult`
/// with `is_error = true` so the model can recover.
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("path denied by sandbox: {0}")]
    PathDenied(String),
    #[error("path outside project_root: {0}")]
    OutsideRoot(String),
    #[error("command denied by sandbox: {0}")]
    CommandDenied(String),
    #[error("shell tokenize error: {0}")]
    ShellParse(String),
    #[error("size limit exceeded: {0}")]
    TooLarge(String),
    #[error("execution timed out")]
    Timeout,
    #[error("unique-match constraint failed: {0}")]
    NotUnique(String),
    #[error("io: {0}")]
    Io(String),
    #[error("internal: {0}")]
    Internal(String),
}

impl From<std::io::Error> for ToolError {
    fn from(e: std::io::Error) -> Self {
        ToolError::Io(e.to_string())
    }
}

impl ToolError {
    /// Render as the `content` of a `ToolResult { is_error: true }`.
    pub fn as_message(&self) -> String {
        format!("{self}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_sets_is_error_false() {
        let r = ToolResult::ok("tu_1", "hi");
        assert!(!r.is_error);
        assert_eq!(r.tool_use_id, "tu_1");
    }

    #[test]
    fn err_sets_is_error_true() {
        let r = ToolResult::err("tu_2", "boom");
        assert!(r.is_error);
        assert_eq!(r.content, "boom");
    }

    #[test]
    fn tool_error_renders_message() {
        let e = ToolError::PathDenied("/etc/passwd".into());
        assert!(e.as_message().contains("/etc/passwd"));
    }

    #[test]
    fn io_error_converts() {
        let io = std::io::Error::new(std::io::ErrorKind::NotFound, "x");
        let e: ToolError = io.into();
        assert!(matches!(e, ToolError::Io(_)));
    }

    #[test]
    fn tool_call_round_trips() {
        let raw = serde_json::json!({"id":"tu","name":"read","input":{"path":"/tmp/x"}});
        let c: ToolCall = serde_json::from_value(raw).unwrap();
        assert_eq!(c.name, "read");
    }
}
