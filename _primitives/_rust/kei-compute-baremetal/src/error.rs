// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Denis Parfionovich

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("DNA: {0}")]
    Dna(#[from] kei_shared::dna::DnaError),

    #[error("operation '{op}' not implemented for bare-metal provider (manual user action required)")]
    NotImplemented { op: &'static str },

    #[error("SSH connection to '{host}' failed: {detail}")]
    ConnectionFailed { host: String, detail: String },

    #[error("VM not found: {0}")]
    NotFound(String),

    #[error("invalid external_id (expected ssh://user@host[:port]): {0}")]
    InvalidExternalId(String),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Map our errors into the runtime-core error type expected by the
/// `ComputeProvider` trait. `NotImplemented` is surfaced as `Provider`
/// since runtime-core has no dedicated variant for it.
impl From<Error> for kei_runtime_core::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::NotFound(s) => kei_runtime_core::Error::NotFound(s),
            Error::Dna(e) => kei_runtime_core::Error::Dna(e),
            Error::NotImplemented { op } => kei_runtime_core::Error::Provider(format!(
                "baremetal: '{op}' not implemented (manual action required)"
            )),
            Error::ConnectionFailed { host, detail } => {
                kei_runtime_core::Error::Provider(format!("baremetal SSH '{host}': {detail}"))
            }
            Error::InvalidExternalId(s) => {
                kei_runtime_core::Error::Provider(format!("baremetal invalid id: {s}"))
            }
            Error::Io(e) => kei_runtime_core::Error::Provider(format!("baremetal io: {e}")),
        }
    }
}
