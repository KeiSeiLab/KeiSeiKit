//! OpenAI tool-call ⇄ kei-cortex tool-name translation.
//!
//! kei-cortex exposes 8 internal tools (read, write, edit, bash, glob,
//! grep, webfetch, agent). OpenAI clients send a `tools[]` array whose
//! `function.name` we accept verbatim if it matches one of those 8 —
//! otherwise we drop the entry and surface a `Warning` header for the
//! frontend to display. We do NOT attempt to alias arbitrary OpenAI
//! function names to our tools; that would mask client bugs.

use super::types::{ChatMessage, OpenAiTool, ToolCall, ToolCallFunction};
use serde_json::Value;

/// The 8 tools kei-cortex's `ToolRegistry::with_project_root` registers.
pub const KEI_TOOLS: [&str; 8] = [
    "read", "write", "edit", "bash", "glob", "grep", "webfetch", "agent",
];

/// Filter the client-supplied `tools[]` array, keeping only entries whose
/// function name maps to a kei-cortex tool. Returns the kept list and the
/// names that were dropped (caller surfaces these in a `Warning` header).
pub fn filter_supported_tools(tools: &[OpenAiTool]) -> (Vec<OpenAiTool>, Vec<String>) {
    let mut kept = Vec::with_capacity(tools.len());
    let mut dropped = Vec::new();
    for t in tools {
        if KEI_TOOLS.contains(&t.function.name.as_str()) {
            kept.push(t.clone());
        } else {
            dropped.push(t.function.name.clone());
        }
    }
    (kept, dropped)
}

/// Build a single `ToolCall` describing a kei-cortex tool invocation,
/// in the OpenAI wire format. `arguments` is JSON-serialised per OpenAI's
/// quirk (it's a string, not an object).
pub fn build_tool_call(call_id: &str, tool_name: &str, args: &Value) -> ToolCall {
    ToolCall {
        id: call_id.to_string(),
        kind: "function".into(),
        function: ToolCallFunction {
            name: tool_name.to_string(),
            arguments: args.to_string(),
        },
    }
}

/// Extract the user-visible prompt from an OpenAI `messages[]` array.
/// We concatenate the `content` fields of `system` and `user` turns so
/// the agent loop sees a single prompt. Tool/assistant turns from prior
/// rounds are passed through unchanged.
pub fn flatten_user_prompt(messages: &[ChatMessage]) -> String {
    let mut out = String::new();
    for m in messages {
        if (m.role == "system" || m.role == "user") && m.content.is_some() {
            if !out.is_empty() {
                out.push_str("\n\n");
            }
            out.push_str(m.content.as_deref().unwrap_or(""));
        }
    }
    out
}

/// Compose the assistant's reply message. Keeps `content` if non-empty
/// AND attaches `tool_calls` if the agent requested any.
pub fn build_assistant_message(content: String, tool_calls: Vec<ToolCall>) -> ChatMessage {
    let content_opt = if content.is_empty() { None } else { Some(content) };
    let tool_calls_opt = if tool_calls.is_empty() {
        None
    } else {
        Some(tool_calls)
    };
    ChatMessage {
        role: "assistant".into(),
        content: content_opt,
        name: None,
        tool_call_id: None,
        tool_calls: tool_calls_opt,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::openai::types::OpenAiToolFunction;

    fn tool(name: &str) -> OpenAiTool {
        OpenAiTool {
            kind: "function".into(),
            function: OpenAiToolFunction {
                name: name.into(),
                description: None,
                parameters: None,
            },
        }
    }

    #[test]
    fn filter_keeps_known_drops_unknown() {
        let tools = vec![tool("read"), tool("unknown"), tool("bash")];
        let (kept, dropped) = filter_supported_tools(&tools);
        assert_eq!(kept.len(), 2);
        assert_eq!(dropped, vec!["unknown".to_string()]);
    }

    #[test]
    fn flatten_concatenates_system_and_user() {
        let msgs = vec![
            ChatMessage {
                role: "system".into(),
                content: Some("be terse".into()),
                name: None, tool_call_id: None, tool_calls: None,
            },
            ChatMessage {
                role: "user".into(),
                content: Some("hi".into()),
                name: None, tool_call_id: None, tool_calls: None,
            },
        ];
        let out = flatten_user_prompt(&msgs);
        assert!(out.contains("be terse"));
        assert!(out.contains("hi"));
    }

    #[test]
    fn build_tool_call_serialises_arguments_as_string() {
        let args = serde_json::json!({"path": "/tmp"});
        let tc = build_tool_call("call_1", "read", &args);
        assert_eq!(tc.function.name, "read");
        assert!(tc.function.arguments.contains("\"path\":\"/tmp\""));
    }
}
