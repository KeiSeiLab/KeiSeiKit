//! Integration test — `kei-runtime invoke` actually executes `kei-task::create`.
//!
//! Wire-up:
//!   1. Pre-build `kei-task` in the workspace target dir.
//!   2. Point the `--root` at the workspace's `_primitives/_rust/` so the
//!      runtime discovers the real atom metadata (`kei-task/atoms/create.md`).
//!   3. Point `KEI_RUNTIME_BIN_DIR` at the target dir so the runtime resolves
//!      the `kei-task` binary without polluting $PATH.
//!   4. Invoke → expect exit 0 and a JSON result containing `id` as integer.

use std::path::{Path, PathBuf};
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_kei-runtime");

/// Absolute path to `_primitives/_rust/` (the atom-discovery root).
fn rust_root() -> PathBuf {
    // This file lives in `_primitives/_rust/kei-runtime/tests/`.
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    here.parent().expect("_primitives/_rust").to_path_buf()
}

/// Build `kei-task` so the runtime can spawn it. Uses the current profile's
/// target dir, then hands that dir to the invoke via KEI_RUNTIME_BIN_DIR.
fn build_kei_task_and_target_dir() -> PathBuf {
    let rust_root = rust_root();
    let status = Command::new(env!("CARGO"))
        .arg("build")
        .arg("-p")
        .arg("kei-task")
        .arg("--quiet")
        .current_dir(&rust_root)
        .status()
        .expect("cargo build kei-task");
    assert!(status.success(), "cargo build kei-task failed");
    // `target` dir — try explicit override first, then fallback to `target/debug`.
    if let Ok(t) = std::env::var("CARGO_TARGET_DIR") {
        return PathBuf::from(t).join("debug");
    }
    rust_root.join("target").join("debug")
}

#[test]
fn invoke_kei_task_create_returns_id() {
    let bin_dir = build_kei_task_and_target_dir();
    assert!(
        bin_dir.join("kei-task").is_file(),
        "kei-task binary not at {}/kei-task",
        bin_dir.display()
    );
    let tmp = tempfile::tempdir().unwrap();
    let db = tmp.path().join("task.sqlite");
    let out = Command::new(BIN)
        .env("KEI_RUNTIME_BIN_DIR", &bin_dir)
        .env("KEI_TASK_DB", &db)
        .arg("invoke")
        .arg("kei-task::create")
        .arg("--input")
        .arg(r#"{"title":"integration"}"#)
        .arg("--root")
        .arg(rust_root())
        .output()
        .expect("spawn kei-runtime");
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert_eq!(
        out.status.code(),
        Some(0),
        "expected exit 0; stdout: {stdout}; stderr: {stderr}"
    );
    let parsed: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("stdout is JSON");
    // Output shape: { "atom": "kei-task::create", "result": { "id": N, "created_at": ... } }
    let result = parsed.get("result").expect("result field");
    let id = result
        .get("id")
        .expect("id field on result")
        .as_i64()
        .expect("id is integer");
    assert!(id >= 1, "id must be a positive integer, got {id}");
}
