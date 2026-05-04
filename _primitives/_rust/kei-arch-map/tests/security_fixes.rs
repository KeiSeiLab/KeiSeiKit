//! Regression tests for Wave 4 security fixes (Task #50 A–F).
//!
//! Mirror of `tests/evidence.rs` style — uses the same `support` module
//! to keep call-sites short. Each #[test] points at the specific fix it
//! protects so a future regression is traceable to its origin task.

mod support;

use std::fs;
use support::{cargo_check, file_exists, http_status, json_field};
use tempfile::TempDir;

fn write(dir: &std::path::Path, rel: &str, body: &str) -> std::path::PathBuf {
    let p = dir.join(rel);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&p, body).unwrap();
    p
}

/// Fix A: hostnames must be DNS-resolved and EVERY resolved address checked
/// for SSRF safety. `localhost` resolves to 127.0.0.1 and must be blocked.
#[test]
fn fix_a_http_status_rejects_localhost_via_dns() {
    let (ok, reason) = http_status("http://localhost/", &[200]);
    assert!(!ok, "localhost-via-DNS must be SSRF-blocked; got reason={}", reason);
    assert!(
        reason.contains("blocked")
            || reason.contains("rejected")
            || reason.contains("loopback"),
        "expected SSRF-rejected reason, got: {}",
        reason
    );
}

/// Fix C: `cargo metadata` (no build.rs run) replaces `cargo check`. Empty
/// crate must validate cleanly via the metadata-only path.
#[test]
fn fix_c_cargo_check_via_metadata_succeeds_on_empty_crate() {
    let td = TempDir::new().unwrap();
    write(
        td.path(),
        "Cargo.toml",
        "[package]\nname = \"meta_probe\"\nversion = \"0.0.1\"\nedition = \"2021\"\n\n[lib]\npath = \"src/lib.rs\"\n",
    );
    write(td.path(), "src/lib.rs", "// empty crate\n");
    let started = std::time::Instant::now();
    let (ok, reason) = cargo_check(".", td.path());
    let elapsed = started.elapsed();
    assert!(
        ok,
        "cargo metadata should pass on empty crate; reason={}",
        reason
    );
    assert!(
        elapsed < std::time::Duration::from_secs(60),
        "cargo metadata took {:?}, expected <60s",
        elapsed
    );
}

/// Fix D: `..` traversal must not let a claim reach above repo root.
#[test]
fn fix_d_resolve_confined_blocks_dotdot() {
    let outer = TempDir::new().unwrap();
    fs::write(outer.path().join("secret.txt"), "leak").unwrap();
    let inner = outer.path().join("repo");
    fs::create_dir_all(&inner).unwrap();
    let (ok, reason) = file_exists("../secret.txt", &inner);
    assert!(!ok, "dotdot must be rejected; got reason={}", reason);
    assert!(
        reason.contains("escapes") || reason.contains("not found"),
        "expected containment-violation reason, got: {}",
        reason
    );
}

/// Fix E: actual JSON value must NOT appear verbatim in the FAIL reason.
/// Replaced by `len=N sha256[:8]=...` fingerprint.
#[test]
fn fix_e_json_field_redacts_actual_value_on_mismatch() {
    let td = TempDir::new().unwrap();
    write(td.path(), "p.json", r#"{"token": "supersecretvalue"}"#);
    let (ok, reason) = json_field("p.json", "token", "expected", td.path());
    assert!(!ok);
    assert!(
        !reason.contains("supersecretvalue"),
        "actual value leaked into reason: {}",
        reason
    );
    assert!(
        reason.contains("sha256"),
        "expected sha256 fingerprint, got: {}",
        reason
    );
    assert!(
        reason.contains("len="),
        "expected length marker, got: {}",
        reason
    );
}
