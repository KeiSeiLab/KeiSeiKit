//! Git state extraction for one project.
//!
//! Constructor Pattern: one cube = git2 wrapper that returns a snapshot
//! of branch / dirty / ahead / behind / last-commit for a single repo.
//! Non-repo paths short-circuit with `None`.

use git2::{BranchType, Repository, StatusOptions};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Snapshot of a repo's state at index time. All fields are optional
/// where the underlying git operation can legitimately fail (no HEAD,
/// no upstream tracking branch, detached HEAD, empty repo).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitState {
    pub branch: Option<String>,
    pub dirty: bool,
    pub ahead: usize,
    pub behind: usize,
    pub last_commit_sha: Option<String>,
    pub last_commit_msg: Option<String>,
    pub last_commit_ts: Option<i64>,
}

/// Resolve the current branch name. Detached HEAD or empty repo → None.
fn detect_branch(repo: &Repository) -> Option<String> {
    let head = repo.head().ok()?;
    if head.is_branch() {
        head.shorthand().map(|s| s.to_string())
    } else {
        None
    }
}

/// Detect uncommitted changes (working-tree OR staged index changes).
/// Untracked files are excluded from "dirty" — they're noise on big
/// portfolios where stray scratch files land in many repos.
fn detect_dirty(repo: &Repository) -> bool {
    let mut opts = StatusOptions::new();
    opts.include_untracked(false).include_ignored(false);
    match repo.statuses(Some(&mut opts)) {
        Ok(statuses) => !statuses.is_empty(),
        Err(_) => false,
    }
}

/// Compute commits ahead / behind upstream tracking branch. Returns
/// `(0, 0)` if the current branch has no upstream configured.
fn detect_ahead_behind(repo: &Repository, branch_name: &str) -> (usize, usize) {
    let local = match repo.find_branch(branch_name, BranchType::Local) {
        Ok(b) => b,
        Err(_) => return (0, 0),
    };
    let upstream = match local.upstream() {
        Ok(u) => u,
        Err(_) => return (0, 0),
    };
    let local_oid = match local.get().target() {
        Some(o) => o,
        None => return (0, 0),
    };
    let upstream_oid = match upstream.get().target() {
        Some(o) => o,
        None => return (0, 0),
    };
    repo.graph_ahead_behind(local_oid, upstream_oid).unwrap_or((0, 0))
}

/// Extract last-commit metadata from HEAD. Returns three `None`s on
/// empty repo (no commits yet).
fn detect_last_commit(repo: &Repository) -> (Option<String>, Option<String>, Option<i64>) {
    let head = match repo.head() {
        Ok(h) => h,
        Err(_) => return (None, None, None),
    };
    let commit = match head.peel_to_commit() {
        Ok(c) => c,
        Err(_) => return (None, None, None),
    };
    let sha = commit.id().to_string();
    let msg = commit.summary().unwrap_or("").to_string();
    let ts = commit.time().seconds();
    (Some(sha), Some(msg), Some(ts))
}

/// Open `project_root` as a git repo and snapshot its state.
///
/// Returns `None` if the path is not a git repository (no `.git/`,
/// corrupt repo, etc.). All other states (empty repo, detached HEAD,
/// no upstream) yield a valid `GitState` with the relevant fields set
/// to `None` / 0.
pub fn detect_git_state(project_root: &Path) -> Option<GitState> {
    let repo = Repository::open(project_root).ok()?;
    let branch = detect_branch(&repo);
    let dirty = detect_dirty(&repo);
    let (ahead, behind) = match &branch {
        Some(b) => detect_ahead_behind(&repo, b),
        None => (0, 0),
    };
    let (last_commit_sha, last_commit_msg, last_commit_ts) = detect_last_commit(&repo);
    Some(GitState {
        branch,
        dirty,
        ahead,
        behind,
        last_commit_sha,
        last_commit_msg,
        last_commit_ts,
    })
}
