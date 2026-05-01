// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Local error type for the Bitbucket backend.
//!
//! Mapped into [`kei_runtime_core::Error`] via `From<Error>` so the trait
//! impls can use `?` against the runtime-core `Result`.

use kei_runtime_core::DnaError;
use thiserror::Error;

/// Crate-local result alias.
pub type Result<T> = std::result::Result<T, Error>;

/// Crate-local error variants.
#[derive(Debug, Error)]
pub enum Error {
    /// Transport / TLS / timeout failure from `reqwest`.
    #[error("http: {0}")]
    Http(#[from] reqwest::Error),

    /// Non-success HTTP status with the (best-effort) body text.
    #[error("api: {0}")]
    Api(String),

    /// DNA construction or parse failure.
    #[error("dna: {0}")]
    Dna(#[from] DnaError),

    /// Local IO (env var read, git CLI invocation, etc.).
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialize / deserialize failure.
    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),

    /// Resource lookup miss (e.g. 404 on get repo / branch).
    #[error("not found: {0}")]
    NotFound(String),

    /// Missing or malformed configuration (env var, remote URL, etc.).
    #[error("config: {0}")]
    Config(String),

    /// Authentication failure (401/403).
    #[error("auth: {0}")]
    Auth(String),
}

impl From<Error> for kei_runtime_core::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::Http(re) => kei_runtime_core::Error::Network(re.to_string()),
            Error::Api(msg) => kei_runtime_core::Error::Provider(msg),
            Error::Dna(de) => kei_runtime_core::Error::Dna(de),
            Error::Io(io) => kei_runtime_core::Error::Io(io),
            Error::Serde(se) => kei_runtime_core::Error::Provider(format!("serde: {se}")),
            Error::NotFound(msg) => kei_runtime_core::Error::NotFound(msg),
            Error::Config(msg) => kei_runtime_core::Error::Config(msg),
            Error::Auth(msg) => kei_runtime_core::Error::Auth(msg),
        }
    }
}
