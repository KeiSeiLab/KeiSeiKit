//! Error type for kei-dna-index.
//!
//! Constructor Pattern: one file = one responsibility (error taxonomy).

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("malformed DNA: {0}")]
    MalformedDna(String),

    #[error("target DNA not found in ledger: {0}")]
    TargetNotFound(String),

    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
