// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Local error type for the Slack notify channel.
//!
//! Mapped into [`kei_runtime_core::Error`] via `From<Error>` so the trait
//! impls can use `?` against the runtime-core `Result`.

use thiserror::Error;

/// Crate-local result alias.
pub type Result<T> = std::result::Result<T, Error>;

/// Crate-local error variants.
#[derive(Debug, Error)]
pub enum Error {
    /// Transport / TLS / timeout failure from `reqwest`.
    #[error("http: {0}")]
    Http(#[from] reqwest::Error),

    /// Non-200 HTTP status with the (best-effort) body text.
    #[error("api: {0}")]
    Api(String),

    /// DNA construction or env-var read failure.
    #[error("dna: {0}")]
    Dna(String),
}

impl From<kei_runtime_core::DnaError> for Error {
    fn from(e: kei_runtime_core::DnaError) -> Self {
        Error::Dna(e.to_string())
    }
}

impl From<Error> for kei_runtime_core::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::Http(re) => kei_runtime_core::Error::Network(re.to_string()),
            Error::Api(msg) => kei_runtime_core::Error::Provider(msg),
            Error::Dna(msg) => kei_runtime_core::Error::Config(format!("dna: {msg}")),
        }
    }
}
