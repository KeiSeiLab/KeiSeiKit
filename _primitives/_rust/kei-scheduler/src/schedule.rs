//! `schedule` + `cancel` operations. INSERT / UPDATE on the
//! `scheduler_tasks` table with trigger validation and initial
//! `next_run_at` computed from `compute_next`.

use crate::error::Error;
use crate::task::status;
use crate::trigger::{compute_next, validate_kind};
use chrono::Utc;
use rusqlite::{params, Connection};

/// Insert a new task row. Validates the trigger spec and pre-computes
/// `next_run_at` from `now = Utc::now().timestamp()`.
///
/// Returns the new row's `id`. Errors:
/// - `Error::Parse(...)` — invalid kind / spec
/// - `Error::NameExists(name)` — name UNIQUE violation
/// - `Error::Sqlite(...)` — other DB failures
pub fn schedule(
    conn: &Connection,
    name: &str,
    trigger_kind: &str,
    trigger_spec: &str,
    command: &str,
) -> Result<i64, Error> {
    let kind = validate_kind(trigger_kind)?;
    let now = Utc::now().timestamp();
    let next = compute_next(kind, trigger_spec, now)?;
    insert_row(conn, name, kind, trigger_spec, command, next, now)
}

fn insert_row(
    conn: &Connection,
    name: &str,
    kind: &str,
    spec: &str,
    command: &str,
    next_run_at: Option<i64>,
    now: i64,
) -> Result<i64, Error> {
    let sql = "INSERT INTO scheduler_tasks \
        (name, trigger_kind, trigger_spec, command, status, \
         last_run_at, next_run_at, last_exit_code, created_at, updated_at) \
        VALUES (?1, ?2, ?3, ?4, ?5, NULL, ?6, NULL, ?7, ?7)";
    let status = status::PENDING;
    let result = conn.execute(
        sql,
        params![name, kind, spec, command, status, next_run_at, now],
    );
    match result {
        Ok(_) => Ok(conn.last_insert_rowid()),
        Err(e) => Err(Error::from_insert(e, name)),
    }
}

/// Mark a task cancelled. Clears `next_run_at` so it cannot match
/// `list_due` again even if somebody re-activates the row manually.
/// Idempotent: cancelling an already-cancelled task is a no-op.
/// Missing id → `Error::NotFound`.
pub fn cancel(conn: &Connection, id: i64) -> Result<(), Error> {
    let now = Utc::now().timestamp();
    let rows = conn.execute(
        "UPDATE scheduler_tasks SET status = ?1, next_run_at = NULL, updated_at = ?2 \
         WHERE id = ?3",
        params![status::CANCELLED, now, id],
    )?;
    if rows == 0 {
        return Err(Error::NotFound(id));
    }
    Ok(())
}
