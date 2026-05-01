//! Integration tests for kei-graph-check.

use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_kei-graph-check"))
}

fn write(root: &Path, rel: &str, body: &str) {
    let full = root.join(rel);
    if let Some(parent) = full.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(full, body).unwrap();
}

#[test]
fn clean_graph_exits_zero() {
    let tmp = TempDir::new().unwrap();
    write(tmp.path(), "a.md", "see [[b]]");
    write(tmp.path(), "b.md", "hello");
    let out = std::process::Command::new(bin())
        .args(["--path"])
        .arg(tmp.path())
        .output()
        .unwrap();
    assert!(out.status.success(), "stderr: {}", String::from_utf8_lossy(&out.stderr));
}

#[test]
fn broken_wikilink_exits_two() {
    let tmp = TempDir::new().unwrap();
    write(tmp.path(), "a.md", "see [[ghost]]");
    let out = std::process::Command::new(bin())
        .args(["--path"])
        .arg(tmp.path())
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn patch_removal_breaks_graph() {
    let tmp = TempDir::new().unwrap();
    write(tmp.path(), "a.md", "see [[b]]");
    write(tmp.path(), "b.md", "hello");
    let patch = tmp.path().join("p.patch");
    fs::write(&patch, "# removed: b.md\n").unwrap();
    let out = std::process::Command::new(bin())
        .args(["--path"])
        .arg(tmp.path())
        .args(["--after-diff"])
        .arg(&patch)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn json_output_schema() {
    let tmp = TempDir::new().unwrap();
    write(tmp.path(), "a.md", "see [[ghost]]");
    let out = std::process::Command::new(bin())
        .args(["--path"])
        .arg(tmp.path())
        .arg("--json")
        .output()
        .unwrap();
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["broken_count"], 1);
    assert_eq!(v["broken"][0]["kind"], "wikilink");
    assert_eq!(v["broken"][0]["target"], "ghost");
}
