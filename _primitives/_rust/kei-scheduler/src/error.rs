//! Typed errors for kei-scheduler. `Error` is the public wrapper;
//! `ParseError` surfaces trigger-spec parse failures separately so
//! callers (and tests) can discriminate without string-matching.

use thiserror::Error;

/// Trigger-spec parse failures. Pure function — no DB contact.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("unknown trigger_kind '{0}' — expected cron / at / interval")]
    UnknownKind(String),
    #[error("invalid cron expression '{0}': {1}")]
    InvalidCron(String, String),
    #[error("invalid ISO-8601 datetime '{0}' — expected YYYY-MM-DDTHH:MM:SSZ")]
    InvalidIsoDatetime(String),
    #[error("invalid interval '{0}' — expected positive integer seconds")]
    InvalidInterval(String),
}

/// Public scheduler error. Wraps rusqlite + anyhow + ParseError.
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error("sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("task not found: id={0}")]
    NotFound(i64),
    #[error("task name already exists: '{0}'")]
    NameExists(String),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Error {
    /// Inspect a rusqlite error and reclassify `UNIQUE constraint
    /// failed: scheduler_tasks.name` into a typed `NameExists`. Other
    /// SQLite errors pass through unchanged.
    pub fn from_insert(err: rusqlite::Error, name: &str) -> Self {
        let msg = err.to_string();
        if msg.contains("UNIQUE constraint failed")
            && msg.contains("scheduler_tasks.name")
        {
            Self::NameExists(name.to_string())
        } else {
            Self::Sqlite(err)
        }
    }
}
