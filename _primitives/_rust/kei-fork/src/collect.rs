//! `collect(agent_id, commit_msg, kit_root)` — merge the fork back.
//!
//! Contract:
//!   1. `.DONE` must exist inside the worktree, else `Error::NotDone`
//!   2. Compute an EXPLICIT path list (untracked + modified), minus the
//!      reserved exclusion set, then `git add <paths>` + `git commit`
//!   3. Capture commit SHA, then `git merge --no-ff fork/<id>` in kit_root
//!   4. Move worktree to `_archive/forks/YYYY-MM-DD/<id>/` (preserving
//!      the agent's artefacts for post-hoc review / rescue)
//!   5. `git worktree prune && git branch -D fork/<id>` to clean up refs
//!   6. `kei-ledger done <id>` unless `KEI_FORK_SKIP_LEDGER=1`
//!
//! HIGH #1 mitigation: the earlier `git add -A` was replaced by an
//! explicit path list. Reserved names (`.DONE`, `.KEI_FORK_META.toml`,
//! `_archive/**`, `_forks/**`) are stripped before staging so they
//! never land in the merge commit even if an agent wrote them.

use crate::error::Error;
use crate::git;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectReport {
    pub files_added: usize,
    pub commit_sha: String,
    pub archive_path: PathBuf,
}

/// Paths that never belong in the merged history.
const EXCLUDED_NAMES: &[&str] = &[".DONE", ".KEI_FORK_META.toml"];
/// Path prefixes (relative to worktree root) that are kit-internal.
const EXCLUDED_PREFIXES: &[&str] = &["_archive/", "_forks/"];

pub fn collect(agent_id: &str, commit_msg: &str, kit_root: &Path) -> Result<CollectReport, Error> {
    let worktree_abs = kit_root.join("_forks").join(agent_id);
    if !worktree_abs.join(".DONE").exists() {
        return Err(Error::NotDone(agent_id.to_string()));
    }

    let stage_list = compute_stage_list(&worktree_abs)?;
    let files_added = stage_list.len();

    let branch = format!("fork/{agent_id}");
    git::add_paths(&worktree_abs, &stage_list)?;
    git::commit(&worktree_abs, commit_msg)?;
    let commit_sha = git::rev_parse_head(&worktree_abs)?;

    let merge_msg = format!("Merge {branch}");
    git::merge_no_ff(kit_root, &branch, &merge_msg)?;

    let archive_path = archive_worktree(kit_root, agent_id, &worktree_abs)?;

    // worktree_remove is unnecessary after fs::rename — prune cleans the
    // stale worktree metadata and branch -D removes the ref.
    let _ = git::worktree_prune(kit_root);
    let _ = git::branch_delete(kit_root, &branch);

    ledger_done(agent_id)?;

    Ok(CollectReport {
        files_added,
        commit_sha,
        archive_path,
    })
}

/// Union of (untracked, exclude-standard) + (modified-tracked),
/// minus any path that matches the reserved exclusion set.
fn compute_stage_list(worktree_abs: &Path) -> Result<Vec<String>, Error> {
    let untracked = git::ls_untracked(worktree_abs)?;
    let modified = git::ls_modified(worktree_abs)?;
    let mut combined: Vec<String> = untracked.into_iter().chain(modified).collect();
    combined.sort();
    combined.dedup();
    combined.retain(|p| !is_excluded(p));
    Ok(combined)
}

fn is_excluded(path: &str) -> bool {
    if EXCLUDED_NAMES.contains(&path) {
        return true;
    }
    if EXCLUDED_PREFIXES.iter().any(|p| path.starts_with(*p)) {
        return true;
    }
    false
}

fn archive_worktree(
    kit_root: &Path,
    agent_id: &str,
    worktree_abs: &Path,
) -> Result<PathBuf, Error> {
    let date = Utc::now().format("%Y-%m-%d").to_string();
    let archive_dir = kit_root.join("_archive/forks").join(&date);
    fs::create_dir_all(&archive_dir)?;
    let target = archive_dir.join(agent_id);
    if target.exists() {
        fs::remove_dir_all(&target)?;
    }
    fs::rename(worktree_abs, &target)?;
    Ok(target)
}

fn ledger_skipped() -> bool {
    std::env::var("KEI_FORK_SKIP_LEDGER").ok().as_deref() == Some("1")
}

fn ledger_done(agent_id: &str) -> Result<(), Error> {
    if ledger_skipped() {
        return Ok(());
    }
    let status = Command::new("kei-ledger")
        .args(["done", agent_id, "--summary", "fork collected"])
        .status();
    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(Error::Ledger(format!("kei-ledger done exit {s}"))),
        Err(e) => Err(Error::Ledger(format!("kei-ledger not runnable: {e}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::is_excluded;

    #[test]
    fn excludes_reserved_names() {
        assert!(is_excluded(".DONE"));
        assert!(is_excluded(".KEI_FORK_META.toml"));
    }

    #[test]
    fn excludes_kit_prefixes() {
        assert!(is_excluded("_archive/forks/2026-04-23/x/y"));
        assert!(is_excluded("_forks/other/file.txt"));
    }

    #[test]
    fn admits_regular_files() {
        assert!(!is_excluded("src/main.rs"));
        assert!(!is_excluded("hello.txt"));
        assert!(!is_excluded("sub/.DONE.txt"));
    }
}
