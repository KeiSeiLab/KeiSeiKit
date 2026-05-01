// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use thiserror::Error;

/// Local crate errors. Mapped into `kei_runtime_core::Error` at the
/// `ComputeProvider` boundary so callers see one error vocabulary.
#[derive(Debug, Error)]
pub enum Error {
    #[error("config: {0}")]
    Config(String),

    #[error("auth: {0}")]
    Auth(String),

    #[error("http: {0}")]
    Http(#[from] reqwest::Error),

    #[error("api: {status} — {body}")]
    Api { status: u16, body: String },

    #[error("not found: {0}")]
    NotFound(String),

    #[error("invalid tier: {0}")]
    InvalidTier(String),

    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("dna: {0}")]
    Dna(#[from] kei_shared::dna::DnaError),

    #[error("other: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Map crate errors into the runtime-core error vocabulary so trait
/// impls satisfy `kei_runtime_core::Result<T>`.
impl From<Error> for kei_runtime_core::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::Config(s) => kei_runtime_core::Error::Config(s),
            Error::Auth(s) => kei_runtime_core::Error::Auth(s),
            Error::Http(e) => kei_runtime_core::Error::Network(e.to_string()),
            Error::Api { status, body } => {
                kei_runtime_core::Error::Provider(format!("linode {status}: {body}"))
            }
            Error::NotFound(s) => kei_runtime_core::Error::NotFound(s),
            Error::InvalidTier(s) => {
                kei_runtime_core::Error::Config(format!("invalid linode tier: {s}"))
            }
            Error::Serde(e) => kei_runtime_core::Error::Provider(e.to_string()),
            Error::Dna(e) => kei_runtime_core::Error::Dna(e),
            Error::Other(s) => kei_runtime_core::Error::Other(s),
        }
    }
}
