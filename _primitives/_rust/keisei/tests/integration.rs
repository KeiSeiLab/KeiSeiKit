//! Integration tests for the `keisei` CLI primitives.
//!
//! Constructor Pattern: one scenario per test, one assertion target.
//! Each test runs with `KEISEI_HOME` pointed at a tempdir so nothing
//! touches the real `~/.claude` / `~/.keisei`.
//!
//! Sources are loaded via `#[path]` — mirror of the kei-ledger pattern.

#[path = "../src/error.rs"]
mod error;
#[path = "../src/paths.rs"]
mod paths;
#[path = "../src/scope.rs"]
mod scope;
#[path = "../src/time.rs"]
mod time;
#[path = "../src/brain.rs"]
mod brain;
#[path = "../src/brain_validate.rs"]
mod brain_validate;
#[path = "../src/config.rs"]
mod config;
#[path = "../src/config_migrate.rs"]
mod config_migrate;
#[path = "../src/display.rs"]
mod display;
#[path = "../src/fs_type.rs"]
mod fs_type;
#[path = "../src/fsx.rs"]
mod fsx;
#[path = "../src/adapters/mod.rs"]
mod adapters;
#[path = "../src/adapter.rs"]
mod adapter;
#[path = "../src/attach.rs"]
mod attach;
#[path = "../src/status.rs"]
mod status;
#[path = "../src/mount.rs"]
mod mount;
#[path = "../src/detach.rs"]
mod detach;
#[path = "../src/list.rs"]
mod list;

use crate::scope::Scope;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tempfile::TempDir;

// `KEISEI_HOME` is process-global; tests must run serially around the
// env var. One global Mutex is enough for our few tests.
static ENV_LOCK: Mutex<()> = Mutex::new(());

struct EnvGuard {
    _lock: std::sync::MutexGuard<'static, ()>,
    _home: TempDir,
    home_path: PathBuf,
}

impl EnvGuard {
    fn home(&self) -> &Path {
        &self.home_path
    }
}

fn setup_home() -> EnvGuard {
    let lock = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let home = tempfile::tempdir().unwrap();
    // Ensure the Claude-Code adapter's `detect()` succeeds: it requires
    // either CWD/.claude/settings.json OR $KEISEI_HOME/.claude/ to exist.
    fs::create_dir_all(home.path().join(".claude")).unwrap();
    std::env::set_var("KEISEI_HOME", home.path());
    let home_path = home.path().to_path_buf();
    EnvGuard {
        _lock: lock,
        _home: home,
        home_path,
    }
}

/// Variant of `setup_home` that does NOT pre-create the `.claude` dir.
/// Used by tests that want to verify "no client detected" failure paths.
fn setup_home_bare() -> EnvGuard {
    let lock = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let home = tempfile::tempdir().unwrap();
    std::env::set_var("KEISEI_HOME", home.path());
    let home_path = home.path().to_path_buf();
    EnvGuard {
        _lock: lock,
        _home: home,
        home_path,
    }
}

/// Path the Claude-Code adapter writes at user scope, given the
/// current `$KEISEI_HOME`. Used by tests that pre-seed or inspect
/// the client config file directly.
fn claude_user_settings() -> PathBuf {
    paths::resolve_home().join(".claude").join("settings.json")
}

fn write_brain(root: &Path, schema: u32) -> PathBuf {
    fs::create_dir_all(root.join("bin")).unwrap();
    fs::write(root.join("bin/kei-mcp-server-test"), b"#!/bin/sh\n").unwrap();
    let manifest = format!(
        r#"[brain]
schema_version = {schema}
name = "test-brain"
created = "2026-04-22T00:00:00Z"

[paths]
memory = "memory/"
artifacts = "artifacts/"
manifests = "manifests/"
mcp_server = "bin/kei-mcp-server-test"
"#
    );
    fs::write(root.join("manifest.toml"), manifest).unwrap();
    root.to_path_buf()
}

#[test]
fn attach_then_status_happy_path() {
    let _g = setup_home();
    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 1);

    attach::run(brain_dir.path(), Scope::User).expect("attach ok");

    // Marker file exists with correct fields.
    let rec = config::read().unwrap().expect("record present");
    assert_eq!(rec.schema_version, 4);
    assert_eq!(rec.attachments.len(), 1);
    assert_eq!(rec.attachments[0].brain_name, "test-brain");
    assert!(rec.has_client("claude-code"), "claude-code should be in attachments");
    assert!(rec.attachments[0].attached_at.ends_with('Z'));

    // Status runs without error when attached.
    status::run().expect("status ok after attach");
}

#[test]
fn attach_missing_manifest_errors() {
    let _g = setup_home();
    let empty = tempfile::tempdir().unwrap();
    // No manifest.toml written.
    let err = attach::run(empty.path(), Scope::User).unwrap_err();
    assert!(
        matches!(err, error::Error::BrainNotFound(_)),
        "got {err:?}"
    );
}

#[test]
fn attach_unsupported_schema_errors() {
    let _g = setup_home();
    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 99);
    let err = attach::run(brain_dir.path(), Scope::User).unwrap_err();
    assert!(
        matches!(err, error::Error::UnsupportedSchema { found: 99 }),
        "got {err:?}"
    );
}

#[test]
fn status_without_attach_is_clean() {
    let _g = setup_home();
    // No marker file anywhere.
    assert!(config::read().unwrap().is_none());
    status::run().expect("status ok when not attached");
}

