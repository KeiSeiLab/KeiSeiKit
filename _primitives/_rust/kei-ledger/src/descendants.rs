//! `descendants()` — lineage walker over `fork_parent_id` + `creator_id`.
//!
//! Constructor Pattern: one cube = one query. Single public fn under 30 LOC.
//! RULE 0.12 v4 lineage lookup: find every agent that was forked-from OR
//! spawned-by a given DNA.

use crate::row::{row_to_agent, AgentRow, SELECT_COLS};
use rusqlite::{params, Connection, Result as SqlResult};

/// Return every row whose `fork_parent_id == dna` OR `creator_id == dna`.
/// Ordered oldest-first so callers can reconstruct a timeline. Callers that
/// want recursive transitive closure should loop on returned ids.
pub fn descendants(conn: &Connection, dna: &str) -> SqlResult<Vec<AgentRow>> {
    let sql = format!(
        "SELECT {SELECT_COLS} FROM agents
         WHERE fork_parent_id = ?1 OR creator_id = ?1
         ORDER BY started_ts ASC"
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt
        .query_map(params![dna], row_to_agent)?
        .collect::<SqlResult<Vec<_>>>()?;
    Ok(rows)
}
