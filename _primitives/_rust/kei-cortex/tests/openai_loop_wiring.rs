//! Hermes Phase 1.1.b wiring witnesses for `/v1/chat/completions`.
//!
//! These tests prove the OpenAI-compat surface drives the REAL agent
//! loop (`tool::run_with_tools` via `agent_runner::collect_reply` /
//! `agent_runner::stream_events`) and NOT the legacy `stub_agent_reply`
//! placeholder kept around as deprecated dead code in `chat_helpers.rs`.
//!
//! Approach: divert Anthropic upstream traffic to `shared_mock_anthropic`
//! which always returns the canned text `"hi"`. If the response carries
//! `[kei-cortex stub] echo:` then the loop was bypassed; if it carries
//! `hi` then the loop ran end-to-end.
//!
//! Constructor Pattern: a sibling test cube to `openai_compat.rs` so each
//! file stays focused (router shape there, loop wiring here).

mod common;

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use kei_cortex::routes::openai::openai_router;
use kei_cortex::state::AppState;
use kei_cortex::AppConfig;
use std::path::PathBuf;
use tower::ServiceExt;

const STUB_MARKER: &str = "[kei-cortex stub] echo:";

fn dummy_state() -> AppState {
    let cfg = AppConfig::new(
        Some(0),
        Some("http://127.0.0.1".into()),
        Some(PathBuf::from("/tmp/kc-tok-tests")),
        Some(PathBuf::from("/tmp/kc-led-tests")),
        Some(PathBuf::from("/tmp/kc-pets-tests")),
        Some(PathBuf::from("/tmp/kc-mem-tests.sqlite")),
        Some(PathBuf::from("/tmp/kc-live2d-tests")),
    );
    AppState::new(cfg, "tok".into())
}

fn ensure_env() {
    std::env::set_var("KEI_API_KEY", "test-key");
    let mock = common::shared_mock_anthropic();
    std::env::set_var("ANTHROPIC_ENDPOINT", mock.uri());
    std::env::set_var("ANTHROPIC_API_KEY", "test-key");
}

fn auth_request(method: &str, uri: &str, body: Body) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .header("authorization", "Bearer test-key")
        .body(body)
        .unwrap()
}

/// Sync /v1/chat/completions — response carries the mock's "hi" and NOT
/// the legacy stub echo. Confirms `agent_runner::collect_reply` is the
/// production code path.
#[tokio::test]
async fn sync_chat_completions_runs_real_loop_not_stub() {
    ensure_env();
    let app = openai_router().with_state(dummy_state());
    let body = serde_json::json!({
        "model": "kei-cortex",
        "messages": [{ "role": "user", "content": "ping" }],
    });
    let resp = app
        .oneshot(auth_request(
            "POST",
            "/v1/chat/completions",
            Body::from(body.to_string()),
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), 65536).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let content = v["choices"][0]["message"]["content"]
        .as_str()
        .expect("content string");
    assert!(
        !content.contains(STUB_MARKER),
        "stub echo leaked through sync path: {content}"
    );
    assert!(
        content.contains("hi"),
        "expected mock anthropic reply 'hi' through real loop, got: {content}"
    );
    assert!(v["usage"]["prompt_tokens"].is_number());
    assert!(v["usage"]["completion_tokens"].is_number());
}

/// Streaming /v1/chat/completions — SSE body carries `delta` chunks fed
/// by the real loop. No stub marker; finish frame present.
#[tokio::test]
async fn streaming_chat_completions_runs_real_loop_not_stub() {
    ensure_env();
    let app = openai_router().with_state(dummy_state());
    let body = serde_json::json!({
        "model": "kei-cortex",
        "messages": [{ "role": "user", "content": "ping" }],
        "stream": true,
    });
    let resp = app
        .oneshot(auth_request(
            "POST",
            "/v1/chat/completions",
            Body::from(body.to_string()),
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), 1_048_576).await.unwrap();
    let s = String::from_utf8_lossy(&bytes);
    assert!(
        !s.contains(STUB_MARKER),
        "stub echo leaked into stream body: {s}"
    );
    assert!(s.contains("data:"), "no SSE data: frame: {s}");
    assert!(s.contains("\"delta\""), "no delta in stream body: {s}");
    assert!(
        s.contains("[DONE]") || s.contains("\"finish_reason\""),
        "stream missing finish marker: {s}"
    );
}

