// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Crate-local error type. Maps into `kei_runtime_core::Error` via `From`
//! so the `MemoryBackend` trait surface stays in runtime-core variants.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("join: {0}")]
    Join(String),

    #[error("provider: {0}")]
    Provider(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<Error> for kei_runtime_core::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Sqlite(e) => kei_runtime_core::Error::Provider(format!("sqlite: {e}")),
            Error::Serde(e) => kei_runtime_core::Error::Serde(e),
            Error::Io(e) => kei_runtime_core::Error::Io(e),
            Error::NotFound(s) => kei_runtime_core::Error::NotFound(s),
            Error::Join(s) => kei_runtime_core::Error::Other(format!("join: {s}")),
            Error::Provider(s) => kei_runtime_core::Error::Provider(s),
        }
    }
}
