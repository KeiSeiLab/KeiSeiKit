//! `mark_run` — record completion of a triggered execution.
//!
//! Caller supplies `now` explicitly so tests are deterministic. The new
//! `next_run_at` is re-computed from `now` using the task's stored
//! trigger_kind / trigger_spec:
//! - `interval` → `now + secs` (never terminal).
//! - `cron`     → next cron occurrence after `now` (falls back to
//!                 terminal `done` if no future occurrence exists).
//! - `at`       → one-shot; status → `done`, next_run_at → NULL.
//!
//! Status transitions: cancelled rows are immutable (function returns
//! `Error::NotFound` to keep the surface minimal — the caller should
//! not be marking runs on cancelled tasks).

use crate::error::Error;
use crate::query::get_task;
use crate::task::status;
use crate::trigger::{compute_next, AT};
use rusqlite::{params, Connection};

/// Record a run outcome and advance the schedule.
///
/// Returns `Ok(())` on success, `Error::NotFound` if `id` doesn't
/// exist or refers to a cancelled task, `Error::Parse` if the stored
/// trigger spec is no longer parseable (should not happen if the row
/// was created via `schedule`).
pub fn mark_run(
    conn: &Connection,
    id: i64,
    exit_code: i64,
    now: i64,
) -> Result<(), Error> {
    let task = match get_task(conn, id)? {
        Some(t) if t.status == status::CANCELLED => return Err(Error::NotFound(id)),
        Some(t) => t,
        None => return Err(Error::NotFound(id)),
    };
    let (next, next_status) = advance(&task.trigger_kind, &task.trigger_spec, exit_code, now)?;
    write_run(conn, id, exit_code, now, next, next_status)
}

/// Compute the next `(next_run_at, status)` pair given the trigger +
/// run outcome. Exit-code 0 on `at` → `done`; non-zero → `failed`.
/// `cron`/`interval` ignore exit code when scheduling next fire.
fn advance(
    kind: &str,
    spec: &str,
    exit_code: i64,
    now: i64,
) -> Result<(Option<i64>, &'static str), Error> {
    if kind == AT {
        let s = if exit_code == 0 { status::DONE } else { status::FAILED };
        return Ok((None, s));
    }
    let next = compute_next(kind, spec, now)?;
    let status_next = match next {
        Some(_) => status::SCHEDULED,
        // Cron schedule with no future occurrence — treat as terminal.
        None => status::DONE,
    };
    Ok((next, status_next))
}

fn write_run(
    conn: &Connection,
    id: i64,
    exit_code: i64,
    now: i64,
    next_run_at: Option<i64>,
    next_status: &str,
) -> Result<(), Error> {
    let rows = conn.execute(
        "UPDATE scheduler_tasks SET \
            last_run_at = ?1, \
            last_exit_code = ?2, \
            next_run_at = ?3, \
            status = ?4, \
            updated_at = ?5 \
         WHERE id = ?6",
        params![now, exit_code, next_run_at, next_status, now, id],
    )?;
    if rows == 0 {
        return Err(Error::NotFound(id));
    }
    Ok(())
}
