//! Smoke tests for kei-provision backends.
//!
//! Strategy: no real cloud calls. We inject a tempdir onto PATH containing
//! fake `hcloud` / `vultr-cli` shell scripts that echo canned JSON matching
//! the real v1 / v3 CLI output shapes. The Backend impls then parse these
//! exactly as they would production output.

use kei_provision::{exec, resolve, CreateOpts};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::sync::Mutex;
use tempfile::TempDir;

// Process-global PATH + env vars are not thread-safe across the parallel
// `cargo test` runner. Serialize tests that mutate env.
static ENV_LOCK: Mutex<()> = Mutex::new(());

/// Create a fake CLI script at `<dir>/<bin>` that emits `stdout` verbatim
/// (regardless of arguments) and exits 0.
fn install_fake(dir: &Path, bin: &str, stdout: &str) {
    let path = dir.join(bin);
    // printf with escaped % for shell robustness — none of our fixtures
    // need printf interpolation, so use `cat <<'EOF'`.
    let script = format!("#!/usr/bin/env bash\ncat <<'EOF'\n{stdout}\nEOF\n");
    fs::write(&path, script).expect("write fake");
    let mut perms = fs::metadata(&path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&path, perms).unwrap();
}

/// Install a fake that always exits non-zero (simulates "server absent").
fn install_fake_fail(dir: &Path, bin: &str) {
    let path = dir.join(bin);
    let script = "#!/usr/bin/env bash\nexit 1\n";
    fs::write(&path, script).expect("write fake");
    let mut perms = fs::metadata(&path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&path, perms).unwrap();
}

/// Install a fake that emits `stderr_text` to stderr then exits non-zero.
/// Used to test error-redaction paths in `run_void` / `run_json_strict`.
fn install_fake_stderr(dir: &Path, bin: &str, stderr_text: &str) {
    let path = dir.join(bin);
    // `cat <<'EOF' 1>&2` preserves literal text including URLs + secrets.
    let script = format!(
        "#!/usr/bin/env bash\ncat <<'EOF' 1>&2\n{stderr_text}\nEOF\nexit 1\n"
    );
    fs::write(&path, script).expect("write fake");
    let mut perms = fs::metadata(&path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&path, perms).unwrap();
}

/// Prepend tempdir to PATH so the fake binary wins, but keep the rest of
/// PATH so `#!/usr/bin/env bash` can still find `bash`.
fn prep_env(dir: &Path, token_var: &str) {
    let old = std::env::var("PATH").unwrap_or_default();
    let new = format!("{}:{}", dir.display(), old);
    std::env::set_var("PATH", new);
    std::env::set_var(token_var, "fake-token-for-tests");
}

const HETZNER_DESCRIBE: &str = r#"{
  "id": 42,
  "name": "test-srv-a",
  "status": "running",
  "public_net": { "ipv4": { "ip": "1.2.3.4" } },
  "server_type": { "name": "cx22" },
  "datacenter": { "location": { "name": "fsn1" } }
}"#;

const HETZNER_LIST: &str = r#"[
  {
    "id": 42,
    "name": "test-srv-a",
    "status": "running",
    "public_net": { "ipv4": { "ip": "1.2.3.4" } }
  },
  {
    "id": 43,
    "name": "test-srv-b",
    "status": "running",
    "public_net": { "ipv4": { "ip": "5.6.7.8" } }
  }
]"#;

const VULTR_LIST: &str = r#"{
  "instances": [
    {
      "id": "abc-123",
      "label": "test-vultr",
      "status": "active",
      "power_status": "running",
      "main_ip": "9.8.7.6",
      "region": "ams",
      "plan": "vc2-1c-1gb"
    }
  ]
}"#;

#[test]
fn hetzner_status_parses_ipv4_and_id() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let dir = TempDir::new().unwrap();
    install_fake(dir.path(), "hcloud", HETZNER_DESCRIBE);
    prep_env(dir.path(), "HCLOUD_TOKEN");

    let b = resolve("hetzner").unwrap();
    let info = b.status("test-srv-a").unwrap().expect("server present");
    assert_eq!(info.name, "test-srv-a");
    assert_eq!(info.id, "42");
    assert_eq!(info.ipv4.as_deref(), Some("1.2.3.4"));
    assert_eq!(info.status, "running");
}

#[test]
fn hetzner_status_absent_returns_none() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let dir = TempDir::new().unwrap();
    install_fake_fail(dir.path(), "hcloud");
    prep_env(dir.path(), "HCLOUD_TOKEN");

    let b = resolve("hetzner").unwrap();
    assert!(b.status("nonexistent").unwrap().is_none());
}

