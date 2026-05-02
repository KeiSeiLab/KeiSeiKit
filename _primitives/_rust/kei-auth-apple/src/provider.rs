// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! [`AppleAuthProvider`] — DNA-bearing [`AuthProvider`] impl for Sign in
//! with Apple.
//!
//! `user_id` on the resulting [`AuthSession`] is taken from the JWT `sub`
//! claim (stable Apple user id). The `verify()` method performs ES256
//! signature verification via [`verify_id_token`] against the caller-supplied
//! JWKS JSON.

use crate::client::{AppleAuthClient, DEFAULT_AUTHORIZE_URL};
use crate::error::{Error, Result as AppleResult};
use crate::jwt::verify_id_token;
use async_trait::async_trait;
use base64::Engine as _;
use kei_runtime_core::traits::auth::{AuthChallenge, AuthProvider, AuthSession};
use kei_runtime_core::{Dna, DnaBuilder, HasDna, Result as CoreResult};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use std::time::{SystemTime, UNIX_EPOCH};

/// DNA-bearing Apple Sign-In auth provider.
#[derive(Debug, Clone)]
pub struct AppleAuthProvider {
    dna: Dna,
    parent: Option<Dna>,
    client: AppleAuthClient,
    /// Raw JWKS JSON from `https://appleid.apple.com/auth/keys`.
    /// Caller is responsible for fetching and refreshing. Required in prod.
    jwks_json: String,
}

impl AppleAuthProvider {
    /// Build a provider with a fresh DNA serial.
    ///
    /// `jwks_json` — the raw JSON body of Apple's JWKS endpoint
    /// (`https://appleid.apple.com/auth/keys`). In production, fetch once
    /// at startup and refresh per Apple's Cache-Control headers.
    pub fn new(
        client: AppleAuthClient,
        jwks_json: impl Into<String>,
        parent: Option<Dna>,
    ) -> AppleResult<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "AS"])
            .scope("keiseikit.dev/primitives/kei-auth-apple")
            .body(b"apple-signin-v1")
            .build()?;
        Ok(Self { dna, parent, client, jwks_json: jwks_json.into() })
    }

    /// Build an authorization URL for the Apple Sign-In redirect.
    ///
    /// `state` — the CSRF nonce you generated; pass the same value back as
    /// `expected_state` in the [`AuthChallenge::OAuthCode`] at callback time.
    ///
    /// `code_verifier` — the plain random PKCE verifier (RFC 7636). The
    /// challenge (`BASE64URL(SHA256(verifier))`) is embedded in the URL.
    /// Pass the same `code_verifier` to the token exchange via
    /// [`AuthChallenge::OAuthCode`].
    pub fn build_auth_url(&self, state: &str, code_verifier: &str) -> String {
        let challenge = pkce_challenge(code_verifier);
        let cid = url_encode(self.client.client_id());
        let redir = url_encode(self.client.redirect_uri());
        let st = url_encode(state);
        let cc = url_encode(&challenge);
        format!(
            "{base}?client_id={cid}&redirect_uri={redir}&response_type=code\
             &scope=name%20email&state={st}\
             &code_challenge={cc}&code_challenge_method=S256",
            base = DEFAULT_AUTHORIZE_URL,
        )
    }

    fn now_ms() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0)
    }

    fn session_dna(user_id: &str) -> AppleResult<Dna> {
        Ok(DnaBuilder::new("session")
            .caps(["AP", "AS"])
            .scope("keiseikit.dev/sessions/apple")
            .body(user_id.as_bytes())
            .build()?)
    }
}

impl HasDna for AppleAuthProvider {
    fn dna(&self) -> &Dna { &self.dna }
    fn parent_dna(&self) -> Option<&Dna> { self.parent.as_ref() }
}

#[async_trait]
impl AuthProvider for AppleAuthProvider {
    fn provider_name(&self) -> &'static str { "apple" }
    fn is_passwordless(&self) -> bool { true }

    async fn issue_challenge(&self, _c: &AuthChallenge) -> CoreResult<()> {
        Ok(())
    }

    async fn verify(&self, c: &AuthChallenge) -> CoreResult<AuthSession> {
        let (code, state, expected_state, code_verifier) = match c {
            AuthChallenge::OAuthCode {
                provider, code, state, expected_state, code_verifier,
            } if provider == "apple" => (
                code.as_str(),
                state.as_str(),
                expected_state.as_str(),
                code_verifier.as_deref(),
            ),
            AuthChallenge::OAuthCode { provider, .. } => {
                return Err(kei_runtime_core::Error::Auth(format!(
                    "wrong provider: expected apple, got {provider}"
                )));
            }
            _ => {
                return Err(kei_runtime_core::Error::Auth(
                    "expected OAuthCode challenge".into(),
                ));
            }
        };
        check_state(state, expected_state)?;
        let token = self
            .client
            .exchange_code(code, code_verifier)
            .await
            .map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        let claims = verify_id_token(
            &token.id_token,
            &self.jwks_json,
            self.client.client_id(),
        )
        .map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        let user_id = claims.sub;
        let session_dna = Self::session_dna(&user_id)
            .map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        let expires_unix_ms = Self::now_ms() + token.expires_in.saturating_mul(1000);
        Ok(AuthSession {
            dna: session_dna,
            parent_dna: self.dna.clone(),
            user_id,
            expires_unix_ms,
            user_agent: None,
        })
    }

    async fn revoke(&self, _session: &Dna) -> CoreResult<()> {
        Ok(())
    }
}

/// Constant-time CSRF state comparison. Returns `CsrfStateMismatch` on
/// any mismatch, preventing timing-oracle attacks.
fn check_state(got: &str, expected: &str) -> CoreResult<()> {
    let ok: bool = got.as_bytes().ct_eq(expected.as_bytes()).into();
    if !ok {
        Err(kei_runtime_core::Error::CsrfStateMismatch)
    } else {
        Ok(())
    }
}

/// Compute the PKCE `code_challenge` from a plain `code_verifier`.
/// Returns `BASE64URL-no-pad(SHA256(verifier))` per RFC 7636 §4.2.
pub fn pkce_challenge(verifier: &str) -> String {
    let hash = Sha256::digest(verifier.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash)
}

fn url_encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for b in input.bytes() {
        let unreserved =
            b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~');
        if unreserved {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}
