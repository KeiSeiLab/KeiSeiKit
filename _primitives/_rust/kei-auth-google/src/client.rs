// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Thin async client for Google OAuth 2.0 token + OIDC userinfo endpoints.
//!
//! Two HTTP calls cover the verify path:
//! 1. `POST {token_url}` (x-www-form-urlencoded) → access_token + id_token
//! 2. `GET {userinfo_url}` with `Authorization: Bearer <access_token>`

use crate::error::{Error, Result};
use kei_runtime_core::SecretString;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_TIMEOUT_SECS: u64 = 30;
const DEFAULT_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const DEFAULT_USERINFO_URL: &str = "https://openidconnect.googleapis.com/v1/userinfo";
/// Authorization endpoint — used only by [`super::provider::GoogleAuthProvider::build_auth_url`].
pub const DEFAULT_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";

/// Token-endpoint response (RFC 6749 §5.1 + OIDC `id_token`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    #[serde(default)]
    pub expires_in: i64,
    #[serde(default)]
    pub id_token: Option<String>,
}

/// Userinfo response (OIDC core §5.3.2 — only the fields we surface).
///
/// `email_verified` is **load-bearing for security**: a Google Workspace
/// admin can mint accounts with arbitrary unverified email aliases, and
/// a relying party that trusts `email` without checking the verified
/// flag is vulnerable to the CVE-2023-7028 class of account-takeover
/// (Booking.com / Slack / GitLab). Always pair the `email` field with
/// the verified flag at the call site.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub sub: String,
    #[serde(default)]
    pub email: String,
    /// OIDC `email_verified` boolean. Defaults to `false` when the
    /// provider omits the claim — that matches the safe interpretation
    /// (refuse rather than trust).
    #[serde(default)]
    pub email_verified: bool,
    #[serde(default)]
    pub name: String,
}

/// Async client wrapping the two relevant Google endpoints.
#[derive(Debug, Clone)]
pub struct GoogleAuthClient {
    http: Client,
    token_url: String,
    userinfo_url: String,
    client_id: String,
    /// Wrapped in `SecretString` so it prints as `<redacted>` in logs.
    client_secret: SecretString,
    redirect_uri: String,
}

impl GoogleAuthClient {
    /// Build from `GOOGLE_OAUTH_CLIENT_ID`, `GOOGLE_OAUTH_CLIENT_SECRET`,
    /// `GOOGLE_OAUTH_REDIRECT_URI`. Uses production token + userinfo URLs.
    pub fn from_env() -> Result<Self> {
        let client_id = std::env::var("GOOGLE_OAUTH_CLIENT_ID")
            .map_err(|_| Error::Config("GOOGLE_OAUTH_CLIENT_ID unset".into()))?;
        let client_secret = std::env::var("GOOGLE_OAUTH_CLIENT_SECRET")
            .map_err(|_| Error::Config("GOOGLE_OAUTH_CLIENT_SECRET unset".into()))?;
        let redirect_uri = std::env::var("GOOGLE_OAUTH_REDIRECT_URI")
            .map_err(|_| Error::Config("GOOGLE_OAUTH_REDIRECT_URI unset".into()))?;
        Self::with_urls(
            DEFAULT_TOKEN_URL, DEFAULT_USERINFO_URL,
            client_id, client_secret, redirect_uri,
        )
    }

    /// Explicit-URL constructor — used by `wiremock` and any caller that
    /// wants to bypass process-env lookup.
    pub fn with_urls(
        token_url: impl Into<String>,
        userinfo_url: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Result<Self> {
        let http = Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()
            .map_err(Error::from)?;
        Ok(Self {
            http,
            token_url: token_url.into(),
            userinfo_url: userinfo_url.into(),
            client_id: client_id.into(),
            client_secret: SecretString::new(client_secret),
            redirect_uri: redirect_uri.into(),
        })
    }

    /// `POST {token_url}` (x-www-form-urlencoded) →
    /// [`TokenResponse`]. RFC 6749 §4.1.3 authorization-code grant.
    ///
    /// If `code_verifier` is `Some`, it is appended as the PKCE
    /// `code_verifier` parameter per RFC 7636 §4.5.
    pub async fn exchange_code(
        &self,
        code: &str,
        code_verifier: Option<&str>,
    ) -> Result<TokenResponse> {
        let secret = self.client_secret.expose();
        let mut form: Vec<(&str, &str)> = vec![
            ("client_id", self.client_id.as_str()),
            ("client_secret", secret),
            ("code", code),
            ("redirect_uri", self.redirect_uri.as_str()),
            ("grant_type", "authorization_code"),
        ];
        if let Some(cv) = code_verifier {
            form.push(("code_verifier", cv));
        }
        let resp = self
            .http
            .post(&self.token_url)
            .header("accept", "application/json")
            .form(&form)
            .send()
            .await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(Error::Api(format!("token http {status}: {body}")));
        }
        let token: TokenResponse = resp.json().await?;
        Ok(token)
    }

    /// `GET {userinfo_url}` with `Authorization: Bearer <access_token>`.
    pub async fn userinfo(&self, access_token: &str) -> Result<UserInfo> {
        let resp = self
            .http
            .get(&self.userinfo_url)
            .bearer_auth(access_token)
            .header("accept", "application/json")
            .send()
            .await?;
        match resp.status() {
            StatusCode::OK => {
                let info: UserInfo = resp.json().await?;
                Ok(info)
            }
            StatusCode::NOT_FOUND => Err(Error::NotFound("userinfo".into())),
            s => {
                let body = resp.text().await.unwrap_or_default();
                Err(Error::Api(format!("userinfo http {s}: {body}")))
            }
        }
    }

    /// Borrow `client_id` (used by `build_auth_url`).
    pub fn client_id(&self) -> &str { &self.client_id }

    /// Borrow `redirect_uri` (used by `build_auth_url`).
    pub fn redirect_uri(&self) -> &str { &self.redirect_uri }
}