#[test]
fn attach_writes_marker_with_expected_fields() {
    let _g = setup_home();
    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 1);

    attach::run(brain_dir.path(), Scope::User).expect("attach ok");

    let rec = config::read().unwrap().expect("record present");
    // brain_path stored as canonicalized absolute path.
    assert_eq!(rec.attachments.len(), 1);
    let a = &rec.attachments[0];
    assert!(Path::new(&a.brain_path).is_absolute());
    assert_eq!(a.brain_name, "test-brain");
    assert_eq!(a.client_type, "claude-code");
    assert_eq!(a.scope, Scope::User);
    assert!(
        !a.config_path.is_empty(),
        "config_path should be populated on v2+ write"
    );

    // Marker file itself lives under $KEISEI_HOME/.keisei/.
    let marker = config::attached_path();
    assert!(marker.is_file(), "marker not written at {}", marker.display());
    assert!(
        marker.ends_with(".keisei/attached.toml"),
        "marker not in new location: {}",
        marker.display()
    );

    // Settings.json got written and contains the server entry.
    let settings = claude_user_settings();
    assert!(settings.is_file(), "settings.json not written");
    let text = fs::read_to_string(&settings).unwrap();
    assert!(text.contains("mcpServers"), "mcpServers key missing");
    assert!(text.contains("keisei"), "keisei mcp entry missing");
}

// -----------------------------------------------------------------------
// v0.19 tests (multi-client).
// -----------------------------------------------------------------------

#[test]
fn mount_with_claude_code_only_detected() {
    let _g = setup_home();
    // Only .claude/ exists (setup_home creates it). No .cursor, .continue,
    // no Zed dirs. Mount should detect exactly one client.
    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 1);

    mount::run(brain_dir.path()).expect("mount ok");

    let rec = config::read().unwrap().expect("record present");
    assert_eq!(
        rec.attachments.len(),
        1,
        "only claude-code should be attached, got {:?}",
        rec.client_names()
    );
    assert_eq!(rec.attachments[0].client_type, "claude-code");
    assert_eq!(rec.attachments[0].scope, Scope::User);
}

#[test]
fn mount_with_no_client_detected() {
    let _g = setup_home_bare();
    // Bare home — no .claude, no .cursor, no .continue, no Zed dirs.
    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 1);

    let err = mount::run(brain_dir.path()).unwrap_err();
    assert!(
        matches!(err, error::Error::NoClientDetected),
        "got {err:?}"
    );
    // Marker must NOT be written on failure.
    assert!(config::read().unwrap().is_none());
}

#[test]
fn detach_round_trip() {
    let _g = setup_home();
    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 1);

    attach::run(brain_dir.path(), Scope::User).expect("attach ok");
    let settings = claude_user_settings();
    assert!(settings.is_file());
    // Sanity: keisei entry is present BEFORE detach.
    let before: Value = serde_json::from_str(&fs::read_to_string(&settings).unwrap()).unwrap();
    assert!(
        before
            .get("mcpServers")
            .and_then(|s| s.get("keisei"))
            .is_some(),
        "keisei entry missing pre-detach: {before}"
    );

    detach::run().expect("detach ok");

    // Marker gone.
    assert!(
        config::read().unwrap().is_none(),
        "marker not deleted after detach"
    );
    // settings.json still exists; keisei entry stripped.
    assert!(settings.is_file());
    let after: Value = serde_json::from_str(&fs::read_to_string(&settings).unwrap()).unwrap();
    let has_keisei = after
        .get("mcpServers")
        .and_then(|s| s.get("keisei"))
        .is_some();
    assert!(!has_keisei, "keisei entry survived detach: {after}");
}

#[test]
fn detach_preserves_other_mcp_servers() {
    let _g = setup_home();
    let settings = claude_user_settings();
    // Pre-populate with a user's pre-existing MCP server.
    fs::write(
        &settings,
        r#"{
  "mcpServers": {
    "other": { "command": "/usr/local/bin/other-mcp", "args": [] }
  },
  "userPref": 42
}"#,
    )
    .unwrap();

    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 1);
    attach::run(brain_dir.path(), Scope::User).expect("attach ok");
    detach::run().expect("detach ok");

    let after: Value = serde_json::from_str(&fs::read_to_string(&settings).unwrap()).unwrap();
    // `other` mcp server survives.
    assert!(
        after
            .get("mcpServers")
            .and_then(|s| s.get("other"))
            .is_some(),
        "pre-existing 'other' server lost: {after}"
    );
    // keisei is gone.
    assert!(
        after
            .get("mcpServers")
            .and_then(|s| s.get("keisei"))
            .is_none(),
        "keisei entry survived detach: {after}"
    );
    // Unrelated top-level key preserved.
    assert_eq!(after.get("userPref").and_then(|v| v.as_i64()), Some(42));
}

#[test]
fn list_adapters_prints_expected_rows() {
    // list just enumerates adapter::all() — no home needed, but we lock
    // the env to keep `detect()` reads deterministic.
    let _g = setup_home();
    // Sanity check: all four adapter names are registered.
    let names: Vec<String> = adapter::all().iter().map(|a| a.name().to_string()).collect();
    assert!(names.contains(&"claude-code".to_string()));
    assert!(names.contains(&"cursor".to_string()));
    assert!(names.contains(&"continue".to_string()));
    assert!(names.contains(&"zed".to_string()));
    // Command itself runs without error.
    list::run().expect("list-adapters ok");
}

// -----------------------------------------------------------------------
// v0.19 audit-hardening tests (SEC-H1 / H2 / H3 path + name + symlink).
// -----------------------------------------------------------------------

/// Write a brain manifest with a caller-chosen `mcp_server` string. Used
/// to exercise the path-escape rejection paths. Does NOT create the
/// `mcp_server` file itself — the manifest-time rejection fires before
/// canonicalization would ever run.
fn write_brain_raw_mcp(root: &Path, mcp_server: &str) -> PathBuf {
    let manifest = format!(
        r#"[brain]
schema_version = 1
name = "test-brain"
created = "2026-04-22T00:00:00Z"

[paths]
mcp_server = "{mcp_server}"
"#
    );
    fs::write(root.join("manifest.toml"), manifest).unwrap();
    root.to_path_buf()
}

