// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! Thin REST v4 client. PRIVATE-TOKEN header auth.
//! Project identity: numeric `project_id` OR url-encoded `namespace/name`.
//! We always url-encode so callers can pass either form transparently.

use crate::error::{Error, Result};
use serde::Deserialize;

const DEFAULT_BASE: &str = "https://gitlab.com";

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectInfo {
    pub id: u64,
    pub path_with_namespace: String,
    #[serde(default)]
    pub default_branch: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BranchInfo {
    pub name: String,
    pub commit: BranchCommit,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BranchCommit {
    pub id: String,
}

pub struct GitlabClient {
    base: String,
    token: String,
    http: reqwest::Client,
}

impl GitlabClient {
    /// Construct from explicit base URL (used by wiremock tests + self-hosted).
    pub fn with_url(base: impl Into<String>, token: impl Into<String>) -> Result<Self> {
        let base = base.into().trim_end_matches('/').to_string();
        let token = token.into();
        if token.is_empty() {
            return Err(Error::Auth("empty token".into()));
        }
        let http = reqwest::Client::builder()
            .build()
            .map_err(|e| Error::Network(e.to_string()))?;
        Ok(Self { base, token, http })
    }

    /// Construct from `GITLAB_URL` (default https://gitlab.com) + `GITLAB_TOKEN`.
    pub fn from_env() -> Result<Self> {
        let base =
            std::env::var("GITLAB_URL").unwrap_or_else(|_| DEFAULT_BASE.to_string());
        let token = std::env::var("GITLAB_TOKEN")
            .map_err(|_| Error::Auth("GITLAB_TOKEN unset".into()))?;
        Self::with_url(base, token)
    }

    /// `path_with_namespace` is the "owner/repo" form (NOT url-encoded — we
    /// encode internally). Returns Ok(true) on 200, Ok(false) on 404.
    pub async fn project_exists(&self, path_with_namespace: &str) -> Result<bool> {
        let id = urlencoding::encode(path_with_namespace);
        let url = format!("{}/api/v4/projects/{}", self.base, id);
        let resp = self.send(self.http.get(&url)).await?;
        match resp.status().as_u16() {
            200 => Ok(true),
            404 => Ok(false),
            s => Err(Error::Api { status: s, body: read_body(resp).await }),
        }
    }

    /// Create a private project under the authenticated user's namespace.
    /// `name` is the bare repo name (no slash).
    pub async fn create_project(&self, name: &str) -> Result<ProjectInfo> {
        if name.contains('/') {
            return Err(Error::Config(format!(
                "create_project takes bare name, got: {name}"
            )));
        }
        let url = format!("{}/api/v4/projects", self.base);
        let body = serde_json::json!({ "name": name, "visibility": "private" });
        let resp = self.send(self.http.post(&url).json(&body)).await?;
        match resp.status().as_u16() {
            200 | 201 => {
                let info: ProjectInfo = resp.json().await?;
                Ok(info)
            }
            s => Err(Error::Api { status: s, body: read_body(resp).await }),
        }
    }

    /// Branch SHA. `id_or_path` accepts numeric `id` OR `owner/repo`.
    pub async fn get_branch_sha(&self, id_or_path: &str, branch: &str) -> Result<String> {
        let id = urlencoding::encode(id_or_path);
        let br = urlencoding::encode(branch);
        let url = format!(
            "{}/api/v4/projects/{}/repository/branches/{}",
            self.base, id, br
        );
        let resp = self.send(self.http.get(&url)).await?;
        match resp.status().as_u16() {
            200 => {
                let info: BranchInfo = resp.json().await?;
                Ok(info.commit.id)
            }
            404 => Err(Error::NotFound(format!("{id_or_path}@{branch}"))),
            s => Err(Error::Api { status: s, body: read_body(resp).await }),
        }
    }

    async fn send(&self, rb: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        let resp = rb
            .header("PRIVATE-TOKEN", &self.token)
            .header("Accept", "application/json")
            .send()
            .await?;
        Ok(resp)
    }
}

async fn read_body(resp: reqwest::Response) -> String {
    resp.text().await.unwrap_or_default()
}

/// Parse `owner/repo` from a remote URL. Accepts https://, http://, scp-style
/// (`git@host:owner/repo`), and ssh:// forms. Returns the bare `owner/repo`
/// (no `.git`, no trailing slash).
pub fn parse_owner_repo(remote_url: &str) -> Result<String> {
    let s = remote_url.trim();
    let after_host = if let Some(rest) = s.strip_prefix("git@") {
        rest.split_once(':').map(|(_, r)| r).ok_or_else(|| {
            Error::Config(format!("malformed scp-style remote: {remote_url}"))
        })?
    } else if let Some(rest) = s.strip_prefix("https://") {
        rest.split_once('/').map(|(_, r)| r).ok_or_else(|| {
            Error::Config(format!("malformed https remote: {remote_url}"))
        })?
    } else if let Some(rest) = s.strip_prefix("http://") {
        rest.split_once('/').map(|(_, r)| r).ok_or_else(|| {
            Error::Config(format!("malformed http remote: {remote_url}"))
        })?
    } else if let Some(rest) = s.strip_prefix("ssh://") {
        let no_user = rest.trim_start_matches("git@");
        no_user.split_once('/').map(|(_, r)| r).ok_or_else(|| {
            Error::Config(format!("malformed ssh remote: {remote_url}"))
        })?
    } else {
        return Err(Error::Config(format!("unrecognized remote: {remote_url}")));
    };
    let trimmed = after_host.trim_end_matches('/').trim_end_matches(".git");
    if !trimmed.contains('/') {
        return Err(Error::Config(format!(
            "remote missing namespace/name: {remote_url}"
        )));
    }
    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_https_remote() {
        assert_eq!(
            parse_owner_repo("https://gitlab.com/alice/proj.git").unwrap(),
            "alice/proj"
        );
        assert_eq!(
            parse_owner_repo("https://gitlab.example.com/grp/sub/proj").unwrap(),
            "grp/sub/proj"
        );
    }

    #[test]
    fn parse_scp_and_ssh_remotes() {
        assert_eq!(
            parse_owner_repo("git@gitlab.com:alice/proj.git").unwrap(),
            "alice/proj"
        );
        assert_eq!(
            parse_owner_repo("ssh://git@gitlab.com/alice/proj.git").unwrap(),
            "alice/proj"
        );
    }

    #[test]
    fn parse_rejects_garbage() {
        assert!(parse_owner_repo("not-a-url").is_err());
        assert!(parse_owner_repo("https://gitlab.com/no-namespace").is_err());
    }

    #[test]
    fn empty_token_rejected() {
        assert!(GitlabClient::with_url("https://x", "").is_err());
    }
}
