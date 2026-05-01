//! Error type for kei-memory.
//!
//! Constructor Pattern: this cube only declares the error enum + Result alias.
//! Wave A motive — `ingest.rs:55-56` was abusing
//! `rusqlite::Error::InvalidParameterName` to wrap an `io::Error`. That hides
//! the real failure source from callers and confuses operators reading logs.
//! `KeiMemoryError` separates the four failure domains we actually have.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeiMemoryError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("parse: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("db: {0}")]
    Db(#[from] rusqlite::Error),

    #[error("schema: {0}")]
    Schema(String),
}

/// Crate-wide Result alias for paths that mix IO + parse + DB.
pub type Result<T> = std::result::Result<T, KeiMemoryError>;