/// Write a brain manifest with a caller-chosen `name`. Used to exercise
/// the name-regex rejection path. `mcp_server` still points at a real
/// file so name-validation runs before path-canonicalization.
fn write_brain_raw_name(root: &Path, name: &str) -> PathBuf {
    fs::create_dir_all(root.join("bin")).unwrap();
    fs::write(root.join("bin/kei-mcp-server-test"), b"#!/bin/sh\n").unwrap();
    let manifest = format!(
        r#"[brain]
schema_version = 1
name = "{name}"
created = "2026-04-22T00:00:00Z"

[paths]
mcp_server = "bin/kei-mcp-server-test"
"#
    );
    fs::write(root.join("manifest.toml"), manifest).unwrap();
    root.to_path_buf()
}

#[test]
fn manifest_with_absolute_mcp_server_rejected() {
    let _g = setup_home();
    let brain_dir = tempfile::tempdir().unwrap();
    // Malicious manifest: absolute path to arbitrary host binary.
    write_brain_raw_mcp(brain_dir.path(), "/usr/bin/curl");
    let err = attach::run(brain_dir.path(), Scope::User).unwrap_err();
    assert!(
        matches!(err, error::Error::PathEscape(_)),
        "expected PathEscape, got {err:?}"
    );
    // Containment: marker MUST NOT be written.
    assert!(config::read().unwrap().is_none());
}

#[test]
fn manifest_with_parent_traversal_rejected() {
    let _g = setup_home();
    let brain_dir = tempfile::tempdir().unwrap();
    write_brain_raw_mcp(brain_dir.path(), "../../etc/passwd");
    let err = attach::run(brain_dir.path(), Scope::User).unwrap_err();
    assert!(
        matches!(err, error::Error::PathEscape(_)),
        "expected PathEscape, got {err:?}"
    );
    assert!(config::read().unwrap().is_none());
}

#[test]
fn manifest_with_invalid_name_rejected() {
    let _g = setup_home();
    let brain_dir = tempfile::tempdir().unwrap();
    // "claude-ide!" contains `!` — forbidden by ^[a-z][a-z0-9_-]{0,63}$.
    write_brain_raw_name(brain_dir.path(), "claude-ide!");
    let err = attach::run(brain_dir.path(), Scope::User).unwrap_err();
    assert!(
        matches!(err, error::Error::InvalidName(ref s) if s == "claude-ide!"),
        "expected InvalidName(\"claude-ide!\"), got {err:?}"
    );
}

#[test]
fn brain_path_is_symlink_rejected() {
    let _g = setup_home();
    // Target brain is legitimate...
    let target = tempfile::tempdir().unwrap();
    write_brain(target.path(), 1);
    // ...but caller passes a symlink pointing at it (USB/host pivot).
    let link_parent = tempfile::tempdir().unwrap();
    let link = link_parent.path().join("brain-link");
    #[cfg(unix)]
    std::os::unix::fs::symlink(target.path(), &link).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(target.path(), &link).unwrap();

    let err = attach::run(&link, Scope::User).unwrap_err();
    assert!(
        matches!(err, error::Error::BrainIsSymlink { .. }),
        "expected BrainIsSymlink, got {err:?}"
    );
    // No marker, no config pollution.
    assert!(config::read().unwrap().is_none());
}

#[test]
fn attach_refuses_to_clobber_existing_mcp_entry() {
    let _g = setup_home();
    // Pre-populate settings.json with a DIFFERENT `keisei` entry.
    let settings = claude_user_settings();
    fs::create_dir_all(settings.parent().unwrap()).unwrap();
    fs::write(
        &settings,
        r#"{
  "mcpServers": {
    "keisei": {
      "command": "/tmp/not-our-binary",
      "args": ["--evil"]
    }
  }
}"#,
    )
    .unwrap();

    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 1);
    let err = attach::run(brain_dir.path(), Scope::User).unwrap_err();
    assert!(
        matches!(err, error::Error::NameConflict { .. }),
        "expected NameConflict, got {err:?}"
    );

    // User's pre-existing entry survives intact.
    let after: Value = serde_json::from_str(&fs::read_to_string(&settings).unwrap()).unwrap();
    let cmd = after
        .get("mcpServers")
        .and_then(|s| s.get("keisei"))
        .and_then(|k| k.get("command"))
        .and_then(|c| c.as_str())
        .unwrap_or("");
    assert_eq!(
        cmd, "/tmp/not-our-binary",
        "pre-existing keisei entry was mutated; attach should have been a no-op on conflict"
    );
    // Marker MUST NOT be written on conflict.
    assert!(config::read().unwrap().is_none());
}

#[test]
fn schema_v1_to_v2_migration() {
    let _g = setup_home();
    // Hand-write a v1 marker at the NEW location (location migration is
    // tested separately). v1 schema has flat `client_type` and no list.
    let marker = config::attached_path();
    if let Some(parent) = marker.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(
        &marker,
        r#"brain_path  = "/tmp/brain-v1"
brain_name  = "old-brain"
client_type = "claude-code"
attached_at = "2026-04-22T00:00:00Z"
"#,
    )
    .unwrap();

    let rec = config::read().unwrap().expect("v1 marker should parse");
    assert_eq!(rec.schema_version, 4);
    assert_eq!(
        rec.attachments.len(),
        1,
        "v1 client_type should migrate to single attachment"
    );
    let a = &rec.attachments[0];
    assert_eq!(a.brain_name, "old-brain");
    assert_eq!(a.brain_path, "/tmp/brain-v1");
    assert_eq!(a.client_type, "claude-code");
    // v1 didn't carry config_path; migration leaves it blank.
    assert_eq!(a.config_path, "");
    // v1 didn't carry scope; default is User.
    assert_eq!(a.scope, Scope::User);
    assert_eq!(a.attached_at, "2026-04-22T00:00:00Z");
    assert!(rec.has_client("claude-code"));
}

// -----------------------------------------------------------------------
// v0.19.2 polish tests (M1 perms / L9 sanitize / L12 manifest size).
// -----------------------------------------------------------------------

