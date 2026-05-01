// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Typed HTTP client for the Gitea `/api/v1` surface. Three calls are
//! exposed — repo existence probe, user-repo creation, branch SHA
//! lookup — which together cover what `GiteaBackend::ensure_repo`
//! needs. Authentication is a `Bearer <GITEA_TOKEN>` header on every
//! request. The client takes `base_url` + `token` explicitly so tests
//! can point it at a wiremock server.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

const DEFAULT_BASE_URL: &str = "https://gitea.com";

/// Request body for `POST /api/v1/user/repos`. Field names match the
/// Gitea schema verbatim — Gitea accepts unknown extras silently but
/// the canonical set is small so we keep it tight.
#[derive(Debug, Clone, Serialize)]
pub struct CreateRepoRequest {
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    pub private: bool,
    pub auto_init: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub default_branch: String,
}

impl CreateRepoRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            private: true,
            auto_init: true,
            default_branch: "main".into(),
        }
    }
}

/// Subset of Gitea's repository response we consume. Gitea returns
/// many additional fields; serde silently drops them via the default
/// `deny_unknown_fields=false`.
#[derive(Debug, Clone, Deserialize)]
pub struct RepoInfo {
    pub full_name: String,
    pub default_branch: String,
    pub private: bool,
}

/// Branch endpoint returns `{ commit: { id: "<sha>", ... } }`.
#[derive(Debug, Deserialize)]
struct BranchResponse {
    commit: BranchCommit,
}

#[derive(Debug, Deserialize)]
struct BranchCommit {
    id: String,
}

pub struct GiteaClient {
    http: reqwest::Client,
    base_url: String,
    token: String,
}

impl GiteaClient {
    /// Construct from explicit base URL + bearer token. Use this in
    /// tests; in production prefer [`GiteaClient::from_env`].
    pub fn new(base_url: impl Into<String>, token: impl Into<String>) -> Self {
        let base_url = base_url.into();
        let base_url = base_url.trim_end_matches('/').to_string();
        Self {
            http: reqwest::Client::new(),
            base_url,
            token: token.into(),
        }
    }

    /// Read `GITEA_URL` (default `https://gitea.com`) and `GITEA_TOKEN`.
    /// Missing token is `Error::Auth`.
    pub fn from_env() -> Result<Self> {
        let base_url = std::env::var("GITEA_URL")
            .unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());
        let token = std::env::var("GITEA_TOKEN")
            .map_err(|_| Error::Auth("GITEA_TOKEN not set".into()))?;
        Ok(Self::new(base_url, token))
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// `GET /api/v1/repos/{owner}/{repo}` — `Ok(true)` on 200,
    /// `Ok(false)` on 404, `Err(Error::Api)` on anything else.
    pub async fn repo_exists(&self, owner: &str, repo: &str) -> Result<bool> {
        let url = format!("{}/api/v1/repos/{}/{}", self.base_url, owner, repo);
        let resp = self
            .http
            .get(&url)
            .bearer_auth(&self.token)
            .send()
            .await?;
        match resp.status().as_u16() {
            200 => Ok(true),
            404 => Ok(false),
            other => Err(Error::Api {
                status: other,
                endpoint: format!("GET /api/v1/repos/{owner}/{repo}"),
                body: resp.text().await.unwrap_or_default(),
            }),
        }
    }

    /// `POST /api/v1/user/repos` — creates a repo owned by the
    /// authenticated user. Returns the parsed [`RepoInfo`].
    pub async fn create_user_repo(&self, req: &CreateRepoRequest) -> Result<RepoInfo> {
        let url = format!("{}/api/v1/user/repos", self.base_url);
        let resp = self
            .http
            .post(&url)
            .bearer_auth(&self.token)
            .json(req)
            .send()
            .await?;
        let status = resp.status().as_u16();
        if status != 201 && status != 200 {
            return Err(Error::Api {
                status,
                endpoint: "POST /api/v1/user/repos".into(),
                body: resp.text().await.unwrap_or_default(),
            });
        }
        let info: RepoInfo = resp.json().await?;
        Ok(info)
    }

    /// `GET /api/v1/repos/{owner}/{repo}/branches/{branch}` — returns
    /// the tip SHA. 404 maps to `Error::NotFound`.
    pub async fn get_default_branch_sha(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<String> {
        let url = format!(
            "{}/api/v1/repos/{}/{}/branches/{}",
            self.base_url, owner, repo, branch
        );
        let resp = self
            .http
            .get(&url)
            .bearer_auth(&self.token)
            .send()
            .await?;
        let status = resp.status().as_u16();
        if status == 404 {
            return Err(Error::NotFound(format!(
                "branch {branch} on {owner}/{repo}"
            )));
        }
        if status != 200 {
            return Err(Error::Api {
                status,
                endpoint: format!("GET /api/v1/repos/{owner}/{repo}/branches/{branch}"),
                body: resp.text().await.unwrap_or_default(),
            });
        }
        let parsed: BranchResponse = resp.json().await?;
        Ok(parsed.commit.id)
    }
}
