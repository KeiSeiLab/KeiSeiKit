// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use thiserror::Error;

/// Crate-local error. Maps cleanly into `kei_runtime_core::Error` so
/// `GitBackend` impls can `?` through this and surface a substrate
/// error to callers without leaking transport details.
#[derive(Debug, Error)]
pub enum Error {
    #[error("DNA: {0}")]
    Dna(#[from] kei_runtime_core::DnaError),

    #[error("config: {0}")]
    Config(String),

    #[error("auth: {0}")]
    Auth(String),

    #[error("http: {0}")]
    Http(String),

    #[error("api: status {status} on {endpoint}: {body}")]
    Api {
        status: u16,
        endpoint: String,
        body: String,
    },

    #[error("not found: {0}")]
    NotFound(String),

    #[error("git CLI: {0}")]
    GitCli(String),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Http(e.to_string())
    }
}

/// Bridge into substrate-level error. Variants funnel into the closest
/// matching `kei_runtime_core::Error` variant; the Gitea-specific
/// shape is preserved in the message string.
impl From<Error> for kei_runtime_core::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::Dna(e) => kei_runtime_core::Error::Dna(e),
            Error::Config(s) => kei_runtime_core::Error::Config(s),
            Error::Auth(s) => kei_runtime_core::Error::Auth(s),
            Error::Http(s) => kei_runtime_core::Error::Network(s),
            Error::Api { status, endpoint, body } => {
                kei_runtime_core::Error::Provider(format!(
                    "gitea api {status} on {endpoint}: {body}"
                ))
            }
            Error::NotFound(s) => kei_runtime_core::Error::NotFound(s),
            Error::GitCli(s) => kei_runtime_core::Error::Provider(format!("git cli: {s}")),
            Error::Io(e) => kei_runtime_core::Error::Io(e),
            Error::Serde(e) => kei_runtime_core::Error::Serde(e),
        }
    }
}
