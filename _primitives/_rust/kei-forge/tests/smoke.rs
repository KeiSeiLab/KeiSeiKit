//! Integration smoke test for kei-forge.
//!
//! Exercises GET / and POST /forge via `tower::ServiceExt::oneshot` on
//! the Router — no real socket. With pure-Rust templating, the generator
//! is hermetic when pointed at a non-existent crate name: it returns a
//! structured `CrateNotFound` without touching the filesystem, so these
//! tests can run in any working directory without creating or mutating
//! real atoms on disk.
//!
//! Unit tests for the pure-Rust pipeline (happy path, file-exists refuse,
//! crate-not-found, template-missing) live inside `src/generate.rs` and
//! its three Constructor-Pattern submodules (placeholders, paths,
//! rollback) — they use `tempfile::TempDir` for full hermetic runs.
//!
//! Run with: `cargo test -p kei-forge`

use axum::{
    body::{to_bytes, Body},
    http::{header, Request, StatusCode},
};
use kei_forge::server;
use serde_json::Value;
use tower::ServiceExt;

const LOCAL_HOST: &str = "127.0.0.1:8747";

fn get(uri: &str) -> Request<Body> {
    Request::builder()
        .uri(uri)
        .header("host", LOCAL_HOST)
        .body(Body::empty())
        .unwrap()
}

fn post_json(uri: &str, body: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header("host", LOCAL_HOST)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

#[tokio::test]
async fn get_root_serves_form() {
    let app = server::app();
    let resp = app.oneshot(get("/")).await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = to_bytes(resp.into_body(), 64 * 1024).await.unwrap();
    let html = std::str::from_utf8(&body).unwrap();
    assert!(html.contains("kei-forge"));
    assert!(html.contains("<form"));
    assert!(html.contains("name=\"verb\""));
    assert!(html.contains("name=\"kind\""));
}

#[tokio::test]
async fn post_forge_returns_json_shape() {
    // Use a crate name guaranteed not to exist under _primitives/_rust/
    // so the generator returns CrateNotFound (422) without mutating disk.
    let app = server::app();
    let body = r#"{"crate":"kei-nonexistent-test-crate","verb":"add-dependency","kind":"command","description":"test desc"}"#;

    let resp = app.oneshot(post_json("/forge", body)).await.unwrap();

    let status = resp.status();
    let bytes = to_bytes(resp.into_body(), 64 * 1024).await.unwrap();
    let json: Value = serde_json::from_slice(&bytes).expect("response is JSON");

    assert!(json.get("success").is_some(), "missing success field");
    assert!(json.get("files").is_some(), "missing files field");
    assert!(json.get("errors").is_some(), "missing errors field");

    assert!(
        status == StatusCode::OK
            || status == StatusCode::UNPROCESSABLE_ENTITY
            || status == StatusCode::BAD_REQUEST,
        "unexpected status {status}"
    );
}

#[tokio::test]
async fn post_forge_rejects_bad_kind() {
    let app = server::app();
    let body = r#"{"crate":"kei-task","verb":"x","kind":"saga","description":"y"}"#;

    let resp = app.oneshot(post_json("/forge", body)).await.unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let bytes = to_bytes(resp.into_body(), 64 * 1024).await.unwrap();
    let json: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["success"], Value::Bool(false));
    let errs = json["errors"].as_array().unwrap();
    assert!(!errs.is_empty());
}

// ---------------------------------------------------------------------
// Security hardening — the four new tests required by the fix contract.
// ---------------------------------------------------------------------

/// FIX A (DNS rebinding): a POST whose `Host:` header names an attacker
/// domain — even when the underlying socket is 127.0.0.1 — MUST be
/// rejected before the handler sees it.
#[tokio::test]
async fn post_with_evil_host_is_rejected() {
    let app = server::app();
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/forge")
                .header("host", "evil.com")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"crate":"kei-task","verb":"x","kind":"command","description":"y"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_ne!(resp.status(), StatusCode::OK);
    assert!(
        resp.status() == StatusCode::MISDIRECTED_REQUEST
            || resp.status() == StatusCode::FORBIDDEN,
        "expected 403 or 421, got {}",
        resp.status()
    );
}

/// FIX B (CSRF): `application/x-www-form-urlencoded` is SOP-safe, so a
/// malicious `<form>` on any site could POST to us without preflight.
/// Must be rejected with 415 Unsupported Media Type.
#[tokio::test]
async fn post_urlencoded_is_rejected() {
    let app = server::app();
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/forge")
                .header("host", LOCAL_HOST)
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(
                    "crate=kei-task&verb=x&kind=command&description=y",
                ))
                .unwrap(),
            )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
}

/// FIX C (description injection): a newline in `description` could
/// escape the `sed` substitution inside `scripts/new-atom.sh` and
/// append a hostile `-e` expression. Must fail validation with 400.
#[tokio::test]
async fn post_description_with_newline_is_rejected() {
    let app = server::app();
    // JSON escape for newline is \n literal in the string.
    let body = r#"{"crate":"kei-task","verb":"noop","kind":"command","description":"foo\nevil"}"#;
    let resp = app.oneshot(post_json("/forge", body)).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let bytes = to_bytes(resp.into_body(), 64 * 1024).await.unwrap();
    let json: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["success"], Value::Bool(false));
    let err = json["errors"][0].as_str().unwrap();
    assert!(
        err.contains("description"),
        "expected description error, got: {err}"
    );
}

/// FIX (defence-in-depth): GET / must carry the four hardening
/// headers so an iframe / reflected-XSS pivot cannot escalate.
#[tokio::test]
async fn get_root_has_security_headers() {
    let app = server::app();
    let resp = app.oneshot(get("/")).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let h = resp.headers();
    assert!(
        h.contains_key(header::CONTENT_SECURITY_POLICY),
        "missing CSP header"
    );
    assert!(
        h.contains_key(header::X_CONTENT_TYPE_OPTIONS),
        "missing X-Content-Type-Options"
    );
    assert!(
        h.contains_key(header::X_FRAME_OPTIONS),
        "missing X-Frame-Options"
    );
    assert!(
        h.contains_key(header::REFERRER_POLICY),
        "missing Referrer-Policy"
    );
}
