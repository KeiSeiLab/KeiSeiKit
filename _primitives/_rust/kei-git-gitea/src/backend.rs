// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! `GiteaBackend` — `GitBackend` impl over [`GiteaClient`]. API calls
//! handle existence + creation; clone / push / mirror shell out to the
//! `git` CLI (no libgit2 dependency, mirrors `kei-git-keigit`).

use crate::client::{CreateRepoRequest, GiteaClient};
use crate::error::{Error as GtError, Result as GtResult};
use kei_runtime_core::traits::git::{CommitMeta, GitBackend, GitRemote};
use kei_runtime_core::{Dna, DnaBuilder, HasDna};
use std::path::Path;
use std::process::Command;

pub struct GiteaBackend {
    dna: Dna,
    parent: Option<Dna>,
    client: GiteaClient,
}

impl GiteaBackend {
    pub fn new(client: GiteaClient, parent: Option<Dna>) -> GtResult<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "GT"])
            .scope("keiseikit.dev/primitives/kei-git-gitea")
            .body(b"gitea-v1")
            .build()?;
        Ok(Self { dna, parent, client })
    }

    /// Construct from `GITEA_URL` + `GITEA_TOKEN` env vars.
    pub fn from_env(parent: Option<Dna>) -> GtResult<Self> {
        Self::new(GiteaClient::from_env()?, parent)
    }

    pub fn client(&self) -> &GiteaClient {
        &self.client
    }
}

impl HasDna for GiteaBackend {
    fn dna(&self) -> &Dna { &self.dna }
    fn parent_dna(&self) -> Option<&Dna> { self.parent.as_ref() }
}

#[async_trait::async_trait]
impl GitBackend for GiteaBackend {
    fn provider_name(&self) -> &'static str { "gitea" }
    fn supports_auto_create(&self) -> bool { true }

    async fn ensure_repo(&self, remote: &GitRemote) -> kei_runtime_core::Result<()> {
        let (owner, repo) = parse_owner_repo(&remote.url).map_err(GtError::from)?;
        let exists = self.client.repo_exists(&owner, &repo).await
            .map_err(kei_runtime_core::Error::from)?;
        if !exists {
            let req = CreateRepoRequest::new(repo);
            self.client.create_user_repo(&req).await
                .map_err(kei_runtime_core::Error::from)?;
        }
        Ok(())
    }

    async fn clone(&self, remote: &GitRemote, dest: &Path) -> kei_runtime_core::Result<()> {
        let dest_str = dest.to_string_lossy().to_string();
        run_git(&["clone", "--branch", &remote.branch, &remote.url, &dest_str])
            .map_err(kei_runtime_core::Error::from)
    }

    async fn push(&self, dir: &Path, remote: &GitRemote) -> kei_runtime_core::Result<CommitMeta> {
        let cwd = dir.to_path_buf();
        run_git_in(&cwd, &["push", &remote.url, &remote.branch])
            .map_err(kei_runtime_core::Error::from)?;
        let sha = git_capture_in(&cwd, &["rev-parse", "HEAD"])
            .map_err(kei_runtime_core::Error::from)?;
        let message = git_capture_in(&cwd, &["log", "-1", "--pretty=%s"])
            .map_err(kei_runtime_core::Error::from)?;
        let author_email = git_capture_in(&cwd, &["log", "-1", "--pretty=%ae"])
            .map_err(kei_runtime_core::Error::from)?;
        let ts_secs = git_capture_in(&cwd, &["log", "-1", "--pretty=%ct"])
            .map_err(kei_runtime_core::Error::from)?;
        let committed_at_ms = ts_secs.trim().parse::<i64>().unwrap_or(0) * 1000;
        Ok(CommitMeta {
            sha: sha.trim().to_string(),
            message: message.trim().to_string(),
            author_email: author_email.trim().to_string(),
            committed_at_ms,
        })
    }

    async fn mirror(&self, src: &GitRemote, dst: &GitRemote) -> kei_runtime_core::Result<()> {
        let tmp = std::env::temp_dir()
            .join(format!("kei-git-gitea-mirror-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        let tmp_str = tmp.to_string_lossy().to_string();
        run_git(&["clone", "--mirror", &src.url, &tmp_str])
            .map_err(kei_runtime_core::Error::from)?;
        run_git_in(&tmp, &["push", "--mirror", &dst.url])
            .map_err(kei_runtime_core::Error::from)?;
        let _ = std::fs::remove_dir_all(&tmp);
        Ok(())
    }
}

/// Parse `https://gitea.example/<owner>/<repo>(.git)` → `(owner, repo)`.
fn parse_owner_repo(url: &str) -> GtResult<(String, String)> {
    let trimmed = url.trim_end_matches(".git");
    let segs: Vec<&str> = trimmed.rsplitn(3, '/').collect();
    if segs.len() < 2 || segs[0].is_empty() || segs[1].is_empty() {
        return Err(GtError::Config(format!("cannot parse owner/repo from {url}")));
    }
    Ok((segs[1].to_string(), segs[0].to_string()))
}

fn run_git(args: &[&str]) -> GtResult<()> {
    let status = Command::new("git").args(args).status()
        .map_err(|e| GtError::GitCli(format!("spawn git: {e}")))?;
    if !status.success() {
        return Err(GtError::GitCli(format!("git {} failed (exit={:?})", args.join(" "), status.code())));
    }
    Ok(())
}

fn run_git_in(cwd: &Path, args: &[&str]) -> GtResult<()> {
    let status = Command::new("git").current_dir(cwd).args(args).status()
        .map_err(|e| GtError::GitCli(format!("spawn git: {e}")))?;
    if !status.success() {
        return Err(GtError::GitCli(format!("git {} failed (exit={:?})", args.join(" "), status.code())));
    }
    Ok(())
}

fn git_capture_in(cwd: &Path, args: &[&str]) -> GtResult<String> {
    let out = Command::new("git").current_dir(cwd).args(args).output()
        .map_err(|e| GtError::GitCli(format!("spawn git: {e}")))?;
    if !out.status.success() {
        return Err(GtError::GitCli(format!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr)
        )));
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

