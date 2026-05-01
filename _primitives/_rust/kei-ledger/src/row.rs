//! `AgentRow` — the ledger's hydrated record.
//!
//! Constructor Pattern: one cube = struct + SELECT column list + row mapper.
//! Kept separate from `ledger.rs` so both stay under the 200-LOC cap.

use rusqlite::Result as SqlResult;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct AgentRow {
    pub id: String,
    pub branch: String,
    pub parent_branch: Option<String>,
    pub spec_sha: String,
    pub status: String,
    pub started_ts: i64,
    pub finished_ts: Option<i64>,
    pub summary: Option<String>,
    pub worktree_path: Option<String>,
    /// Layer G composition fingerprint; `None` for pre-v2 rows.
    pub dna: Option<String>,
    /// DNA/human id of the spawner; `None` for pre-v4 rows (v4 lineage).
    pub creator_id: Option<String>,
    /// DNA of forked-from agent if this row is itself a fork; `None` otherwise.
    pub fork_parent_id: Option<String>,
}

/// Column list shared by all SELECTs that hydrate an `AgentRow`. Order must
/// match `row_to_agent` indices 0..12.
pub const SELECT_COLS: &str =
    "id, branch, parent_branch, spec_sha, status, started_ts, finished_ts, \
     summary, worktree_path, dna, creator_id, fork_parent_id";

pub fn row_to_agent(r: &rusqlite::Row) -> SqlResult<AgentRow> {
    Ok(AgentRow {
        id: r.get(0)?,
        branch: r.get(1)?,
        parent_branch: r.get(2)?,
        spec_sha: r.get(3)?,
        status: r.get(4)?,
        started_ts: r.get(5)?,
        finished_ts: r.get(6)?,
        summary: r.get(7)?,
        worktree_path: r.get(8)?,
        dna: r.get(9)?,
        creator_id: r.get(10)?,
        fork_parent_id: r.get(11)?,
    })
}
