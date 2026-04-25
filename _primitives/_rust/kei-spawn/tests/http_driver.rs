//! http_driver — end-to-end tests for the `http-driver` feature.
//!
//! Uses `httpmock` to stand up a local HTTP server and `KEI_ANTHROPIC_ENDPOINT`
//! to redirect the driver at it. `KEI_ANTHROPIC_KEY` is set per-test so the
//! tests never require real credentials.
//!
//! Every test is self-contained: fresh MockServer + per-test env vars. The
//! env_lock mutex below ensures concurrent tests don't trample each other's
//! process-global env.

#![cfg(feature = "http-driver")]

use std::sync::Mutex;

use httpmock::prelude::*;
use kei_spawn::{AnthropicDriver, DriveError, HttpDriver};

/// Cargo test harness runs tests in parallel by default — env vars are
/// process-global, so serialize access.
static ENV_LOCK: Mutex<()> = Mutex::new(());

struct EnvGuard {
    key_prev: Option<String>,
    endpoint_prev: Option<String>,
    _guard: std::sync::MutexGuard<'static, ()>,
}

impl EnvGuard {
    fn new(key: Option<&str>, endpoint: Option<&str>) -> Self {
        let guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let key_prev = std::env::var("KEI_ANTHROPIC_KEY").ok();
        let endpoint_prev = std::env::var("KEI_ANTHROPIC_ENDPOINT").ok();
        match key {
            Some(v) => std::env::set_var("KEI_ANTHROPIC_KEY", v),
            None => std::env::remove_var("KEI_ANTHROPIC_KEY"),
        }
        match endpoint {
            Some(v) => std::env::set_var("KEI_ANTHROPIC_ENDPOINT", v),
            None => std::env::remove_var("KEI_ANTHROPIC_ENDPOINT"),
        }
        Self {
            key_prev,
            endpoint_prev,
            _guard: guard,
        }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.key_prev {
            Some(v) => std::env::set_var("KEI_ANTHROPIC_KEY", v),
            None => std::env::remove_var("KEI_ANTHROPIC_KEY"),
        }
        match &self.endpoint_prev {
            Some(v) => std::env::set_var("KEI_ANTHROPIC_ENDPOINT", v),
            None => std::env::remove_var("KEI_ANTHROPIC_ENDPOINT"),
        }
    }
}

#[test]
fn missing_key_returns_transport_error() {
    let _env = EnvGuard::new(None, Some("http://127.0.0.1:1/never"));
    let d = HttpDriver;
    let err = d.invoke("hi", "code-implementer", Some("worktree")).unwrap_err();
    match err {
        DriveError::Transport { message } => {
            assert!(message.contains("KEI_ANTHROPIC_KEY"), "msg: {message}");
        }
        other => panic!("expected Transport, got {other}"),
    }
}

#[test]
fn ok_200_roundtrip_populates_agent_result() {
    let server = MockServer::start();
    let _env = EnvGuard::new(Some("test-key-xxx"), Some(&server.url("/v1/messages")));

    let m = server.mock(|when, then| {
        when.method(POST)
            .path("/v1/messages")
            .header("x-api-key", "test-key-xxx")
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .body_contains("[kei-spawn routing] subagent_type=code-implementer")
            .body_contains("claude-opus-4-7");
        then.status(200)
            .header("content-type", "application/json")
            .body(
                r#"{
                    "id": "msg_test_01",
                    "content": [
                        {"type":"text","text":"hello "},
                        {"type":"text","text":"world"}
                    ],
                    "stop_reason": "end_turn"
                }"#,
            );
    });

    let d = HttpDriver;
    let out = d
        .invoke("please do X", "code-implementer", Some("worktree"))
        .expect("ok roundtrip");

    m.assert();
    assert_eq!(out.agent_id, "msg_test_01");
    assert_eq!(out.transcript, "hello world");
    assert_eq!(out.finish_reason, "end_turn");
}

