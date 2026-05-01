//! drive_http_parse — request / response DTOs for Anthropic `/v1/messages`.
//!
//! Kept in its own module so the `drive_http` HTTP glue stays under the
//! Constructor Pattern ≤200 LOC budget and the DTO surface is unit-testable
//! without a live reqwest client.

#![cfg(feature = "http-driver")]

use serde::{Deserialize, Serialize};

use crate::drive::{AgentResult, DriveError};

/// Model id used for every `kei-spawn drive` request.
pub const MODEL_ID: &str = "claude-opus-4-7";

/// max_tokens limit per Anthropic spec (plenty for report envelopes).
pub const MAX_TOKENS: u32 = 4096;

/// Anthropic API version header value.
pub const ANTHROPIC_VERSION: &str = "2023-06-01";

/// Default endpoint; overridable via `KEI_ANTHROPIC_ENDPOINT` for tests.
pub const DEFAULT_ENDPOINT: &str = "https://api.anthropic.com/v1/messages";

/// Outbound POST body.
#[derive(Debug, Serialize)]
pub struct MessagesRequest<'a> {
    pub model: &'a str,
    pub max_tokens: u32,
    pub messages: Vec<Message<'a>>,
}

#[derive(Debug, Serialize)]
pub struct Message<'a> {
    pub role: &'a str,
    pub content: &'a str,
}

/// Inbound response shape.
#[derive(Debug, Deserialize)]
pub struct MessagesResponse {
    pub id: String,
    #[serde(default)]
    pub content: Vec<ContentBlock>,
    #[serde(default)]
    pub stop_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub text: Option<String>,
}

/// Fold the parsed response into the public `AgentResult` envelope.
///
/// Concatenates every `text`-typed content block; non-text blocks
/// (tool_use, image, etc.) are silently skipped — kei-spawn drive only
/// surfaces transcript text.
pub fn to_agent_result(r: MessagesResponse) -> AgentResult {
    let transcript = r
        .content
        .into_iter()
        .filter(|b| b.kind == "text")
        .filter_map(|b| b.text)
        .collect::<Vec<_>>()
        .join("");
    AgentResult {
        agent_id: r.id,
        transcript,
        finish_reason: r.stop_reason.unwrap_or_else(|| "unknown".to_string()),
    }
}

/// Build the `[kei-spawn routing] …` preamble required by the task spec.
pub fn build_preamble(subagent_type: &str, isolation: Option<&str>) -> String {
    format!(
        "[kei-spawn routing] subagent_type={}, isolation={}\n\n",
        subagent_type,
        isolation.unwrap_or("<none>")
    )
}

/// Build the full user message (preamble + prompt).
pub fn compose_user_content(prompt: &str, subagent_type: &str, isolation: Option<&str>) -> String {
    let mut s = build_preamble(subagent_type, isolation);
    s.push_str(prompt);
    s
}

/// Parse a JSON response body. Errors map to `Transport` with the
/// parse error message and the first 512 bytes of the body as context.
pub fn parse_response(body: &str) -> Result<AgentResult, DriveError> {
    match serde_json::from_str::<MessagesResponse>(body) {
        Ok(r) => Ok(to_agent_result(r)),
        Err(e) => Err(DriveError::Transport {
            message: format!("parse response: {e}; body[:512]={}", excerpt(body, 512)),
        }),
    }
}

/// Truncate `s` to at most `n` bytes at a char boundary.
pub fn excerpt(s: &str, n: usize) -> String {
    if s.len() <= n {
        return s.to_string();
    }
    let mut end = n;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    s[..end].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preamble_format_matches_spec() {
        let p = build_preamble("code-implementer", Some("worktree"));
        assert_eq!(
            p,
            "[kei-spawn routing] subagent_type=code-implementer, isolation=worktree\n\n"
        );
    }

    #[test]
    fn preamble_without_isolation_falls_back() {
        let p = build_preamble("critic", None);
        assert!(p.contains("isolation=<none>"));
    }

    #[test]
    fn compose_appends_prompt() {
        let c = compose_user_content("hi", "x", Some("w"));
        assert!(c.starts_with("[kei-spawn routing]"));
        assert!(c.ends_with("hi"));
    }

    #[test]
    fn parse_ok_multi_text_blocks() {
        let body = r#"{
            "id": "msg_01",
            "content": [
                {"type":"text","text":"hello "},
                {"type":"tool_use","id":"t1"},
                {"type":"text","text":"world"}
            ],
            "stop_reason": "end_turn"
        }"#;
        let r = parse_response(body).unwrap();
        assert_eq!(r.agent_id, "msg_01");
        assert_eq!(r.transcript, "hello world");
        assert_eq!(r.finish_reason, "end_turn");
    }

    #[test]
    fn parse_missing_stop_reason_defaults() {
        let body = r#"{"id":"x","content":[{"type":"text","text":"y"}]}"#;
        let r = parse_response(body).unwrap();
        assert_eq!(r.finish_reason, "unknown");
    }

    #[test]
    fn parse_malformed_maps_to_transport() {
        let err = parse_response("{not json").unwrap_err();
        match err {
            DriveError::Transport { message } => {
                assert!(message.contains("parse response"));
                assert!(message.contains("body[:512]="));
            }
            other => panic!("expected Transport, got {other}"),
        }
    }

    #[test]
    fn excerpt_respects_char_boundary() {
        let s = "αβγδ"; // 2 bytes each
        let out = excerpt(s, 3);
        // should truncate to a valid boundary (2 bytes = "α")
        assert!(s.starts_with(&out));
    }
}
