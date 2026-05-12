// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! `ContactsError` — typed error variants for the iCloud CardDAV client.

use thiserror::Error;

/// Errors produced by [`crate::ICloudCardDavClient`].
#[derive(Debug, Error)]
pub enum ContactsError {
    /// Network or non-auth HTTP error.
    #[error("http error: {0}")]
    Http(String),

    /// Authentication failure (401 or 403 from iCloud).
    #[error("auth error: {0}")]
    Auth(String),

    /// XML or response body parsing failure.
    #[error("parse error: {0}")]
    Parse(String),

    /// vCard content could not be parsed into a contact.
    #[error("invalid vcard: {0}")]
    InvalidVCard(String),
}
