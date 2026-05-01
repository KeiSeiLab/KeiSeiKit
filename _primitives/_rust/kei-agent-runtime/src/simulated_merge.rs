//! Simulated-merge executor + glob matcher.
//!
//! Schema §Verify execution — worktree short-circuit → simulated merge:
//! orchestrator creates temp worktree off main, applies agent's diff, runs
//! verifies from that vantage to catch integration regressions invisible
//! in agent's isolated worktree.

use crate::validate::validate_agent_id;
use anyhow::{anyhow, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Create a temp worktree off `main_repo` at HEAD of `main`, apply the agent's
/// diff, return the temp worktree path. Caller cleans up.
///
/// Validates `agent_id` before constructing any tmp path — path-traversal
/// defence per the HIGH-risk agent_id sink audit.
pub fn run_simulated_merge(
    agent_id: &str,
    agent_worktree: &Path,
    main_repo: &Path,
) -> Result<PathBuf> {
    validate_agent_id(agent_id)
        .map_err(|e| anyhow!("agent_id rejected in run_simulated_merge: {e}"))?;
    let tmp = std::env::temp_dir().join(format!("kei-test-merge-{agent_id}"));
    let _ = std::fs::remove_dir_all(&tmp);
    run_git(main_repo, &["worktree", "add", "-d", tmp.to_str().unwrap(), "main"])
        .context("git worktree add failed")?;
    let diff = run_git(agent_worktree, &["diff", "main"])
        .context("git diff against main failed")?;
    if !diff.trim().is_empty() {
        apply_diff(&tmp, &diff)?;
    }
    Ok(tmp)
}

/// Apply a unified diff to `dir` via `git apply --index`. Empty diff is a no-op.
pub fn apply_diff(dir: &Path, diff: &str) -> Result<()> {
    use std::io::Write;
    let mut child = Command::new("git")
        .arg("apply")
        .arg("--index")
        .current_dir(dir)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .context("spawn git apply")?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(diff.as_bytes()).context("write diff stdin")?;
    }
    let out = child.wait_with_output().context("git apply wait")?;
    if !out.status.success() {
        anyhow::bail!("git apply failed: {}", String::from_utf8_lossy(&out.stderr));
    }
    Ok(())
}

/// Run `git <args>` in `dir`, return stdout as UTF-8 string.
pub fn run_git(dir: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .with_context(|| format!("git {}", args.join(" ")))?;
    if !out.status.success() {
        anyhow::bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// Shell-style glob match. Supports `**` (any directories) and `*` (any chars
/// except `/`). Bracketed classes and `?` not supported — task specs use
/// simple patterns.
pub fn glob_match(pattern: &str, path: &str) -> bool {
    let re = glob_to_regex(pattern);
    match regex::Regex::new(&re) {
        Ok(r) => r.is_match(path),
        Err(_) => false,
    }
}

fn glob_to_regex(pattern: &str) -> String {
    let mut out = String::from("^");
    let bytes = pattern.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c == '*' && i + 1 < bytes.len() && bytes[i + 1] as char == '*' {
            out.push_str(".*");
            i += 2;
            if i < bytes.len() && bytes[i] as char == '/' {
                i += 1;
            }
        } else if c == '*' {
            out.push_str("[^/]*");
            i += 1;
        } else if "().+?|^$\\[]{}".contains(c) {
            out.push('\\');
            out.push(c);
            i += 1;
        } else {
            out.push(c);
            i += 1;
        }
    }
    out.push('$');
    out
}
