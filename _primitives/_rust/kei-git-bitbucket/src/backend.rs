// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! [`BitbucketBackend`] — DNA-bearing [`GitBackend`] impl over [`BitbucketClient`].
//!
//! `ensure_repo` parses `workspace/slug` from the remote URL path. `clone` and
//! `push` shell out to the `git` CLI (no libgit2 dep). `mirror` is a
//! clone-then-push composition.

use crate::client::BitbucketClient;
use crate::error::{Error as BbError, Result as BbResult};
use kei_runtime_core::traits::git::{CommitMeta, GitBackend, GitRemote};
use kei_runtime_core::{Dna, DnaBuilder, HasDna};
use std::path::Path;
use std::process::Command;

pub struct BitbucketBackend {
    dna: Dna,
    parent: Option<Dna>,
    client: BitbucketClient,
}

impl BitbucketBackend {
    pub fn new(client: BitbucketClient, parent: Option<Dna>) -> BbResult<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "BB"])
            .scope("keiseikit.dev/primitives/kei-git-bitbucket")
            .body(b"bitbucket-v2")
            .build()?;
        Ok(Self { dna, parent, client })
    }

    pub fn client(&self) -> &BitbucketClient { &self.client }
}

impl HasDna for BitbucketBackend {
    fn dna(&self) -> &Dna { &self.dna }
    fn parent_dna(&self) -> Option<&Dna> { self.parent.as_ref() }
}

#[async_trait::async_trait]
impl GitBackend for BitbucketBackend {
    fn provider_name(&self) -> &'static str { "bitbucket" }

    async fn ensure_repo(&self, remote: &GitRemote) -> kei_runtime_core::Result<()> {
        let (ws, slug) = parse_workspace_slug(&remote.url).map_err(BbError::from)?;
        let exists = self.client.repo_exists(&ws, &slug).await.map_err(BbError::from)?;
        if !exists {
            self.client.create_repo(&ws, &slug).await.map_err(BbError::from)?;
        }
        Ok(())
    }

    async fn clone(&self, remote: &GitRemote, dest: &Path) -> kei_runtime_core::Result<()> {
        let dest_s = dest.to_string_lossy().to_string();
        run_git(None, &["clone", "--branch", &remote.branch, &remote.url, &dest_s])
            .map_err(BbError::from)?;
        Ok(())
    }

    async fn push(&self, dir: &Path, remote: &GitRemote) -> kei_runtime_core::Result<CommitMeta> {
        let d = dir.to_string_lossy().to_string();
        run_git(Some(&d), &["push", &remote.url, &remote.branch]).map_err(BbError::from)?;
        let sha = run_git(Some(&d), &["rev-parse", "HEAD"]).map_err(BbError::from)?;
        let msg = run_git(Some(&d), &["log", "-1", "--pretty=%B"]).map_err(BbError::from)?;
        let email = run_git(Some(&d), &["log", "-1", "--pretty=%ae"]).map_err(BbError::from)?;
        let ts = run_git(Some(&d), &["log", "-1", "--pretty=%ct"]).map_err(BbError::from)?;
        Ok(CommitMeta {
            sha: sha.trim().into(),
            message: msg.trim().into(),
            author_email: email.trim().into(),
            committed_at_ms: ts.trim().parse::<i64>().unwrap_or(0) * 1000,
        })
    }

    async fn mirror(&self, src: &GitRemote, dst: &GitRemote) -> kei_runtime_core::Result<()> {
        let tmp = std::env::temp_dir()
            .join(format!("kei-git-bitbucket-mirror-{}", std::process::id()));
        let tmp_s = tmp.to_string_lossy().to_string();
        run_git(None, &["clone", "--mirror", &src.url, &tmp_s]).map_err(BbError::from)?;
        run_git(Some(&tmp_s), &["push", "--mirror", &dst.url]).map_err(BbError::from)?;
        Ok(())
    }

    fn supports_auto_create(&self) -> bool { true }
}

/// Parse `workspace/slug` from a Bitbucket remote URL.
/// Supports `https://bitbucket.org/{ws}/{slug}(.git)?` and
/// `git@bitbucket.org:{ws}/{slug}(.git)?`.
fn parse_workspace_slug(url: &str) -> std::result::Result<(String, String), BbError> {
    let path = if let Some(rest) = url.strip_prefix("git@") {
        rest.split_once(':').map(|(_, p)| p).unwrap_or(rest)
    } else if let Some(rest) = url.split("://").nth(1) {
        rest.split_once('/').map(|(_, p)| p).unwrap_or("")
    } else {
        url
    };
    let trimmed = path.trim_end_matches(".git").trim_matches('/');
    let mut parts = trimmed.splitn(2, '/');
    let ws = parts.next().unwrap_or("").to_string();
    let slug = parts.next().unwrap_or("").to_string();
    if ws.is_empty() || slug.is_empty() {
        return Err(BbError::Config(format!(
            "remote URL not parseable as workspace/slug: {url}"
        )));
    }
    Ok((ws, slug))
}

fn run_git(dir: Option<&str>, args: &[&str]) -> std::io::Result<String> {
    let mut cmd = Command::new("git");
    if let Some(d) = dir { cmd.current_dir(d); }
    let out = cmd.args(args).output()?;
    if !out.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("git {} failed: {}", args.join(" "), String::from_utf8_lossy(&out.stderr)),
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_https_url() {
        let (ws, slug) = parse_workspace_slug("https://bitbucket.org/myws/myrepo.git").unwrap();
        assert_eq!((ws.as_str(), slug.as_str()), ("myws", "myrepo"));
    }

    #[test]
    fn parses_ssh_url() {
        let (ws, slug) = parse_workspace_slug("git@bitbucket.org:acme/widget.git").unwrap();
        assert_eq!((ws.as_str(), slug.as_str()), ("acme", "widget"));
    }

    #[test]
    fn rejects_malformed_url() {
        assert!(parse_workspace_slug("https://bitbucket.org/").is_err());
        assert!(parse_workspace_slug("not-a-url").is_err());
    }

    #[test]
    fn dna_has_bb_cap() {
        let client = BitbucketClient::with_url("u", "p", "http://localhost").unwrap();
        let b = BitbucketBackend::new(client, None).unwrap();
        assert!(b.dna().caps().contains("BB"));
        assert_eq!(b.provider_name(), "bitbucket");
        assert!(b.supports_auto_create());
    }
}
