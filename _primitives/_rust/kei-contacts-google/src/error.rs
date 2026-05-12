// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! `ContactsError` — typed error variants for the Google People API client.

use thiserror::Error;

/// Errors produced by [`crate::GooglePeopleClient`].
#[derive(Debug, Error)]
pub enum ContactsError {
    /// Network or non-auth HTTP error.
    #[error("http error: {0}")]
    Http(String),

    /// JSON deserialization failure.
    #[error("parse error: {0}")]
    Parse(String),

    /// Google People API returned 401 — token expired or invalid.
    #[error("auth error: token expired or invalid")]
    Auth(String),
}
