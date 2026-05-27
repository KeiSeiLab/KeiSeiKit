//! `scheduler_bridge` — kei-scheduler → kei-pipe executor glue.
//!
//! Pumps `kei_scheduler::list_due` → `sh -c <command>` →
//! `kei_scheduler::mark_run`, once per call. Caller owns the tick cadence.
//!
//! # Security / trust boundary — CALLER RESTRICTION
//!
//! `tasks.command` is a shell string exec'd via `sh -c`. Privileged surface.
//! Two defenses live here:
//!   1. Argv0 allow-list + substring denylist ([`scheduler_denylist`]) —
//!      mirrors `kei-cortex::tool::bash` layered defense. Rejection
//!      returns [`Error::Denied`]; task is NOT marked-run.
//!   2. Per-task wall-time cap ([`SAFE_TOOL_TIMEOUT_SECS`], mirrors
//!      `kei-mcp::safe_tools`). Tasks over 60s are killed; `exit_code = -1`.
//!
//! Out of scope: full sandbox (namespace / cgroup / seccomp), CPU/memory
//! caps, stdout capture.
//!
//! ## TODO — `RootExecutor` newtype
//!
//! Today any in-graph caller can invoke `run_due_tasks(&Connection)`. Too
//! loose: scheduler execution is root-level by trust model and MUST NOT
//! be wired up from MCP/HTTP/agent handlers. Wrap the entry point in a
//! `RootExecutor(())` newtype constructible only from the `kei-pipe`
//! daemon binary, so any caller outside the daemon fails to compile.
//! Callers outside the `kei-pipe` daemon binary MUST NOT have access to
//! `schedule()` either — `schedule()` writes the command that gets
//! exec'd, so exposing it via HTTP/MCP is RCE.

use crate::scheduler_denylist::{deny_dangerous, Denied};
use kei_scheduler::{list_due, mark_run, Error as SchedError};
use rusqlite::Connection;
use std::process::Stdio;
use std::time::{Duration, Instant};
use tokio::process::Command;
use tokio::runtime::Builder;
use tokio::time::timeout;

/// Per-task wall-time cap. Mirrors `kei-mcp::safe_tools::SAFE_TOOL_TIMEOUT_SECS`.
pub const SAFE_TOOL_TIMEOUT_SECS: u64 = 60;

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
    Denied(Denied),
    Runtime(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Scheduler(e) => write!(f, "scheduler: {e}"),
            Error::Spawn(e) => write!(f, "spawn sh: {e}"),
            Error::Denied(e) => write!(f, "{e}"),
            Error::Runtime(e) => write!(f, "tokio runtime: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Scheduler(e) => Some(e),
            Error::Spawn(e) => Some(e),
            Error::Denied(e) => Some(e),
            Error::Runtime(e) => Some(e),
        }
    }
}

impl From<SchedError> for Error {
    fn from(e: SchedError) -> Self {
        Error::Scheduler(e)
    }
}

impl From<Denied> for Error {
    fn from(e: Denied) -> Self {
        Error::Denied(e)
    }
}

/// Fetch every due task at `now`, exec each via `sh -c`, `mark_run` it.
/// DB failure / spawn failure / denylist rejection short-circuits the
/// batch; a denied task stays due (NOT marked-run) so it surfaces in
/// audit. Non-zero exit is NOT an error. Timeout → killed, `-1`.
pub fn run_due_tasks(conn: &Connection, now: i64) -> Result<Vec<RunResult>, Error> {
    let due = list_due(conn, now)?;
    let mut out = Vec::with_capacity(due.len());
    for task in due {
        deny_dangerous(&task.command)?;
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

/// Spawn `sh -c <cmd>` under a single-thread tokio runtime, enforce
/// [`SAFE_TOOL_TIMEOUT_SECS`] via `tokio::time::timeout`. Timeout or
/// signal-kill → `exit_code = -1`.
fn exec_shell(cmd: &str) -> Result<(i32, u64), Error> {
    let start = Instant::now();
    let rt = Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(Error::Runtime)?;
    let code = rt.block_on(async {
        let mut child = Command::new("sh")
            .args(["-c", cmd])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(Error::Spawn)?;
        let wait = child.wait();
        match timeout(Duration::from_secs(SAFE_TOOL_TIMEOUT_SECS), wait).await {
            Ok(Ok(status)) => Ok::<i32, Error>(status.code().unwrap_or(-1)),
            Ok(Err(e)) => Err(Error::Spawn(e)),
            Err(_) => {
                let _ = child.kill().await;
                let _ = child.wait().await;
                Ok::<i32, Error>(-1)
            }
        }
    })?;
    let dur = start.elapsed().as_millis() as u64;
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
        let query_ts = chrono_now() + 3600;
        let out = run_due_tasks(s.conn(), query_ts).expect("run");
        assert_eq!(out.len(), 1, "exactly one due task");
        assert_eq!(out[0].exit_code, 0, "`true` exits 0");
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

    #[test]
    fn rejects_denylisted_command() {
        let s = setup_store();
        schedule(s.conn(), "bad", INTERVAL, "60", "sudo rm -rf /").unwrap();
        let query_ts = chrono_now() + 3600;
        let err = run_due_tasks(s.conn(), query_ts).expect_err("must deny");
        assert!(matches!(err, Error::Denied(_)));
    }

    fn chrono_now() -> i64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
}
