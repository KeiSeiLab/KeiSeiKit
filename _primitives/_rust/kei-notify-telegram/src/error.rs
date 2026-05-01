// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Crate-local error. Bridges into `kei_runtime_core::Error` so
//! `NotifyChannel::send` impls can `?` through this and surface a
//! substrate-level error to callers without leaking transport details.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("http: {0}")]
    Http(#[from] reqwest::Error),

    #[error("api: {0}")]
    Api(String),

    #[error("DNA: {0}")]
    Dna(String),

    #[error("missing env var: {0}")]
    MissingEnv(String),

    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<kei_runtime_core::DnaError> for Error {
    fn from(e: kei_runtime_core::DnaError) -> Self {
        Error::Dna(e.to_string())
    }
}

/// Bridge into substrate-level error. Telegram-specific failures are
/// funnelled into `Provider`; the message string preserves the upstream
/// shape (Telegram description / HTTP status / parse error).
impl From<Error> for kei_runtime_core::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::Http(re) => {
                kei_runtime_core::Error::Network(format!("telegram http: {re}"))
            }
            Error::Api(s) => {
                kei_runtime_core::Error::Provider(format!("telegram api: {s}"))
            }
            Error::Dna(s) => {
                kei_runtime_core::Error::Provider(format!("telegram dna: {s}"))
            }
            Error::MissingEnv(s) => {
                kei_runtime_core::Error::Config(format!("telegram missing env: {s}"))
            }
            Error::Serde(se) => kei_runtime_core::Error::Serde(se),
        }
    }
}
