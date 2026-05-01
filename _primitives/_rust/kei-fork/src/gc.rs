//! `gc(kit_root, older_than_hours)` — prune stale forks.
//!
//! A fork is STALE when `.DONE` is absent AND `age > older_than_hours`.
//! For each stale fork we:
//!   1. `git worktree remove --force <worktree>`
//!   2. `git branch -D fork/<id>`
//!   3. `kei-ledger fail <id>` unless `KEI_FORK_SKIP_LEDGER=1`
//!
//! Returns the list of agent_ids pruned. Errors on individual forks are
//! swallowed into the report so a single bad fork cannot block cleanup
//! of the rest.

use crate::error::Error;
use crate::git;
use crate::handle::ForkStatus;
use crate::list::live_with_status;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GcReport {
    pub pruned: Vec<String>,
    pub skipped: Vec<String>,
}

pub fn gc(kit_root: &Path, older_than_hours: u32) -> Result<GcReport, Error> {
    let mut report = GcReport::default();
    for (worktree_abs, handle, status) in live_with_status(kit_root) {
        if !is_prunable(status, handle.started_ts, older_than_hours) {
            continue;
        }
        match prune_one(kit_root, &worktree_abs, &handle.branch, &handle.agent_id) {
            Ok(()) => report.pruned.push(handle.agent_id),
            Err(_) => report.skipped.push(handle.agent_id),
        }
    }
    Ok(report)
}

fn is_prunable(status: ForkStatus, started_ts: i64, threshold_h: u32) -> bool {
    if status != ForkStatus::Stale && status != ForkStatus::Active {
        return false;
    }
    let age = age_hours(started_ts);
    age >= threshold_h
}

fn age_hours(started_ts: i64) -> u32 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(started_ts);
    let delta = (now - started_ts).max(0);
    (delta / 3600) as u32
}

fn prune_one(
    kit_root: &Path,
    worktree_abs: &Path,
    branch: &str,
    agent_id: &str,
) -> Result<(), Error> {
    git::worktree_remove_force(kit_root, worktree_abs)?;
    let _ = git::branch_delete(kit_root, branch);
    let _ = ledger_fail(agent_id);
    Ok(())
}

fn ledger_skipped() -> bool {
    std::env::var("KEI_FORK_SKIP_LEDGER").ok().as_deref() == Some("1")
}

fn ledger_fail(agent_id: &str) -> Result<(), Error> {
    if ledger_skipped() {
        return Ok(());
    }
    let status = Command::new("kei-ledger")
        .args(["fail", agent_id, "--reason", "gc: stale fork"])
        .status();
    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(Error::Ledger(format!("kei-ledger fail exit {s}"))),
        Err(e) => Err(Error::Ledger(format!("kei-ledger not runnable: {e}"))),
    }
}
