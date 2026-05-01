//! Crate error type. Wraps SQLite + IO + serde failures into one variant.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("sqlite: {0}")]
    Sql(#[from] rusqlite::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("schema migration aborted: {0}")]
    Migration(String),
}
