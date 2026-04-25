//! Unit tests for S3CloudStore — no network, mock-endpoint only.
//!
//! These tests verify builder correctness + path-safety guards + SSRF /
//! IMDS-leak endpoint validation + explicit-credential wiring. They do
//! NOT exercise real S3 round-trips (that would require live AWS/MinIO
//! and would fail in CI without credentials). See `tests/s3_smoke.rs`
//! for the cross-crate smoke integration.

use super::client::{effective_endpoint, resolve_explicit_creds, validate_endpoint};
use super::*;
use crate::store_trait::MemoryStore;
use crate::test_env::env_lock;

fn cfg(endpoint: &str) -> S3Cfg {
    // Tests that build an `S3CloudStore` use the INSECURE+INTERNAL overrides
    // via a shared helper — set them here so the builder's validate_endpoint
    // step passes on the loopback mock endpoint.
    S3Cfg {
        endpoint: Some(endpoint.to_string()),
        bucket: Some("test-bucket".to_string()),
        region: Some("us-east-1".to_string()),
        access_key_env: Some("KEI_TEST_MOCK_ACCESS".to_string()),
        secret_key_env: Some("KEI_TEST_MOCK_SECRET".to_string()),
        cache_path: None,
    }
}

/// Set up the env for a local-mock build: allow-internal + allow-insecure,
/// plus the mock credential pair. Returns the guard so the lock stays held
/// for the whole test body.
fn with_local_env() -> std::sync::MutexGuard<'static, ()> {
    let g = env_lock();
    std::env::set_var("KEI_STORE_S3_ALLOW_INTERNAL", "1");
    std::env::set_var("KEI_STORE_S3_ALLOW_INSECURE", "1");
    std::env::set_var("KEI_TEST_MOCK_ACCESS", "AKIAEXAMPLE");
    std::env::set_var("KEI_TEST_MOCK_SECRET", "secret-value");
    g
}

fn clear_local_env() {
    std::env::remove_var("KEI_STORE_S3_ALLOW_INTERNAL");
    std::env::remove_var("KEI_STORE_S3_ALLOW_INSECURE");
    std::env::remove_var("KEI_TEST_MOCK_ACCESS");
    std::env::remove_var("KEI_TEST_MOCK_SECRET");
}

#[test]
fn new_rejects_missing_bucket() {
    let _g = with_local_env();
    let c = S3Cfg {
        endpoint: Some("http://127.0.0.1:9999".to_string()),
        region: Some("us-east-1".to_string()),
        access_key_env: Some("KEI_TEST_MOCK_ACCESS".to_string()),
        secret_key_env: Some("KEI_TEST_MOCK_SECRET".to_string()),
        ..Default::default()
    };
    let err = S3CloudStore::new(c)
        .err()
        .expect("missing bucket should error");
    clear_local_env();
    assert!(format!("{err:#}").contains("bucket"));
}

#[test]
fn new_builds_with_mock_endpoint() {
    let _g = with_local_env();
    let store = S3CloudStore::new(cfg("http://127.0.0.1:9999")).unwrap();
    clear_local_env();
    assert_eq!(store.backend_name(), "s3-cloud");
    assert_eq!(store.current_branch(), "main");
}

#[test]
fn branch_updates_prefix() {
    let _g = with_local_env();
    let store = S3CloudStore::new(cfg("http://127.0.0.1:9999")).unwrap();
    clear_local_env();
    store.branch("feat/foo").unwrap();
    assert_eq!(
        store.key("traces/a.jsonl").unwrap(),
        "feat/foo/traces/a.jsonl"
    );
}

#[test]
fn branch_rejects_parent() {
    let _g = with_local_env();
    let store = S3CloudStore::new(cfg("http://127.0.0.1:9999")).unwrap();
    clear_local_env();
    let err = store.branch("../escape").unwrap_err();
    assert!(format!("{err:#}").contains("parent-dir"));
}

#[test]
fn key_rejects_absolute() {
    let _g = with_local_env();
    let store = S3CloudStore::new(cfg("http://127.0.0.1:9999")).unwrap();
    clear_local_env();
    let err = store.key("/etc/passwd").unwrap_err();
    assert!(format!("{err:#}").contains("absolute"));
}

