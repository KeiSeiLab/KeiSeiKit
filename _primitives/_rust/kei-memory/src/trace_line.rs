//! TraceLine — superset of real-trace + legacy-flat trace fields.
//!
//! Constructor Pattern: this cube only declares the deserialised line
//! plus tiny helpers (text extraction, ts resolution). Decoding is
//! `serde_json` driven; persistence + classification live elsewhere.
//!
//! Real Claude Code trace shape (sample 51a176c0-*.jsonl, 2026-04-30):
//!   {"type": "assistant" | "user" | ..., "timestamp": "<rfc3339>",
//!    "sessionId": "...", "cwd": "...", "gitBranch": "...",
//!    "uuid": "...", "parentUuid": "...",
//!    "message": {"role": "...", "content": [...]}}
//!
//! Legacy KeiSeiKit flat shape (still supported for back-compat tests):
//!   {"ts": 1700000000, "kind": "tool_use", "tool": "Bash",
//!    "file_path": "...", "is_error": false, "message": "..."}

use crate::extract::parse_timestamp_to_epoch;
use chrono::Utc;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Default)]
pub struct TraceLine {
    // ----- real Claude Code trace -----
    #[serde(rename = "type", default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(rename = "sessionId", default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub cwd: Option<String>,
    #[serde(rename = "gitBranch", default)]
    pub git_branch: Option<String>,
    #[serde(rename = "parentUuid", default)]
    pub parent_uuid: Option<String>,
    #[serde(default)]
    pub uuid: Option<String>,
    #[serde(default)]
    pub subtype: Option<String>,
    #[serde(default)]
    pub message: Option<Value>,
    #[serde(rename = "toolUseID", default)]
    pub tool_use_id: Option<String>,
    #[serde(rename = "toolUseResult", default)]
    pub tool_use_result: Option<Value>,
    // ----- legacy KeiSeiKit flat -----
    #[serde(default)]
    pub ts: Option<i64>,
    #[serde(default)]
    pub tool: Option<String>,
    #[serde(default)]
    pub file_path: Option<String>,
    #[serde(default)]
    pub is_error: Option<bool>,
    #[serde(default)]
    pub event_class: Option<String>,
}

impl TraceLine {
    /// Best-effort plain text from `message` field for guard + persist.
    /// Returns None when message is absent or not a JSON String/Object.
    /// For object-form messages, serializes back to JSON for persistence.
    pub fn message_text(&self) -> Option<String> {
        match self.message.as_ref()? {
            Value::String(s) => Some(s.clone()),
            v @ Value::Object(_) => Some(v.to_string()),
            _ => None,
        }
    }

    /// Resolve event timestamp, preferring legacy `ts` (epoch i64) over
    /// real-trace `timestamp` (RFC-3339 string), falling back to "now".
    pub fn resolved_ts(&self) -> i64 {
        if let Some(t) = self.ts {
            return t;
        }
        if let Some(s) = self.timestamp.as_deref() {
            if let Some(epoch) = parse_timestamp_to_epoch(s) {
                return epoch;
            }
        }
        Utc::now().timestamp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_real_trace_assistant_line() {
        let json = r#"{"type":"assistant","timestamp":"2026-04-30T18:27:10Z",
            "sessionId":"sx","cwd":"/x","gitBranch":"main","uuid":"u1",
            "message":{"role":"assistant","content":[
                {"type":"tool_use","id":"t1","name":"Read","input":{"file_path":"/a"}}
            ]}}"#;
        let t: TraceLine = serde_json::from_str(json).unwrap();
        assert_eq!(t.kind.as_deref(), Some("assistant"));
        assert_eq!(t.cwd.as_deref(), Some("/x"));
        assert!(t.message.is_some());
    }

    #[test]
    fn deserialize_legacy_flat_line() {
        let json = r#"{"ts":1700000000,"kind":"tool_use","tool":"Bash","message":"ok"}"#;
        let t: TraceLine = serde_json::from_str(json).unwrap();
        assert_eq!(t.ts, Some(1700000000));
        assert_eq!(t.tool.as_deref(), Some("Bash"));
        assert_eq!(t.message_text().as_deref(), Some("ok"));
    }

    #[test]
    fn message_text_object_serialises_back() {
        let t = TraceLine {
            message: Some(serde_json::json!({"role":"user"})),
            ..Default::default()
        };
        let s = t.message_text().unwrap();
        assert!(s.contains("\"role\""));
    }

    #[test]
    fn resolved_ts_prefers_ts_over_timestamp() {
        let t = TraceLine {
            ts: Some(42),
            timestamp: Some("2026-04-30T18:27:10Z".into()),
            ..Default::default()
        };
        assert_eq!(t.resolved_ts(), 42);
    }
}
