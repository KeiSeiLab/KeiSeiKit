//! `ForkHandle` value type + `ForkStatus` enum.
//!
//! `ForkHandle` is the return of `create()` and each row of `list()`. Its
//! fields are derived from `.KEI_FORK_META.toml` plus the worktree path
//! on disk. The handle is `Clone`, `serde::Serialize`, and
//! `serde::Deserialize` so the CLI can emit JSON and downstream callers
//! can round-trip it without touching the TOML file.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForkHandle {
    pub agent_id: String,
    pub worktree: PathBuf,
    pub branch: String,
    pub ledger_id: String,
    pub started_ts: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ForkStatus {
    Active,
    Done,
    Stale,
    Merged,
}

impl ForkStatus {
    /// Parse CLI `--status` value. Returns `None` for unknown strings so
    /// the CLI layer can emit a domain-appropriate error.
    pub fn from_cli(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "active" => Some(ForkStatus::Active),
            "done" => Some(ForkStatus::Done),
            "stale" => Some(ForkStatus::Stale),
            "merged" => Some(ForkStatus::Merged),
            _ => None,
        }
    }
}
