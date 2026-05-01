// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use thiserror::Error;

/// GitLab backend errors. Mapped into `kei_runtime_core::Error` via the
/// `From` impl below so consumers of `GitBackend` see uniform variants.
#[derive(Debug, Error)]
pub enum Error {
    #[error("DNA: {0}")]
    Dna(#[from] kei_runtime_core::DnaError),

    #[error("auth: {0}")]
    Auth(String),

    #[error("network: {0}")]
    Network(String),

    #[error("config: {0}")]
    Config(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("api: {status} {body}")]
    Api { status: u16, body: String },

    #[error("git cli: {0}")]
    Git(String),

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
            Error::Auth(s) => kei_runtime_core::Error::Auth(s),
            Error::Network(s) => kei_runtime_core::Error::Network(s),
            Error::Config(s) => kei_runtime_core::Error::Config(s),
            Error::NotFound(s) => kei_runtime_core::Error::NotFound(s),
            Error::Api { status, body } => kei_runtime_core::Error::Provider(
                format!("gitlab api {status}: {body}"),
            ),
            Error::Git(s) => kei_runtime_core::Error::Provider(format!("git cli: {s}")),
            Error::Io(e) => kei_runtime_core::Error::Io(e),
            Error::Serde(e) => kei_runtime_core::Error::Serde(e),
        }
    }
}
