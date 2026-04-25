//! `scheduler_bridge` — kei-scheduler → kei-pipe executor glue.
//!
//! Pumps `kei_scheduler::list_due` → `sh -c <command>` →
//! `kei_scheduler::mark_run`, once per call. Caller owns the tick cadence
//! (typical: 1 Hz loop inside a daemon, or a one-shot drain from cron).
//!
//! # Security / trust boundary
//!
//! The scheduler stores `command` as a shell string (not an argv). This
//! bridge therefore execs via `sh -c <command>` — the caller is
//! responsible for ensuring `tasks.command` is trusted. There is NO
//! sandbox, NO wall-time cap, NO stdout capture. A runaway task blocks
//! the current thread until `sh` exits.
//!
//! Out of scope for v0.1: timeouts, CPU/memory limits, argv-form tasks,
//! stdout/stderr capture. Add higher up in the call stack if needed.

use kei_scheduler::{list_due, mark_run, Error as SchedError};
use rusqlite::Connection;
use std::process::Command;
use std::time::Instant;

/// Per-task execution outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunResult {
    pub task_id: i64,
    pub exit_code: i32,
    pub duration_ms: u64,
}

/// Public error surface for the bridge.
#[derive(Debug)]
pub enum Error {
    Scheduler(SchedError),
    Spawn(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Scheduler(e) => write!(f, "scheduler: {e}"),
            Error::Spawn(e) => write!(f, "spawn sh: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Scheduler(e) => Some(e),
            Error::Spawn(e) => Some(e),
        }
    }
}

impl From<SchedError> for Error {
    fn from(e: SchedError) -> Self {
        Error::Scheduler(e)
    }
}

/// Fetch every due task at `now`, exec each via `sh -c`, `mark_run` it,
/// and return one [`RunResult`] per task in scheduler order.
///
/// Errors short-circuit: a DB failure at any point aborts the batch. A
/// failed `sh` spawn (rare — missing /bin/sh) likewise aborts. An exec
/// that returns a non-zero exit code is NOT an error — it's captured in
/// `RunResult.exit_code` and passed through to `mark_run`.
pub fn run_due_tasks(conn: &Connection, now: i64) -> Result<Vec<RunResult>, Error> {
    let due = list_due(conn, now)?;
    let mut out = Vec::with_capacity(due.len());
    for task in due {
        let (exit_code, duration_ms) = exec_shell(&task.command)?;
        mark_run(conn, task.id, exit_code as i64, now)?;
        out.push(RunResult {
            task_id: task.id,
            exit_code,
            duration_ms,
        });
    }
    Ok(out)
}

/// Spawn `sh -c <cmd>`, block for completion, return `(exit_code, wall_ms)`.
/// On platforms where the process was killed by a signal (exit code
/// unavailable), we report `-1`.
fn exec_shell(cmd: &str) -> Result<(i32, u64), Error> {
    let start = Instant::now();
    let status = Command::new("sh")
        .args(["-c", cmd])
        .status()
        .map_err(Error::Spawn)?;
    let dur = start.elapsed().as_millis() as u64;
    let code = status.code().unwrap_or(-1);
    Ok((code, dur))
}

#[cfg(test)]
mod tests {
    use super::*;
    use kei_scheduler::{open_memory, schedule, INTERVAL};

    fn setup_store() -> kei_scheduler::Store {
        open_memory().expect("in-memory scheduler DB")
    }

    #[test]
    fn no_due_tasks_returns_empty() {
        let s = setup_store();
        let now = chrono_now();
        let out = run_due_tasks(s.conn(), now).expect("run");
        assert!(out.is_empty(), "fresh DB → no tasks, got {out:?}");
    }

    #[test]
    fn runs_due_interval_task() {
        let s = setup_store();
        schedule(s.conn(), "ok_task", INTERVAL, "60", "true").unwrap();
        // Query far enough in the future that the interval trigger is
        // eligible (interval sets next_run_at ≈ now+60).
        let query_ts = chrono_now() + 3600;
        let out = run_due_tasks(s.conn(), query_ts).expect("run");
        assert_eq!(out.len(), 1, "exactly one due task");
        assert_eq!(out[0].exit_code, 0, "`true` exits 0");
        // After the run, next_run_at is advanced to query_ts + 60, so
        // re-polling at the same `query_ts` finds nothing.
        let again = run_due_tasks(s.conn(), query_ts).expect("run again");
        assert!(again.is_empty(), "interval advanced; expected empty");
    }

    #[test]
    fn marks_run_on_failure() {
        let s = setup_store();
        schedule(s.conn(), "bad_task", INTERVAL, "60", "false").unwrap();
        let query_ts = chrono_now() + 3600;
        let out = run_due_tasks(s.conn(), query_ts).expect("run");
        assert_eq!(out.len(), 1);
        assert_ne!(out[0].exit_code, 0, "`false` exits non-zero");
    }

    /// Tests don't import chrono directly — read wall clock via std so
    /// the bridge crate's dep surface stays minimal.
    fn chrono_now() -> i64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
}
