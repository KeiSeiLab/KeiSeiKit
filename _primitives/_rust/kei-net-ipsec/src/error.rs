// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Local error type for the IPsec / strongSwan adapter.
//!
//! Mapped into [`kei_runtime_core::Error`] via `From<Error>` so the
//! [`crate::network::IpsecMode`] trait impls can use `?` against the
//! runtime-core `Result`.

use thiserror::Error;

/// Crate-local result alias.
pub type Result<T> = std::result::Result<T, Error>;

/// Crate-local error variants.
#[derive(Debug, Error)]
pub enum Error {
    /// `swanctl` invocation completed but returned a non-zero exit code.
    /// The captured stderr / synthetic message rides along.
    #[error("swanctl failed: {0}")]
    SwanctlFailed(String),

    /// Local IO / spawn failure (e.g. `swanctl` binary missing on PATH).
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    /// Parser could not extract a structured `PeerStatus` from the
    /// `swanctl --list-sas` text output.
    #[error("parse: {0}")]
    Parse(String),

    /// DNA construction failure.
    #[error("dna: {0}")]
    Dna(String),
}

impl From<Error> for kei_runtime_core::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::SwanctlFailed(msg) => kei_runtime_core::Error::Network(msg),
            Error::Io(io) => kei_runtime_core::Error::Io(io),
            Error::Parse(msg) => kei_runtime_core::Error::Network(format!("ipsec parse: {msg}")),
            Error::Dna(msg) => kei_runtime_core::Error::Provider(format!("ipsec dna: {msg}")),
        }
    }
}

impl From<kei_runtime_core::DnaError> for Error {
    fn from(e: kei_runtime_core::DnaError) -> Self {
        Error::Dna(e.to_string())
    }
}
