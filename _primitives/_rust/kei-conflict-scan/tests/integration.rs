//! Integration tests for kei-conflict-scan.

use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_kei-conflict-scan"))
}

fn write(root: &Path, rel: &str, body: &str) {
    let full = root.join(rel);
    if let Some(parent) = full.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&full, body).unwrap();
}

fn run(root: &Path, extra: &[&str]) -> serde_json::Value {
    let mut args = vec!["--path".to_string(), root.to_string_lossy().into_owned()];
    args.extend(extra.iter().map(|s| s.to_string()));
    let out = std::process::Command::new(bin()).args(&args).output().unwrap();
    assert!(out.status.success(), "stderr: {}", String::from_utf8_lossy(&out.stderr));
    serde_json::from_slice(&out.stdout).unwrap()
}

#[test]
fn empty_tree_is_clean() {
    let tmp = TempDir::new().unwrap();
    let v = run(tmp.path(), &[]);
    assert_eq!(v["hit_count"], 0);
}

#[test]
fn contradictory_rules_flagged() {
    let tmp = TempDir::new().unwrap();
    write(tmp.path(), "rules/a.md", "Never: push to github\n");
    write(tmp.path(), "rules/b.md", "Always: push to github\n");
    let v = run(tmp.path(), &["--only", "rules"]);
    assert!(v["hit_count"].as_u64().unwrap() >= 1, "{}", v);
    assert_eq!(v["conflicts"][0]["category"], "rules");
}

#[test]
fn duplicate_blocks_flagged() {
    let tmp = TempDir::new().unwrap();
    let body =
        "this is a long shared paragraph with many identical words over and over again repeated";
    write(tmp.path(), "_blocks/a.md", body);
    write(tmp.path(), "_blocks/b.md", body);
    let v = run(tmp.path(), &["--only", "blocks"]);
    assert!(v["hit_count"].as_u64().unwrap() >= 1, "{}", v);
    assert_eq!(v["conflicts"][0]["category"], "blocks");
}

#[test]
fn orphan_wikilinks_flagged() {
    let tmp = TempDir::new().unwrap();
    write(tmp.path(), "docs/a.md", "see [[nonexistent-target]] for details");
    let v = run(tmp.path(), &["--only", "orphans"]);
    assert!(v["hit_count"].as_u64().unwrap() >= 1, "{}", v);
    assert_eq!(v["conflicts"][0]["category"], "orphans");
}

#[test]
fn cross_repo_wikilink_not_flagged() {
    // `[[../../../rules/X]]` escapes the scan root — engine cannot validate,
    // must not false-positive.
    let tmp = TempDir::new().unwrap();
    write(tmp.path(), "memory/MEMORY.md", "see [[../../../rules/recurrence-escalate]]");
    let v = run(tmp.path(), &["--only", "orphans"]);
    assert_eq!(v["hit_count"].as_u64().unwrap(), 0, "{}", v);
}

#[test]
fn path_prefixed_wikilink_matches_basename() {
    // `[[chatlogs/X/Y]]` should resolve when `Y.md` exists anywhere in the tree.
    let tmp = TempDir::new().unwrap();
    write(tmp.path(), "chatlogs/X/Y.md", "target body");
    write(tmp.path(), "memory/index.md", "ref to [[chatlogs/X/Y]]");
    let v = run(tmp.path(), &["--only", "orphans"]);
    assert_eq!(v["hit_count"].as_u64().unwrap(), 0, "{}", v);
}

#[test]
fn oversize_file_flagged() {
    let tmp = TempDir::new().unwrap();
    let mut body = String::new();
    for _ in 0..250 {
        body.push_str("line\n");
    }
    write(tmp.path(), "src/big.rs", &body);
    let v = run(tmp.path(), &["--only", "cp"]);
    assert!(v["hit_count"].as_u64().unwrap() >= 1, "{}", v);
    assert_eq!(v["conflicts"][0]["category"], "cp");
}

#[test]
fn json_schema_has_required_fields() {
    let tmp = TempDir::new().unwrap();
    write(tmp.path(), "rules/a.md", "Never: do X\n");
    write(tmp.path(), "rules/b.md", "Always: do X\n");
    let v = run(tmp.path(), &["--only", "rules"]);
    let c = &v["conflicts"][0];
    for k in ["category", "severity", "files", "evidence", "suggested_fix", "auto_resolvable"] {
        assert!(c.get(k).is_some(), "missing field {}: {}", k, c);
    }
}
