// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! [`AppleAuthProvider`] — DNA-bearing [`AuthProvider`] impl for Sign in
//! with Apple.
//!
//! Maps the OAuth code-exchange + unverified id_token decode onto the
//! runtime-core trait surface. `user_id` on the resulting [`AuthSession`]
//! is taken from the JWT `sub` claim (stable Apple user id), NOT `email`
//! — Apple may issue a `@privaterelay.appleid.com` address and the user
//! can change relay/forwarding at any time, so `sub` is the only durable
//! identifier.

use crate::client::AppleAuthClient;
use crate::error::{Error, Result as AppleResult};
use crate::jwt::decode_id_token_unverified;
use async_trait::async_trait;
use kei_runtime_core::traits::auth::{AuthChallenge, AuthProvider, AuthSession};
use kei_runtime_core::{Dna, DnaBuilder, HasDna, Result as CoreResult};
use std::time::{SystemTime, UNIX_EPOCH};

/// DNA-bearing Apple Sign-In auth provider.
#[derive(Debug, Clone)]
pub struct AppleAuthProvider {
    dna: Dna,
    parent: Option<Dna>,
    client: AppleAuthClient,
}

impl AppleAuthProvider {
    /// Build a provider with a fresh DNA serial.
    ///
    /// DNA caps:
    /// - `PR` — primitive
    /// - `AP` — apple
    /// - `AS` — auth (sign-in)
    pub fn new(client: AppleAuthClient, parent: Option<Dna>) -> AppleResult<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "AS"])
            .scope("keiseikit.dev/primitives/kei-auth-apple")
            .body(b"apple-signin-v1")
            .build()?;
        Ok(Self { dna, parent, client })
    }

    fn now_ms() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0)
    }

    /// Synthesize an opaque per-session DNA. The user_id (Apple `sub`) is
    /// hashed into the body so two sessions for the same user produce
    /// distinct serials only via the random nonce.
    fn session_dna(user_id: &str) -> AppleResult<Dna> {
        Ok(DnaBuilder::new("session")
            .caps(["AP", "AS"])
            .scope("keiseikit.dev/sessions/apple")
            .body(user_id.as_bytes())
            .build()?)
    }
}

impl HasDna for AppleAuthProvider {
    fn dna(&self) -> &Dna {
        &self.dna
    }
    fn parent_dna(&self) -> Option<&Dna> {
        self.parent.as_ref()
    }
}

#[async_trait]
impl AuthProvider for AppleAuthProvider {
    fn provider_name(&self) -> &'static str {
        "apple"
    }

    fn is_passwordless(&self) -> bool {
        true
    }

    /// Apple Sign-In has no server-issued challenge step — the user
    /// authorizes in the browser via the redirect to
    /// `https://appleid.apple.com/auth/authorize` and the verifier
    /// receives a `code` on the callback. v0.1 returns Ok(()) here.
    async fn issue_challenge(&self, _c: &AuthChallenge) -> CoreResult<()> {
        Ok(())
    }

    async fn verify(&self, c: &AuthChallenge) -> CoreResult<AuthSession> {
        let code = match c {
            AuthChallenge::OAuthCode { provider, code, .. } if provider == "apple" => code,
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
        let token = self
            .client
            .exchange_code(code)
            .await
            .map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        let claims = decode_id_token_unverified(&token.id_token)
            .map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        let user_id = claims.sub;
        let session_dna = Self::session_dna(&user_id)
            .map_err(|e: Error| -> kei_runtime_core::Error { e.into() })?;
        // expires_in is seconds-from-now per RFC 6749.
        let expires_unix_ms = Self::now_ms() + token.expires_in.saturating_mul(1000);
        Ok(AuthSession {
            dna: session_dna,
            parent_dna: self.dna.clone(),
            user_id,
            expires_unix_ms,
            user_agent: None,
        })
    }

    /// Apple has a `/auth/revoke` endpoint but v0.1 does not invoke it;
    /// the caller is expected to forget the session locally. Full revoke
    /// support is deferred to v0.2 of this cube.
    async fn revoke(&self, _session: &Dna) -> CoreResult<()> {
        Ok(())
    }
}
