// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Crate-local error type. Maps into `kei_runtime_core::Error` via `From`
//! so the `MemoryBackend` trait surface stays in runtime-core variants.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("redis: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("dna: {0}")]
    Dna(String),

    #[error("config: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<Error> for kei_runtime_core::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::Redis(re) => kei_runtime_core::Error::Network(re.to_string()),
            Error::Serde(se) => kei_runtime_core::Error::Serde(se),
            Error::NotFound(k) => kei_runtime_core::Error::NotFound(k),
            Error::Dna(s) => kei_runtime_core::Error::Provider(format!("dna: {s}")),
            Error::Config(s) => kei_runtime_core::Error::Config(s),
        }
    }
}
