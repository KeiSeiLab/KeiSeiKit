//! MEDIUM-severity hardening of `safe_join`.
//!
//! Covers two regressions that the original lexical-fallback implementation
//! missed:
//!   1. Accepting a non-existent `base` (no well-defined sandbox).
//!   2. Accepting a symlinked target that escapes `base`.

use kei_atom_discovery::{safe_join, Error};
use std::fs;
use tempfile::tempdir;

#[test]
fn safe_join_rejects_nonexistent_base() {
    let tmp = tempdir().unwrap();
    let ghost = tmp.path().join("does-not-exist");
    // `ghost` was never created → canonicalize fails → safe_join rejects.
    let err = safe_join(&ghost, "schemas/foo.json").expect_err("must reject ghost base");
    assert!(
        matches!(err, Error::Canonicalize { .. }),
        "expected Canonicalize, got {err:?}"
    );
}

#[test]
fn safe_join_accepts_valid_existing_base_and_rel() {
    let tmp = tempdir().unwrap();
    let target = tmp.path().join("schemas");
    fs::create_dir_all(&target).unwrap();
    let joined = safe_join(tmp.path(), "schemas").expect("valid join");
    assert!(joined.ends_with("schemas"));
}

#[test]
fn safe_join_accepts_nonexistent_rel_when_parent_exists() {
    // Parent-dir canonicalize succeeds → no symlink can redirect → accept.
    let tmp = tempdir().unwrap();
    let joined =
        safe_join(tmp.path(), "not-yet-created.json").expect("nonexistent rel should join");
    assert!(joined.ends_with("not-yet-created.json"));
}

#[test]
fn safe_join_accepts_deeply_nonexistent_rel() {
    // Neither the file nor its parent dir exists → no symlink can live here.
    let tmp = tempdir().unwrap();
    let joined = safe_join(tmp.path(), "brand/new/tree/file.json")
        .expect("deeply nonexistent rel should join");
    assert!(joined.ends_with("brand/new/tree/file.json"));
}

#[cfg(unix)]
#[test]
fn safe_join_rejects_symlink_escape() {
    use std::os::unix::fs::symlink as unix_symlink;

    // Layout:
    //   outside_root/secret.json              ← the attacker target
    //   sandbox/                              ← our safe base
    //   sandbox/escape -> ../outside_root     ← symlinked dir
    //
    // `safe_join(sandbox, "escape/secret.json")` must REJECT: after
    // canonicalisation, the resolved path leaves `sandbox`.
    let tmp = tempdir().unwrap();
    let outside_root = tmp.path().join("outside_root");
    let sandbox = tmp.path().join("sandbox");
    fs::create_dir_all(&outside_root).unwrap();
    fs::create_dir_all(&sandbox).unwrap();
    fs::write(outside_root.join("secret.json"), "pwned").unwrap();
    unix_symlink(&outside_root, sandbox.join("escape")).unwrap();

    let err = safe_join(&sandbox, "escape/secret.json")
        .expect_err("symlink-escape must be rejected");
    assert!(
        matches!(err, Error::PathEscape { .. }),
        "expected PathEscape, got {err:?}"
    );
}

#[cfg(unix)]
#[test]
fn safe_join_rejects_symlink_escape_to_nonexistent_target() {
    // Same shape as above, but the dangling target inside outside_root doesn't
    // exist. The parent (`escape`) still canonicalizes into `outside_root`, so
    // the escape must still be detected.
    use std::os::unix::fs::symlink as unix_symlink;

    let tmp = tempdir().unwrap();
    let outside_root = tmp.path().join("outside_root2");
    let sandbox = tmp.path().join("sandbox2");
    fs::create_dir_all(&outside_root).unwrap();
    fs::create_dir_all(&sandbox).unwrap();
    unix_symlink(&outside_root, sandbox.join("escape")).unwrap();

    let err = safe_join(&sandbox, "escape/not-yet.json")
        .expect_err("symlink-escape with nonexistent tail must be rejected");
    assert!(
        matches!(err, Error::PathEscape { .. }),
        "expected PathEscape, got {err:?}"
    );
}

#[cfg(unix)]
#[test]
fn safe_join_accepts_symlink_that_stays_inside_base() {
    // A symlink that resolves BACK INTO the sandbox must still be accepted.
    use std::os::unix::fs::symlink as unix_symlink;

    let tmp = tempdir().unwrap();
    let sandbox = tmp.path().join("sandbox3");
    fs::create_dir_all(sandbox.join("schemas")).unwrap();
    unix_symlink(sandbox.join("schemas"), sandbox.join("alias")).unwrap();

    let ok = safe_join(&sandbox, "alias").expect("inside-base symlink is fine");
    assert!(ok.ends_with("alias"));
}
