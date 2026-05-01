//! `list(kit_root, status_filter)` — enumerate known forks.
//!
//! Walks two roots:
//!   - `_forks/<id>/` — live worktrees (Active, Done, Stale)
//!   - `_archive/forks/<date>/<id>/` — post-collect (Merged)
//!
//! For each discovered directory, reads `.KEI_FORK_META.toml` to build
//! a `ForkHandle`, classifies status, and filters. Returns `Vec` sorted
//! by `started_ts` ascending so oldest forks list first.

use crate::error::Error;
use crate::handle::{ForkHandle, ForkStatus};
use crate::meta::read_meta;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const STALE_HOURS_DEFAULT: u32 = 24;

pub fn list(kit_root: &Path, status: Option<ForkStatus>) -> Result<Vec<ForkHandle>, Error> {
    let mut out = Vec::new();
    collect_live(&kit_root.join("_forks"), &mut out, status);
    collect_archive(&kit_root.join("_archive/forks"), &mut out, status);
    out.sort_by_key(|h| h.started_ts);
    Ok(out)
}

fn collect_live(root: &Path, out: &mut Vec<ForkHandle>, filter: Option<ForkStatus>) {
    let Ok(rd) = fs::read_dir(root) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if !p.is_dir() {
            continue;
        }
        let Ok(meta) = read_meta(&p) else { continue };
        let status = classify_live(&p, meta.started_ts);
        if matches_filter(filter, status) {
            out.push(meta.into_handle(p));
        }
    }
}

fn collect_archive(root: &Path, out: &mut Vec<ForkHandle>, filter: Option<ForkStatus>) {
    let Ok(dates) = fs::read_dir(root) else { return };
    for date_entry in dates.flatten() {
        let date_dir = date_entry.path();
        if !date_dir.is_dir() {
            continue;
        }
        scan_date_dir(&date_dir, out, filter);
    }
}

fn scan_date_dir(date_dir: &Path, out: &mut Vec<ForkHandle>, filter: Option<ForkStatus>) {
    let Ok(rd) = fs::read_dir(date_dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if !p.is_dir() {
            continue;
        }
        let Ok(meta) = read_meta(&p) else { continue };
        let status = ForkStatus::Merged;
        if matches_filter(filter, status) {
            out.push(meta.into_handle(p));
        }
    }
}

fn classify_live(worktree: &Path, started_ts: i64) -> ForkStatus {
    if worktree.join(".DONE").exists() {
        return ForkStatus::Done;
    }
    let age_h = age_hours(started_ts);
    if age_h >= STALE_HOURS_DEFAULT {
        ForkStatus::Stale
    } else {
        ForkStatus::Active
    }
}

fn age_hours(started_ts: i64) -> u32 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(started_ts);
    let delta = (now - started_ts).max(0);
    (delta / 3600) as u32
}

fn matches_filter(filter: Option<ForkStatus>, s: ForkStatus) -> bool {
    match filter {
        None => true,
        Some(want) => want == s,
    }
}

/// Helper reused by `gc` — enumerate live worktrees with their
/// classified status, without filter.
pub(crate) fn live_with_status(kit_root: &Path) -> Vec<(PathBuf, ForkHandle, ForkStatus)> {
    let mut out = Vec::new();
    let root = kit_root.join("_forks");
    let Ok(rd) = fs::read_dir(&root) else { return out };
    for e in rd.flatten() {
        let p = e.path();
        if !p.is_dir() {
            continue;
        }
        let Ok(meta) = read_meta(&p) else { continue };
        let status = classify_live(&p, meta.started_ts);
        let handle = meta.into_handle(p.clone());
        out.push((p, handle, status));
    }
    out
}
