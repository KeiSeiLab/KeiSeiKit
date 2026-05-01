//! Integration tests for the OpenAI-compatible /v1/* surface.
//!
//! Exercises the wire format end-to-end against an in-process axum
//! `Router` via `tower::ServiceExt::oneshot`. Bypasses the HTTP socket
//! so tests are fast and deterministic. Streaming endpoints are
//! exercised by reading the response body to bytes and asserting the
//! presence of the expected SSE frames.
//!
//! Upstream Anthropic traffic is diverted to a process-wide axum mock
//! served by `common::shared_mock_anthropic`. The mock runs on a
//! dedicated OS-thread runtime so it outlives every `#[tokio::test]`
//! runtime in the binary; tests set `ANTHROPIC_ENDPOINT` to its URI
//! (idempotent across tests because the singleton URI never changes).
//! `anthropic::endpoint()` reads the env at call time, so the redirect
//! is picked up without any source-side wiring.

mod common;

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use kei_cortex::routes::openai::openai_router;
use kei_cortex::state::AppState;
use kei_cortex::AppConfig;
use std::path::PathBuf;
use tower::ServiceExt;

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

/// Set `KEI_API_KEY` for tests so the auth middleware uses the bearer
/// path (it's hard to mock loopback `ConnectInfo` in oneshot tests).
fn ensure_api_key() {
    std::env::set_var("KEI_API_KEY", "test-key");
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

#[tokio::test]
async fn list_models_returns_kei_cortex() {
    ensure_api_key();
    let app = openai_router().with_state(dummy_state());
    let resp = app
        .oneshot(auth_request("GET", "/v1/models", Body::empty()))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), 65536).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["object"], "list");
    assert!(body["data"].as_array().unwrap().len() >= 1);
}

#[tokio::test]
async fn unauthorized_without_bearer() {
    // Use the same key value as `ensure_api_key` so this test does not
    // race other tests' bearer checks when run in parallel — the
    // assertion here is "missing header ⇒ 401", which doesn't depend
    // on the configured key value.
    std::env::set_var("KEI_API_KEY", "test-key");
    let app = openai_router().with_state(dummy_state());
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/v1/models")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn chat_completions_sync_returns_choices() {
    ensure_api_key();
    let mock = common::shared_mock_anthropic();
    std::env::set_var("ANTHROPIC_ENDPOINT", mock.uri());
    std::env::set_var("ANTHROPIC_API_KEY", "test-key");
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
    assert_eq!(v["object"], "chat.completion");
    assert!(v["choices"][0]["message"]["content"].is_string());
}

#[tokio::test]
async fn chat_completions_stream_emits_sse() {
    ensure_api_key();
    let mock = common::shared_mock_anthropic();
    std::env::set_var("ANTHROPIC_ENDPOINT", mock.uri());
    std::env::set_var("ANTHROPIC_API_KEY", "test-key");
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
    assert!(s.contains("data:"), "expected SSE data frame, got: {s}");
}

#[tokio::test]
async fn run_create_returns_202_and_id() {
    ensure_api_key();
    let mock = common::shared_mock_anthropic();
    std::env::set_var("ANTHROPIC_ENDPOINT", mock.uri());
    std::env::set_var("ANTHROPIC_API_KEY", "test-key");
    let app = openai_router().with_state(dummy_state());
    let body = serde_json::json!({
        "model": "kei-cortex",
        "messages": [{ "role": "user", "content": "go" }],
    });
    let resp = app
        .clone()
        .oneshot(auth_request("POST", "/v1/runs", Body::from(body.to_string())))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::ACCEPTED);
    let bytes = to_bytes(resp.into_body(), 65536).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let id = v["id"].as_str().unwrap();
    assert!(id.starts_with("run_"));
    // Registry is process-global — /stop on the same id reaches the
    // same RunRegistry whether or not we re-use `app`.
    let stop = app
        .oneshot(auth_request(
            "POST",
            &format!("/v1/runs/{id}/stop"),
            Body::empty(),
        ))
        .await
        .unwrap();
    assert!(matches!(
        stop.status(),
        StatusCode::OK | StatusCode::NOT_FOUND
    ));
}

#[tokio::test]
async fn responses_get_unknown_id_returns_404() {
    ensure_api_key();
    let app = openai_router().with_state(dummy_state());
    let resp = app
        .oneshot(auth_request("GET", "/v1/responses/missing", Body::empty()))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
