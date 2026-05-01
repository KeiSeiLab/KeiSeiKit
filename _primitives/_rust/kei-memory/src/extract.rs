//! Pull tool_use / tool_result blocks out of a real Claude Code trace line.
//!
//! Constructor Pattern: this cube only walks the JSON shape; classification +
//! persistence live elsewhere. Real trace shape (see ingest.rs::TraceLine):
//!
//!   message.content : array
//!     element {type: "tool_use",   name: <T>, id: <id>, input: {...}}
//!     element {type: "tool_result", tool_use_id: <id>, is_error: bool}
//!     element {type: "text",       text: "..."}
//!
//! Old `tool: <name>` flat field is GONE — it was the schema-mismatch root
//! cause that dropped ~50% of trace lines silently before Wave A.

use chrono::DateTime;
use serde_json::Value;

/// One `tool_use` block extracted from a Claude Code assistant message.
#[derive(Debug, Clone)]
pub struct ToolUse {
    pub name: String,
    pub file_path: Option<String>,
    pub id: Option<String>,
}

/// One `tool_result` block — the user-side counterpart of `ToolUse`.
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub tool_use_id: Option<String>,
    pub is_error: bool,
}

/// Walk `message.content[]`, return every `tool_use` element.
///
/// Returns empty Vec when `message` is None / not an object / has no `content`
/// / `content` is not an array. Never panics on malformed shape.
pub fn extract_tool_uses(message: &Value) -> Vec<ToolUse> {
    let arr = match content_array(message) {
        Some(a) => a,
        None => return Vec::new(),
    };
    arr.iter().filter_map(parse_tool_use).collect()
}

/// Walk `message.content[]`, return the FIRST `tool_result` element if any.
///
/// User lines pair with the assistant's `tool_use` via
/// `tool_result.tool_use_id == tool_use.id`. Used for the `is_error` upgrade
/// in `process_line`. Returns None when no `tool_result` block present.
pub fn extract_tool_result(message: &Value) -> Option<ToolResult> {
    let arr = content_array(message)?;
    arr.iter().find_map(parse_tool_result)
}

fn content_array(message: &Value) -> Option<&Vec<Value>> {
    message.as_object()?.get("content")?.as_array()
}

fn parse_tool_use(elem: &Value) -> Option<ToolUse> {
    let obj = elem.as_object()?;
    if obj.get("type")?.as_str()? != "tool_use" {
        return None;
    }
    Some(ToolUse {
        name: obj.get("name")?.as_str()?.to_string(),
        file_path: tool_use_file_path(obj.get("input")),
        id: obj.get("id").and_then(|v| v.as_str()).map(String::from),
    })
}

fn parse_tool_result(elem: &Value) -> Option<ToolResult> {
    let obj = elem.as_object()?;
    if obj.get("type")?.as_str()? != "tool_result" {
        return None;
    }
    Some(ToolResult {
        tool_use_id: obj.get("tool_use_id").and_then(|v| v.as_str()).map(String::from),
        is_error: obj.get("is_error").and_then(|v| v.as_bool()).unwrap_or(false),
    })
}

/// Best-effort: grab `input.file_path` if present (Edit/Read/Write tools).
fn tool_use_file_path(input: Option<&Value>) -> Option<String> {
    input?
        .as_object()?
        .get("file_path")?
        .as_str()
        .map(String::from)
}

/// Parse an ISO-8601 / RFC-3339 timestamp string to Unix epoch seconds.
///
/// Returns None when:
///   - input is None or empty,
///   - input is not RFC-3339 parseable (do NOT panic — silently skip).
///
/// Real trace examples that MUST parse:
///   "2026-04-30T18:27:10.311Z"
///   "2026-04-30T18:27:10Z"
///   "2026-04-30T18:27:10+02:00"
pub fn parse_timestamp_to_epoch(s: &str) -> Option<i64> {
    if s.is_empty() {
        return None;
    }
    DateTime::parse_from_rfc3339(s).ok().map(|dt| dt.timestamp())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extract_one_tool_use() {
        let msg = json!({
            "role": "assistant",
            "content": [
                {"type": "text", "text": "let me read"},
                {"type": "tool_use", "id": "toolu_1", "name": "Read",
                 "input": {"file_path": "/a.rs"}}
            ]
        });
        let uses = extract_tool_uses(&msg);
        assert_eq!(uses.len(), 1);
        assert_eq!(uses[0].name, "Read");
        assert_eq!(uses[0].file_path.as_deref(), Some("/a.rs"));
        assert_eq!(uses[0].id.as_deref(), Some("toolu_1"));
    }

    #[test]
    fn extract_two_tool_uses_in_one_message() {
        let msg = json!({
            "role": "assistant",
            "content": [
                {"type": "tool_use", "id": "toolu_a", "name": "Bash",
                 "input": {"command": "ls"}},
                {"type": "tool_use", "id": "toolu_b", "name": "Read",
                 "input": {"file_path": "/x.rs"}}
            ]
        });
        let uses = extract_tool_uses(&msg);
        assert_eq!(uses.len(), 2);
        assert_eq!(uses[0].name, "Bash");
        assert_eq!(uses[0].file_path, None);
        assert_eq!(uses[1].name, "Read");
        assert_eq!(uses[1].file_path.as_deref(), Some("/x.rs"));
    }

    #[test]
    fn extract_tool_result_with_error() {
        let msg = json!({
            "role": "user",
            "content": [
                {"type": "tool_result", "tool_use_id": "toolu_1",
                 "is_error": true, "content": "404"}
            ]
        });
        let r = extract_tool_result(&msg).unwrap();
        assert_eq!(r.tool_use_id.as_deref(), Some("toolu_1"));
        assert!(r.is_error);
    }

    #[test]
    fn no_content_returns_empty() {
        assert!(extract_tool_uses(&json!({"role": "assistant"})).is_empty());
        assert!(extract_tool_result(&json!({"role": "user"})).is_none());
    }

    #[test]
    fn parse_timestamp_iso() {
        let ts = parse_timestamp_to_epoch("2026-04-30T18:27:10.311Z").unwrap();
        assert!(ts > 1_700_000_000);
    }

    #[test]
    fn parse_timestamp_invalid_returns_none() {
        assert!(parse_timestamp_to_epoch("not-a-time").is_none());
        assert!(parse_timestamp_to_epoch("").is_none());
    }
}
