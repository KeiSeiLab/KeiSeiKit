// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! [`ForgejoBackend`] — `GitBackend` impl for public Forgejo / Codeberg.
//!
//! Network IO splits two ways:
//! - Repo lifecycle (exists / create / branch SHA) → REST via `ForgejoClient`.
//! - Working-copy ops (clone / push / mirror) → shell-out to `git`.
//!
//! `provider_name = "forgejo"`. `supports_auto_create = true` because the
//! `/api/v1/user/repos` POST will create on demand inside `ensure_repo`.

use crate::client::ForgejoClient;
use crate::error::{Error, Result};
use async_trait::async_trait;
use kei_runtime_core::traits::git::{CommitMeta, GitBackend, GitRemote};
use kei_runtime_core::{Dna, DnaBuilder, HasDna};
use std::path::Path;
use tokio::process::Command;

pub struct ForgejoBackend {
    dna: Dna,
    parent: Option<Dna>,
    client: ForgejoClient,
}

impl ForgejoBackend {
    /// Build a backend from a pre-configured client. The DNA is a fresh
    /// primitive serial with caps `["PR", "AP", "FJ"]`.
    pub fn new(client: ForgejoClient, parent: Option<Dna>) -> Result<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "FJ"])
            .scope("keiseikit.dev/primitives/kei-git-forgejo")
            .body(b"forgejo-pub-v1")
            .build()?;
        Ok(Self { dna, parent, client })
    }

    /// Borrow the underlying client (for callers that need direct REST
    /// access beyond the `GitBackend` trait surface).
    pub fn client(&self) -> &ForgejoClient {
        &self.client
    }

    /// Embed PAT into a clone/push URL. `https://host/o/r.git` →
    /// `https://x-access-token:{tok}@host/o/r.git`. Non-https URLs are
    /// passed through unchanged (SSH paths etc.).
    fn auth_url(&self, url: &str) -> String {
        if let Some(rest) = url.strip_prefix("https://") {
            format!("https://x-access-token:{}@{}", self.client.token(), rest)
        } else {
            url.to_string()
        }
    }
}

impl HasDna for ForgejoBackend {
    fn dna(&self) -> &Dna { &self.dna }
    fn parent_dna(&self) -> Option<&Dna> { self.parent.as_ref() }
}

#[async_trait]
impl GitBackend for ForgejoBackend {
    fn provider_name(&self) -> &'static str { "forgejo" }

    fn supports_auto_create(&self) -> bool { true }

    async fn ensure_repo(&self, remote: &GitRemote) -> kei_runtime_core::Result<()> {
        let (owner, name) = parse_owner_repo(&remote.url).map_err(kei_runtime_core::Error::from)?;
        let exists = self.client.repo_exists(&owner, &name).await.map_err(kei_runtime_core::Error::from)?;
        if !exists {
            self.client
                .create_user_repo(&name, /* private */ false, &remote.branch)
                .await
                .map_err(kei_runtime_core::Error::from)?;
        }
        Ok(())
    }

    async fn clone(&self, remote: &GitRemote, dest: &Path) -> kei_runtime_core::Result<()> {
        let url = self.auth_url(&remote.url);
        let status = Command::new("git")
            .arg("clone")
            .arg("--branch").arg(&remote.branch)
            .arg(&url)
            .arg(dest)
            .status()
            .await
            .map_err(|e| kei_runtime_core::Error::from(Error::Io(e)))?;
        if !status.success() {
            return Err(kei_runtime_core::Error::Provider(format!("git clone exit {status}")));
        }
        Ok(())
    }

    async fn push(&self, dir: &Path, remote: &GitRemote) -> kei_runtime_core::Result<CommitMeta> {
        let url = self.auth_url(&remote.url);
        let status = Command::new("git")
            .current_dir(dir)
            .arg("push")
            .arg(&url)
            .arg(&remote.branch)
            .status()
            .await
            .map_err(|e| kei_runtime_core::Error::from(Error::Io(e)))?;
        if !status.success() {
            return Err(kei_runtime_core::Error::Provider(format!("git push exit {status}")));
        }
        let (owner, name) = parse_owner_repo(&remote.url).map_err(kei_runtime_core::Error::from)?;
        let sha = self
            .client
            .get_default_branch_sha(&owner, &name, &remote.branch)
            .await
            .map_err(kei_runtime_core::Error::from)?;
        Ok(CommitMeta {
            sha,
            message: String::new(),
            author_email: String::new(),
            committed_at_ms: 0,
        })
    }

    async fn mirror(&self, src: &GitRemote, dst: &GitRemote) -> kei_runtime_core::Result<()> {
        let tmp = std::env::temp_dir()
            .join(format!("kei-git-forgejo-mirror-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        let src_url = self.auth_url(&src.url);
        let mirror_status = Command::new("git")
            .arg("clone").arg("--mirror")
            .arg(&src_url).arg(&tmp)
            .status()
            .await
            .map_err(|e| kei_runtime_core::Error::from(Error::Io(e)))?;
        if !mirror_status.success() {
            return Err(kei_runtime_core::Error::Provider(format!("mirror clone exit {mirror_status}")));
        }
        let dst_url = self.auth_url(&dst.url);
        let push_status = Command::new("git")
            .current_dir(&tmp)
            .arg("push").arg("--mirror").arg(&dst_url)
            .status()
            .await
            .map_err(|e| kei_runtime_core::Error::from(Error::Io(e)))?;
        let _ = std::fs::remove_dir_all(&tmp);
        if !push_status.success() {
            return Err(kei_runtime_core::Error::Provider(format!("mirror push exit {push_status}")));
        }
        Ok(())
    }
}

/// Extract `(owner, repo)` from a Forgejo-style HTTPS URL. Accepts
/// `https://host/owner/repo.git` and `https://host/owner/repo`.
fn parse_owner_repo(url: &str) -> Result<(String, String)> {
    let stripped = url.trim_end_matches(".git");
    let segments: Vec<&str> = stripped.rsplit('/').take(2).collect();
    if segments.len() != 2 || segments.iter().any(|s| s.is_empty()) {
        return Err(Error::Config(format!("cannot parse owner/repo from {url}")));
    }
    Ok((segments[1].to_string(), segments[0].to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_https_dot_git() {
        let (o, r) = parse_owner_repo("https://codeberg.org/me/demo.git").unwrap();
        assert_eq!(o, "me");
        assert_eq!(r, "demo");
    }

    #[test]
    fn parse_https_no_suffix() {
        let (o, r) = parse_owner_repo("https://codeberg.org/me/demo").unwrap();
        assert_eq!(o, "me");
        assert_eq!(r, "demo");
    }

    #[test]
    fn parse_rejects_short() {
        assert!(parse_owner_repo("https://codeberg.org/").is_err());
    }
}
