//! kei-scheduler — durable task scheduler primitive (cron / at /
//! interval triggers). Metadata store only; execution is the caller's
//! responsibility. `kei-pipe` or a cron-wrapper agent pumps
//! `list_due` → invoke → `mark_run` on an external cadence.
//!
//! Shape mirrors the sibling kei-task / kei-chat-store pattern: the
//! `kei-entity-store` engine owns DDL + migrations, and this crate adds
//! the scheduler-specific SQL helpers (`schedule`, `cancel`, `list_due`,
//! `mark_run`) on top of its `Store` shim.
//!
//! Public API surface (all I/O is synchronous rusqlite; no runtime):
//! - [`open`] / [`open_memory`] — build a `Store` with the scheduler schema.
//! - [`schedule`] — insert a new task + pre-compute `next_run_at`.
//! - [`cancel`] — set status=cancelled, clear `next_run_at`.
//! - [`list_due`] — rows where `next_run_at <= now` AND status is
//!   pending/scheduled.
//! - [`mark_run`] — stamp last_run / last_exit_code / advance schedule.
//! - [`compute_next`] — pure function, no DB.

pub mod error;
pub mod query;
pub mod run;
pub mod schedule;
pub mod schema;
pub mod store;
pub mod task;
pub mod trigger;

pub use error::{Error, ParseError};
pub use query::{get_by_name, get_task, list_due};
pub use run::mark_run;
pub use schedule::{cancel, schedule};
pub use schema::{ALL_SCHEMAS, SCHEDULER_SCHEMA};
pub use store::Store;
pub use task::{status as task_status, Task};
pub use trigger::{compute_next, validate_kind, AT, CRON, INTERVAL};

use std::path::Path;

/// Convenience constructor — opens the scheduler DB at `path`, creating
/// parent dirs + running migrations. Wraps `Store::open` so callers who
/// only need the raw Store don't import the submodule.
pub fn open(path: &Path) -> anyhow::Result<Store> {
    Store::open(path)
}

/// In-memory scheduler — used by unit tests and by callers who want a
/// throwaway queue (e.g. a dry-run planner).
pub fn open_memory() -> anyhow::Result<Store> {
    Store::open_memory()
}