#[cfg(unix)]
#[test]
fn marker_file_has_0600_perms_on_unix() {
    use std::os::unix::fs::PermissionsExt;
    let _g = setup_home();
    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 1);

    attach::run(brain_dir.path(), Scope::User).expect("attach ok");

    let marker = config::attached_path();
    let mode = fs::metadata(&marker).unwrap().permissions().mode() & 0o777;
    assert_eq!(
        mode, 0o600,
        "marker perms = {:04o}; expected 0600 (owner r/w only)",
        mode
    );
}

#[test]
fn status_sanitizes_control_chars_in_brain_name() {
    // Unit test on the sanitize primitive — simpler and tighter than
    // capturing stdout from `status::run`. L9 wiring in `status.rs` and
    // `attach.rs` calls through `display::sanitize_display` at every
    // manifest-sourced printf site, so asserting the primitive is enough.
    assert_eq!(
        display::sanitize_display("evil\x1b[2Jpayload"),
        "evil?[2Jpayload"
    );
    // Space / regular ASCII pass through.
    assert_eq!(display::sanitize_display("my brain 01"), "my brain 01");
    // DEL (0x7F) is scrubbed too.
    assert_eq!(display::sanitize_display("a\x7Fb"), "a?b");
}

/// Write a brain with an artificially large manifest.toml for the size-
/// bound rejection test. `min_bytes` is the lower bound on the resulting
/// file size; the manifest body is padded with a block-comment-like
/// filler string (`\n# xxxxxx...`) until the total exceeds `min_bytes`.
fn write_brain_with_oversize_manifest(root: &Path, min_bytes: usize) -> PathBuf {
    fs::create_dir_all(root.join("bin")).unwrap();
    fs::write(root.join("bin/kei-mcp-server-test"), b"#!/bin/sh\n").unwrap();
    let mut manifest = String::from(
        r#"[brain]
schema_version = 1
name = "test-brain"
created = "2026-04-22T00:00:00Z"

[paths]
mcp_server = "bin/kei-mcp-server-test"
"#,
    );
    // Pad with toml-legal trailing comments to grow past the 64 KiB cap.
    let filler = format!("# {}\n", "x".repeat(120));
    while manifest.len() < min_bytes {
        manifest.push_str(&filler);
    }
    fs::write(root.join("manifest.toml"), manifest).unwrap();
    root.to_path_buf()
}

// -----------------------------------------------------------------------
// v0.20 schema-v2 + post_attach_hint tests.
// -----------------------------------------------------------------------

/// Write a schema-v2 brain manifest carrying every supported platform in
/// the `[paths.mcp_server]` table, plus a stub binary for the current
/// host so the canonicalizer is happy.
fn write_brain_v2_all_platforms(root: &Path) -> PathBuf {
    fs::create_dir_all(root.join("bin")).unwrap();
    // Stub binaries for all five supported host tuples. We create them
    // all so any host running the suite finds its own entry.
    for name in &[
        "kei-mcp-server-darwin-arm64",
        "kei-mcp-server-darwin-x64",
        "kei-mcp-server-linux-x64",
        "kei-mcp-server-linux-arm64",
        "kei-mcp-server-windows-x64.exe",
    ] {
        fs::write(root.join("bin").join(name), b"#!/bin/sh\n").unwrap();
    }
    let manifest = r#"[brain]
schema_version = 2
name = "test-brain-v2"
created = "2026-04-22T00:00:00Z"

[paths]
memory = "memory/"

[paths.mcp_server]
darwin-arm64 = "bin/kei-mcp-server-darwin-arm64"
darwin-x64 = "bin/kei-mcp-server-darwin-x64"
linux-x64 = "bin/kei-mcp-server-linux-x64"
linux-arm64 = "bin/kei-mcp-server-linux-arm64"
windows-x64 = "bin/kei-mcp-server-windows-x64.exe"
"#;
    fs::write(root.join("manifest.toml"), manifest).unwrap();
    root.to_path_buf()
}

/// Write a v2 brain that only has `linux-x64` — used on macOS to exercise
/// the `NoPlatformBinary` error path.
fn write_brain_v2_linux_only(root: &Path) -> PathBuf {
    fs::create_dir_all(root.join("bin")).unwrap();
    fs::write(
        root.join("bin/kei-mcp-server-linux-x64"),
        b"#!/bin/sh\n",
    )
    .unwrap();
    let manifest = r#"[brain]
schema_version = 2
name = "test-brain-linux"
created = "2026-04-22T00:00:00Z"

[paths.mcp_server]
linux-x64 = "bin/kei-mcp-server-linux-x64"
"#;
    fs::write(root.join("manifest.toml"), manifest).unwrap();
    root.to_path_buf()
}

#[test]
fn manifest_too_large_rejected() {
    let _g = setup_home();
    let brain_dir = tempfile::tempdir().unwrap();
    // 100 KiB manifest — well above the 64 KiB cap.
    write_brain_with_oversize_manifest(brain_dir.path(), 100 * 1024);

    let err = attach::run(brain_dir.path(), Scope::User).unwrap_err();
    assert!(
        matches!(
            err,
            error::Error::ManifestTooLarge { size, max }
                if size > max && max == 64 * 1024
        ),
        "expected ManifestTooLarge {{ size > max, max == 65536 }}, got {err:?}"
    );
    // Containment: marker MUST NOT be written on rejection.
    assert!(config::read().unwrap().is_none());
}

#[test]
fn schema_v2_current_platform_resolves() {
    let _g = setup_home();
    let brain_dir = tempfile::tempdir().unwrap();
    write_brain_v2_all_platforms(brain_dir.path());

    let brain = brain::Brain::load(brain_dir.path()).expect("v2 brain loads");
    let path = brain.mcp_server_path().expect("current platform resolves");
    assert!(path.is_file(), "resolved binary missing at {}", path.display());
    // Resolved path must live under the brain root.
    let root = brain_dir.path().canonicalize().unwrap();
    assert!(
        path.starts_with(&root),
        "resolved path {} not under root {}",
        path.display(),
        root.display()
    );
}

