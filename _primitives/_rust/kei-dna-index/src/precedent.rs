//! Precedent lookup: find rows sharing a given body_sha, optionally filtered by status.
//!
//! Constructor Pattern: one file = one responsibility.

use crate::adjacency::{AdjacencyResult, Relationship};
use crate::db::load_rows;
use crate::error::Result;
use rusqlite::Connection;

pub fn precedent(
    conn: &Connection,
    body_sha: &str,
    status_filter: Option<&str>,
) -> Result<Vec<AdjacencyResult>> {
    let rows = load_rows(conn)?;
    let mut out: Vec<AdjacencyResult> = rows
        .into_iter()
        .filter(|r| r.parsed.body_sha.eq_ignore_ascii_case(body_sha))
        .filter(|r| match status_filter {
            None => true,
            Some("all") => true,
            Some(s) => r.status == s,
        })
        .map(|r| AdjacencyResult {
            dna: r.dna,
            agent_id: r.agent_id,
            status: r.status,
            distance: 0,
            relationship: Relationship::SameBody,
        })
        .collect();
    out.sort_by(|a, b| a.agent_id.cmp(&b.agent_id));
    Ok(out)
}
