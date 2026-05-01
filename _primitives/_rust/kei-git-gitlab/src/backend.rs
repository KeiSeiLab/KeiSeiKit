// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! `GitlabBackend` — `GitBackend` trait impl over GitLab REST v4 + git CLI.
//! API: existence / auto-create / branch-SHA. Heavy ops (clone/push/mirror)
//! shell out to system `git`. Auth: PRIVATE-TOKEN for API; git CLI uses URL
//! credentials or the user's git credential helper.

use crate::client::{parse_owner_repo, GitlabClient};
use crate::error::{Error, Result};
use kei_runtime_core::traits::git::{CommitMeta, GitBackend, GitRemote};
use kei_runtime_core::{Dna, DnaBuilder, HasDna};
use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;

pub struct GitlabBackend {
    dna: Dna,
    parent: Option<Dna>,
    client: GitlabClient,
}

impl GitlabBackend {
    pub fn new(client: GitlabClient, parent: Option<Dna>) -> Result<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "GL"])
            .scope("keiseikit.dev/primitives/kei-git-gitlab")
            .body(b"gitlab-v4")
            .build()?;
        Ok(Self { dna, parent, client })
    }

    /// Convenience: build from `GITLAB_URL` + `GITLAB_TOKEN`.
    pub fn from_env(parent: Option<Dna>) -> Result<Self> {
        let client = GitlabClient::from_env()?;
        Self::new(client, parent)
    }

    pub fn client(&self) -> &GitlabClient { &self.client }
}

impl HasDna for GitlabBackend {
    fn dna(&self) -> &Dna { &self.dna }
    fn parent_dna(&self) -> Option<&Dna> { self.parent.as_ref() }
}

#[async_trait::async_trait]
impl GitBackend for GitlabBackend {
    fn provider_name(&self) -> &'static str { "gitlab" }

    async fn ensure_repo(&self, remote: &GitRemote) -> kei_runtime_core::Result<()> {
        let path = parse_owner_repo(&remote.url).map_err(kei_runtime_core::Error::from)?;
        let exists = self
            .client
            .project_exists(&path)
            .await
            .map_err(kei_runtime_core::Error::from)?;
        if exists {
            return Ok(());
        }
        let bare_name = path
            .rsplit_once('/')
            .map(|(_, n)| n.to_string())
            .unwrap_or(path.clone());
        self.client
            .create_project(&bare_name)
            .await
            .map_err(kei_runtime_core::Error::from)?;
        Ok(())
    }

    async fn clone(
        &self,
        remote: &GitRemote,
        dest: &Path,
    ) -> kei_runtime_core::Result<()> {
        run_git(
            &["clone", "--branch", &remote.branch, &remote.url, "."],
            Some(dest),
        )
        .await
        .map_err(kei_runtime_core::Error::from)
    }

    async fn push(
        &self,
        dir: &Path,
        remote: &GitRemote,
    ) -> kei_runtime_core::Result<CommitMeta> {
        run_git(&["push", "origin", &remote.branch], Some(dir))
            .await
            .map_err(kei_runtime_core::Error::from)?;
        let path = parse_owner_repo(&remote.url).map_err(kei_runtime_core::Error::from)?;
        let sha = self
            .client
            .get_branch_sha(&path, &remote.branch)
            .await
            .map_err(kei_runtime_core::Error::from)?;
        let message = git_capture(&["log", "-1", "--pretty=%s"], dir).await?;
        let author_email = git_capture(&["log", "-1", "--pretty=%ae"], dir).await?;
        let ts_str = git_capture(&["log", "-1", "--pretty=%ct"], dir).await?;
        let committed_at_ms = ts_str
            .trim()
            .parse::<i64>()
            .map(|s| s * 1000)
            .unwrap_or(0);
        Ok(CommitMeta {
            sha,
            message: message.trim().to_string(),
            author_email: author_email.trim().to_string(),
            committed_at_ms,
        })
    }

    async fn mirror(
        &self,
        src: &GitRemote,
        dst: &GitRemote,
    ) -> kei_runtime_core::Result<()> {
        let tmp = tempdir_path()?;
        run_git(&["clone", "--mirror", &src.url, "."], Some(&tmp))
            .await
            .map_err(kei_runtime_core::Error::from)?;
        run_git(&["remote", "set-url", "--push", "origin", &dst.url], Some(&tmp))
            .await
            .map_err(kei_runtime_core::Error::from)?;
        run_git(&["push", "--mirror"], Some(&tmp))
            .await
            .map_err(kei_runtime_core::Error::from)?;
        Ok(())
    }

    fn supports_auto_create(&self) -> bool { true }
}

async fn run_git(args: &[&str], cwd: Option<&Path>) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());
    if let Some(d) = cwd {
        cmd.current_dir(d);
    }
    let out = cmd.output().await.map_err(Error::Io)?;
    if !out.status.success() {
        return Err(Error::Git(format!(
            "git {} -> {}: {}",
            args.join(" "),
            out.status,
            String::from_utf8_lossy(&out.stderr)
        )));
    }
    Ok(())
}

async fn git_capture(args: &[&str], cwd: &Path) -> kei_runtime_core::Result<String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .await
        .map_err(|e| kei_runtime_core::Error::Io(e))?;
    if !out.status.success() {
        return Err(kei_runtime_core::Error::Provider(format!(
            "git {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr)
        )));
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

fn tempdir_path() -> Result<std::path::PathBuf> {
    let base = std::env::temp_dir();
    let nonce: u64 = {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0)
    };
    let p = base.join(format!("kei-git-gitlab-{nonce:x}"));
    std::fs::create_dir_all(&p)?;
    Ok(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dna_has_gl_cap() {
        let client = GitlabClient::with_url("http://127.0.0.1:1", "tok").unwrap();
        let b = GitlabBackend::new(client, None).unwrap();
        assert!(b.dna().caps().contains("GL"));
        assert_eq!(b.provider_name(), "gitlab");
        assert!(b.supports_auto_create());
    }
}
