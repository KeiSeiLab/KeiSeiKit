// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Error types for `kei-notify-sms`. Maps cleanly into
//! `kei_runtime_core::Error` so the channel can fulfill `NotifyChannel`.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("DNA: {0}")]
    Dna(String),

    #[error("missing env: {0}")]
    MissingEnv(String),

    #[error("http: {0}")]
    Http(#[from] reqwest::Error),

    #[error("api: {0}")]
    Api(String),
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
            Error::Dna(s) => kei_runtime_core::Error::Other(format!("dna: {s}")),
            Error::MissingEnv(s) => {
                kei_runtime_core::Error::Config(format!("missing env: {s}"))
            }
            Error::Http(e) => kei_runtime_core::Error::Network(e.to_string()),
            Error::Api(s) => kei_runtime_core::Error::Provider(s),
        }
    }
}