// ----------------------------------------------------------------------
// Endpoint / credential unit tests (H2 SSRF + HIGH-2 creds-wire).
// ----------------------------------------------------------------------

fn cfg_with_endpoint(endpoint: &str) -> S3Cfg {
    S3Cfg {
        endpoint: Some(endpoint.to_string()),
        bucket: Some("test-bucket".to_string()),
        region: Some("us-east-1".to_string()),
        access_key_env: None,
        secret_key_env: None,
        cache_path: None,
    }
}

#[test]
fn effective_endpoint_env_overrides_cfg() {
    let _g = env_lock();
    std::env::set_var("KEI_STORE_S3_ENDPOINT", "http://127.0.0.1:9000");
    let c = cfg_with_endpoint("http://other:8080");
    let got = effective_endpoint(&c);
    std::env::remove_var("KEI_STORE_S3_ENDPOINT");
    assert_eq!(got.as_deref(), Some("http://127.0.0.1:9000"));
}

#[test]
fn effective_endpoint_cfg_when_no_env() {
    let _g = env_lock();
    std::env::remove_var("KEI_STORE_S3_ENDPOINT");
    let c = cfg_with_endpoint("http://127.0.0.1:9999");
    assert_eq!(
        effective_endpoint(&c).as_deref(),
        Some("http://127.0.0.1:9999")
    );
}

#[test]
fn effective_endpoint_none_when_no_env_no_cfg() {
    let _g = env_lock();
    std::env::remove_var("KEI_STORE_S3_ENDPOINT");
    let c = S3Cfg::default();
    assert_eq!(effective_endpoint(&c), None);
}

#[test]
fn rejects_imds_endpoint() {
    let _g = env_lock();
    std::env::remove_var("KEI_STORE_S3_ALLOW_INTERNAL");
    std::env::set_var("KEI_STORE_S3_ALLOW_INSECURE", "1");
    let err = validate_endpoint("http://169.254.169.254/latest").unwrap_err();
    std::env::remove_var("KEI_STORE_S3_ALLOW_INSECURE");
    let msg = format!("{err:#}");
    assert!(msg.contains("link-local") || msg.contains("169.254"), "err: {msg}");
}

#[test]
fn rejects_loopback_default() {
    let _g = env_lock();
    std::env::remove_var("KEI_STORE_S3_ALLOW_INTERNAL");
    std::env::set_var("KEI_STORE_S3_ALLOW_INSECURE", "1");
    let err = validate_endpoint("http://127.0.0.1:9000").unwrap_err();
    std::env::remove_var("KEI_STORE_S3_ALLOW_INSECURE");
    let msg = format!("{err:#}");
    assert!(msg.contains("loopback") || msg.contains("127.0.0.1"), "err: {msg}");
}

#[test]
fn accepts_loopback_with_override() {
    let _g = env_lock();
    std::env::set_var("KEI_STORE_S3_ALLOW_INTERNAL", "1");
    std::env::set_var("KEI_STORE_S3_ALLOW_INSECURE", "1");
    let r = validate_endpoint("http://127.0.0.1:9000");
    std::env::remove_var("KEI_STORE_S3_ALLOW_INTERNAL");
    std::env::remove_var("KEI_STORE_S3_ALLOW_INSECURE");
    assert!(r.is_ok(), "should accept loopback with override: {:?}", r);
}

#[test]
fn rejects_non_https_default() {
    let _g = env_lock();
    std::env::remove_var("KEI_STORE_S3_ALLOW_INSECURE");
    let err = validate_endpoint("http://s3.example.com").unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("http"), "err: {msg}");
}

#[test]
fn accepts_https_public() {
    let _g = env_lock();
    std::env::remove_var("KEI_STORE_S3_ALLOW_INTERNAL");
    std::env::remove_var("KEI_STORE_S3_ALLOW_INSECURE");
    let r = validate_endpoint("https://s3.amazonaws.com");
    assert!(r.is_ok(), "public https should be allowed: {:?}", r);
}

#[test]
fn rejects_metadata_hostname() {
    let _g = env_lock();
    std::env::remove_var("KEI_STORE_S3_ALLOW_INTERNAL");
    let err = validate_endpoint("https://metadata.google.internal").unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("metadata") || msg.contains("link-local"), "err: {msg}");
}

