// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Thin async REST 2.0 client for Bitbucket Cloud.
//!
//! No upstream Rust SDK is used — we hit the public surface directly
//! (`https://api.bitbucket.org/2.0`) with HTTP Basic auth read from
//! `BITBUCKET_USERNAME` + `BITBUCKET_APP_PASSWORD`. Base URL is overridable
//! for `wiremock` tests via `BITBUCKET_URL`.

use crate::error::{Error, Result};
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine as _;
use reqwest::{Client, Method, RequestBuilder, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Default REST root.
pub const DEFAULT_BASE_URL: &str = "https://api.bitbucket.org/2.0";
/// Per-request timeout.
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Subset of the Bitbucket repository object we depend on.
#[derive(Debug, Clone, Deserialize)]
pub struct Repository {
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub full_name: String,
    #[serde(default)]
    pub scm: String,
    #[serde(default)]
    pub is_private: bool,
}

/// Subset of the branch ref object we depend on.
#[derive(Debug, Clone, Deserialize)]
pub struct BranchRef {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub target: BranchTarget,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct BranchTarget {
    #[serde(default)]
    pub hash: String,
}

/// Body for POST /repositories/{ws}/{slug}.
#[derive(Debug, Clone, Serialize)]
struct CreateRepoBody {
    scm: &'static str,
    is_private: bool,
}

/// REST client. Cheap to clone (`Arc` inside `reqwest::Client`).
#[derive(Debug, Clone)]
pub struct BitbucketClient {
    http: Client,
    base_url: String,
    auth_header: String,
}

impl BitbucketClient {
    /// Build with explicit credentials + base URL (use [`DEFAULT_BASE_URL`] in prod).
    pub fn new(
        username: impl AsRef<str>,
        app_password: impl AsRef<str>,
        base_url: impl Into<String>,
    ) -> Result<Self> {
        let http = Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()?;
        let raw = format!("{}:{}", username.as_ref(), app_password.as_ref());
        let auth_header = format!("Basic {}", B64.encode(raw.as_bytes()));
        Ok(Self { http, base_url: base_url.into(), auth_header })
    }

    /// Read `BITBUCKET_USERNAME` + `BITBUCKET_APP_PASSWORD` (and optional
    /// `BITBUCKET_URL`) from env.
    pub fn from_env() -> Result<Self> {
        let username = std::env::var("BITBUCKET_USERNAME")
            .map_err(|_| Error::Config("BITBUCKET_USERNAME env var not set".into()))?;
        let pw = std::env::var("BITBUCKET_APP_PASSWORD")
            .map_err(|_| Error::Config("BITBUCKET_APP_PASSWORD env var not set".into()))?;
        let base = std::env::var("BITBUCKET_URL")
            .unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());
        Self::new(username, pw, base)
    }

    /// Override base URL (for wiremock tests).
    pub fn with_url(
        username: impl AsRef<str>,
        app_password: impl AsRef<str>,
        base_url: impl Into<String>,
    ) -> Result<Self> {
        Self::new(username, app_password, base_url)
    }

    /// Accessor for the configured base URL.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// GET /repositories/{workspace}/{repo_slug} — `Ok(true)` on 200,
    /// `Ok(false)` on 404, `Err` otherwise.
    pub async fn repo_exists(&self, workspace: &str, slug: &str) -> Result<bool> {
        let url = format!("{}/repositories/{}/{}", self.base_url, workspace, slug);
        let resp = self.req(Method::GET, &url).send().await?;
        let status = resp.status();
        if status.is_success() {
            return Ok(true);
        }
        if status.as_u16() == 404 {
            return Ok(false);
        }
        let body = resp.text().await.unwrap_or_default();
        Err(classify(status, body))
    }

    /// POST /repositories/{workspace}/{repo_slug} with `{scm:"git", is_private:true}`.
    pub async fn create_repo(&self, workspace: &str, slug: &str) -> Result<Repository> {
        let url = format!("{}/repositories/{}/{}", self.base_url, workspace, slug);
        let body = CreateRepoBody { scm: "git", is_private: true };
        let resp = self.send(self.req(Method::POST, &url).json(&body)).await?;
        parse_json(resp).await
    }

    /// GET /repositories/{ws}/{slug}/refs/branches/{branch} — branch SHA.
    pub async fn get_branch_sha(
        &self,
        workspace: &str,
        slug: &str,
        branch: &str,
    ) -> Result<String> {
        let url = format!(
            "{}/repositories/{}/{}/refs/branches/{}",
            self.base_url, workspace, slug, branch
        );
        let resp = self.send(self.req(Method::GET, &url)).await?;
        let br: BranchRef = parse_json(resp).await?;
        Ok(br.target.hash)
    }

    fn req(&self, method: Method, url: &str) -> RequestBuilder {
        self.http
            .request(method, url)
            .header("authorization", &self.auth_header)
            .header("accept", "application/json")
    }

    async fn send(&self, builder: RequestBuilder) -> Result<Response> {
        let resp = builder.send().await?;
        let status = resp.status();
        if status.is_success() {
            return Ok(resp);
        }
        let body = resp.text().await.unwrap_or_default();
        Err(classify(status, body))
    }
}

fn classify(status: StatusCode, body: String) -> Error {
    match status.as_u16() {
        404 => Error::NotFound(body),
        401 | 403 => Error::Auth(format!("http {}: {}", status, body)),
        _ => Error::Api(format!("http {}: {}", status, body)),
    }
}

async fn parse_json<T: serde::de::DeserializeOwned>(resp: Response) -> Result<T> {
    let bytes = resp.bytes().await?;
    if bytes.is_empty() {
        return Err(Error::Api("empty body where JSON expected".into()));
    }
    Ok(serde_json::from_slice(&bytes)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_distinguishes_404_401_other() {
        assert!(matches!(classify(StatusCode::NOT_FOUND, "x".into()), Error::NotFound(_)));
        assert!(matches!(classify(StatusCode::UNAUTHORIZED, "y".into()), Error::Auth(_)));
        assert!(matches!(classify(StatusCode::INTERNAL_SERVER_ERROR, "z".into()), Error::Api(_)));
    }

    #[test]
    fn with_url_builds_client() {
        let c = BitbucketClient::with_url("u", "p", "http://localhost").unwrap();
        assert_eq!(c.base_url(), "http://localhost");
    }
}
