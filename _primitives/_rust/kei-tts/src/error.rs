// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! `TtsError` — crate-level error enum for all TTS backends.
//!
//! Each variant carries a human-readable string so call-sites can log
//! without leaking transport internals. `thiserror` provides `Display`
//! and the `Error` trait automatically.

use thiserror::Error;

/// Errors that can occur across any TTS backend.
#[derive(Debug, Error)]
pub enum TtsError {
    /// HTTP transport or API error from a cloud backend.
    #[error("http: {0}")]
    Http(String),

    /// Subprocess (piper-tts) spawn or IO error.
    #[error("subprocess: {0}")]
    Subprocess(String),

    /// Required environment variable is absent.
    #[error("missing env var: {0}")]
    MissingEnv(String),

    /// Backend name was requested but its Cargo feature is not compiled in.
    #[error("backend not enabled: {0}")]
    BackendNotEnabled(String),

    /// Unexpected or malformed response from a backend.
    #[error("invalid response: {0}")]
    InvalidResponse(String),
}

impl From<reqwest::Error> for TtsError {
    fn from(e: reqwest::Error) -> Self {
        TtsError::Http(e.to_string())
    }
}