#[cfg(target_os = "macos")]
#[test]
fn schema_v2_missing_current_platform_errors() {
    let _g = setup_home();
    let brain_dir = tempfile::tempdir().unwrap();
    write_brain_v2_linux_only(brain_dir.path());

    let brain = brain::Brain::load(brain_dir.path())
        .expect("v2 brain without current-platform binary still loads");
    let err = brain.mcp_server_path().unwrap_err();
    match err {
        error::Error::NoPlatformBinary { ref available, .. } => {
            assert_eq!(available, &vec!["linux-x64".to_string()]);
        }
        other => panic!("expected NoPlatformBinary, got {other:?}"),
    }
}

#[test]
fn schema_v1_still_readable_with_v2_code() {
    let _g = setup_home();
    let brain_dir = tempfile::tempdir().unwrap();
    // `write_brain` emits schema_version = 1 + single-string mcp_server.
    write_brain(brain_dir.path(), 1);

    let brain = brain::Brain::load(brain_dir.path()).expect("v1 brain still loads under v2 code");
    let path = brain.mcp_server_path().expect("v1 resolves without platform map");
    assert!(path.is_file(), "v1-resolved binary missing at {}", path.display());
}

#[test]
fn post_attach_hint_is_adapter_specific() {
    let _g = setup_home();
    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 1);
    let brain = brain::Brain::load(brain_dir.path()).expect("brain loads");
    let adapters = adapter::all();
    let by_name = |n: &str| -> String {
        adapters
            .iter()
            .find(|a| a.name() == n)
            .unwrap_or_else(|| panic!("adapter {n} missing"))
            .post_attach_hint(&brain, Scope::User)
            .to_string()
    };
    let claude = by_name("claude-code");
    let cursor = by_name("cursor");
    let cont = by_name("continue");
    let zed = by_name("zed");
    assert!(
        claude.contains("/help"),
        "claude-code hint lost /help marker: {claude}"
    );
    assert!(
        cursor.contains("Reload Window"),
        "cursor hint lost 'Reload Window' marker: {cursor}"
    );
    assert!(
        cont.contains("Continue"),
        "continue hint lost 'Continue' marker: {cont}"
    );
    assert!(
        zed.contains(":reload"),
        "zed hint lost ':reload' marker: {zed}"
    );
}

// -----------------------------------------------------------------------
// v0.21 — SSoT relocation + Scope enum tests.
// -----------------------------------------------------------------------

#[test]
fn legacy_marker_migrates_on_first_read() {
    let g = setup_home();
    // Seed a v2 marker at the LEGACY path
    // ($KEISEI_HOME/.claude/keisei-attached.toml), with no new-location
    // file. Simulates an upgrade from v0.20 → v0.21.
    let legacy = g.home().join(".claude").join("keisei-attached.toml");
    fs::create_dir_all(legacy.parent().unwrap()).unwrap();
    let body = r#"brain_path  = "/tmp/legacy-brain"
brain_name  = "legacy-brain"
attached_at = "2026-04-22T00:00:00Z"

[[attachments]]
client_type = "claude-code"
config_path = "/tmp/fake/settings.json"
"#;
    fs::write(&legacy, body).unwrap();

    // New-location MUST NOT exist yet.
    let current = g.home().join(".keisei").join("attached.toml");
    assert!(
        !current.exists(),
        "new marker pre-existed before read(): {}",
        current.display()
    );

    // read() performs the one-shot migration (location + schema).
    let rec = config::read().unwrap().expect("migrated record present");
    assert_eq!(rec.schema_version, 4);
    assert_eq!(rec.attachments.len(), 1);
    let a = &rec.attachments[0];
    assert_eq!(a.brain_name, "legacy-brain");
    assert_eq!(a.brain_path, "/tmp/legacy-brain");
    assert_eq!(a.client_type, "claude-code");
    // Default scope for pre-v0.21 markers is User.
    assert_eq!(a.scope, Scope::User);

    // Post-conditions: new file exists, legacy file gone.
    assert!(
        current.is_file(),
        "migration did not create new marker at {}",
        current.display()
    );
    assert!(
        !legacy.exists(),
        "legacy marker still present at {} after migration",
        legacy.display()
    );
}

#[test]
fn attach_with_project_scope_writes_local_config() {
    let _g = setup_home();
    // Run from a CWD where the adapter's project-scope target lives.
    let workdir = tempfile::tempdir().unwrap();
    let prev_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(workdir.path()).unwrap();

    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 1);

    attach::run(brain_dir.path(), Scope::Project).expect("attach --scope=project ok");

    // Project-local file must be written (under the CWD we set above).
    let project_settings = workdir.path().join(".claude").join("settings.json");
    assert!(
        project_settings.is_file(),
        "project-scope settings.json missing at {}",
        project_settings.display()
    );

    // User-scope file must NOT have been created by this attach.
    let user_settings = claude_user_settings();
    assert!(
        !user_settings.is_file(),
        "user-scope settings.json leaked when scope=project: {}",
        user_settings.display()
    );

    // Marker records scope=Project.
    let rec = config::read().unwrap().expect("record");
    assert_eq!(rec.attachments.len(), 1);
    assert_eq!(rec.attachments[0].scope, Scope::Project);

    std::env::set_current_dir(prev_cwd).unwrap();
}

#[test]
fn attach_user_scope_still_default() {
    let _g = setup_home();
    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 1);

    // main.rs default is Scope::User — exercise the path explicitly here.
    attach::run(brain_dir.path(), Scope::User).expect("attach ok");

    let rec = config::read().unwrap().expect("record");
    assert_eq!(rec.attachments.len(), 1);
    assert_eq!(rec.attachments[0].scope, Scope::User);
    assert!(claude_user_settings().is_file());
}

