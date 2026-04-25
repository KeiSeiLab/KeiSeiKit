//! Core verbs: `candidates` + `mark_retired`.
//!
//! Constructor Pattern: one cube = the two write/read verbs that touch
//! the sidecar + the `agents` table together. Kept <30 LOC per fn by
//! splitting the row-extract and the existence-probe into helpers.

use crate::candidate::PruneCandidate;
use crate::error::PruneError;
use rusqlite::{params, Connection, Row};

/// Seconds per day — integer arithmetic only (no chrono).
const SECONDS_PER_DAY: i64 = 86_400;

/// Return all agents eligible for retirement.
///
/// Eligibility:
/// - `status IN ('running','done','merged')`
/// - NOT present in `prune_retirements`
/// - `(now - started_ts) / 86400 >= min_idle_days`
///
/// Status `'failed'` and `'rejected'` rows are deliberately excluded —
/// they represent terminal states the operator already triaged, not
/// dormant fleet members.
pub fn candidates(
    conn: &Connection,
    now: i64,
    min_idle_days: u32,
) -> Result<Vec<PruneCandidate>, PruneError> {
    let sql = "\
        SELECT a.id,
               COALESCE(a.dna, '') AS dna,
               COALESCE(a.finished_ts, a.started_ts) AS last_used_ts,
               (? - a.started_ts) / ? AS age_days
        FROM agents a
        WHERE a.status IN ('running','done','merged')
          AND NOT EXISTS (
              SELECT 1 FROM prune_retirements r WHERE r.agent_id = a.id
          )
          AND (? - a.started_ts) / ? >= ?
        ORDER BY age_days DESC, a.id ASC";
    let mut stmt = conn.prepare(sql)?;
    let idle_days = min_idle_days as i64;
    let rows = stmt.query_map(
        params![now, SECONDS_PER_DAY, now, SECONDS_PER_DAY, idle_days],
        row_to_candidate,
    )?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

/// Map a `SELECT id, dna, last_used_ts, age_days` row to a candidate DTO.
fn row_to_candidate(row: &Row<'_>) -> rusqlite::Result<PruneCandidate> {
    Ok(PruneCandidate {
        id: row.get(0)?,
        dna: row.get(1)?,
        last_used_ts: row.get(2)?,
        age_days: row.get(3)?,
    })
}

/// Mark an agent as retired. Idempotent — a repeat call on an already
/// retired id is a no-op and preserves the original `retired_ts`.
///
/// Errors:
/// - `UnknownAgent(id)` if no `agents.id = id` row exists.
/// - `Sql(_)` for any SQLite-level failure.
pub fn mark_retired(conn: &Connection, id: &str, now: i64) -> Result<(), PruneError> {
    if !agent_exists(conn, id)? {
        return Err(PruneError::UnknownAgent(id.to_string()));
    }
    conn.execute(
        "INSERT OR IGNORE INTO prune_retirements(agent_id, retired_ts) VALUES (?, ?)",
        params![id, now],
    )?;
    Ok(())
}

/// Probe `agents.id` existence without loading the full row.
fn agent_exists(conn: &Connection, id: &str) -> Result<bool, PruneError> {
    let n: i64 = conn.query_row(
        "SELECT COUNT(*) FROM agents WHERE id = ?",
        params![id],
        |r| r.get(0),
    )?;
    Ok(n > 0)
}
