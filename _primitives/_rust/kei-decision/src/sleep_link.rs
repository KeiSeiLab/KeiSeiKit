//! Sleep-layer Phase B integration: scan known research-source roots for
//! files newer than a `--since` timestamp and return them as a unified queue.
//!
//! Today we look at two roots:
//!   - `~/.keisei/memory/sync-repo/sleep-results/`  (Phase A incubation outputs)
//!   - `~/Projects/KnowledgeVault/research/*/MASTER-REPORT.md`  (research outputs)
//!
//! No parsing here — the caller can decide which entries to feed back through
//! `parse_master_report` / `rank_actions`.

use anyhow::Result;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize)]
pub struct ResearchHit {
    pub path: PathBuf,
    pub modified_unix_secs: u64,
    pub source_kind: SourceKind,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    SleepResults,
    KnowledgeVault,
}

#[derive(Debug, Clone, Serialize)]
pub struct SleepScanOutput {
    pub since_unix_secs: u64,
    pub hits: Vec<ResearchHit>,
}

/// Walk both known roots; return any `*.md` (sleep) or `MASTER-REPORT.md`
/// (vault) modified strictly after `since_unix_secs`.
pub fn scan_research_sources(since_unix_secs: u64) -> Result<SleepScanOutput> {
    let mut hits = Vec::new();
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let sleep_root = Path::new(&home).join(".keisei/memory/sync-repo/sleep-results");
    walk_sleep_results(&sleep_root, since_unix_secs, &mut hits);
    let vault_root = Path::new(&home).join("Projects/KnowledgeVault/research");
    walk_vault_masters(&vault_root, since_unix_secs, &mut hits);
    hits.sort_by(|a, b| a.modified_unix_secs.cmp(&b.modified_unix_secs));
    Ok(SleepScanOutput { since_unix_secs, hits })
}

fn walk_sleep_results(root: &Path, since: u64, hits: &mut Vec<ResearchHit>) {
    if !root.exists() {
        return;
    }
    for entry in WalkDir::new(root).max_depth(2).into_iter().flatten() {
        let p = entry.path();
        if !p.is_file() { continue; }
        if p.extension().map(|e| e != "md").unwrap_or(true) { continue; }
        if let Some(ts) = file_modified_unix(p) {
            if ts > since {
                hits.push(ResearchHit { path: p.to_path_buf(), modified_unix_secs: ts, source_kind: SourceKind::SleepResults });
            }
        }
    }
}

fn walk_vault_masters(root: &Path, since: u64, hits: &mut Vec<ResearchHit>) {
    if !root.exists() {
        return;
    }
    for entry in WalkDir::new(root).max_depth(3).into_iter().flatten() {
        let p = entry.path();
        if !p.is_file() { continue; }
        if p.file_name().map(|n| n != "MASTER-REPORT.md").unwrap_or(true) { continue; }
        if let Some(ts) = file_modified_unix(p) {
            if ts > since {
                hits.push(ResearchHit { path: p.to_path_buf(), modified_unix_secs: ts, source_kind: SourceKind::KnowledgeVault });
            }
        }
    }
}

fn file_modified_unix(p: &Path) -> Option<u64> {
    let meta = std::fs::metadata(p).ok()?;
    let modified = meta.modified().ok()?;
    modified.duration_since(SystemTime::UNIX_EPOCH).ok().map(|d| d.as_secs())
}