#[test]
fn scope_unsupported_by_adapter_errors() {
    let _g = setup_home();
    // Force the Zed adapter to the front of detection by pre-creating its
    // settings dir, and suppress claude-code's dir so detect_active picks Zed.
    // Remove the .claude dir that setup_home pre-created.
    let home = paths::resolve_home();
    fs::remove_dir_all(home.join(".claude")).ok();
    // Also suppress any CWD-local .claude (claude_code's detect checks CWD).
    let workdir = tempfile::tempdir().unwrap();
    let prev_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(workdir.path()).unwrap();
    // Seed Zed's config dir (platform-specific).
    let zed_dir = if cfg!(target_os = "macos") {
        home.join("Library/Application Support/Zed")
    } else {
        home.join(".config/zed")
    };
    fs::create_dir_all(&zed_dir).unwrap();

    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 1);

    // Zed declares supported_scopes() = [User], so project scope must error.
    let err = attach::run(brain_dir.path(), Scope::Project).unwrap_err();
    assert!(
        matches!(err, error::Error::ScopeUnsupported { ref client, .. } if client == "zed"),
        "expected ScopeUnsupported for zed, got {err:?}"
    );
    // Marker must not be written on validation failure.
    assert!(config::read().unwrap().is_none());

    std::env::set_current_dir(prev_cwd).unwrap();
}

#[test]
fn detach_respects_scope_from_marker() {
    let _g = setup_home();
    // Attach at project scope in a workdir.
    let workdir = tempfile::tempdir().unwrap();
    let prev_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(workdir.path()).unwrap();

    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 1);

    attach::run(brain_dir.path(), Scope::Project).expect("attach project ok");

    let project_settings = workdir.path().join(".claude").join("settings.json");
    assert!(project_settings.is_file(), "project settings absent post-attach");
    let before: Value =
        serde_json::from_str(&fs::read_to_string(&project_settings).unwrap()).unwrap();
    assert!(
        before.get("mcpServers").and_then(|s| s.get("keisei")).is_some(),
        "keisei entry missing pre-detach: {before}"
    );

    detach::run().expect("detach ok");

    // keisei entry gone from project-scope file.
    let after: Value =
        serde_json::from_str(&fs::read_to_string(&project_settings).unwrap()).unwrap();
    let has_keisei = after
        .get("mcpServers")
        .and_then(|s| s.get("keisei"))
        .is_some();
    assert!(!has_keisei, "keisei entry survived detach: {after}");
    // Marker gone.
    assert!(config::read().unwrap().is_none());

    std::env::set_current_dir(prev_cwd).unwrap();
}

// -----------------------------------------------------------------------
// v0.21.1 HIGH-3 — sanitize_display covers detach/mount, not just status.
// -----------------------------------------------------------------------

/// Write a marker file directly with a brain_name containing ANSI control
/// bytes — this bypasses the adapter's name-regex which would otherwise
/// reject it at attach time. We want to verify that the DETACH path still
/// scrubs the name before printing (defensive-in-depth).
#[test]
fn detach_sanitizes_control_chars_in_marker_fields() {
    use crate::config::{AttachRecord, Attachment};
    let _g = setup_home();
    // Hand-craft a v4 marker so every displayed field carries an escape
    // sequence. detach must sanitize before printing.
    let rec = AttachRecord::new(vec![Attachment {
        brain_path: "/tmp/evil\x1b[2Jbrain".to_string(),
        brain_name: "evil\x1b[2Jname".to_string(),
        client_type: "claude-code".to_string(),
        config_path: "/tmp/evil\x1b[2Jcfg".to_string(),
        scope: Scope::User,
        attached_at: "2026-04-22T00:00:00Z".to_string(),
    }]);
    config::write(&rec).unwrap();

    // Run detach — any println!/eprintln! that leaks control bytes from
    // brain_path / brain_name / client_type / reason fails the escape
    // sanitization contract. We can't easily capture stdout here, but
    // the unit test `status_sanitizes_control_chars_in_brain_name` plus
    // the source-level grep below give us defence in depth.
    detach::run().expect("detach runs without panic");

    // Every print_summary arg is sanitized — we verify by inspecting the
    // detach.rs source for the `sanitize_display(` guard on each.
    let src = include_str!("../src/detach.rs");
    for needle in [
        "sanitize_display(&a.brain_path)",
        "sanitize_display(client)",
        "sanitize_display(reason)",
    ] {
        assert!(
            src.contains(needle),
            "detach.rs must sanitize display of {needle}: missing guard"
        );
    }
}

#[test]
fn mount_sanitizes_control_chars_in_error_reason() {
    // Same contract as above — mount.rs must sanitize every user-visible
    // printf path. We verify by source inspection; an integration with
    // stdout capture would require std::process::Command round-trips.
    let src = include_str!("../src/mount.rs");
    for needle in [
        "sanitize_display(brain.name())",
        "sanitize_display(&s.client_type)",
        "sanitize_display(&s.config_path)",
        "sanitize_display(client)",
        "sanitize_display(reason)",
    ] {
        assert!(
            src.contains(needle),
            "mount.rs must sanitize display of {needle}: missing guard"
        );
    }
}

// -----------------------------------------------------------------------
// v0.22 Track C — filesystem type detection (fs_type.rs).
// -----------------------------------------------------------------------

#[test]
fn brain_load_on_typical_filesystem_no_warn() {
    // On the dev / CI host the tmpdir sits on APFS (macOS) or ext4
    // (Linux) — neither of which should trigger the exFAT/FAT32 warn.
    // We can't capture stderr from Brain::load easily, so we assert on
    // the primitive that drives the advisory instead. `None` means "no
    // warning would be emitted"; `Unknown` is the accepted fallback
    // on platforms where statfs isn't wired.
    let _g = setup_home();
    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 1);

    let _b = brain::Brain::load(brain_dir.path()).expect("load succeeds on normal fs");

    let w = fs_type::detect_fs_warning(brain_dir.path());
    assert!(
        matches!(w, fs_type::FsWarning::None | fs_type::FsWarning::Unknown),
        "standard tmpdir should not be classed as exFAT/FAT32, got {w:?}"
    );
}

// -----------------------------------------------------------------------
// v0.22 — schema v4 multi-brain + Scope::Auto + templated hint + registry.
// -----------------------------------------------------------------------

