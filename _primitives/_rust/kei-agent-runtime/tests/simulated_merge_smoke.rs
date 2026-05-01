//! Simulated-merge smoke test — initialize a tempdir git repo, create a
//! feature branch with a file change, run the simulated-merge flow, assert
//! the temp worktree contains the agent's change on top of main.

use kei_agent_runtime::simulated_merge::{glob_match, run_simulated_merge};
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

fn sh(dir: &Path, args: &[&str]) {
    let out = Command::new("git").args(args).current_dir(dir).output().unwrap();
    assert!(
        out.status.success(),
        "git {}: {}",
        args.join(" "),
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn simulated_merge_applies_agent_diff() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    sh(repo, &["init", "-q", "-b", "main"]);
    sh(repo, &["config", "user.email", "t@t"]);
    sh(repo, &["config", "user.name", "t"]);
    std::fs::write(repo.join("README.md"), "seed\n").unwrap();
    sh(repo, &["add", "."]);
    sh(repo, &["commit", "-q", "-m", "seed"]);

    // Agent makes a change on a feature branch
    sh(repo, &["checkout", "-q", "-b", "agent/x"]);
    std::fs::write(repo.join("new.txt"), "agent wrote this\n").unwrap();
    sh(repo, &["add", "."]);
    sh(repo, &["commit", "-q", "-m", "agent change"]);

    let merged = run_simulated_merge("test123", repo, repo).expect("simulated merge");
    let content = std::fs::read_to_string(merged.join("new.txt"))
        .expect("agent diff applied in merged worktree");
    assert_eq!(content, "agent wrote this\n");

    // Cleanup
    let _ = Command::new("git")
        .args(["worktree", "remove", "--force", merged.to_str().unwrap()])
        .current_dir(repo)
        .output();
}

#[test]
fn glob_match_handles_double_star() {
    assert!(glob_match("_primitives/_rust/kei-forge/**", "_primitives/_rust/kei-forge/src/lib.rs"));
    assert!(!glob_match("_primitives/_rust/kei-forge/**", "hooks/foo.sh"));
}

#[test]
fn glob_match_single_star_path_component() {
    assert!(glob_match("src/*.rs", "src/main.rs"));
    assert!(!glob_match("src/*.rs", "src/mod/main.rs"));
}

#[test]
fn glob_match_exact_path() {
    assert!(glob_match("Cargo.toml", "Cargo.toml"));
    assert!(!glob_match("Cargo.toml", "src/Cargo.toml"));
}
