// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use thiserror::Error;

/// Crate-local error. Mapped into `kei_runtime_core::Error` via the
/// `From` impl below so the trait surface stays substrate-native.
#[derive(Debug, Error)]
pub enum Error {
    #[error("missing VULTR_API_KEY env var")]
    MissingToken,

    #[error("Vultr HTTP {status}: {body}")]
    Http { status: u16, body: String },

    #[error("reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("DNA: {0}")]
    Dna(#[from] kei_shared::dna::DnaError),

    #[error("unknown tier: {0}")]
    UnknownTier(String),

    #[error("instance not found: {0}")]
    NotFound(String),

    #[error("provisioning failed: {0}")]
    ProvisioningFailed(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<Error> for kei_runtime_core::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::MissingToken => kei_runtime_core::Error::Auth(e.to_string()),
            Error::Http { status, .. } if status == 404 => {
                kei_runtime_core::Error::NotFound(e.to_string())
            }
            Error::Http { .. } | Error::Reqwest(_) => {
                kei_runtime_core::Error::Network(e.to_string())
            }
            Error::Serde(_) => kei_runtime_core::Error::Provider(e.to_string()),
            Error::Dna(d) => kei_runtime_core::Error::Dna(d),
            Error::UnknownTier(_) => kei_runtime_core::Error::Config(e.to_string()),
            Error::NotFound(_) => kei_runtime_core::Error::NotFound(e.to_string()),
            Error::ProvisioningFailed(_) => kei_runtime_core::Error::Provider(e.to_string()),
        }
    }
}
