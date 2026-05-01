//! Wave 44b F-CRIT-4 regression tests for symlink-based escape attempts
//! against `tool_apply::apply`. Split out of `tool_apply_test.rs` to keep
//! the file under the Constructor Pattern 200-LOC ceiling. Includes the
//! shared helpers from the parent module via `use super::*`.

use super::tests::{req_edit, req_write, state_at};
use super::*;
use std::path::PathBuf;
use tempfile::TempDir;

/// Outside path that the symlink will point at (a real file under another
/// tempdir, simulating exfiltration of `/etc/passwd`-like targets without
/// touching system dirs that `deny_system_dirs` already blocks).
fn make_outside_target() -> (TempDir, PathBuf) {
    let outside = TempDir::new().unwrap();
    let target = outside.path().join("victim.txt");
    std::fs::write(&target, "DO_NOT_OVERWRITE").unwrap();
    (outside, target)
}

#[tokio::test]
async fn write_refuses_to_follow_existing_symlink_at_leaf() {
    use std::os::unix::fs::symlink;
    let tmp = TempDir::new().unwrap();
    let (_outside_keep, target) = make_outside_target();
    // Plant a symlink BEFORE the apply call. resolve_under_root walks up to
    // the deepest existing ancestor (the symlink itself canonicalises to the
    // outside file), so it 403s — but if it ever stopped doing so, the
    // O_NOFOLLOW openat in atomic_write_nofollow would catch the leaf.
    let trap = tmp.path().join("trap.txt");
    symlink(&target, &trap).unwrap();
    let st = state_at(tmp.path());
    let err = apply(State(st), Json(req_write(&trap, "PWNED", true)))
        .await
        .unwrap_err();
    // Either the early canonical check OR the post-write check rejects it.
    assert!(matches!(err, AppError::Forbidden | AppError::Internal(_)));
    let after = std::fs::read_to_string(&target).unwrap();
    assert_eq!(after, "DO_NOT_OVERWRITE", "victim must be untouched");
}

#[tokio::test]
async fn write_post_canonicalize_check_keeps_path_under_root() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("nested/file.txt");
    let st = state_at(tmp.path());
    let resp = apply(State(st), Json(req_write(&path, "ok\n", false)))
        .await
        .unwrap();
    assert!(resp.applied);
    let canon = path.canonicalize().unwrap();
    let root_canon = tmp.path().canonicalize().unwrap();
    assert!(
        canon.starts_with(&root_canon),
        "post-write canon must stay under root"
    );
}

#[tokio::test]
async fn write_succeeds_concurrent_with_unrelated_symlink_in_dir() {
    // A symlink elsewhere in the parent dir must NOT poison the write.
    use std::os::unix::fs::symlink;
    let tmp = TempDir::new().unwrap();
    let (_outside_keep, target) = make_outside_target();
    symlink(&target, tmp.path().join("noise-link")).unwrap();
    let path = tmp.path().join("real.txt");
    let st = state_at(tmp.path());
    let resp = apply(State(st), Json(req_write(&path, "real-content\n", false)))
        .await
        .unwrap();
    assert!(resp.applied);
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "real-content\n");
    // Sanity: the unrelated symlink target is still pristine.
    assert_eq!(std::fs::read_to_string(&target).unwrap(), "DO_NOT_OVERWRITE");
}

#[tokio::test]
async fn edit_via_symlink_at_leaf_is_blocked() {
    // Edit path: file_to_edit is reached via a symlink at the leaf level.
    // O_NOFOLLOW on the staging openat is irrelevant for read, but the
    // post-rename canonical re-check guarantees the final inode lives under
    // the project root.
    use std::os::unix::fs::symlink;
    let tmp = TempDir::new().unwrap();
    let (_outside_keep, target) = make_outside_target();
    let link = tmp.path().join("link.txt");
    symlink(&target, &link).unwrap();
    let st = state_at(tmp.path());
    let err = apply(State(st), Json(req_edit(&link, "DO_NOT_OVERWRITE", "PWNED", false)))
        .await
        .unwrap_err();
    assert!(
        matches!(
            err,
            AppError::Forbidden | AppError::Internal(_) | AppError::NotFound(_)
        ),
        "edit through symlink must not write outside root, got {err:?}"
    );
    assert_eq!(std::fs::read_to_string(&target).unwrap(), "DO_NOT_OVERWRITE");
}
