//! kei-task CLI exit-code smoke tests (§Runtime contract).
//!
//! Atom-layer errors (validation / semantic) → exit 2.
//! Storage/IO errors → exit 1.
//!
//! `create --title ""` is the canonical validation-failure case: the
//! atom's typed Error enum returns `InvalidTitle`, which main.rs maps
//! to exit 2, NOT the old anyhow collapse at exit 1.

use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_kei-task");

#[test]
fn create_empty_title_exits_2() {
    let tmp = tempfile::tempdir().unwrap();
    let db = tmp.path().join("task.sqlite");
    let out = Command::new(BIN)
        .arg("--db")
        .arg(&db)
        .arg("create")
        .arg("")
        .output()
        .expect("spawn kei-task");
    assert_eq!(out.status.code(), Some(2),
        "expected exit 2 on InvalidTitle; stdout: {}, stderr: {}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("InvalidTitle"),
        "expected 'InvalidTitle' in stderr: {stderr}");
}
