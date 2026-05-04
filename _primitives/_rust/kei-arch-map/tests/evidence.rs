//! Evidence-kind self-tests. Drives the kei-arch-map binary's library code
//! through public re-exports declared in tests/support.rs (the binary crate
//! has no library target, so we exercise behaviour via the schema + a thin
//! private dispatcher mirror).

mod support;

use std::fs;
use support::{
    cargo_check, file_exists, file_size, grep_count, http_status, json_field, regex_match,
};
use tempfile::TempDir;

fn write(dir: &std::path::Path, rel: &str, body: &str) -> std::path::PathBuf {
    let p = dir.join(rel);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&p, body).unwrap();
    p
}

#[test]
fn file_exists_passes_when_present() {
    let td = TempDir::new().unwrap();
    write(td.path(), "a.txt", "hello");
    let (ok, _) = file_exists("a.txt", td.path());
    assert!(ok);
}

#[test]
fn file_exists_fails_when_absent() {
    let td = TempDir::new().unwrap();
    let (ok, reason) = file_exists("missing.txt", td.path());
    assert!(!ok);
    assert!(reason.contains("not found"));
}

#[test]
fn regex_match_passes_on_match() {
    let td = TempDir::new().unwrap();
    write(td.path(), "x.md", "version 0.14.5 here");
    let (ok, _) = regex_match("x.md", r"version \d+\.\d+\.\d+", td.path());
    assert!(ok);
}

#[test]
fn regex_match_fails_on_mismatch() {
    let td = TempDir::new().unwrap();
    write(td.path(), "x.md", "no version");
    let (ok, _) = regex_match("x.md", r"\d+\.\d+\.\d+", td.path());
    assert!(!ok);
}

#[test]
fn grep_count_passes_at_expected() {
    let td = TempDir::new().unwrap();
    write(td.path(), "log.txt", "ERROR a\nINFO b\nERROR c\n");
    let (ok, _) = grep_count("log.txt", "^ERROR", 2, td.path());
    assert!(ok);
}

#[test]
fn grep_count_fails_off_by_one() {
    let td = TempDir::new().unwrap();
    write(td.path(), "log.txt", "ERROR a\nINFO b\nERROR c\n");
    let (ok, reason) = grep_count("log.txt", "^ERROR", 3, td.path());
    assert!(!ok);
    assert!(reason.contains("actual=2"));
}

#[test]
fn file_size_passes_in_range() {
    let td = TempDir::new().unwrap();
    let body = "x".repeat(100);
    write(td.path(), "blob.bin", &body);
    let (ok, _) = file_size("blob.bin", &[50, 200], td.path());
    assert!(ok);
}

#[test]
fn file_size_fails_out_of_range() {
    let td = TempDir::new().unwrap();
    let body = "x".repeat(10);
    write(td.path(), "blob.bin", &body);
    let (ok, reason) = file_size("blob.bin", &[100, 200], td.path());
    assert!(!ok);
    assert!(reason.contains("not in"));
}

#[test]
fn json_field_passes_when_match() {
    let td = TempDir::new().unwrap();
    write(td.path(), "p.json", r#"{"version": "0.14.5", "name": "x"}"#);
    let (ok, _) = json_field("p.json", "version", "0.14.5", td.path());
    assert!(ok);
}

#[test]
fn json_field_fails_on_mismatch() {
    let td = TempDir::new().unwrap();
    write(td.path(), "p.json", r#"{"version": "0.14.5"}"#);
    let (ok, reason) = json_field("p.json", "version", "9.9.9", td.path());
    assert!(!ok);
    // Track A security fix E: actual value redacted, only len + sha256[:8] in FAIL message
    assert!(!reason.contains("0.14.5"), "secret leaked into FAIL message: {}", reason);
    assert!(reason.contains("len=") || reason.contains("sha256"), "expected redacted form: {}", reason);
}

#[test]
fn cargo_check_clean_skipped_no_manifest() {
    let td = TempDir::new().unwrap();
    let (ok, reason) = cargo_check("not_a_crate", td.path());
    assert!(!ok);
    assert!(reason.contains("Cargo.toml"));
}

#[test]
fn cargo_check_clean_drains_pipe_under_timeout() {
    // Regression for #49: wait_timeout without pipe drain deadlocked on
    // workspaces whose JSON stream exceeded the 64 KiB pipe buffer. Even
    // a one-crate workspace exercises the new draining wait path; the
    // assertion here is "completes well under TIMEOUT (300s)".
    let td = TempDir::new().unwrap();
    write(
        td.path(),
        "Cargo.toml",
        "[package]\nname = \"deadlock_probe\"\nversion = \"0.0.1\"\nedition = \"2021\"\n\n[lib]\npath = \"src/lib.rs\"\n",
    );
    write(td.path(), "src/lib.rs", "// empty crate\n");
    let started = std::time::Instant::now();
    let (ok, reason) = cargo_check(".", td.path());
    let elapsed = started.elapsed();
    assert!(ok, "cargo check should pass on empty crate; reason={}", reason);
    assert!(
        elapsed < std::time::Duration::from_secs(30),
        "cargo check took {:?}, expected <30s (drain regression?)",
        elapsed
    );
}

#[test]
fn http_status_rejects_loopback_url() {
    let (ok, reason) = http_status("http://127.0.0.1:80/", &[200]);
    assert!(!ok);
    assert!(reason.contains("blocked") || reason.contains("loopback"));
}

#[test]
fn http_status_rejects_file_scheme() {
    let (ok, reason) = http_status("file:///etc/passwd", &[200]);
    assert!(!ok);
    assert!(reason.contains("scheme"));
}

#[test]
fn http_status_blocks_wiremock_loopback() {
    // wiremock always binds 127.0.0.1 — exactly what the SSRF guard rejects.
    // This test asserts the guard fires on a real running mock server,
    // confirming end-to-end behaviour (URL parse → IpAddr inspection → block).
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let (ok, reason) = http_status(&server.uri(), &[204]);
        assert!(!ok, "wiremock loopback must be SSRF-blocked");
        assert!(
            reason.contains("blocked") || reason.contains("loopback"),
            "expected SSRF-block reason, got: {}",
            reason
        );
    });
}

#[test]
fn http_status_rejects_private_v4() {
    let (ok, reason) = http_status("http://10.0.0.1/", &[200]);
    assert!(!ok);
    assert!(reason.contains("blocked") || reason.contains("private"));
}
