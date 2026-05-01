//! Path-layout test for the per-user persistence cube. Asserts that every
//! file resolver returns the exact filename pattern the spec promised:
//!   `<home>/.claude/frustration/<user>.firmware.gz`
//!   `<home>/.claude/frustration/<user>.last-scan.ts`
//!   `<home>/.claude/frustration/<user>.feedback.jsonl`
//!   `<home>/.claude/frustration/queue.jsonl`

use kei_frustration_loop::persistence::{
    ensure_dir, feedback_path, frustration_dir, last_scan_ts_path, queue_path,
    user_firmware_path, FRUSTRATION_DIR,
};
use std::path::Path;
use tempfile::TempDir;

#[test]
fn dir_constant_matches_spec() {
    assert_eq!(FRUSTRATION_DIR, ".claude/frustration");
}

#[test]
fn per_user_paths_match_spec() {
    let home = Path::new("/tmp/fake-home");
    let p = user_firmware_path(home, "alice");
    assert_eq!(p, home.join(".claude/frustration/alice.firmware.gz"));

    let l = last_scan_ts_path(home, "alice");
    assert_eq!(l, home.join(".claude/frustration/alice.last-scan.ts"));

    let f = feedback_path(home, "alice");
    assert_eq!(f, home.join(".claude/frustration/alice.feedback.jsonl"));

    let q = queue_path(home);
    assert_eq!(q, home.join(".claude/frustration/queue.jsonl"));
}

#[test]
fn ensure_dir_creates_directory_with_correct_path() {
    let dir = TempDir::new().unwrap();
    let home = dir.path();
    let resolved = ensure_dir(home).unwrap();
    assert_eq!(resolved, frustration_dir(home));
    assert!(resolved.is_dir(), "ensure_dir should create the dir");
    // Calling twice must be idempotent (no error).
    let again = ensure_dir(home).unwrap();
    assert_eq!(again, resolved);
}

#[cfg(unix)]
#[test]
fn ensure_dir_applies_0700_on_unix() {
    use std::os::unix::fs::PermissionsExt;
    let dir = TempDir::new().unwrap();
    let home = dir.path();
    let p = ensure_dir(home).unwrap();
    let mode = std::fs::metadata(&p).unwrap().permissions().mode() & 0o7777;
    assert_eq!(mode, 0o700, "expected 0700, got {mode:o}");
}

#[test]
fn different_users_get_distinct_paths() {
    let home = Path::new("/h");
    let a = user_firmware_path(home, "alice");
    let b = user_firmware_path(home, "bob");
    assert_ne!(a, b);
    assert!(a.to_string_lossy().contains("alice."));
    assert!(b.to_string_lossy().contains("bob."));
}
