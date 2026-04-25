//! Validates the edit tool's unique-match guarantee:
//! - duplicate `old_string` without `replace_all` errors as `NotUnique`
//! - unique `old_string` succeeds and updates the file
//! - `replace_all = true` rewrites every occurrence and reports the count
//! - missing `old_string` errors with NotUnique
//! - `old_string == new_string` errors with InvalidInput

use crate::tool::edit;
use crate::tool::types::ToolError;

#[tokio::test]
async fn duplicate_without_replace_all_errors() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("dup.txt");
    tokio::fs::write(&path, "foo bar foo").await.unwrap();
    let raw = serde_json::json!({
        "path": path.to_str().unwrap(),
        "old_string": "foo",
        "new_string": "FOO",
    });
    let err = edit::run(raw, dir.path()).await.unwrap_err();
    match err {
        ToolError::NotUnique(msg) => assert!(msg.contains("matched 2 times")),
        other => panic!("expected NotUnique, got {other:?}"),
    }
    // File must NOT have been modified.
    let after = tokio::fs::read_to_string(&path).await.unwrap();
    assert_eq!(after, "foo bar foo");
}

#[tokio::test]
async fn unique_match_succeeds_and_persists() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("unique.txt");
    tokio::fs::write(&path, "alpha beta gamma").await.unwrap();
    let raw = serde_json::json!({
        "path": path.to_str().unwrap(),
        "old_string": "beta",
        "new_string": "BETA",
    });
    let msg = edit::run(raw, dir.path()).await.unwrap();
    assert!(msg.contains("1 replacement"));
    let after = tokio::fs::read_to_string(&path).await.unwrap();
    assert_eq!(after, "alpha BETA gamma");
}

#[tokio::test]
async fn replace_all_rewrites_every_occurrence() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("ra.txt");
    tokio::fs::write(&path, "x x x x x").await.unwrap();
    let raw = serde_json::json!({
        "path": path.to_str().unwrap(),
        "old_string": "x",
        "new_string": "y",
        "replace_all": true,
    });
    let msg = edit::run(raw, dir.path()).await.unwrap();
    assert!(msg.contains("5 replacement"));
    let after = tokio::fs::read_to_string(&path).await.unwrap();
    assert_eq!(after, "y y y y y");
}

#[tokio::test]
async fn missing_old_string_errors() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("miss.txt");
    tokio::fs::write(&path, "alpha beta").await.unwrap();
    let raw = serde_json::json!({
        "path": path.to_str().unwrap(),
        "old_string": "gamma",
        "new_string": "delta",
    });
    let err = edit::run(raw, dir.path()).await.unwrap_err();
    assert!(matches!(err, ToolError::NotUnique(_)));
}

#[tokio::test]
async fn equal_old_and_new_errors() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("noop.txt");
    tokio::fs::write(&path, "hi").await.unwrap();
    let raw = serde_json::json!({
        "path": path.to_str().unwrap(),
        "old_string": "hi",
        "new_string": "hi",
    });
    let err = edit::run(raw, dir.path()).await.unwrap_err();
    assert!(matches!(err, ToolError::InvalidInput(_)));
}
