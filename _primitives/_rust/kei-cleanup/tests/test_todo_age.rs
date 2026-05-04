//! Integration test for the todo_age scanner.
//!
//! Builds a tmp git repo with a TODO line, commits it, runs scanner.
//! If `git` is missing on the test host, the scanner returns
//! `ToolNotFound` and the test marks itself as a graceful skip.

use kei_cleanup::{scanners, Config, FindingKind};
use std::fs;
use std::process::Command;
use tempfile::TempDir;

fn git_available() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn run(cmd: &mut Command) {
    let out = cmd.output().expect("spawn git");
    assert!(out.status.success(), "git failed: {out:?}");
}

#[test]
fn todo_age_emits_finding_for_committed_todo() {
    if !git_available() {
        eprintln!("skipping: git not on PATH");
        return;
    }
    let tmp = TempDir::new().expect("tmp");
    let dir = tmp.path();
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"f\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    fs::write(
        dir.join("src/lib.rs"),
        "// TODO: refactor this into smaller pieces\npub fn x() {}\n",
    )
    .unwrap();

    run(Command::new("git").args(["init", "-q"]).current_dir(dir));
    run(Command::new("git")
        .args(["config", "user.email", "t@e"])
        .current_dir(dir));
    run(Command::new("git")
        .args(["config", "user.name", "t"])
        .current_dir(dir));
    run(Command::new("git").args(["add", "."]).current_dir(dir));
    run(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(dir));

    let cfg = Config::default();
    let findings = scanners::todo_age::scan(dir, &cfg).expect("scan");
    assert!(
        findings.iter().any(|f| f.kind == FindingKind::StaleTodo),
        "expected ≥1 StaleTodo, got {findings:?}"
    );
}
