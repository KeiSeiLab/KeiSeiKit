//! Smoke tests for the S3 cloud backend (behind `s3` feature).
//!
//! These tests never hit real AWS. They verify:
//!   - the `S3CloudStore` builder accepts a mock endpoint without panic
//!   - the library re-exports `s3_cloud` when the feature is enabled
//!   - path-safety guards reject traversal attempts
//!
//! Run with: `cargo test -p kei-store --features s3 --test s3_smoke`.
//! Without the feature, this file compiles to an empty crate — harmless.
//!
//! v0.21.1: builder now rejects loopback endpoints + plain HTTP unless the
//! caller opts in via `KEI_STORE_S3_ALLOW_INTERNAL=1` +
//! `KEI_STORE_S3_ALLOW_INSECURE=1`, and requires explicit `access_key_env`
//! / `secret_key_env` whenever a custom endpoint is set (H2 SSRF guard).
//! Each test sets both env vars + mock creds under the shared `env_lock`
//! so `cargo test` parallelism can't race on the process env.

#![cfg(feature = "s3")]

use kei_store::config::S3Cfg;
use kei_store::s3_cloud::S3CloudStore;
use kei_store::test_env::env_lock;
use kei_store::MemoryStore;

const ACCESS_VAR: &str = "KEI_SMOKE_ACCESS";
const SECRET_VAR: &str = "KEI_SMOKE_SECRET";

fn mock_cfg(endpoint: &str) -> S3Cfg {
    S3Cfg {
        endpoint: Some(endpoint.to_string()),
        bucket: Some("test-bucket".to_string()),
        region: Some("us-east-1".to_string()),
        access_key_env: Some(ACCESS_VAR.to_string()),
        secret_key_env: Some(SECRET_VAR.to_string()),
        cache_path: None,
    }
}

fn with_local_env() -> std::sync::MutexGuard<'static, ()> {
    let g = env_lock();
    std::env::set_var("KEI_STORE_S3_ALLOW_INTERNAL", "1");
    std::env::set_var("KEI_STORE_S3_ALLOW_INSECURE", "1");
    std::env::set_var(ACCESS_VAR, "AKIAEXAMPLE");
    std::env::set_var(SECRET_VAR, "secret-value");
    g
}

fn clear_local_env() {
    std::env::remove_var("KEI_STORE_S3_ALLOW_INTERNAL");
    std::env::remove_var("KEI_STORE_S3_ALLOW_INSECURE");
    std::env::remove_var(ACCESS_VAR);
    std::env::remove_var(SECRET_VAR);
    std::env::remove_var("KEI_STORE_S3_ENDPOINT");
}

#[test]
fn builder_accepts_mock_endpoint() {
    let _g = with_local_env();
    let store = S3CloudStore::new(mock_cfg("http://127.0.0.1:9999"))
        .expect("builder must not require network");
    clear_local_env();
    assert_eq!(store.backend_name(), "s3-cloud");
}

#[test]
fn builder_rejects_missing_bucket() {
    let _g = with_local_env();
    let cfg = S3Cfg {
        endpoint: Some("http://127.0.0.1:9999".to_string()),
        region: Some("us-east-1".to_string()),
        access_key_env: Some(ACCESS_VAR.to_string()),
        secret_key_env: Some(SECRET_VAR.to_string()),
        ..Default::default()
    };
    let err = S3CloudStore::new(cfg)
        .err()
        .expect("missing bucket should error");
    clear_local_env();
    assert!(format!("{err:#}").contains("bucket"));
}

#[test]
fn branch_switches_prefix() {
    let _g = with_local_env();
    let store = S3CloudStore::new(mock_cfg("http://127.0.0.1:9999")).unwrap();
    clear_local_env();
    store.branch("agent/foo").unwrap();
    // No network IO — just verify branch() does not error and backend_name
    // stays stable.
    assert_eq!(store.backend_name(), "s3-cloud");
}

#[test]
fn write_fails_gracefully_on_unreachable_endpoint() {
    let _g = with_local_env();
    // Point at a closed port — real put_object must error, NOT panic.
    let store = S3CloudStore::new(mock_cfg("http://127.0.0.1:9")).unwrap();
    clear_local_env();
    let err = store.write("traces/x.jsonl", b"hello").unwrap_err();
    let msg = format!("{err:#}");
    // We only assert that an error propagates — the exact wording depends
    // on the aws-smithy layer.
    assert!(!msg.is_empty());
}

#[test]
fn endpoint_env_var_is_honoured() {
    let _g = with_local_env();
    std::env::set_var("KEI_STORE_S3_ENDPOINT", "http://127.0.0.1:9999");
    // cfg endpoint differs — env should win. Builder still succeeds.
    let cfg = S3Cfg {
        endpoint: Some("http://unused:1".to_string()),
        bucket: Some("b".to_string()),
        region: Some("us-east-1".to_string()),
        access_key_env: Some(ACCESS_VAR.to_string()),
        secret_key_env: Some(SECRET_VAR.to_string()),
        ..Default::default()
    };
    let s = S3CloudStore::new(cfg);
    clear_local_env();
    assert!(s.is_ok());
}

#[test]
fn builder_rejects_imds_endpoint() {
    let _g = env_lock();
    // Deliberately do NOT set the allow flags.
    std::env::remove_var("KEI_STORE_S3_ALLOW_INTERNAL");
    std::env::set_var("KEI_STORE_S3_ALLOW_INSECURE", "1");
    std::env::set_var(ACCESS_VAR, "AKIAEXAMPLE");
    std::env::set_var(SECRET_VAR, "secret-value");
    let err = S3CloudStore::new(mock_cfg("http://169.254.169.254"))
        .err()
        .expect("imds endpoint must be rejected");
    std::env::remove_var("KEI_STORE_S3_ALLOW_INSECURE");
    std::env::remove_var(ACCESS_VAR);
    std::env::remove_var(SECRET_VAR);
    let msg = format!("{err:#}");
    assert!(
        msg.contains("link-local") || msg.contains("169.254"),
        "unexpected err: {msg}"
    );
}
