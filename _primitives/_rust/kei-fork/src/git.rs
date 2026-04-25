//! Thin `Command::new(git_bin())` wrappers.
//!
//! Every helper runs `git` in `kit_root` (or a specified worktree),
//! captures stdout/stderr, and returns `Error::Git` on non-zero exit.
//! No parsing beyond `trim()` on stdout — callers interpret the string.
//!
//! PATH hijack mitigation (HIGH #4): the git binary is resolved via
//! `git_bin()`, which honours `KEI_FORK_GIT_BIN` if set. Ops can pin to
//! an absolute path (e.g. `/usr/bin/git`) in trusted environments.
//!
//! Arg-injection mitigation (HIGH #3): `worktree_add` uses the `--`
//! sentinel before the base commit-ish and validates the refname shape.

use crate::error::Error;
use std::ffi::OsString;
use std::path::Path;
use std::process::{Command, Output};

/// Resolve the `git` binary. Honours `KEI_FORK_GIT_BIN` for hardening.
pub fn git_bin() -> OsString {
    std::env::var_os("KEI_FORK_GIT_BIN").unwrap_or_else(|| OsString::from("git"))
}

fn run(cmd_desc: &str, c: &mut Command) -> Result<Output, Error> {
    let out = c.output().map_err(Error::Io)?;
    if !out.status.success() {
        return Err(Error::Git {
            cmd: cmd_desc.to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    Ok(out)
}

/// Conservative git refname validator. Accepts the subset we emit and
/// the subset a caller may reasonably pass as `base`. Rejects leading
/// `-` (option injection), NUL, newline, and characters outside a
/// deliberately narrow allowlist.
pub fn is_safe_refname(s: &str) -> bool {
    if s.is_empty() || s.len() > 255 {
        return false;
    }
    let first = s.as_bytes()[0];
    if first == b'-' || first == b'.' || first == b'/' {
        return false;
    }
    for b in s.bytes() {
        let ok = b.is_ascii_alphanumeric()
            || matches!(b, b'_' | b'-' | b'.' | b'/');
        if !ok {
            return false;
        }
    }
    // No consecutive dots, no `..` traversal, no trailing `.lock`.
    if s.contains("..") || s.ends_with(".lock") || s.ends_with('/') {
        return false;
    }
    true
}

pub fn worktree_add(
    kit_root: &Path,
    worktree_rel: &Path,
    new_branch: &str,
    base: &str,
) -> Result<(), Error> {
    if !is_safe_refname(new_branch) {
        return Err(Error::InvalidRef(new_branch.to_string()));
    }
    if !is_safe_refname(base) {
        return Err(Error::InvalidRef(base.to_string()));
    }
    let path_str = worktree_rel.to_str().ok_or_else(|| {
        Error::Validate("worktree path is not valid UTF-8".to_string())
    })?;
    let mut c = Command::new(git_bin());
    // `--` sentinel: everything after is positional. The branch name is
    // pinned by `-b` (we already refname-validated it). The `base` is
    // the commit-ish; placing it after `--` stops any accidental
    // promotion to a flag if a future git version reorders parsing.
    c.current_dir(kit_root).args([
        "worktree",
        "add",
        "-b",
        new_branch,
        "--",
        path_str,
        base,
    ]);
    run("git worktree add", &mut c)?;
    Ok(())
}

/// Stage an explicit list of paths. Replacement for `add -A` which
/// bled unwanted markers (`.DONE`, meta files) into commits.
pub fn add_paths(cwd: &Path, paths: &[String]) -> Result<(), Error> {
    if paths.is_empty() {
        return Ok(());
    }
    let mut c = Command::new(git_bin());
    c.current_dir(cwd).arg("add").arg("--");
    for p in paths {
        c.arg(p);
    }
    run("git add", &mut c)?;
    Ok(())
}

/// List untracked files (respects `.gitignore`) relative to `cwd`.
pub fn ls_untracked(cwd: &Path) -> Result<Vec<String>, Error> {
    let mut c = Command::new(git_bin());
    c.current_dir(cwd)
        .args(["ls-files", "-o", "--exclude-standard"]);
    let out = run("git ls-files -o", &mut c)?;
    Ok(split_lines(&out.stdout))
}

/// List modified tracked files relative to `cwd`.
pub fn ls_modified(cwd: &Path) -> Result<Vec<String>, Error> {
    let mut c = Command::new(git_bin());
    c.current_dir(cwd).args(["diff", "--name-only"]);
    let out = run("git diff --name-only", &mut c)?;
    Ok(split_lines(&out.stdout))
}

fn split_lines(stdout: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(stdout)
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect()
}

pub fn commit(cwd: &Path, msg: &str) -> Result<(), Error> {
    let mut c = Command::new(git_bin());
    c.current_dir(cwd).args(["commit", "--allow-empty", "-m", msg]);
    run("git commit", &mut c)?;
    Ok(())
}

pub fn rev_parse_head(cwd: &Path) -> Result<String, Error> {
    let mut c = Command::new(git_bin());
    c.current_dir(cwd).args(["rev-parse", "HEAD"]);
    let out = run("git rev-parse HEAD", &mut c)?;
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

pub fn merge_no_ff(kit_root: &Path, branch: &str, msg: &str) -> Result<(), Error> {
    if !is_safe_refname(branch) {
        return Err(Error::InvalidRef(branch.to_string()));
    }
    let mut c = Command::new(git_bin());
    c.current_dir(kit_root)
        .args(["merge", "--no-ff", branch, "-m", msg]);
    run("git merge --no-ff", &mut c)?;
    Ok(())
}

pub fn worktree_prune(kit_root: &Path) -> Result<(), Error> {
    let mut c = Command::new(git_bin());
    c.current_dir(kit_root).args(["worktree", "prune"]);
    run("git worktree prune", &mut c)?;
    Ok(())
}

pub fn worktree_remove_force(kit_root: &Path, worktree_abs: &Path) -> Result<(), Error> {
    let path_str = worktree_abs.to_str().ok_or_else(|| {
        Error::Validate("worktree path is not valid UTF-8".to_string())
    })?;
    let mut c = Command::new(git_bin());
    c.current_dir(kit_root)
        .args(["worktree", "remove", "--force", "--", path_str]);
    run("git worktree remove --force", &mut c)?;
    Ok(())
}

pub fn branch_delete(kit_root: &Path, branch: &str) -> Result<(), Error> {
    if !is_safe_refname(branch) {
        return Err(Error::InvalidRef(branch.to_string()));
    }
    let mut c = Command::new(git_bin());
    c.current_dir(kit_root).args(["branch", "-D", branch]);
    run("git branch -D", &mut c)?;
    Ok(())
}

/// Check whether `branch` exists. `git show-ref` exits 0 if the ref is
/// present, non-zero otherwise — we treat both as valid data, no error.
pub fn branch_exists(kit_root: &Path, branch: &str) -> bool {
    if !is_safe_refname(branch) {
        return false;
    }
    let full = format!("refs/heads/{branch}");
    let mut c = Command::new(git_bin());
    c.current_dir(kit_root)
        .args(["show-ref", "--verify", "--quiet", &full]);
    c.status().map(|s| s.success()).unwrap_or(false)
}
