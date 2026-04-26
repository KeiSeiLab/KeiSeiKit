//! `ProjectRow` — value type mirroring one row of the `projects` table.
//!
//! Constructor Pattern: one cube = one struct + its serde derive. Kept
//! separate from `index.rs` so the orchestrator stays under the 120-LOC
//! cap and the schema's row shape lives in a single, easily-diffable cube.

use serde::{Deserialize, Serialize};

/// One row of the `projects` table. Mirrors the SQL schema verbatim.
/// Consumed by `kei-cortex` (dashboard JSON) and integration tests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRow {
    pub path: String,
    pub name: String,
    pub has_git: bool,
    pub branch: Option<String>,
    pub dirty: bool,
    pub ahead: i64,
    pub behind: i64,
    pub last_commit_sha: Option<String>,
    pub last_commit_msg: Option<String>,
    pub last_commit_ts: Option<i64>,
    pub has_claude_md: bool,
    pub has_decisions_md: bool,
    pub has_runbook_md: bool,
    pub has_readme: bool,
    pub sqlite_count: i64,
    pub last_indexed_ts: i64,
}