#[test]
fn hetzner_list_parses_array() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let dir = TempDir::new().unwrap();
    install_fake(dir.path(), "hcloud", HETZNER_LIST);
    prep_env(dir.path(), "HCLOUD_TOKEN");

    let b = resolve("hetzner").unwrap();
    let servers = b.list().unwrap();
    assert_eq!(servers.len(), 2);
    assert_eq!(servers[0].name, "test-srv-a");
    assert_eq!(servers[1].ipv4.as_deref(), Some("5.6.7.8"));
}

#[test]
fn vultr_status_matches_label() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let dir = TempDir::new().unwrap();
    install_fake(dir.path(), "vultr-cli", VULTR_LIST);
    prep_env(dir.path(), "VULTR_API_KEY");

    let b = resolve("vultr").unwrap();
    let info = b.status("test-vultr").unwrap().expect("found");
    assert_eq!(info.id, "abc-123");
    assert_eq!(info.ipv4.as_deref(), Some("9.8.7.6"));
    assert_eq!(info.status, "active");
}

#[test]
fn vultr_status_absent_when_label_missing() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let dir = TempDir::new().unwrap();
    install_fake(dir.path(), "vultr-cli", VULTR_LIST);
    prep_env(dir.path(), "VULTR_API_KEY");

    let b = resolve("vultr").unwrap();
    assert!(b.status("not-in-list").unwrap().is_none());
}

#[test]
fn vultr_destroy_absent_is_idempotent() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let dir = TempDir::new().unwrap();
    install_fake(dir.path(), "vultr-cli", VULTR_LIST);
    prep_env(dir.path(), "VULTR_API_KEY");

    let b = resolve("vultr").unwrap();
    // "ghost" is not in VULTR_LIST → destroy must succeed no-op
    b.destroy("ghost", true).unwrap();
}

#[test]
fn unknown_backend_errors_out() {
    let err = match resolve("gcp") {
        Ok(_) => panic!("gcp should not resolve"),
        Err(e) => e.to_string(),
    };
    assert!(err.contains("unknown backend"), "got: {err}");
}

#[test]
fn create_opts_default_is_none_everywhere() {
    let o = CreateOpts::default();
    assert!(o.server_type.is_none());
    assert!(o.location.is_none());
    assert!(o.image.is_none());
    assert!(o.ssh_key.is_none());
    assert!(o.firewall.is_none());
    assert!(o.user_data_path.is_none());
}

// ---- security: error-message redaction (MEDIUM info-disclosure) ----

#[test]
fn error_redacts_full_argv_out_of_message() {
    // If a future refactor ever routes `--api-key sk-live-ABCDEF...` as
    // an inline arg, the failure path must NOT leak it into logs.
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let dir = TempDir::new().unwrap();
    install_fake_stderr(dir.path(), "fakebin", "boom");
    prep_env(dir.path(), "IGNORED_TOKEN");

    let err = exec::run_void(
        "fakebin",
        &["--api-key", "sk-live-SECRET-DO-NOT-LEAK", "--region", "fsn1"],
    )
    .unwrap_err()
    .to_string();

    // No full arg list; no secret-looking token.
    assert!(!err.contains("sk-live-SECRET-DO-NOT-LEAK"), "leaked: {err}");
    assert!(!err.contains("--api-key"), "leaked flag: {err}");
    // But the count + binary name MUST remain for debugging.
    assert!(err.contains("fakebin"), "missing bin: {err}");
    assert!(err.contains("4 args"), "missing count: {err}");
}

#[test]
fn error_truncates_long_stderr_to_200_chars_plus_marker() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let dir = TempDir::new().unwrap();
    // 400 ASCII chars — must truncate.
    let long: String = "A".repeat(400);
    install_fake_stderr(dir.path(), "fakebin2", &long);
    prep_env(dir.path(), "IGNORED_TOKEN");

    let err = exec::run_void("fakebin2", &["--op", "create"])
        .unwrap_err()
        .to_string();

    assert!(err.contains("(truncated)"), "no marker: {err}");
    // Full 400-byte blob must not be present.
    assert!(!err.contains(&"A".repeat(400)), "not truncated: {err}");
    // 200 retained is allowed.
    assert!(err.contains(&"A".repeat(200)), "kept too little: {err}");
}

#[test]
fn error_truncation_is_utf8_safe_on_cyrillic() {
    // Cyrillic chars are multi-byte — a naive byte-slice would panic
    // on a non-char-boundary. Assert the helper survives + produces
    // well-formed UTF-8.
    let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let dir = TempDir::new().unwrap();
    let cyr: String = "Ошибка".repeat(80); // ~480 chars, multi-byte
    install_fake_stderr(dir.path(), "fakebin3", &cyr);
    prep_env(dir.path(), "IGNORED_TOKEN");

    let err = exec::run_void("fakebin3", &["--any"])
        .unwrap_err()
        .to_string();

    // Didn't panic, is valid UTF-8 (it's a Rust String), contains marker.
    assert!(err.contains("(truncated)"), "no marker: {err}");
    assert!(err.contains("Ошибка"), "lost cyrillic: {err}");
}
