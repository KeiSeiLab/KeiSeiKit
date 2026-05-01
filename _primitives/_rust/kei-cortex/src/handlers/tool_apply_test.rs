//! Inline unit tests for `tool_apply.rs`. Drives the handler against a
//! tempdir-rooted `AppState`, exercising both edit and write semantics.

use super::*;
use crate::config::AppConfig;
use std::path::PathBuf;
use tempfile::TempDir;

/// Build a minimal `AppState` pinned to `root` as project_root.
pub(super) fn state_at(root: &Path) -> AppState {
    let canon = root.canonicalize().unwrap();
    let cfg = AppConfig::try_new(
        None, None, None, None, None, None, None,
        Some(canon.clone()), Some(canon), None, None,
    ).unwrap();
    AppState::new(cfg, "test-token".into())
}

fn write_file(root: &Path, name: &str, content: &str) -> PathBuf {
    let p = root.join(name);
    std::fs::write(&p, content).unwrap();
    p
}

pub(super) fn req_edit(path: &Path, old: &str, new: &str, replace_all: bool) -> ApplyRequest {
    ApplyRequest {
        tool: None, path: path.to_string_lossy().into(),
        old_string: Some(old.into()), new_string: Some(new.into()),
        old_text: None, new_text: None, content: None,
        replace_all, force: false,
    }
}

pub(super) fn req_write(path: &Path, content: &str, force: bool) -> ApplyRequest {
    ApplyRequest {
        tool: Some("write".into()), path: path.to_string_lossy().into(),
        old_string: None, new_string: None, old_text: None, new_text: None,
        content: Some(content.into()),
        replace_all: false, force,
    }
}

#[tokio::test]
async fn edit_applies_cleanly_atomic_rename() {
    let tmp = TempDir::new().unwrap();
    let path = write_file(tmp.path(), "f.txt", "alpha beta gamma");
    let st = state_at(tmp.path());
    let resp = apply(State(st), Json(req_edit(&path, "beta", "BETA", false)))
        .await.unwrap();
    assert!(resp.applied);
    assert_eq!(resp.diff_summary.lines_changed, 1);
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "alpha BETA gamma");
}

#[tokio::test]
async fn edit_409_on_duplicate_without_replace_all() {
    let tmp = TempDir::new().unwrap();
    let path = write_file(tmp.path(), "f.txt", "foo foo foo");
    let st = state_at(tmp.path());
    let err = apply(State(st), Json(req_edit(&path, "foo", "bar", false)))
        .await.unwrap_err();
    assert!(matches!(err, AppError::Conflict(_)));
}

#[tokio::test]
async fn edit_409_on_missing_old_string() {
    let tmp = TempDir::new().unwrap();
    let path = write_file(tmp.path(), "f.txt", "alpha");
    let st = state_at(tmp.path());
    let err = apply(State(st), Json(req_edit(&path, "missing", "X", false)))
        .await.unwrap_err();
    assert!(matches!(err, AppError::Conflict(_)));
}

#[tokio::test]
async fn edit_403_on_path_outside_project_root() {
    let tmp = TempDir::new().unwrap();
    let st = state_at(tmp.path());
    let outside = Path::new("/tmp/definitely-not-under-this-root-xyz.txt");
    let err = apply(State(st), Json(req_edit(outside, "a", "b", false)))
        .await.unwrap_err();
    assert!(matches!(err, AppError::Forbidden));
}

#[tokio::test]
async fn edit_403_on_system_dir() {
    let tmp = TempDir::new().unwrap();
    let st = state_at(tmp.path());
    let sys = Path::new("/etc/passwd-like");
    let err = apply(State(st), Json(req_edit(sys, "a", "b", false)))
        .await.unwrap_err();
    assert!(matches!(err, AppError::Forbidden));
}

#[tokio::test]
async fn edit_400_on_relative_path() {
    let tmp = TempDir::new().unwrap();
    let st = state_at(tmp.path());
    let rel = Path::new("relative/path.txt");
    let err = apply(State(st), Json(req_edit(rel, "a", "b", false)))
        .await.unwrap_err();
    assert!(matches!(err, AppError::BadRequest(_)));
}

#[tokio::test]
async fn edit_413_on_oversize_file() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("big.bin");
    let big = "x".repeat((MAX_BYTES + 1) as usize);
    std::fs::write(&path, &big).unwrap();
    let st = state_at(tmp.path());
    let err = apply(State(st), Json(req_edit(&path, "x", "y", false)))
        .await.unwrap_err();
    assert!(matches!(err, AppError::PayloadTooLarge(_)));
}

#[tokio::test]
async fn edit_404_on_missing_file() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("does-not-exist.txt");
    let st = state_at(tmp.path());
    let err = apply(State(st), Json(req_edit(&path, "x", "y", false)))
        .await.unwrap_err();
    assert!(matches!(err, AppError::NotFound(_)));
}

#[tokio::test]
async fn edit_replace_all_changes_every_occurrence() {
    let tmp = TempDir::new().unwrap();
    let path = write_file(tmp.path(), "f.txt", "foo foo foo");
    let st = state_at(tmp.path());
    let resp = apply(State(st), Json(req_edit(&path, "foo", "bar", true)))
        .await.unwrap();
    assert!(resp.applied);
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "bar bar bar");
}

#[tokio::test]
async fn edit_400_on_empty_old_string() {
    let tmp = TempDir::new().unwrap();
    let path = write_file(tmp.path(), "f.txt", "anything");
    let st = state_at(tmp.path());
    let err = apply(State(st), Json(req_edit(&path, "", "X", false)))
        .await.unwrap_err();
    assert!(matches!(err, AppError::BadRequest(_)));
}

#[tokio::test]
async fn edit_accepts_old_text_new_text_alias() {
    let tmp = TempDir::new().unwrap();
    let path = write_file(tmp.path(), "f.txt", "one two three");
    let st = state_at(tmp.path());
    let req = ApplyRequest {
        tool: None, path: path.to_string_lossy().into(),
        old_string: None, new_string: None,
        old_text: Some("two".into()), new_text: Some("TWO".into()),
        content: None, replace_all: false, force: false,
    };
    let resp = apply(State(st), Json(req)).await.unwrap();
    assert!(resp.applied);
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "one TWO three");
}

// `write`-tool branch tests live in `tool_apply_write_test.rs`.
// Symlink-escape regression tests live in `tool_apply_symlink_test.rs`
// (Wave 44b F-CRIT-4). Both included as sibling test modules from
// `tool_apply.rs`.
