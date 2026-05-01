// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! [`GoogleAuthProvider`] — `AuthProvider` impl over Google OAuth 2.0 +
//! OIDC userinfo. Builds an [`AuthSession`] whose `user_id` is the OIDC
//! `email` (with `sub` available via the userinfo result if needed).
//!
//! `provider_name = "google"`. `is_passwordless = true`.
//!
//! `revoke` is a no-op for v0.1: Google does expose
//! `https://oauth2.googleapis.com/revoke`, but the primitive treats that
//! as the operator's responsibility — surfacing a half-implemented revoke
//! would violate RULE 0.16 (functional vs scaffolding).

use crate::client::{GoogleAuthClient, DEFAULT_AUTH_URL};
use crate::error::{Error, Result};
use async_trait::async_trait;
use kei_runtime_core::traits::auth::{AuthChallenge, AuthProvider, AuthSession};
use kei_runtime_core::{Dna, DnaBuilder, HasDna};
use std::time::{SystemTime, UNIX_EPOCH};

/// Default scope set: OIDC profile + email. Sufficient to populate
/// [`AuthSession::user_id`] from the userinfo endpoint.
pub const DEFAULT_SCOPES: &str = "openid email profile";

/// `AuthProvider` for Google OAuth 2.0.
pub struct GoogleAuthProvider {
    dna: Dna,
    parent: Option<Dna>,
    client: GoogleAuthClient,
}

impl GoogleAuthProvider {
    /// Build a provider over a pre-configured client. The DNA is a fresh
    /// primitive serial with caps `["PR", "AP", "GO"]`.
    pub fn new(client: GoogleAuthClient, parent: Option<Dna>) -> Result<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "GO"])
            .scope("keiseikit.dev/primitives/kei-auth-google")
            .body(b"google-oauth2")
            .build()?;
        Ok(Self { dna, parent, client })
    }

    /// Build the redirect URL the caller's web layer should send the user
    /// to. Caller is responsible for generating + persisting `state`
    /// (CSRF) before redirect, and validating it on the callback.
    pub fn build_auth_url(&self, state: &str) -> String {
        let cid = url_encode(self.client.client_id());
        let redir = url_encode(self.client.redirect_uri());
        let scope = url_encode(DEFAULT_SCOPES);
        let st = url_encode(state);
        format!(
            "{base}?client_id={cid}&redirect_uri={redir}&response_type=code&scope={scope}&state={st}",
            base = DEFAULT_AUTH_URL,
            cid = cid,
            redir = redir,
            scope = scope,
            st = st,
        )
    }

    /// Borrow the underlying client (for callers that need direct
    /// token-exchange / userinfo access beyond the trait surface).
    pub fn client(&self) -> &GoogleAuthClient {
        &self.client
    }
}

impl HasDna for GoogleAuthProvider {
    fn dna(&self) -> &Dna { &self.dna }
    fn parent_dna(&self) -> Option<&Dna> { self.parent.as_ref() }
}

#[async_trait]
impl AuthProvider for GoogleAuthProvider {
    fn provider_name(&self) -> &'static str { "google" }

    fn is_passwordless(&self) -> bool { true }

    async fn issue_challenge(&self, c: &AuthChallenge) -> kei_runtime_core::Result<()> {
        match c {
            AuthChallenge::OAuthCode { provider, .. } if provider == "google" => Ok(()),
            AuthChallenge::OAuthCode { provider, .. } => {
                Err(kei_runtime_core::Error::Auth(format!(
                    "wrong provider for google: {provider}"
                )))
            }
            _ => Err(kei_runtime_core::Error::Auth(
                "google AuthProvider only accepts OAuthCode".into(),
            )),
        }
    }

    async fn verify(&self, c: &AuthChallenge) -> kei_runtime_core::Result<AuthSession> {
        let (code, state) = match c {
            AuthChallenge::OAuthCode { provider, code, state } if provider == "google" => {
                (code.as_str(), state.as_str())
            }
            AuthChallenge::OAuthCode { provider, .. } => {
                return Err(kei_runtime_core::Error::Auth(format!(
                    "wrong provider for google: {provider}"
                )));
            }
            _ => return Err(kei_runtime_core::Error::from(Error::MissingState)),
        };
        if state.is_empty() {
            return Err(kei_runtime_core::Error::from(Error::MissingState));
        }
        let token = self.client.exchange_code(code).await.map_err(kei_runtime_core::Error::from)?;
        let info = self
            .client
            .userinfo(&token.access_token)
            .await
            .map_err(kei_runtime_core::Error::from)?;
        let session_dna = DnaBuilder::new("session")
            .caps(["UI"])
            .scope("keiseikit.dev/primitives/kei-auth-google/session")
            .body(state.as_bytes())
            .build()
            .map_err(kei_runtime_core::Error::Dna)?;
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);
        let expires_unix_ms = now_ms.saturating_add(token.expires_in.saturating_mul(1000));
        let user_id = if !info.email.is_empty() { info.email.clone() } else { info.sub.clone() };
        Ok(AuthSession {
            dna: session_dna,
            parent_dna: self.dna.clone(),
            user_id,
            expires_unix_ms,
            user_agent: None,
        })
    }

    async fn revoke(&self, _session: &Dna) -> kei_runtime_core::Result<()> {
        // v0.1 — see module docs.
        Ok(())
    }
}

/// Minimal application/x-www-form-urlencoded percent-encoder. We only
/// need it for `build_auth_url` (a single non-test callsite). RFC 3986
/// unreserved set: `A-Z a-z 0-9 - _ . ~`. Everything else → %HH.
fn url_encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for b in input.bytes() {
        let unreserved = b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~');
        if unreserved {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_encode_basics() {
        assert_eq!(url_encode("a b"), "a%20b");
        assert_eq!(url_encode("openid email profile"), "openid%20email%20profile");
        assert_eq!(url_encode("https://x/cb"), "https%3A%2F%2Fx%2Fcb");
        assert_eq!(url_encode("safe-_.~"), "safe-_.~");
    }

    #[test]
    fn provider_dna_carries_go_cap() {
        let client = GoogleAuthClient::with_urls(
            "http://t/x", "http://u/x", "cid", "secret", "http://r/cb",
        ).unwrap();
        let provider = GoogleAuthProvider::new(client, None).unwrap();
        assert_eq!(provider.provider_name(), "google");
        assert!(provider.is_passwordless());
        let caps = provider.dna().caps();
        assert!(caps.contains("GO"), "expected GO in caps, got {caps}");
        assert!(caps.contains("PR"));
        assert!(caps.contains("AP"));
    }

    #[test]
    fn build_auth_url_has_required_params() {
        let client = GoogleAuthClient::with_urls(
            "http://t/x", "http://u/x", "my-cid", "secret",
            "https://example.com/cb",
        ).unwrap();
        let provider = GoogleAuthProvider::new(client, None).unwrap();
        let url = provider.build_auth_url("xyz");
        assert!(url.starts_with("https://accounts.google.com/o/oauth2/v2/auth?"));
        assert!(url.contains("client_id=my-cid"));
        assert!(url.contains("response_type=code"));
        assert!(url.contains("state=xyz"));
        assert!(url.contains("scope=openid%20email%20profile"));
        assert!(url.contains("redirect_uri=https%3A%2F%2Fexample.com%2Fcb"));
    }
}
