// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use thiserror::Error;

/// Crate-local error. Maps into `kei_runtime_core::Error` at the
/// async-trait surface so downstream users see a single error type.
#[derive(Debug, Error)]
pub enum Error {
    #[error("sled: {0}")]
    Sled(#[from] sled::Error),

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
            Error::Sled(e) => kei_runtime_core::Error::Provider(format!("sled: {e}")),
            Error::Serde(e) => kei_runtime_core::Error::Serde(e),
            Error::Io(e) => kei_runtime_core::Error::Io(e),
            Error::NotFound(s) => kei_runtime_core::Error::NotFound(s),
            Error::Join(s) => kei_runtime_core::Error::Other(format!("join: {s}")),
            Error::Provider(s) => kei_runtime_core::Error::Provider(s),
        }
    }
}