/// Streaming /v1/chat/completions — body MUST be chunked across multiple
/// SSE frames (per-LoopEvent forwarding via `stream_forwarder::forward_chat_completions`),
/// NOT a single accumulated frame from `agent_runner::collect_reply`.
///
/// Witnesses the p11e fix: the regression bug delivered the full reply as
/// one SSE frame because `handle_stream` previously drained the loop with
/// `collect_reply` then sent a single `AgentChunk::Delta(reply_text)`. The
/// fix wires `agent_runner::stream_events` → `forward_chat_completions`,
/// emitting one frame per `LoopEvent` plus a finish chunk plus the
/// `[DONE]` sentinel.
///
/// Structural invariants checked:
///   1. ≥ 3 `data:` frames (delta + finish + sentinel).
///   2. The delta-bearing chunk has `finish_reason: null` (in-flight).
///   3. A SEPARATE chunk has `finish_reason: "stop"` (terminal).
///   4. The terminal `[DONE]` sentinel is the last `data:` line.
///   5. The finish-chunk index is strictly less than the sentinel index
///      (proves ordering: finish, then sentinel — not bundled).
#[tokio::test]
async fn streaming_chat_completions_emits_chunked_frames_not_single_blob() {
    ensure_env();
    let app = openai_router().with_state(dummy_state());
    let body = serde_json::json!({
        "model": "kei-cortex",
        "messages": [{ "role": "user", "content": "ping" }],
        "stream": true,
    });
    let resp = app
        .oneshot(auth_request(
            "POST",
            "/v1/chat/completions",
            Body::from(body.to_string()),
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), 1_048_576).await.unwrap();
    let s = String::from_utf8_lossy(&bytes);

    let data_lines: Vec<&str> = s
        .lines()
        .filter(|l| l.starts_with("data:"))
        .collect();
    assert!(
        data_lines.len() >= 3,
        "expected ≥3 SSE data frames (delta + finish + [DONE]), got {}: {s}",
        data_lines.len()
    );

    let delta_in_flight = data_lines.iter().any(|line| {
        line.contains("\"delta\"")
            && line.contains("\"content\"")
            && line.contains("\"finish_reason\":null")
    });
    assert!(
        delta_in_flight,
        "no in-flight delta chunk (delta.content + finish_reason:null) found in: {s}"
    );

    let finish_idx = data_lines
        .iter()
        .position(|line| line.contains("\"finish_reason\":\"stop\""))
        .unwrap_or_else(|| panic!("no finish_reason:stop frame found in: {s}"));

    let sentinel_idx = data_lines
        .iter()
        .position(|line| line.trim() == "data: [DONE]")
        .unwrap_or_else(|| panic!("no [DONE] sentinel frame found in: {s}"));

    assert!(
        finish_idx < sentinel_idx,
        "finish_chunk must precede [DONE] sentinel; got finish_idx={finish_idx} sentinel_idx={sentinel_idx}: {s}"
    );
    assert_eq!(
        sentinel_idx,
        data_lines.len() - 1,
        "[DONE] sentinel must be the LAST data: frame; was at {sentinel_idx} of {} total: {s}",
        data_lines.len()
    );

    let finish_frame = data_lines[finish_idx];
    assert!(
        !finish_frame.contains("\"content\":\"hi\""),
        "finish_chunk must NOT bundle the delta content (regression signal): {finish_frame}"
    );
}

/// /v1/responses — sync mode — response.output[0].text carries the
/// mock reply, not the stub echo. Witnesses `responses::handle_sync`
/// is wired through `agent_runner::collect_reply` (P1.1.c).
#[tokio::test]
async fn sync_responses_runs_real_loop_not_stub() {
    ensure_env();
    let app = openai_router().with_state(dummy_state());
    let body = serde_json::json!({
        "model": "kei-cortex",
        "input": "ping",
    });
    let resp = app
        .oneshot(auth_request(
            "POST",
            "/v1/responses",
            Body::from(body.to_string()),
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), 65536).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let text = v["output"][0]["text"].as_str().unwrap_or("");
    assert!(
        !text.contains(STUB_MARKER),
        "stub echo leaked through /v1/responses: {text}"
    );
    assert!(
        text.contains("hi"),
        "expected mock reply 'hi' through /v1/responses real loop, got: {text}"
    );
}