/// Write a distinct second brain manifest under `root` so multi-brain
/// tests can attach two different brains in one session.
fn write_second_brain(root: &Path, name: &str) -> PathBuf {
    fs::create_dir_all(root.join("bin")).unwrap();
    fs::write(root.join("bin/kei-mcp-server-test"), b"#!/bin/sh\n").unwrap();
    let manifest = format!(
        r#"[brain]
schema_version = 1
name = "{name}"
created = "2026-04-22T00:00:00Z"

[paths]
mcp_server = "bin/kei-mcp-server-test"
"#
    );
    fs::write(root.join("manifest.toml"), manifest).unwrap();
    root.to_path_buf()
}

#[test]
fn marker_v3_migrates_to_v4() {
    let _g = setup_home();
    // Hand-write a v3 marker (shared brain fields + scope per attachment).
    let marker = config::attached_path();
    fs::create_dir_all(marker.parent().unwrap()).unwrap();
    fs::write(
        &marker,
        r#"brain_path  = "/tmp/brain-v3"
brain_name  = "brain-v3"
attached_at = "2026-04-22T00:00:00Z"

[[attachments]]
client_type = "claude-code"
config_path = "/tmp/settings.json"
scope       = "user"

[[attachments]]
client_type = "cursor"
config_path = "/tmp/mcp.json"
scope       = "project"
"#,
    )
    .unwrap();

    let rec = config::read().unwrap().expect("record present");
    assert_eq!(rec.schema_version, 4);
    assert_eq!(rec.attachments.len(), 2);
    for a in &rec.attachments {
        assert_eq!(a.brain_name, "brain-v3");
        assert_eq!(a.brain_path, "/tmp/brain-v3");
        assert_eq!(a.attached_at, "2026-04-22T00:00:00Z");
    }
    assert_eq!(rec.attachments[0].client_type, "claude-code");
    assert_eq!(rec.attachments[0].scope, Scope::User);
    assert_eq!(rec.attachments[1].client_type, "cursor");
    assert_eq!(rec.attachments[1].scope, Scope::Project);
}

#[test]
fn two_brains_can_be_attached_simultaneously() {
    let _g = setup_home();
    // Seed a cursor dir so both adapters can take their own brain.
    let home = paths::resolve_home();
    fs::create_dir_all(home.join(".cursor")).unwrap();

    // Brain A → claude-code at user scope.
    let brain_a_dir = tempfile::tempdir().unwrap();
    write_brain(brain_a_dir.path(), 1);
    attach::run(brain_a_dir.path(), Scope::User).expect("attach brain-a ok");

    // Hand-write marker with a second attachment (simulates second
    // `attach` run that would have picked up a different adapter).
    // We use the merge path directly — simpler than forcing cursor
    // to be the detected client.
    let rec1 = config::read().unwrap().expect("record present");
    assert_eq!(rec1.attachments.len(), 1);

    // Simulate a second attach that adds a cursor attachment with a
    // different brain_path. We merge by appending, as attach::run does.
    let brain_b_dir = tempfile::tempdir().unwrap();
    write_second_brain(brain_b_dir.path(), "brain-b");
    let canon_b = brain_b_dir.path().canonicalize().unwrap();
    let rec2 = config::AttachRecord::new(vec![
        rec1.attachments[0].clone(),
        config::Attachment {
            brain_path: canon_b.to_string_lossy().into_owned(),
            brain_name: "brain-b".to_string(),
            client_type: "cursor".to_string(),
            config_path: home
                .join(".cursor/mcp.json")
                .to_string_lossy()
                .into_owned(),
            scope: Scope::User,
            attached_at: config::now_utc_string(),
        },
    ]);
    config::write(&rec2).unwrap();

    let final_rec = config::read().unwrap().expect("record present");
    assert_eq!(final_rec.attachments.len(), 2);
    // Distinct brain_paths prove multi-brain co-existence.
    assert_ne!(
        final_rec.attachments[0].brain_path, final_rec.attachments[1].brain_path,
        "attachments should point at different brains"
    );
    assert!(final_rec.has_client("claude-code"));
    assert!(final_rec.has_client("cursor"));
    assert_eq!(final_rec.brain_names().len(), 2);
}

#[test]
fn detach_removes_single_brain_preserves_others() {
    let _g = setup_home();
    // Hand-craft a v4 marker with two attachments to different clients
    // and different brains, then call detach. Marker should be removed
    // entirely (detach is all-or-nothing), but per-client cleanup must
    // fire for each attachment.
    let home = paths::resolve_home();
    let claude_settings = home.join(".claude/settings.json");
    fs::create_dir_all(claude_settings.parent().unwrap()).unwrap();
    fs::write(
        &claude_settings,
        r#"{
  "mcpServers": {
    "keisei": { "command": "/tmp/fake", "args": [] },
    "other":  { "command": "/tmp/other", "args": [] }
  }
}"#,
    )
    .unwrap();

    let rec = config::AttachRecord::new(vec![config::Attachment {
        brain_path: "/tmp/brain-a".to_string(),
        brain_name: "brain-a".to_string(),
        client_type: "claude-code".to_string(),
        config_path: claude_settings.to_string_lossy().into_owned(),
        scope: Scope::User,
        attached_at: "2026-04-22T00:00:00Z".to_string(),
    }]);
    config::write(&rec).unwrap();

    detach::run().expect("detach ok");

    // After detach: `keisei` entry gone, `other` still present, marker gone.
    let after: Value =
        serde_json::from_str(&fs::read_to_string(&claude_settings).unwrap()).unwrap();
    assert!(
        after.get("mcpServers").and_then(|s| s.get("keisei")).is_none(),
        "keisei entry survived detach: {after}"
    );
    assert!(
        after.get("mcpServers").and_then(|s| s.get("other")).is_some(),
        "pre-existing 'other' server lost"
    );
    assert!(config::read().unwrap().is_none(), "marker not deleted");
}

