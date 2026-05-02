// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use crate::dna::{Dna, HasDna};
use crate::error::Result;
use crate::secrets::SecretString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession {
    pub dna: Dna,
    pub parent_dna: Dna,            // user's DNA
    pub user_id: String,
    pub expires_unix_ms: i64,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthChallenge {
    MagicLink { email: String },
    /// `password` is wrapped in [`SecretString`] so it prints as
    /// `<redacted>` in logs and is zeroed on drop.
    Password { email: String, password: SecretString },
    /// `state` — the value returned by the OAuth provider in the callback.
    /// `expected_state` — the nonce generated when the auth URL was built;
    /// must equal `state` (verified via constant-time comparison in each
    /// provider's `verify()` impl).
    ///
    /// `code_verifier` — the plain PKCE verifier (RFC 7636) originally passed
    /// to `build_auth_url`. Store this alongside `state` in the session-store
    /// when building the auth URL; pass it back here at callback time so
    /// `verify()` can thread it through to the token exchange endpoint.
    /// `None` retains legacy "no PKCE" behavior.
    OAuthCode {
        provider: String,
        code: String,
        state: String,
        expected_state: String,
        code_verifier: Option<String>,
    },
    SshKeySig { key_id: String, signature: String },
}

#[async_trait::async_trait]
pub trait AuthProvider: HasDna + Send + Sync {
    fn provider_name(&self) -> &'static str;

    async fn issue_challenge(&self, c: &AuthChallenge) -> Result<()>;
    async fn verify(&self, c: &AuthChallenge) -> Result<AuthSession>;
    async fn revoke(&self, session: &Dna) -> Result<()>;

    /// True if this provider supports passwordless flows.
    fn is_passwordless(&self) -> bool;
}
