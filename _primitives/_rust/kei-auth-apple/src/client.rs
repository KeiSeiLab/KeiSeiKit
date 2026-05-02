// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Thin async OAuth client for Apple Sign-In code exchange.
//!
//! Implements only the `POST /auth/token` step (RFC 6749 §4.1.3
//! Authorization Code grant) against the Apple ID endpoint. Apple's
//! `client_secret` is itself an ES256-signed JWT — this cube does NOT
//! sign it; the caller MUST supply a pre-built JWT.

use crate::error::{Error, Result};
use kei_runtime_core::SecretString;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Default authorization endpoint (browser-facing redirect target).
pub const DEFAULT_AUTHORIZE_URL: &str = "https://appleid.apple.com/auth/authorize";
/// Default token endpoint (server-side code exchange POST).
pub const DEFAULT_TOKEN_URL: &str = "https://appleid.apple.com/auth/token";
/// Per-request timeout.
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Apple `/auth/token` response shape (RFC 6749 + Apple-specific fields).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub id_token: String,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub token_type: Option<String>,
}

/// REST client for the Apple `/auth/token` endpoint. Cheap to clone.
#[derive(Debug, Clone)]
pub struct AppleAuthClient {
    http: Client,
    token_url: String,
    client_id: String,
    /// Wrapped in `SecretString` so logs never reveal the JWT.
    client_secret_jwt: SecretString,
    redirect_uri: String,
}

impl AppleAuthClient {
    /// Build with explicit values (use [`DEFAULT_TOKEN_URL`] in prod).
    pub fn with_url(
        token_url: impl Into<String>,
        client_id: impl Into<String>,
        client_secret_jwt: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Result<Self> {
        let http = Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()?;
        Ok(Self {
            http,
            token_url: token_url.into(),
            client_id: client_id.into(),
            client_secret_jwt: SecretString::new(client_secret_jwt),
            redirect_uri: redirect_uri.into(),
        })
    }

    /// Read all three required values from env, default token URL.
    ///
    /// Required env:
    /// - `APPLE_OAUTH_CLIENT_ID`
    /// - `APPLE_CLIENT_SECRET_JWT`
    /// - `APPLE_OAUTH_REDIRECT_URI`
    pub fn from_env() -> Result<Self> {
        let client_id = std::env::var("APPLE_OAUTH_CLIENT_ID").map_err(|_| {
            Error::Api("APPLE_OAUTH_CLIENT_ID env var not set".into())
        })?;
        let client_secret_jwt = std::env::var("APPLE_CLIENT_SECRET_JWT").map_err(|_| {
            Error::Api("APPLE_CLIENT_SECRET_JWT env var not set".into())
        })?;
        let redirect_uri = std::env::var("APPLE_OAUTH_REDIRECT_URI").map_err(|_| {
            Error::Api("APPLE_OAUTH_REDIRECT_URI env var not set".into())
        })?;
        Self::with_url(DEFAULT_TOKEN_URL, client_id, client_secret_jwt, redirect_uri)
    }

    /// Borrow `client_id` (used by `build_auth_url`).
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Borrow `redirect_uri` (used by `build_auth_url`).
    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }

    /// POST application/x-www-form-urlencoded body to `/auth/token`.
    ///
    /// If `code_verifier` is `Some`, it is included as the PKCE
    /// `code_verifier` parameter per RFC 7636 §4.5.
    pub async fn exchange_code(
        &self,
        code: &str,
        code_verifier: Option<&str>,
    ) -> Result<TokenResponse> {
        let secret = self.client_secret_jwt.expose();
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
            return Err(classify(status, body));
        }
        let bytes = resp.bytes().await?;
        if bytes.is_empty() {
            return Err(Error::Api("empty body where token JSON expected".into()));
        }
        let parsed: TokenResponse = serde_json::from_slice(&bytes)?;
        Ok(parsed)
    }
}

fn classify(status: StatusCode, body: String) -> Error {
    Error::Api(format!("http {}: {}", status, body))
}
