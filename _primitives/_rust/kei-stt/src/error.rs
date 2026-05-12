// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! `SttError` — crate-level error enum for all STT backends.
//!
//! Each variant carries a human-readable string so call-sites can log
//! without leaking transport internals. `thiserror` provides `Display`
//! and the `Error` trait automatically.

use thiserror::Error;

/// Errors that can occur across any STT backend.
#[derive(Debug, Error)]
pub enum SttError {
    /// HTTP transport or API error from a cloud backend.
    #[error("http: {0}")]
    Http(String),

    /// Subprocess (whisper CLI) spawn or IO error.
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

    /// Input audio bytes are invalid or in an unsupported format.
    #[error("invalid audio: {0}")]
    InvalidAudio(String),
}

impl From<reqwest::Error> for SttError {
    fn from(e: reqwest::Error) -> Self {
        SttError::Http(e.to_string())
    }
}