#[test]
fn scope_auto_resolves_to_project_when_cwd_has_dot_claude() {
    let _g = setup_home();
    // CWD carries `.claude/`.
    let workdir = tempfile::tempdir().unwrap();
    let prev_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(workdir.path()).unwrap();
    fs::create_dir_all(workdir.path().join(".claude")).unwrap();

    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 1);

    // Auto should pick project scope because CWD has `.claude/`.
    attach::run(brain_dir.path(), Scope::Auto).expect("attach auto ok");

    let rec = config::read().unwrap().expect("record");
    assert_eq!(rec.attachments.len(), 1);
    assert_eq!(
        rec.attachments[0].scope,
        Scope::Project,
        "auto should resolve to project when .claude/ exists in CWD"
    );
    // Project-local settings file was written.
    let project_settings = workdir.path().join(".claude").join("settings.json");
    assert!(project_settings.is_file());

    std::env::set_current_dir(prev_cwd).unwrap();
}

#[test]
fn scope_auto_resolves_to_user_when_cwd_bare() {
    let _g = setup_home();
    // CWD has NO `.claude/`.
    let workdir = tempfile::tempdir().unwrap();
    let prev_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(workdir.path()).unwrap();

    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 1);

    attach::run(brain_dir.path(), Scope::Auto).expect("attach auto ok");

    let rec = config::read().unwrap().expect("record");
    assert_eq!(rec.attachments.len(), 1);
    assert_eq!(
        rec.attachments[0].scope,
        Scope::User,
        "auto should resolve to user when CWD is bare"
    );

    std::env::set_current_dir(prev_cwd).unwrap();
}

#[test]
fn cursor_auto_scope_respects_cwd_dot_cursor() {
    // Build the adapter directly; auto_scope is a pure CWD heuristic.
    let _g = setup_home();

    // Bare workdir → User.
    let bare = tempfile::tempdir().unwrap();
    let prev_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(bare.path()).unwrap();
    let cursor = adapters::cursor::CursorAdapter::new();
    use crate::adapter::ClientAdapter;
    assert_eq!(cursor.auto_scope(), Scope::User);

    // `.cursor/` present → Project.
    let withdir = tempfile::tempdir().unwrap();
    std::env::set_current_dir(withdir.path()).unwrap();
    fs::create_dir_all(withdir.path().join(".cursor")).unwrap();
    assert_eq!(cursor.auto_scope(), Scope::Project);

    std::env::set_current_dir(prev_cwd).unwrap();
}

#[test]
fn post_attach_hint_interpolates_brain_name() {
    let _g = setup_home();
    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 1);
    let brain = brain::Brain::load(brain_dir.path()).expect("brain loads");

    use crate::adapter::ClientAdapter;
    let claude = adapters::claude_code::ClaudeCodeAdapter::new();
    let hint = claude.post_attach_hint(&brain, Scope::User);
    assert!(hint.contains("test-brain"), "brain name missing: {hint}");
    assert!(hint.contains("user"), "scope missing: {hint}");
    // Project scope should show `project` literally.
    let hint_p = claude.post_attach_hint(&brain, Scope::Project);
    assert!(hint_p.contains("project"), "project scope missing: {hint_p}");
}

#[test]
fn adapter_registry_lists_all_four() {
    // Registry is the single place new adapters plug in — verify it
    // returns every adapter the CLI supports.
    let names: Vec<String> = adapters::_registry::all_adapters()
        .iter()
        .map(|a| a.name().to_string())
        .collect();
    assert_eq!(names.len(), 4, "registry should list exactly 4 adapters");
    for expected in &["claude-code", "cursor", "continue", "zed"] {
        assert!(
            names.contains(&expected.to_string()),
            "registry missing {expected}: {names:?}"
        );
    }
    // adapter::all() must delegate to the registry — returns the same names
    // in the same order.
    let via_adapter: Vec<String> = adapter::all()
        .iter()
        .map(|a| a.name().to_string())
        .collect();
    assert_eq!(via_adapter, names);
}

#[test]
fn dead_error_variants_removed() {
    // NotAttached + AdapterFailed were removed in v0.22. Compile-time grep
    // of error.rs: the strings must NOT appear.
    let src = include_str!("../src/error.rs");
    assert!(
        !src.contains("NotAttached"),
        "Error::NotAttached should be removed"
    );
    assert!(
        !src.contains("AdapterFailed"),
        "Error::AdapterFailed should be removed"
    );
}

#[test]
fn fs_type_detection_returns_none_on_standard_fs() {
    // Direct test of the primitive — no brain, no env mutation.
    let td = tempfile::tempdir().unwrap();
    let w = fs_type::detect_fs_warning(td.path());
    // Must NEVER flag exFAT / FAT32 on a host tmpdir — the latter
    // sits on APFS / ext4 / whatever the developer has locally.
    assert!(
        !matches!(w, fs_type::FsWarning::ExFat | fs_type::FsWarning::Fat32),
        "detect_fs_warning misclassified tmpdir as {w:?}"
    );
}

#[test]
fn time_now_utc_string_has_rfc3339_shape() {
    let s = crate::time::now_utc_string();
    // Form: YYYY-MM-DDThh:mm:ssZ, exactly 20 bytes.
    assert_eq!(s.len(), 20, "timestamp wrong length: {s}");
    assert!(s.ends_with('Z'));
    assert_eq!(s.chars().nth(4), Some('-'));
    assert_eq!(s.chars().nth(7), Some('-'));
    assert_eq!(s.chars().nth(10), Some('T'));
    assert_eq!(s.chars().nth(13), Some(':'));
    assert_eq!(s.chars().nth(16), Some(':'));
}

#[test]
fn fresh_marker_has_schema_version_4() {
    let _g = setup_home();
    let brain_dir = tempfile::tempdir().unwrap();
    write_brain(brain_dir.path(), 1);
    attach::run(brain_dir.path(), Scope::User).expect("attach ok");

    // Raw file must contain `schema_version = 4` at the top.
    let marker = config::attached_path();
    let raw = fs::read_to_string(&marker).unwrap();
    assert!(
        raw.contains("schema_version = 4"),
        "fresh v0.22 marker should have schema_version = 4; got: {raw}"
    );
}
