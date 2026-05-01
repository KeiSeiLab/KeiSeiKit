// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Error types for `kei-auth-webauthn`. Maps cleanly into
//! [`kei_runtime_core::Error`] so [`crate::WebauthnProvider`] can fulfil
//! [`kei_runtime_core::traits::auth::AuthProvider`].

use thiserror::Error;
use webauthn_rs::prelude::WebauthnError;

#[derive(Debug, Error)]
pub enum Error {
    /// Underlying webauthn-rs ceremony failure (validation, parse, crypto).
    #[error("webauthn-rs: {0}")]
    WebauthnRs(#[from] WebauthnError),

    /// Invalid relying-party origin URL (must be parseable by `url::Url`).
    #[error("url: {0}")]
    Url(#[from] url::ParseError),

    /// DNA composition failed (only possible if the literal scope/body
    /// inputs in [`crate::WebauthnProvider::new`] violate the wire format —
    /// should never trip in practice).
    #[error("dna: {0}")]
    Dna(String),

    /// Caller attempted to drive a WebAuthn ceremony through the
    /// trait-method path ([`AuthProvider::issue_challenge`] /
    /// [`AuthProvider::verify`]) instead of the explicit helpers
    /// ([`WebauthnProvider::start_registration`] etc.). The message names
    /// the helper to call.
    #[error("trait-misuse: {0}")]
    TraitMisuse(String),
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
            Error::WebauthnRs(w) => kei_runtime_core::Error::Provider(format!("webauthn: {w}")),
            Error::Url(u) => kei_runtime_core::Error::Config(format!("url: {u}")),
            Error::Dna(s) => kei_runtime_core::Error::Provider(format!("dna: {s}")),
            Error::TraitMisuse(s) => kei_runtime_core::Error::Provider(format!(
                "kei-auth-webauthn: trait method called for WebAuthn ceremony — \
                 use {s} instead (see lib.rs trait-extension convention)"
            )),
        }
    }
}
