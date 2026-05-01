//! `write`-tool branch tests for `tool_apply::apply`. Split out of
//! `tool_apply_test.rs` to keep both files under the Constructor Pattern
//! 200-LOC ceiling. Shares helpers via `super::tests::*`.

use super::tests::{req_write, state_at};
use super::*;
use tempfile::TempDir;

fn write_file(root: &std::path::Path, name: &str, content: &str) -> std::path::PathBuf {
    let p = root.join(name);
    std::fs::write(&p, content).unwrap();
    p
}

#[tokio::test]
async fn write_409_rejects_existing_path_without_force() {
    let tmp = TempDir::new().unwrap();
    let path = write_file(tmp.path(), "exists.txt", "old");
    let st = state_at(tmp.path());
    let err = apply(State(st), Json(req_write(&path, "new", false)))
        .await
        .unwrap_err();
    assert!(matches!(err, AppError::Conflict(_)));
}

#[tokio::test]
async fn write_succeeds_on_new_path_creating_parents() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("nested/deep/new.txt");
    let st = state_at(tmp.path());
    let resp = apply(State(st), Json(req_write(&path, "hello\nworld\n", false)))
        .await
        .unwrap();
    assert!(resp.applied);
    assert_eq!(resp.diff_summary.lines_changed, 2);
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "hello\nworld\n");
}

#[tokio::test]
async fn write_succeeds_on_existing_path_with_force() {
    let tmp = TempDir::new().unwrap();
    let path = write_file(tmp.path(), "exists.txt", "old content");
    let st = state_at(tmp.path());
    let resp = apply(State(st), Json(req_write(&path, "fresh", true)))
        .await
        .unwrap();
    assert!(resp.applied);
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "fresh");
}

#[tokio::test]
async fn write_400_on_missing_content() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("new.txt");
    let st = state_at(tmp.path());
    let req = ApplyRequest {
        tool: Some("write".into()),
        path: path.to_string_lossy().into(),
        old_string: None,
        new_string: None,
        old_text: None,
        new_text: None,
        content: None,
        replace_all: false,
        force: false,
    };
    let err = apply(State(st), Json(req)).await.unwrap_err();
    assert!(matches!(err, AppError::BadRequest(_)));
}

#[tokio::test]
async fn unknown_tool_returns_400() {
    let tmp = TempDir::new().unwrap();
    let path = write_file(tmp.path(), "f.txt", "x");
    let st = state_at(tmp.path());
    let req = ApplyRequest {
        tool: Some("delete".into()),
        path: path.to_string_lossy().into(),
        old_string: None,
        new_string: None,
        old_text: None,
        new_text: None,
        content: None,
        replace_all: false,
        force: false,
    };
    let err = apply(State(st), Json(req)).await.unwrap_err();
    assert!(matches!(err, AppError::BadRequest(_)));
}