#[test]
fn rejects_partial_creds_config() {
    let _g = env_lock();
    let c = S3Cfg {
        access_key_env: Some("KEI_TEST_A".to_string()),
        secret_key_env: None,
        ..Default::default()
    };
    let err = resolve_explicit_creds(&c).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("both be set"), "err: {msg}");
}

#[test]
fn resolves_both_creds_when_set() {
    let _g = env_lock();
    std::env::set_var("KEI_TEST_ACCESS_OK", "AKIAEXAMPLE");
    std::env::set_var("KEI_TEST_SECRET_OK", "secret-value");
    let c = S3Cfg {
        access_key_env: Some("KEI_TEST_ACCESS_OK".to_string()),
        secret_key_env: Some("KEI_TEST_SECRET_OK".to_string()),
        ..Default::default()
    };
    let got = resolve_explicit_creds(&c);
    std::env::remove_var("KEI_TEST_ACCESS_OK");
    std::env::remove_var("KEI_TEST_SECRET_OK");
    assert!(got.unwrap().is_some());
}

#[test]
fn rejects_empty_resolved_creds() {
    let _g = env_lock();
    std::env::set_var("KEI_TEST_EMPTY_A", "");
    std::env::set_var("KEI_TEST_EMPTY_S", "");
    let c = S3Cfg {
        access_key_env: Some("KEI_TEST_EMPTY_A".to_string()),
        secret_key_env: Some("KEI_TEST_EMPTY_S".to_string()),
        ..Default::default()
    };
    let err = resolve_explicit_creds(&c).unwrap_err();
    std::env::remove_var("KEI_TEST_EMPTY_A");
    std::env::remove_var("KEI_TEST_EMPTY_S");
    let msg = format!("{err:#}");
    assert!(msg.contains("empty"), "err: {msg}");
}

// ----------------------------------------------------------------------
// commit() recursive-list fix (HIGH-1) — compile-smoke only.
// ----------------------------------------------------------------------

/// Compile-time assertion that `list_recursive` exists on the underlying
/// AsyncBackend with an async signature returning `Result<Vec<String>>`.
/// v0.22 Track B: the method moved off `S3CloudStore` onto `S3AsyncBackend`
/// (via the `AsyncBackend` trait); we reach it through `.backend()`.
#[allow(dead_code)]
async fn _compile_smoke_list_recursive_exists(store: &S3CloudStore, prefix: &str) {
    use crate::async_backend::AsyncBackend;
    let _out: Vec<String> = store.backend().list_recursive(prefix).await.unwrap();
}

// ----------------------------------------------------------------------
// v0.22 Track B — shared runtime across multiple Store instances.
// ----------------------------------------------------------------------

/// Previously, each `S3CloudStore` built its own `current_thread` tokio
/// runtime. If a single process held two instances (e.g. migrate A→B), a
/// `block_on` call from one runtime's thread that tried to use the other
/// instance's runtime would panic. With the shared multi-thread runtime,
/// both instances should coexist fine.
#[test]
fn async_backend_shared_runtime_handles_two_store_instances() {
    let _g = with_local_env();
    let a = S3CloudStore::new(cfg("http://127.0.0.1:9999")).expect("first store");
    let b = S3CloudStore::new(cfg("http://127.0.0.1:9999")).expect("second store");
    clear_local_env();
    assert_eq!(a.backend_name(), "s3-cloud");
    assert_eq!(b.backend_name(), "s3-cloud");
    // Each instance keeps its own branch; the runtime is shared.
    a.branch("branch-a").unwrap();
    b.branch("branch-b").unwrap();
    assert_eq!(a.current_branch(), "branch-a");
    assert_eq!(b.current_branch(), "branch-b");
}

/// The shared runtime is multi-thread (needed for the N=2-Store fix).
/// Verify via `tokio::runtime::Handle::current` from inside a spawned
/// task — `current_thread` runtimes have `num_workers == 1`, multi-thread
/// runtimes report >1.
#[test]
fn async_backend_runtime_is_multi_thread() {
    use crate::async_backend::shared_runtime;
    let workers = shared_runtime().block_on(async {
        let h = tokio::runtime::Handle::current();
        h.metrics().num_workers()
    });
    assert!(
        workers >= 2,
        "shared runtime must have >=2 workers, got {workers}"
    );
}
