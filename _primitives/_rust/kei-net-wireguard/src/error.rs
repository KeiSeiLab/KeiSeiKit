// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Error types for `kei-net-wireguard`. Maps cleanly into
//! `kei_runtime_core::Error` so `WireguardMode` can fulfill `NetworkMode`.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// `wg-quick`/`wg` exited non-zero. Carries the rendered command line +
    /// stderr tail for diagnostics.
    #[error("wg cmd: {0}")]
    WgCmd(String),

    /// Underlying I/O failure (spawn / read / wait).
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    /// `wg show ... dump` produced output we could not parse.
    #[error("parse: {0}")]
    Parse(String),

    /// DNA construction failed.
    #[error("dna: {0}")]
    Dna(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<kei_runtime_core::DnaError> for Error {
    fn from(e: kei_runtime_core::DnaError) -> Self {
        Error::Dna(e.to_string())
    }
}

impl From<Error> for kei_runtime_core::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::WgCmd(s) => kei_runtime_core::Error::Network(format!("wg: {s}")),
            Error::Io(e) => kei_runtime_core::Error::Io(e),
            Error::Parse(s) => kei_runtime_core::Error::Network(format!("parse: {s}")),
            Error::Dna(s) => kei_runtime_core::Error::Other(format!("dna: {s}")),
        }
    }
}
