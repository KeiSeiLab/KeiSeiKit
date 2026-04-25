//! drive_http — reqwest::blocking-backed Anthropic driver.
//!
//! Gated behind the `http-driver` Cargo feature. Reads `KEI_ANTHROPIC_KEY`
//! at every `invoke` call (so key rotation takes effect without rebuilds).
//!
//! Endpoint defaults to <https://api.anthropic.com/v1/messages> and can be
//! overridden via `KEI_ANTHROPIC_ENDPOINT` (test hook for httpmock).
//!
//! Constructor Pattern: one struct + one impl + small helpers, every fn
//! ≤30 LOC, file ≤200 LOC.

#![cfg(feature = "http-driver")]

use std::io::Read as _;
use std::time::Duration;

use crate::drive::{AgentResult, AnthropicDriver, DriveError};
use crate::drive_http_parse::{
    compose_user_content, excerpt, parse_response, Message, MessagesRequest, ANTHROPIC_VERSION,
    DEFAULT_ENDPOINT, MAX_TOKENS, MODEL_ID,
};

const ENV_API_KEY: &str = "KEI_ANTHROPIC_KEY";
const ENV_ENDPOINT: &str = "KEI_ANTHROPIC_ENDPOINT";
const TIMEOUT_TOTAL: Duration = Duration::from_secs(300);
// reqwest 0.12 blocking ClientBuilder exposes `connect_timeout` but not
// a per-read timeout; we cap the TCP+TLS handshake at 60s (matches the
// "60s read" intent — request-body read is bounded by the 300s total).
const TIMEOUT_CONNECT: Duration = Duration::from_secs(60);
const ERR_BODY_EXCERPT: usize = 512;
/// Cap response body at 10 MiB to mitigate memory-DoS from an oversized
/// or malicious upstream (CWE-400). Anthropic `messages` responses are
/// well under 1 MiB in practice; 10 MiB leaves ample headroom.
const MAX_BODY_BYTES: usize = 10 * 1024 * 1024;

/// Real Anthropic-backed driver. Zero-state: key + endpoint read per call.
pub struct HttpDriver;

impl AnthropicDriver for HttpDriver {
    fn invoke(
        &self,
        prompt: &str,
        subagent_type: &str,
        isolation: Option<&str>,
    ) -> Result<AgentResult, DriveError> {
        let key = read_key()?;
        let endpoint = read_endpoint();
        let client = build_client()?;
        let user_content = compose_user_content(prompt, subagent_type, isolation);
        let body = build_request_body(&user_content);
        send_and_parse(&client, &endpoint, &key, &body)
    }
}

fn read_key() -> Result<String, DriveError> {
    std::env::var(ENV_API_KEY).map_err(|_| DriveError::Transport {
        message: format!("{ENV_API_KEY} is not set in the environment"),
    })
}

fn read_endpoint() -> String {
    std::env::var(ENV_ENDPOINT).unwrap_or_else(|_| DEFAULT_ENDPOINT.to_string())
}

fn build_client() -> Result<reqwest::blocking::Client, DriveError> {
    reqwest::blocking::Client::builder()
        .timeout(TIMEOUT_TOTAL)
        .connect_timeout(TIMEOUT_CONNECT)
        .build()
        .map_err(|e| DriveError::Transport {
            message: format!("build reqwest client: {e}"),
        })
}

fn build_request_body(user_content: &str) -> String {
    let req = MessagesRequest {
        model: MODEL_ID,
        max_tokens: MAX_TOKENS,
        messages: vec![Message {
            role: "user",
            content: user_content,
        }],
    };
    // Safe: types are `Serialize` with only `&str`/`u32`/`Vec`.
    serde_json::to_string(&req).unwrap_or_else(|_| "{}".to_string())
}

fn send_and_parse(
    client: &reqwest::blocking::Client,
    endpoint: &str,
    key: &str,
    body: &str,
) -> Result<AgentResult, DriveError> {
    let resp = client
        .post(endpoint)
        .header("x-api-key", key)
        .header("anthropic-version", ANTHROPIC_VERSION)
        .header("content-type", "application/json")
        .body(body.to_string())
        .send()
        .map_err(map_network_error)?;
    let status = resp.status();
    let text = read_body_bounded(resp)?;
    if status.is_success() {
        parse_response(&text)
    } else {
        Err(http_error(status.as_u16(), &text))
    }
}

/// Read response body with a hard cap of `MAX_BODY_BYTES`.
///
/// Two defences layered:
///   1. `content-length` pre-check rejects declared-huge bodies without
///      allocating (saves memory on honest servers advertising the size).
///   2. `io::Read::take(MAX + 1)` caps the actual bytes consumed from the
///      wire — covers chunked-encoding where `content_length()` is `None`.
///      If the reader yields MAX+1 bytes we reject as oversize.
fn read_body_bounded(resp: reqwest::blocking::Response) -> Result<String, DriveError> {
    if let Some(len) = resp.content_length() {
        if len > MAX_BODY_BYTES as u64 {
            return Err(DriveError::Transport {
                message: format!(
                    "response body {len} bytes exceeds {MAX_BODY_BYTES}-byte limit (content-length)"
                ),
            });
        }
    }
    let mut buf = Vec::with_capacity(8192);
    let mut limited = resp.take(MAX_BODY_BYTES as u64 + 1);
    limited
        .read_to_end(&mut buf)
        .map_err(|e| DriveError::Transport {
            message: format!("read response body: {e}"),
        })?;
    if buf.len() > MAX_BODY_BYTES {
        return Err(DriveError::Transport {
            message: format!("response body exceeds {MAX_BODY_BYTES}-byte limit (streamed)"),
        });
    }
    String::from_utf8(buf).map_err(|e| DriveError::Transport {
        message: format!("response body not utf-8: {e}"),
    })
}

fn map_network_error(e: reqwest::Error) -> DriveError {
    DriveError::Transport {
        message: format!("network error: {e}"),
    }
}

fn http_error(status: u16, body: &str) -> DriveError {
    DriveError::Transport {
        message: format!(
            "HTTP {status}: body[:{ERR_BODY_EXCERPT}]={}",
            excerpt(body, ERR_BODY_EXCERPT)
        ),
    }
}

#[cfg(test)]
mod tests {
    //! Unit-level tests for helpers. End-to-end tests (with httpmock)
    //! live in `tests/http_driver.rs`.
    use super::*;

    #[test]
    fn build_request_body_contains_model_and_prompt() {
        let body = build_request_body("hello");
        assert!(body.contains("\"model\":\"claude-opus-4-7\""));
        assert!(body.contains("\"max_tokens\":4096"));
        assert!(body.contains("\"role\":\"user\""));
        assert!(body.contains("\"content\":\"hello\""));
    }

    #[test]
    fn http_error_truncates_long_body() {
        let long = "x".repeat(5_000);
        let err = http_error(429, &long);
        match err {
            DriveError::Transport { message } => {
                assert!(message.contains("HTTP 429"));
                assert!(message.len() < 5_000);
            }
            other => panic!("expected Transport, got {other}"),
        }
    }

    #[test]
    fn read_endpoint_returns_default_when_unset() {
        // Save + clear so the assertion is deterministic.
        let prev = std::env::var(ENV_ENDPOINT).ok();
        std::env::remove_var(ENV_ENDPOINT);
        let got = read_endpoint();
        if let Some(p) = prev {
            std::env::set_var(ENV_ENDPOINT, p);
        }
        assert_eq!(got, DEFAULT_ENDPOINT);
    }
}
