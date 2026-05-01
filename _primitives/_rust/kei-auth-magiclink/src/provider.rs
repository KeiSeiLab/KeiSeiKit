// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

//! [`MagicLinkProvider`] — passwordless `AuthProvider` impl.
//!
//! Stateless HMAC-signed tokens. No DB. The provider is a value-typed
//! cube: `dna`, `parent`, `hmac_key`, `ttl_secs`. Construct via
//! [`MagicLinkProvider::new`] (explicit) or [`MagicLinkProvider::from_env`]
//! (reads `MAGICLINK_HMAC_KEY` and `MAGICLINK_TTL_SECS`).
//!
//! ## Trait convention quirk
//!
//! [`AuthChallenge::MagicLink`] only carries an `email` field. Two paths use it:
//!
//! - [`MagicLinkProvider::issue_challenge`] — `email` is the user's address.
//!   The provider does NOT send the email itself (no dependency on
//!   `kei-notify-*`); callers build the URL via
//!   [`MagicLinkProvider::build_magic_url`] and dispatch through their own
//!   notify channel.
//! - [`MagicLinkProvider::verify`] — `email` MUST be the FULL token string
//!   returned in the URL's `?token=…` query param. Callers wire the web
//!   handler to slot the token into the `email` field. This is the minimum
//!   change consistent with the trait surface as of v0.1; a future
//!   `AuthChallenge::MagicLinkVerify { token }` variant would be cleaner.

use crate::env::{read_env, MIN_KEY_LEN};
use crate::error::{Error, Result};
use crate::token::{build_token, parse_token};
use async_trait::async_trait;
use kei_runtime_core::dna::{Dna, DnaBuilder, HasDna};
use kei_runtime_core::traits::auth::{AuthChallenge, AuthProvider, AuthSession};

/// Stateless HMAC-SHA256 magic-link provider.
#[derive(Debug)]
pub struct MagicLinkProvider {
    dna: Dna,
    parent: Dna,
    hmac_key: Vec<u8>,
    ttl_secs: i64,
}

impl MagicLinkProvider {
    /// Construct with an explicit parent DNA, key bytes, and TTL.
    pub fn new(parent: Dna, hmac_key: Vec<u8>, ttl_secs: i64) -> Result<Self> {
        if hmac_key.len() < MIN_KEY_LEN {
            return Err(Error::KeyMissing(format!(
                "hmac key must be ≥ {MIN_KEY_LEN} bytes, got {}",
                hmac_key.len()
            )));
        }
        let dna = build_provider_dna()?;
        Ok(Self { dna, parent, hmac_key, ttl_secs })
    }

    /// Construct from environment. See [`crate::env`] for variable names.
    pub fn from_env(parent: Dna) -> Result<Self> {
        let (key, ttl) = read_env()?;
        Self::new(parent, key, ttl)
    }

    /// Build the URL the caller emails to the user.
    pub fn build_magic_url(&self, base_url: &str, email: &str) -> String {
        let expires = now_unix_ms().saturating_add(self.ttl_secs * 1000);
        let token = build_token(email, expires, &self.hmac_key);
        format!("{}/auth/magic?token={}", base_url.trim_end_matches('/'), token)
    }

    /// Configured TTL in seconds.
    pub fn ttl_secs(&self) -> i64 {
        self.ttl_secs
    }
}

impl HasDna for MagicLinkProvider {
    fn dna(&self) -> &Dna {
        &self.dna
    }
    fn parent_dna(&self) -> Option<&Dna> {
        Some(&self.parent)
    }
}

#[async_trait]
impl AuthProvider for MagicLinkProvider {
    fn provider_name(&self) -> &'static str {
        "magiclink"
    }

    async fn issue_challenge(
        &self,
        c: &AuthChallenge,
    ) -> kei_runtime_core::Result<()> {
        match c {
            AuthChallenge::MagicLink { email } if !email.is_empty() => {
                // Stateless: issuing IS building a token. Caller emails it.
                // We pre-flight build to fail fast if HMAC machinery breaks.
                let expires = now_unix_ms().saturating_add(self.ttl_secs * 1000);
                let _ = build_token(email, expires, &self.hmac_key);
                Ok(())
            }
            AuthChallenge::MagicLink { .. } => Err(kei_runtime_core::Error::Auth(
                "magiclink: empty email".into(),
            )),
            _ => Err(kei_runtime_core::Error::Auth(
                "magiclink: unsupported challenge variant".into(),
            )),
        }
    }

    async fn verify(
        &self,
        c: &AuthChallenge,
    ) -> kei_runtime_core::Result<AuthSession> {
        let token = match c {
            AuthChallenge::MagicLink { email } => email,
            _ => {
                return Err(kei_runtime_core::Error::Auth(
                    "magiclink: unsupported challenge variant".into(),
                ))
            }
        };
        let (email, expires_unix_ms) =
            parse_token(token, &self.hmac_key, now_unix_ms()).map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        let session_dna = build_session_dna(&email).map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        Ok(AuthSession {
            dna: session_dna,
            parent_dna: self.parent.clone(),
            user_id: email,
            expires_unix_ms,
            user_agent: None,
        })
    }

    async fn revoke(&self, _session: &Dna) -> kei_runtime_core::Result<()> {
        // v0.1: stateless tokens have no server-side revocation.
        // Callers maintain a deny-list externally if needed.
        Ok(())
    }

    fn is_passwordless(&self) -> bool {
        true
    }
}

fn build_provider_dna() -> Result<Dna> {
    DnaBuilder::new("primitive")
        .caps(["PR", "AP", "ML"])
        .scope("keiseikit.dev/primitives/kei-auth-magiclink")
        .body(b"magiclink-v1")
        .build()
        .map_err(|e| Error::Dna(e.to_string()))
}

fn build_session_dna(email: &str) -> Result<Dna> {
    DnaBuilder::new("session")
        .caps(["AS", "ML"])
        .scope("keiseikit.dev/sessions/magiclink")
        .body(email.as_bytes())
        .build()
        .map_err(|e| Error::Dna(e.to_string()))
}

fn now_unix_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}
