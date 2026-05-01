//! Read-side queries: `list_due` and `get_task`.
//!
//! `list_due` is the hot path driven by external tickers (`kei-pipe`,
//! cron-wrapper agents). It selects rows whose `next_run_at <= now` AND
//! `status IN (pending, scheduled)` AND `next_run_at IS NOT NULL`. The
//! NOT-NULL filter excludes cancelled + one-shot-completed rows.

use crate::error::Error;
use crate::task::{status, Task, SELECT_COLS};
use rusqlite::{params, Connection};

/// Fetch all rows whose `next_run_at <= now` and status makes them
/// eligible to run. Ordered by `next_run_at ASC` so the earliest-due
/// task surfaces first.
pub fn list_due(conn: &Connection, now: i64) -> Result<Vec<Task>, Error> {
    let sql = format!(
        "SELECT {cols} FROM scheduler_tasks \
         WHERE next_run_at IS NOT NULL \
           AND next_run_at <= ?1 \
           AND status IN (?2, ?3) \
         ORDER BY next_run_at ASC, id ASC",
        cols = SELECT_COLS,
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt
        .query_map(params![now, status::PENDING, status::SCHEDULED], Task::from_row)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

/// Fetch a single task by id. `Ok(None)` if no such row.
pub fn get_task(conn: &Connection, id: i64) -> Result<Option<Task>, Error> {
    let sql = format!(
        "SELECT {cols} FROM scheduler_tasks WHERE id = ?1",
        cols = SELECT_COLS,
    );
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query_map(params![id], Task::from_row)?;
    match rows.next() {
        Some(r) => Ok(Some(r?)),
        None => Ok(None),
    }
}

/// Fetch a single task by unique name. Used by the CLI's `cancel --name`
/// convenience; kept thin so the query-layer responsibilities stay in
/// one module.
pub fn get_by_name(conn: &Connection, name: &str) -> Result<Option<Task>, Error> {
    let sql = format!(
        "SELECT {cols} FROM scheduler_tasks WHERE name = ?1",
        cols = SELECT_COLS,
    );
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query_map(params![name], Task::from_row)?;
    match rows.next() {
        Some(r) => Ok(Some(r?)),
        None => Ok(None),
    }
}
