// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Apple id_token claim types deserialized from the JWT payload.

use serde::{Deserialize, Serialize};

/// Subset of standard OIDC + Apple-specific claims we read.
///
/// Apple's id_token always carries `sub` (the stable Apple user id) and
/// `iss` (`https://appleid.apple.com`). `email` is present on first
/// authorization but may be absent on subsequent ones; it may also be a
/// private-relay address (`@privaterelay.appleid.com`).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdTokenClaims {
    pub sub: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub exp: i64,
    #[serde(default)]
    pub iat: i64,
    #[serde(default)]
    pub iss: String,
    #[serde(default)]
    pub aud: AudClaim,
}

/// `aud` can be a single string or an array — Apple sends a single string.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(untagged)]
pub enum AudClaim {
    One(String),
    Many(Vec<String>),
    #[default]
    Missing,
}

impl AudClaim {
    pub(crate) fn contains(&self, s: &str) -> bool {
        match self {
            AudClaim::One(v) => v == s,
            AudClaim::Many(vs) => vs.iter().any(|v| v == s),
            AudClaim::Missing => false,
        }
    }
}