#[test]
fn http_4xx_maps_to_transport_with_body_excerpt() {
    let server = MockServer::start();
    let _env = EnvGuard::new(Some("bad-key"), Some(&server.url("/v1/messages")));

    let body_msg = "{\"type\":\"error\",\"error\":{\"type\":\"invalid_api_key\",\"message\":\"bad key\"}}";
    server.mock(|when, then| {
        when.method(POST).path("/v1/messages");
        then.status(401)
            .header("content-type", "application/json")
            .body(body_msg);
    });

    let d = HttpDriver;
    let err = d.invoke("x", "code-implementer", None).unwrap_err();
    match err {
        DriveError::Transport { message } => {
            assert!(message.contains("HTTP 401"), "msg: {message}");
            assert!(message.contains("invalid_api_key"), "msg: {message}");
        }
        other => panic!("expected Transport, got {other}"),
    }
}

#[test]
fn http_5xx_maps_to_transport() {
    let server = MockServer::start();
    let _env = EnvGuard::new(Some("k"), Some(&server.url("/v1/messages")));

    server.mock(|when, then| {
        when.method(POST).path("/v1/messages");
        then.status(503)
            .header("content-type", "text/plain")
            .body("upstream overloaded");
    });

    let d = HttpDriver;
    let err = d.invoke("x", "y", None).unwrap_err();
    match err {
        DriveError::Transport { message } => {
            assert!(message.contains("HTTP 503"), "msg: {message}");
            assert!(message.contains("upstream overloaded"), "msg: {message}");
        }
        other => panic!("expected Transport, got {other}"),
    }
}

#[test]
fn malformed_json_on_200_maps_to_transport() {
    let server = MockServer::start();
    let _env = EnvGuard::new(Some("k"), Some(&server.url("/v1/messages")));

    server.mock(|when, then| {
        when.method(POST).path("/v1/messages");
        then.status(200)
            .header("content-type", "application/json")
            .body("{not-json");
    });

    let d = HttpDriver;
    let err = d.invoke("x", "y", None).unwrap_err();
    match err {
        DriveError::Transport { message } => {
            assert!(message.contains("parse response"), "msg: {message}");
            assert!(message.contains("body[:512]="), "msg: {message}");
        }
        other => panic!("expected Transport, got {other}"),
    }
}

/// Oversize response body must be rejected with a Transport error
/// containing `exceeds`. Covers the `content-length` pre-check path:
/// httpmock sends `content-length` automatically for a known-size body,
/// so an 11 MiB payload trips the pre-check (no 11 MiB allocation).
/// Protects the orchestrator process from memory-DoS (CWE-400).
#[test]
fn body_size_limit_rejects_oversized_body() {
    let server = MockServer::start();
    let _env = EnvGuard::new(Some("k"), Some(&server.url("/v1/messages")));

    // Just over the 10 MiB cap — smallest payload that exercises the
    // limit without wasting test-harness memory.
    let big_body = "a".repeat(11 * 1024 * 1024);
    server.mock(|when, then| {
        when.method(POST).path("/v1/messages");
        then.status(200)
            .header("content-type", "application/json")
            .body(&big_body);
    });

    let d = HttpDriver;
    let err = d.invoke("x", "y", None).unwrap_err();
    match err {
        DriveError::Transport { message } => {
            assert!(message.contains("exceeds"), "msg: {message}");
        }
        other => panic!("expected Transport, got {other}"),
    }
}

/// Body just under the 10 MiB cap must succeed through the parse stage
/// (parse then fails because the body isn't valid JSON — that's the
/// expected outcome here; we only want to prove the size-gate doesn't
/// fire for sub-limit bodies).
#[test]
fn body_size_limit_allows_under_cap() {
    let server = MockServer::start();
    let _env = EnvGuard::new(Some("k"), Some(&server.url("/v1/messages")));

    // Well under 10 MiB but large enough to rule out trivial paths.
    let body = "z".repeat(1024 * 1024); // 1 MiB of garbage
    server.mock(|when, then| {
        when.method(POST).path("/v1/messages");
        then.status(200)
            .header("content-type", "application/json")
            .body(&body);
    });

    let d = HttpDriver;
    let err = d.invoke("x", "y", None).unwrap_err();
    match err {
        // Size-gate MUST NOT fire; parse failure is the expected path.
        DriveError::Transport { message } => {
            assert!(
                !message.contains("exceeds"),
                "size-gate falsely fired: {message}"
            );
            assert!(message.contains("parse response"), "msg: {message}");
        }
        other => panic!("expected Transport, got {other}"),
    }
}
