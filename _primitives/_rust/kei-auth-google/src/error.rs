// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Error types for `kei-auth-google`. Maps cleanly into
//! [`kei_runtime_core::Error`] so the provider can fulfil
//! [`kei_runtime_core::traits::auth::AuthProvider`].

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// Transport-level reqwest failure (connect, TLS, decode).
    #[error("http: {0}")]
    Http(String),

    /// Google API returned a non-success status with a body we surface
    /// verbatim (token endpoint 400, userinfo 401, etc.).
    #[error("api: {0}")]
    Api(String),

    /// Caller passed a non-OAuthCode challenge OR omitted the `state` ⇄ code
    /// pairing required by the verify path.
    #[error("missing state")]
    MissingState,

    /// Userinfo lookup returned 404 or the requested resource is absent.
    #[error("not found: {0}")]
    NotFound(String),

    /// DNA composition failed (only possible if scope/body inputs violate
    /// the wire format — should never trip in practice).
    #[error("dna: {0}")]
    Dna(String),

    /// Underlying serde decode failure on a JSON body Google returned.
    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),

    /// Configuration mismatch (env var unset, both URLs absent, etc.).
    #[error("config: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Http(e.to_string())
    }
}

impl From<kei_runtime_core::DnaError> for Error {
    fn from(e: kei_runtime_core::DnaError) -> Self {
        Error::Dna(e.to_string())
    }
}

impl From<Error> for kei_runtime_core::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::Http(s) => kei_runtime_core::Error::Network(s),
            Error::Api(s) => kei_runtime_core::Error::Provider(s),
            Error::MissingState => kei_runtime_core::Error::Auth("missing state".into()),
            Error::NotFound(s) => kei_runtime_core::Error::NotFound(s),
            Error::Dna(s) => kei_runtime_core::Error::Provider(format!("dna: {s}")),
            Error::Serde(e) => kei_runtime_core::Error::Serde(e),
            Error::Config(s) => kei_runtime_core::Error::Config(s),
        }
    }
}
