//! Error type for kei-prune.
//!
//! Constructor Pattern: one cube = one error enum. Keeps `rusqlite::Error`
//! wrapped so callers don't need to depend on rusqlite directly.

use thiserror::Error;

/// All failure modes produced by the kei-prune public API.
#[derive(Debug, Error)]
pub enum PruneError {
    /// Underlying SQLite error (schema DDL, query, insert).
    #[error("sqlite error: {0}")]
    Sql(#[from] rusqlite::Error),

    /// Agent id not present in the ledger `agents` table.
    ///
    /// `mark_retired` returns this when the caller passes an id that has
    /// no matching row — better to fail loudly than to retire a phantom.
    #[error("agent id not found in ledger: {0}")]
    UnknownAgent(String),

    /// JSON serialisation failure (CLI output only).
    #[error("json serialisation error: {0}")]
    Json(#[from] serde_json::Error),
}
