// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Local error type for the Apple Sign-In auth provider.
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

    /// Non-success HTTP status with the (best-effort) body text, or
    /// other Apple-side API protocol failure.
    #[error("api: {0}")]
    Api(String),

    /// id_token shape / base64 / utf8 / json failure during unverified decode.
    #[error("jwt decode: {0}")]
    JwtDecode(String),

    /// id_token decoded but a required claim (e.g. `sub`) was missing.
    #[error("missing claim: {0}")]
    MissingClaim(String),

    /// DNA construction or parse failure.
    #[error("dna: {0}")]
    Dna(#[from] DnaError),

    /// Local IO (env var read, etc.).
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialize / deserialize failure.
    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),
}

impl From<Error> for kei_runtime_core::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::Http(re) => kei_runtime_core::Error::Network(re.to_string()),
            Error::Api(msg) => kei_runtime_core::Error::Provider(msg),
            Error::JwtDecode(msg) => {
                kei_runtime_core::Error::Provider(format!("jwt decode: {msg}"))
            }
            Error::MissingClaim(c) => {
                kei_runtime_core::Error::Provider(format!("missing claim: {c}"))
            }
            Error::Dna(de) => kei_runtime_core::Error::Dna(de),
            Error::Io(io) => kei_runtime_core::Error::Io(io),
            Error::Serde(se) => kei_runtime_core::Error::Provider(format!("serde: {se}")),
        }
    }
}
