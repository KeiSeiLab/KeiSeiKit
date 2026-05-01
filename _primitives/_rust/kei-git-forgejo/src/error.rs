// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Error types for `kei-git-forgejo`. Maps cleanly into
//! `kei_runtime_core::Error` so the backend can fulfill `GitBackend`.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("DNA: {0}")]
    Dna(#[from] kei_runtime_core::DnaError),

    #[error("config: {0}")]
    Config(String),

    #[error("auth: {0}")]
    Auth(String),

    #[error("network: {0}")]
    Network(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("provider: {0}")]
    Provider(String),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Network(e.to_string())
    }
}

impl From<Error> for kei_runtime_core::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::Dna(e) => kei_runtime_core::Error::Dna(e),
            Error::Config(s) => kei_runtime_core::Error::Config(s),
            Error::Auth(s) => kei_runtime_core::Error::Auth(s),
            Error::Network(s) => kei_runtime_core::Error::Network(s),
            Error::NotFound(s) => kei_runtime_core::Error::NotFound(s),
            Error::Provider(s) => kei_runtime_core::Error::Provider(s),
            Error::Io(e) => kei_runtime_core::Error::Io(e),
            Error::Serde(e) => kei_runtime_core::Error::Serde(e),
        }
    }
}
