// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Thin async REST client for a public Forgejo (Codeberg-style) instance.
//!
//! Forgejo's HTTP surface is intentionally Gitea-compatible, so the same
//! `/api/v1` endpoints work against Forgejo, Gitea, and Codeberg without
//! special-casing. We hit only what `GitBackend::ensure_repo` needs:
//! `repo_exists`, `create_user_repo`, `get_default_branch_sha`.

use crate::error::{Error, Result};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_TIMEOUT_SECS: u64 = 30;
const DEFAULT_FORGEJO_URL: &str = "https://codeberg.org";

/// Repo metadata as returned by `GET /api/v1/repos/{owner}/{name}`.
/// Fields are a deliberate subset; Forgejo returns many more we ignore.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoInfo {
    pub name: String,
    #[serde(default)]
    pub default_branch: String,
}

/// Branch metadata as returned by `GET /api/v1/repos/{o}/{n}/branches/{br}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: BranchCommit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchCommit {
    pub id: String,
}

#[derive(Debug, Serialize)]
struct CreateRepoBody<'a> {
    name: &'a str,
    private: bool,
    default_branch: &'a str,
    auto_init: bool,
}

/// Async REST client for a Forgejo/Codeberg/Gitea instance.
#[derive(Debug, Clone)]
pub struct ForgejoClient {
    http: Client,
    base_url: String,
    token: String,
}

impl ForgejoClient {
    /// Build from `FORGEJO_URL` (defaults to `https://codeberg.org`) and
    /// `FORGEJO_TOKEN` (required — public Forgejo bans unauthenticated
    /// writes by default).
    pub fn from_env() -> Result<Self> {
        let base = std::env::var("FORGEJO_URL").unwrap_or_else(|_| DEFAULT_FORGEJO_URL.into());
        let token = std::env::var("FORGEJO_TOKEN")
            .map_err(|_| Error::Config("FORGEJO_TOKEN unset".into()))?;
        Self::with_url(base, token)
    }

    /// Explicit-URL constructor — used by `wiremock` tests and any caller
    /// that doesn't want process-env fallthrough.
    pub fn with_url(base_url: impl Into<String>, token: impl Into<String>) -> Result<Self> {
        let http = Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()
            .map_err(Error::from)?;
        Ok(Self { http, base_url: base_url.into(), token: token.into() })
    }

    /// `GET /api/v1/repos/{owner}/{name}` → `Ok(true)` on 200, `Ok(false)`
    /// on 404, `Err` on any other class.
    pub async fn repo_exists(&self, owner: &str, name: &str) -> Result<bool> {
        let url = format!("{}/api/v1/repos/{}/{}", self.base_url, owner, name);
        let resp = self
            .http
            .get(&url)
            .bearer_auth(&self.token)
            .header("accept", "application/json")
            .send()
            .await?;
        match resp.status() {
            StatusCode::OK => Ok(true),
            StatusCode::NOT_FOUND => Ok(false),
            s if s.is_client_error() && (s == StatusCode::UNAUTHORIZED || s == StatusCode::FORBIDDEN) => {
                Err(Error::Auth(format!("repo_exists {s}")))
            }
            s => Err(Error::Provider(format!("repo_exists http {s}"))),
        }
    }

    /// `POST /api/v1/user/repos` — create under the authenticated user.
    /// The owner segment is implicit (whoever owns `FORGEJO_TOKEN`); we
    /// only pass the repo name.
    pub async fn create_user_repo(&self, name: &str, private: bool, default_branch: &str) -> Result<RepoInfo> {
        let url = format!("{}/api/v1/user/repos", self.base_url);
        let body = CreateRepoBody { name, private, default_branch, auto_init: true };
        let resp = self
            .http
            .post(&url)
            .bearer_auth(&self.token)
            .header("accept", "application/json")
            .json(&body)
            .send()
            .await?;
        let status = resp.status();
        if !status.is_success() {
            let txt = resp.text().await.unwrap_or_default();
            return Err(match status {
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Error::Auth(txt),
                StatusCode::CONFLICT => Error::Provider(format!("repo exists: {txt}")),
                s => Error::Provider(format!("create http {s}: {txt}")),
            });
        }
        let info: RepoInfo = resp.json().await?;
        Ok(info)
    }

    /// `GET /api/v1/repos/{o}/{n}/branches/{br}` — used for sanity-checking
    /// that a default-branch SHA exists after `ensure_repo` returns.
    pub async fn get_default_branch_sha(&self, owner: &str, name: &str, branch: &str) -> Result<String> {
        let url = format!("{}/api/v1/repos/{}/{}/branches/{}", self.base_url, owner, name, branch);
        let resp = self
            .http
            .get(&url)
            .bearer_auth(&self.token)
            .header("accept", "application/json")
            .send()
            .await?;
        match resp.status() {
            StatusCode::OK => {
                let b: Branch = resp.json().await?;
                Ok(b.commit.id)
            }
            StatusCode::NOT_FOUND => Err(Error::NotFound(format!("{owner}/{name}@{branch}"))),
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Err(Error::Auth("branch fetch".into())),
            s => Err(Error::Provider(format!("branch http {s}"))),
        }
    }

    /// Borrow the configured base URL (used by the backend to embed PAT
    /// into clone/push URLs).
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Borrow the configured token (used by the backend to embed PAT into
    /// clone/push URLs).
    pub fn token(&self) -> &str {
        &self.token
    }
}
