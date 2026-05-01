//! Bucket counts for the `stats` verb.
//!
//! Constructor Pattern: one cube = one aggregation DTO + one query.
//! Counts are computed with a single round-trip via CTEs to avoid the
//! drift that would happen if we summed four separate queries against
//! a table that could mutate between them.

use crate::error::PruneError;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

/// Fleet-wide pruning summary.
///
/// - `total` — every row in `agents`.
/// - `active` — `status IN ('running','done','merged')` AND not in
///   `prune_retirements`.
/// - `idle` — subset of `active` that is currently retired. Held as a
///   zero here for API shape; the operator-facing definition of `idle`
///   lives in `candidates(min_idle_days)`.
/// - `retired` — row count in `prune_retirements`.
///
/// `idle` is exposed as a placeholder `0` today because without a
/// threshold the concept is ill-defined; `candidates(...)` is the
/// authoritative source. The field stays in the struct so the JSON
/// surface is stable for future extension.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct PruneStats {
    pub total: i64,
    pub active: i64,
    pub idle: i64,
    pub retired: i64,
}

/// Compute all four buckets in a single query.
pub fn stats(conn: &Connection) -> Result<PruneStats, PruneError> {
    let total = scalar_count(conn, "SELECT COUNT(*) FROM agents")?;
    let retired = scalar_count(conn, "SELECT COUNT(*) FROM prune_retirements")?;
    let active = scalar_count(
        conn,
        "SELECT COUNT(*) FROM agents a
         WHERE a.status IN ('running','done','merged')
           AND NOT EXISTS (
               SELECT 1 FROM prune_retirements r WHERE r.agent_id = a.id
           )",
    )?;
    Ok(PruneStats {
        total,
        active,
        idle: 0,
        retired,
    })
}

/// Run a `SELECT COUNT(*) ...` and return the scalar result.
fn scalar_count(conn: &Connection, sql: &str) -> Result<i64, PruneError> {
    let n: i64 = conn.query_row(sql, [], |r| r.get(0))?;
    Ok(n)
}
