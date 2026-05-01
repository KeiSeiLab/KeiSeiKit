// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

//! Error type for kei-auth-magiclink.
//!
//! Maps cleanly into `kei_runtime_core::Error::Auth(String)` so the
//! [`AuthProvider`] trait surface stays in the substrate's error space.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// Token does not split into the expected three `.`-separated parts,
    /// or any part fails base64url decoding / utf8 parsing / int parsing.
    #[error("magiclink token malformed: {0}")]
    TokenMalformed(String),

    /// Token's expiry timestamp is at or before `now_unix_ms`.
    #[error("magiclink token expired (expires_unix_ms={expires_unix_ms}, now_unix_ms={now_unix_ms})")]
    TokenExpired {
        expires_unix_ms: i64,
        now_unix_ms: i64,
    },

    /// HMAC tag does not match the recomputed tag (constant-time compare).
    #[error("magiclink token signature mismatch")]
    BadSignature,

    /// `MAGICLINK_HMAC_KEY` env var is missing, empty, undecodable,
    /// or shorter than 32 bytes after decoding.
    #[error("magiclink hmac key missing or invalid: {0}")]
    KeyMissing(String),

    /// DNA build / parse error from `kei_runtime_core::dna`.
    #[error("magiclink dna error: {0}")]
    Dna(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<Error> for kei_runtime_core::Error {
    fn from(e: Error) -> Self {
        kei_runtime_core::Error::Auth(e.to_string())
    }
}
