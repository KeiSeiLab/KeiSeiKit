// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! [`GoogleAuthProvider`] — `AuthProvider` impl over Google OAuth 2.0 +
//! OIDC userinfo. Builds an [`AuthSession`] whose `user_id` is the OIDC
//! `email` (with `sub` available via the userinfo result if needed).
//!
//! `provider_name = "google"`. `is_passwordless = true`.

use crate::client::{GoogleAuthClient, DEFAULT_AUTH_URL};
use crate::error::{Error, Result};
use crate::pkce::{pkce_challenge, url_encode};
use async_trait::async_trait;
use kei_runtime_core::traits::auth::{AuthChallenge, AuthProvider, AuthSession};
use kei_runtime_core::{Dna, DnaBuilder, HasDna};
use subtle::ConstantTimeEq;
use std::time::{SystemTime, UNIX_EPOCH};

/// Default scope set: OIDC profile + email.
pub const DEFAULT_SCOPES: &str = "openid email profile";

/// `AuthProvider` for Google OAuth 2.0.
pub struct GoogleAuthProvider {
    dna: Dna,
    parent: Option<Dna>,
    client: GoogleAuthClient,
}

impl GoogleAuthProvider {
    /// Build a provider over a pre-configured client.
    pub fn new(client: GoogleAuthClient, parent: Option<Dna>) -> Result<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "GO"])
            .scope("keiseikit.dev/primitives/kei-auth-google")
            .body(b"google-oauth2")
            .build()?;
        Ok(Self { dna, parent, client })
    }

    /// Build the redirect URL for the Google OAuth 2.0 consent screen.
    ///
    /// `state` — CSRF nonce. Store it server-side; compare against the
    /// `expected_state` field of [`AuthChallenge::OAuthCode`] at callback.
    ///
    /// `code_verifier` — plain PKCE verifier (RFC 7636). The challenge
    /// (`BASE64URL(SHA256(verifier))`) is embedded in the URL. Pass the
    /// same `code_verifier` back in [`AuthChallenge::OAuthCode`].
    pub fn build_auth_url(&self, state: &str, code_verifier: &str) -> String {
        let challenge = pkce_challenge(code_verifier);
        let cid = url_encode(self.client.client_id());
        let redir = url_encode(self.client.redirect_uri());
        let scope = url_encode(DEFAULT_SCOPES);
        let st = url_encode(state);
        let cc = url_encode(&challenge);
        format!(
            "{base}?client_id={cid}&redirect_uri={redir}&response_type=code\
             &scope={scope}&state={st}\
             &code_challenge={cc}&code_challenge_method=S256",
            base = DEFAULT_AUTH_URL,
        )
    }

    /// Borrow the underlying client.
    pub fn client(&self) -> &GoogleAuthClient { &self.client }
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
            AuthChallenge::OAuthCode { provider, .. } => Err(
                kei_runtime_core::Error::Auth(format!("wrong provider for google: {provider}"))
            ),
            _ => Err(kei_runtime_core::Error::Auth(
                "google AuthProvider only accepts OAuthCode".into(),
            )),
        }
    }

    async fn verify(&self, c: &AuthChallenge) -> kei_runtime_core::Result<AuthSession> {
        let (code, state, expected_state, code_verifier) = match c {
            AuthChallenge::OAuthCode {
                provider, code, state, expected_state, code_verifier,
            } if provider == "google" => (
                code.as_str(),
                state.as_str(),
                expected_state.as_str(),
                code_verifier.as_deref(),
            ),
            AuthChallenge::OAuthCode { provider, .. } => {
                return Err(kei_runtime_core::Error::Auth(format!(
                    "wrong provider for google: {provider}"
                )));
            }
            _ => return Err(kei_runtime_core::Error::from(Error::MissingState)),
        };
        check_state(state, expected_state)?;
        let token = self.client.exchange_code(code, code_verifier).await
            .map_err(kei_runtime_core::Error::from)?;
        let info = self.client.userinfo(&token.access_token).await
            .map_err(kei_runtime_core::Error::from)?;
        let session_dna = build_session_dna(state)?;
        let now_ms = now_ms();
        let expires_unix_ms = now_ms.saturating_add(token.expires_in.saturating_mul(1000));
        let user_id = if !info.email.is_empty() { info.email } else { info.sub };
        Ok(AuthSession {
            dna: session_dna,
            parent_dna: self.dna.clone(),
            user_id,
            expires_unix_ms,
            user_agent: None,
        })
    }

    async fn revoke(&self, _session: &Dna) -> kei_runtime_core::Result<()> { Ok(()) }
}

fn check_state(got: &str, expected: &str) -> kei_runtime_core::Result<()> {
    let ok: bool = got.as_bytes().ct_eq(expected.as_bytes()).into();
    if !ok { Err(kei_runtime_core::Error::CsrfStateMismatch) } else { Ok(()) }
}

fn build_session_dna(state: &str) -> kei_runtime_core::Result<Dna> {
    DnaBuilder::new("session")
        .caps(["UI"])
        .scope("keiseikit.dev/primitives/kei-auth-google/session")
        .body(state.as_bytes())
        .build()
        .map_err(kei_runtime_core::Error::Dna)
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let url = provider.build_auth_url("xyz", "my-verifier-1234");
        assert!(url.starts_with("https://accounts.google.com/o/oauth2/v2/auth?"));
        assert!(url.contains("client_id=my-cid"));
        assert!(url.contains("response_type=code"));
        assert!(url.contains("state=xyz"));
        assert!(url.contains("scope=openid%20email%20profile"));
        assert!(url.contains("redirect_uri=https%3A%2F%2Fexample.com%2Fcb"));
        assert!(url.contains("code_challenge="));
        assert!(url.contains("code_challenge_method=S256"));
    }
}
