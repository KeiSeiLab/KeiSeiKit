//! `Task` — in-memory snapshot of a `scheduler_tasks` row.
//!
//! Serializable for the CLI (`list-due` prints JSON). Status is a plain
//! String so callers can introduce new sentinels without a type bump.

use rusqlite::Row;
use serde::{Deserialize, Serialize};

/// Canonical task status sentinels. Schema default is `pending`;
/// lifecycle: `pending` → `scheduled` (optional staging) → `running` →
/// `done` / `failed`. `cancelled` is terminal and set by `cancel()`.
pub mod status {
    pub const PENDING: &str = "pending";
    pub const SCHEDULED: &str = "scheduled";
    pub const RUNNING: &str = "running";
    pub const DONE: &str = "done";
    pub const FAILED: &str = "failed";
    pub const CANCELLED: &str = "cancelled";
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    pub id: i64,
    pub name: String,
    pub trigger_kind: String,
    pub trigger_spec: String,
    pub command: String,
    pub status: String,
    pub last_run_at: Option<i64>,
    pub next_run_at: Option<i64>,
    pub last_exit_code: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Task {
    /// Column order MUST match the SELECT in `query.rs::SELECT_ALL`.
    /// rusqlite returns `NULL` for INTEGER columns as 0 unless we read
    /// into `Option<i64>` explicitly, which is what we want for the
    /// nullable timestamps + exit code.
    pub fn from_row(r: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: r.get(0)?,
            name: r.get(1)?,
            trigger_kind: r.get(2)?,
            trigger_spec: r.get(3)?,
            command: r.get(4)?,
            status: r.get(5)?,
            last_run_at: r.get(6)?,
            next_run_at: r.get(7)?,
            last_exit_code: r.get(8)?,
            created_at: r.get(9)?,
            updated_at: r.get(10)?,
        })
    }
}

/// SELECT column list used by `query.rs` and `run.rs`. Exported so
/// callers building custom queries stay in sync with `Task::from_row`.
pub const SELECT_COLS: &str = "id, name, trigger_kind, trigger_spec, command, status, \
     last_run_at, next_run_at, last_exit_code, created_at, updated_at";
