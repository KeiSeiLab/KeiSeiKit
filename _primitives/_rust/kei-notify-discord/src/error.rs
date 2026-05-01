// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Error types for `kei-notify-discord`. Maps cleanly into
//! `kei_runtime_core::Error` so the channel can fulfill `NotifyChannel`.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("DNA: {0}")]
    Dna(String),

    #[error("http: {0}")]
    Http(#[from] reqwest::Error),

    #[error("api: {0}")]
    Api(String),

    #[error("config: {0}")]
    Config(String),
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
            Error::Dna(s) => kei_runtime_core::Error::Provider(format!("dna: {s}")),
            Error::Http(e) => kei_runtime_core::Error::Provider(format!("http: {e}")),
            Error::Api(s) => kei_runtime_core::Error::Provider(s),
            Error::Config(s) => kei_runtime_core::Error::Config(s),
        }
    }
}
