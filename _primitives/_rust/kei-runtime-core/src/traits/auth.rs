// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use crate::dna::{Dna, HasDna};
use crate::error::Result;
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
    Password { email: String, password: String },
    OAuthCode { provider: String, code: String, state: String },
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
