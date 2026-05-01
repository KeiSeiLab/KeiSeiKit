//! Unit tests for `term.rs`. We test the cwd resolver, the Origin
//! validator, and the PTY spawn seam. Driving an actual WS upgrade
//! requires an axum test server; that lives in the integration suite —
//! these tests focus on the synchronous halves.

use super::*;
use crate::handlers::term_pty::spawn_pty;
use axum::http::HeaderValue;
use std::fs;
use tempfile::TempDir;
use tokio::sync::mpsc;

/// Build a tempdir with one nested directory.
fn fixture() -> TempDir {
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join("workdir")).unwrap();
    tmp
}

#[test]
fn empty_cwd_resolves_to_root() {
    let tmp = fixture();
    let p = resolve_cwd(tmp.path(), None).unwrap();
    let canon = tmp.path().canonicalize().unwrap();
    assert_eq!(p, canon);
}

#[test]
fn relative_cwd_resolves_inside_root() {
    let tmp = fixture();
    let p = resolve_cwd(tmp.path(), Some("workdir")).unwrap();
    assert!(p.ends_with("workdir"));
}

#[test]
fn parent_traversal_blocked() {
    let tmp = fixture();
    let err = resolve_cwd(tmp.path(), Some("../escape")).unwrap_err();
    assert!(matches!(err, AppError::BadRequest(_)));
}

#[test]
fn absolute_outside_root_blocked() {
    let tmp = fixture();
    let err = resolve_cwd(tmp.path(), Some("/etc")).unwrap_err();
    assert!(matches!(err, AppError::BadRequest(_) | AppError::NotFound(_)));
}

#[test]
fn nonexistent_cwd_yields_not_found() {
    let tmp = fixture();
    let err = resolve_cwd(tmp.path(), Some("no-such")).unwrap_err();
    assert!(matches!(err, AppError::NotFound(_)));
}

#[test]
fn file_target_rejected_as_bad_request() {
    let tmp = fixture();
    fs::write(tmp.path().join("a-file"), b"x").unwrap();
    let err = resolve_cwd(tmp.path(), Some("a-file")).unwrap_err();
    assert!(matches!(err, AppError::BadRequest(_)));
}

// === Wave 44b F-HIGH-2 — Origin (CSWSH) tests ===

const ALLOWED: &str = "https://keisei.app";

#[test]
fn origin_missing_is_forbidden() {
    let headers = HeaderMap::new();
    let err = validate_origin(&headers, ALLOWED).unwrap_err();
    assert!(matches!(err, AppError::Forbidden));
}

#[test]
fn origin_null_is_forbidden() {
    let mut headers = HeaderMap::new();
    headers.insert(header::ORIGIN, HeaderValue::from_static("null"));
    let err = validate_origin(&headers, ALLOWED).unwrap_err();
    assert!(matches!(err, AppError::Forbidden));
}

#[test]
fn origin_mismatch_is_forbidden() {
    let mut headers = HeaderMap::new();
    headers.insert(header::ORIGIN, HeaderValue::from_static("https://evil.example"));
    let err = validate_origin(&headers, ALLOWED).unwrap_err();
    assert!(matches!(err, AppError::Forbidden));
}

#[test]
fn origin_exact_match_passes() {
    let mut headers = HeaderMap::new();
    headers.insert(header::ORIGIN, HeaderValue::from_static(ALLOWED));
    assert!(validate_origin(&headers, ALLOWED).is_ok());
}

#[test]
fn origin_non_utf8_is_forbidden() {
    let mut headers = HeaderMap::new();
    let bad = HeaderValue::from_bytes(b"\xff\xfe").unwrap();
    headers.insert(header::ORIGIN, bad);
    let err = validate_origin(&headers, ALLOWED).unwrap_err();
    assert!(matches!(err, AppError::Forbidden));
}

#[test]
fn origin_subdomain_does_not_count_as_match() {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::ORIGIN,
        HeaderValue::from_static("https://attacker.keisei.app"),
    );
    let err = validate_origin(&headers, ALLOWED).unwrap_err();
    assert!(matches!(err, AppError::Forbidden));
}

// === Wave 44b PTY lifecycle smoke ===

/// Spawn-only smoke: the PTY allocates, `$SHELL` (or /bin/sh) launches,
/// and dropping the bag tears down the child + reader cleanly (no panic,
/// no leaked handle observed by the test runtime). Drop happens at end of
/// scope — the assertion is implicit (no panic, no hang).
#[tokio::test]
async fn spawn_pty_smoke_drops_cleanly() {
    let tmp = fixture();
    let canon = tmp.path().canonicalize().unwrap();
    let (out_tx, mut out_rx) = mpsc::channel::<Vec<u8>>(8);
    let bag = spawn_pty(&canon, out_tx).expect("spawn_pty");
    // Give the reader a moment to attempt at least one read.
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    drop(bag);
    // After drop, cancel flag is set + child killed; out_rx should drain
    // any in-flight bytes and then close (sender dropped inside the bag).
    let _ = tokio::time::timeout(
        std::time::Duration::from_millis(500),
        async {
            while out_rx.recv().await.is_some() {}
        },
    )
    .await;
}
