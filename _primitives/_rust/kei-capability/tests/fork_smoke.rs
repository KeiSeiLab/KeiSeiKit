//! Smoke tests for `kei-capability fork`.

use kei_capability::fork;
use std::path::Path;
use tempfile::TempDir;

const FIXED_NOW: &str = "2026-04-23T00:00:00Z";
const SRC_TEXT: &str = "## Test capability\n\nBody line one.\nBody line two.\n";
const SRC_TOML: &str = r#"[capability]
name = "policy::no-git-ops"
category = "policy"
version = "1.0"
description = "Forbid git operations."
rationale = "RULE 0.13."

[restricts]
tool-patterns = ['^git( |$)']
tools-denied = []

[parameterized]
accepts = []

[text]
path = "text.md"

[gate]
rust-module = "gates::policy_no_git_ops"
event = "PreToolUse:Bash"
severity = "block"
"#;

fn seed_source(kit_root: &Path, cat: &str, slug: &str) {
    let dir = kit_root.join("_capabilities").join(cat).join(slug);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("capability.toml"), SRC_TOML).unwrap();
    std::fs::write(dir.join("text.md"), SRC_TEXT).unwrap();
}

#[test]
fn fork_creates_target_with_lineage() {
    let tmp = TempDir::new().unwrap();
    seed_source(tmp.path(), "policy", "no-git-ops");
    let summary = fork::run_fork(
        "policy::no-git-ops",
        "policy::no-git-ops-lax",
        tmp.path(),
        FIXED_NOW,
    )
    .expect("fork should succeed");
    assert_eq!(summary.target, "policy::no-git-ops-lax");
    assert!(summary.diff_count >= 1);
    let target_dir = tmp
        .path()
        .join("_capabilities")
        .join("policy")
        .join("no-git-ops-lax");
    assert!(target_dir.join("capability.toml").exists());
    assert!(target_dir.join("text.md").exists());
    let out = std::fs::read_to_string(target_dir.join("capability.toml")).unwrap();
    let parsed: toml::Value = toml::from_str(&out).unwrap();
    let cap = parsed.get("capability").and_then(|v| v.as_table()).unwrap();
    assert_eq!(cap.get("name").unwrap().as_str(), Some("policy::no-git-ops-lax"));
    let lin = parsed.get("lineage").and_then(|v| v.as_table()).unwrap();
    assert_eq!(
        lin.get("fork_from").unwrap().as_str(),
        Some("policy::no-git-ops")
    );
    let parents = lin.get("parents").and_then(|v| v.as_array()).unwrap();
    assert_eq!(parents.len(), 1);
    assert_eq!(parents[0].as_str(), Some("policy::no-git-ops"));
    assert_eq!(lin.get("created").unwrap().as_str(), Some(FIXED_NOW));
    assert!(lin.get("creator").unwrap().as_str().is_some());
}

#[test]
fn fork_refuses_when_target_exists() {
    let tmp = TempDir::new().unwrap();
    seed_source(tmp.path(), "policy", "no-git-ops");
    // Pre-create target so fork must refuse to clobber.
    let target = tmp
        .path()
        .join("_capabilities")
        .join("policy")
        .join("no-git-ops-lax");
    std::fs::create_dir_all(&target).unwrap();
    let err = fork::run_fork(
        "policy::no-git-ops",
        "policy::no-git-ops-lax",
        tmp.path(),
        FIXED_NOW,
    )
    .expect_err("fork should refuse existing target");
    let msg = format!("{err:#}");
    assert!(msg.contains("already exists"), "expected clobber refusal, got: {msg}");
}

#[test]
fn fork_validates_new_name_regex() {
    let tmp = TempDir::new().unwrap();
    seed_source(tmp.path(), "policy", "no-git-ops");
    // Upper-case and bad chars must be rejected via shared NAME_RE.
    assert!(fork::run_fork(
        "policy::no-git-ops",
        "Policy::no-git-ops-lax",
        tmp.path(),
        FIXED_NOW,
    )
    .is_err());
    assert!(fork::run_fork(
        "policy::no-git-ops",
        "policy::BadSlug",
        tmp.path(),
        FIXED_NOW,
    )
    .is_err());
    // Missing separator.
    assert!(fork::run_fork(
        "policy::no-git-ops",
        "no-separator",
        tmp.path(),
        FIXED_NOW,
    )
    .is_err());
}

#[test]
fn fork_copies_text_md_byte_identical() {
    let tmp = TempDir::new().unwrap();
    seed_source(tmp.path(), "policy", "no-git-ops");
    fork::run_fork(
        "policy::no-git-ops",
        "policy::no-git-ops-lax",
        tmp.path(),
        FIXED_NOW,
    )
    .unwrap();
    let target_text = tmp
        .path()
        .join("_capabilities")
        .join("policy")
        .join("no-git-ops-lax")
        .join("text.md");
    let copied = std::fs::read(&target_text).unwrap();
    assert_eq!(copied, SRC_TEXT.as_bytes());
}
