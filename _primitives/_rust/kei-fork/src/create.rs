//! `create(agent_id, base_branch, kit_root)` — spawn a managed fork.
//!
//! Steps:
//!   1. `validate_agent_id` (path-traversal defence)
//!   2. Reject if `_forks/<agent_id>/` OR branch `fork/<agent_id>` already exist
//!   3. `git worktree add _forks/<agent_id> -b fork/<agent_id> <base>`
//!   4. Write `.KEI_FORK_META.toml` with agent_id + started_ts + base_branch + ledger_id
//!   5. `kei-ledger fork` unless env `KEI_FORK_SKIP_LEDGER=1`
//!
//! HIGH #2 mitigation: after the worktree exists, any failure in
//! steps 4 or 5 triggers a rollback — the worktree is force-removed
//! and the branch is deleted — so `create()` is either fully-committed
//! or leaves no trace. Callers can retry safely.
//!
//! Test hook: if env `KEI_FORK_FORCE_LEDGER_FAIL=1` is set, the ledger
//! call returns `Error::Ledger` unconditionally (regardless of
//! `KEI_FORK_SKIP_LEDGER`). Used by rollback regression tests.
//!
//! Worktree path is indexed by `agent_id`, not UUID, so `rescue()` /
//! `collect()` can be resolved from a human-readable CLI arg.

use crate::error::Error;
use crate::git;
use crate::handle::ForkHandle;
use crate::meta::{write_meta, ForkMeta};
use kei_agent_runtime::validate::validate_agent_id;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn create(agent_id: &str, base_branch: &str, kit_root: &Path) -> Result<ForkHandle, Error> {
    validate_agent_id(agent_id).map_err(|e| Error::Validate(e.reason))?;
    let worktree_rel = PathBuf::from("_forks").join(agent_id);
    let worktree_abs = kit_root.join(&worktree_rel);
    let branch = format!("fork/{agent_id}");
    if worktree_abs.exists() || git::branch_exists(kit_root, &branch) {
        return Err(Error::Duplicate(agent_id.to_string()));
    }
    if let Some(parent) = worktree_abs.parent() {
        fs::create_dir_all(parent)?;
    }
    git::worktree_add(kit_root, &worktree_rel, &branch, base_branch)?;

    // From here on, worktree + branch exist on disk. If any step fails,
    // we MUST roll them back so the caller sees a clean "no fork" state.
    let started_ts = unix_now();
    let meta = build_meta(agent_id, base_branch, started_ts);
    if let Err(e) = write_meta(&worktree_abs, &meta) {
        rollback(kit_root, &worktree_abs, &branch);
        return Err(e);
    }
    if let Err(e) = ledger_fork(agent_id, &branch, base_branch) {
        rollback(kit_root, &worktree_abs, &branch);
        return Err(e);
    }

    Ok(ForkHandle {
        agent_id: agent_id.to_string(),
        worktree: worktree_abs,
        branch,
        ledger_id: meta.ledger_id,
        started_ts,
    })
}

/// Best-effort cleanup after a partial failure. Errors from the
/// individual commands are intentionally swallowed — the outer error
/// is the real cause; a follow-up `gc` can clean any residue.
fn rollback(kit_root: &Path, worktree_abs: &Path, branch: &str) {
    let _ = git::worktree_remove_force(kit_root, worktree_abs);
    let _ = git::branch_delete(kit_root, branch);
    // If `worktree remove` failed (e.g. git's ref db is out of sync),
    // also clear the directory directly so the next `create` sees a
    // clean slate.
    if worktree_abs.exists() {
        let _ = fs::remove_dir_all(worktree_abs);
    }
}

fn build_meta(agent_id: &str, base_branch: &str, started_ts: i64) -> ForkMeta {
    ForkMeta {
        agent_id: agent_id.to_string(),
        started_ts,
        base_branch: base_branch.to_string(),
        ledger_id: agent_id.to_string(),
    }
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn ledger_skipped() -> bool {
    std::env::var("KEI_FORK_SKIP_LEDGER").ok().as_deref() == Some("1")
}

fn ledger_force_fail() -> bool {
    std::env::var("KEI_FORK_FORCE_LEDGER_FAIL").ok().as_deref() == Some("1")
}

fn ledger_fork(agent_id: &str, branch: &str, base: &str) -> Result<(), Error> {
    if ledger_force_fail() {
        return Err(Error::Ledger(
            "forced failure via KEI_FORK_FORCE_LEDGER_FAIL (test hook)".to_string(),
        ));
    }
    if ledger_skipped() {
        return Ok(());
    }
    // Best-effort spec_sha placeholder: caller stamps real sha post-commit.
    let status = Command::new("kei-ledger")
        .args([
            "fork",
            agent_id,
            branch,
            "--parent",
            base,
            "--spec-sha",
            "pending",
        ])
        .status();
    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(Error::Ledger(format!("kei-ledger fork exit {s}"))),
        Err(e) => Err(Error::Ledger(format!("kei-ledger not runnable: {e}"))),
    }
}