/// /v1/responses — stream mode — SSE body carries `response.output_text.delta`
/// frames produced by the real loop and finishes with `response.completed`.
/// Witnesses `responses::handle_stream` is wired through
/// `agent_runner::stream_events` + `stream_forwarder::forward_responses`.
#[tokio::test]
async fn streaming_responses_runs_real_loop_not_stub() {
    ensure_env();
    let app = openai_router().with_state(dummy_state());
    let body = serde_json::json!({
        "model": "kei-cortex",
        "input": "ping",
        "stream": true,
    });
    let resp = app
        .oneshot(auth_request(
            "POST",
            "/v1/responses",
            Body::from(body.to_string()),
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), 1_048_576).await.unwrap();
    let s = String::from_utf8_lossy(&bytes);
    assert!(
        !s.contains(STUB_MARKER),
        "stub echo leaked into responses stream: {s}"
    );
    assert!(
        s.contains("response.output_text.delta"),
        "no responses delta event in stream: {s}"
    );
    assert!(
        s.contains("response.completed"),
        "stream missing response.completed: {s}"
    );
}

/// /v1/runs — POST returns 202 + run id, GET events SSE carries the
/// mock's "hi" inside a `run.message.delta` frame and terminates with
/// `run.completed`. Witnesses `runs::create_run` + `run_agent::run_real`
/// are wired through `agent_runner::stream_events` +
/// `stream_forwarder::forward_runs` (P1.1.d). The previous `run_stub`
/// would have leaked `[run stub] echo:` into the delta payload.
#[tokio::test]
async fn runs_events_stream_runs_real_loop_not_stub() {
    ensure_env();
    let app = openai_router().with_state(dummy_state());
    let body = serde_json::json!({
        "model": "kei-cortex",
        "messages": [{ "role": "user", "content": "ping" }],
    });
    let create = app
        .clone()
        .oneshot(auth_request(
            "POST",
            "/v1/runs",
            Body::from(body.to_string()),
        ))
        .await
        .unwrap();
    assert_eq!(create.status(), StatusCode::ACCEPTED);
    let create_bytes = to_bytes(create.into_body(), 65536).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&create_bytes).unwrap();
    let id = v["id"].as_str().expect("run id").to_string();
    assert_eq!(v["status"].as_str(), Some("queued"));

    let events = app
        .oneshot(auth_request(
            "GET",
            &format!("/v1/runs/{id}/events"),
            Body::empty(),
        ))
        .await
        .unwrap();
    assert_eq!(events.status(), StatusCode::OK);
    let bytes = to_bytes(events.into_body(), 1_048_576).await.unwrap();
    let s = String::from_utf8_lossy(&bytes);
    assert!(
        !s.contains("[run stub] echo:"),
        "legacy run_stub echo leaked into runs stream: {s}"
    );
    assert!(
        !s.contains(STUB_MARKER),
        "chat stub echo leaked into runs stream: {s}"
    );
    assert!(
        s.contains("run.message.delta"),
        "no run.message.delta event in stream: {s}"
    );
    assert!(
        s.contains("\"hi\""),
        "expected mock anthropic reply 'hi' inside delta payload, got: {s}"
    );
    assert!(
        s.contains("run.completed"),
        "stream missing run.completed event: {s}"
    );
    assert!(
        s.contains(&format!("\"run_id\":\"{id}\"")),
        "run.completed payload missing run_id={id}: {s}"
    );
}

/// /v1/runs/{id}/events second subscriber — once the SSE stream has been
/// consumed, the registry slot's receiver is taken; a second GET must
/// 404. Witnesses the `take_receiver` first-subscriber-wins contract is
/// preserved by the real-loop wiring (it was true under the stub too).
#[tokio::test]
async fn runs_events_second_subscriber_returns_not_found() {
    ensure_env();
    let app = openai_router().with_state(dummy_state());
    let body = serde_json::json!({
        "model": "kei-cortex",
        "messages": [{ "role": "user", "content": "ping" }],
    });
    let create = app
        .clone()
        .oneshot(auth_request(
            "POST",
            "/v1/runs",
            Body::from(body.to_string()),
        ))
        .await
        .unwrap();
    assert_eq!(create.status(), StatusCode::ACCEPTED);
    let v: serde_json::Value =
        serde_json::from_slice(&to_bytes(create.into_body(), 65536).await.unwrap()).unwrap();
    let id = v["id"].as_str().unwrap().to_string();

    // First subscriber drains the channel.
    let first = app
        .clone()
        .oneshot(auth_request(
            "GET",
            &format!("/v1/runs/{id}/events"),
            Body::empty(),
        ))
        .await
        .unwrap();
    assert_eq!(first.status(), StatusCode::OK);
    let _ = to_bytes(first.into_body(), 1_048_576).await.unwrap();

    // Second subscriber must 404 — receiver was already taken.
    let second = app
        .oneshot(auth_request(
            "GET",
            &format!("/v1/runs/{id}/events"),
            Body::empty(),
        ))
        .await
        .unwrap();
    assert_eq!(second.status(), StatusCode::NOT_FOUND);
}
