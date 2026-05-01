//! GitHubStore — git-over-SSH/HTTPS.
//!
//! Wraps FilesystemStore for local ops, adds push/pull to a configured
//! remote. SSH auth via `KEI_MEMORY_SSH_KEY` (path to key); HTTPS via
//! `KEI_MEMORY_PAT` (token). Exactly the pattern used in v0.11
//! `kei-sleep-setup.sh`.
//!
//! v0.14.1: pushes to `github.com` are blocked by default under RULE 0.1
//! (patent-IP protection). Forks on Forgejo / Gitea / self-hosted are
//! unaffected since they do not resolve to `github.com`. Override for a
//! genuinely public repo: `KEI_STORE_ALLOW_GITHUB_PUSH=1`.

use crate::config::GitRemoteCfg;
use crate::filesystem::FilesystemStore;
use crate::store_trait::MemoryStore;
use anyhow::{bail, Context, Result};
use std::path::PathBuf;

pub struct GitHubStore {
    inner: FilesystemStore,
    remote: GitRemoteCfg,
    name: &'static str,
}

impl GitHubStore {
    pub fn new(local: PathBuf, remote: GitRemoteCfg) -> Result<Self> {
        Self::with_name(local, remote, "github")
    }

    pub fn with_name(local: PathBuf, remote: GitRemoteCfg, name: &'static str) -> Result<Self> {
        let inner = FilesystemStore::new(local)?;
        Ok(Self { inner, remote, name })
    }

    fn callbacks(&self) -> git2::RemoteCallbacks<'_> {
        let cfg = self.remote.clone();
        let mut cbs = git2::RemoteCallbacks::new();
        cbs.credentials(move |_url, user, _types| credential(&cfg, user));
        cbs
    }

    fn remote_url(&self) -> Result<&str> {
        self.remote
            .url
            .as_deref()
            .context("remote url missing from config")
    }
}

fn credential(cfg: &GitRemoteCfg, user: Option<&str>) -> std::result::Result<git2::Cred, git2::Error> {
    if let Some(var) = cfg.ssh_key_env.as_ref() {
        if let Ok(key_path) = std::env::var(var) {
            let u = user.unwrap_or("git");
            return git2::Cred::ssh_key(u, None, std::path::Path::new(&key_path), None);
        }
    }
    if let Some(var) = cfg.pat_env.as_ref() {
        if let Ok(token) = std::env::var(var) {
            return git2::Cred::userpass_plaintext(user.unwrap_or("x-access-token"), &token);
        }
    }
    git2::Cred::default()
}

impl MemoryStore for GitHubStore {
    fn read(&self, path: &str) -> Result<Vec<u8>> {
        self.inner.read(path)
    }
    fn write(&self, path: &str, bytes: &[u8]) -> Result<()> {
        self.inner.write(path, bytes)
    }
    fn list(&self, dir: &str) -> Result<Vec<String>> {
        self.inner.list(dir)
    }
    fn branch(&self, name: &str) -> Result<()> {
        self.inner.branch(name)
    }
    fn commit(&self, message: &str) -> Result<String> {
        self.inner.commit(message)
    }

    fn push(&self, branch: &str) -> Result<()> {
        let url = self.remote_url()?;
        enforce_github_push_guard(url)?;
        let repo = git2::Repository::open(&self.inner.root)?;
        let mut remote = match repo.find_remote("origin") {
            Ok(r) => r,
            Err(_) => repo.remote("origin", url)?,
        };
        let mut opts = git2::PushOptions::new();
        opts.remote_callbacks(self.callbacks());
        let refspec = format!("refs/heads/{b}:refs/heads/{b}", b = branch);
        remote.push(&[&refspec], Some(&mut opts))?;
        Ok(())
    }

    fn pull(&self, branch: &str) -> Result<()> {
        let repo = git2::Repository::open(&self.inner.root)?;
        let url = self.remote_url()?;
        let mut remote = match repo.find_remote("origin") {
            Ok(r) => r,
            Err(_) => repo.remote("origin", url)?,
        };
        let mut opts = git2::FetchOptions::new();
        opts.remote_callbacks(self.callbacks());
        remote.fetch(&[branch], Some(&mut opts), None)?;
        Ok(())
    }

    fn backend_name(&self) -> &'static str {
        self.name
    }
}

/// RULE 0.1 enforcement point for the kei-store push path.
///
/// Blocks pushes whose URL contains `github.com` unless the caller
/// explicitly opts-in via `KEI_STORE_ALLOW_GITHUB_PUSH=1`. Forks on
/// Forgejo / Gitea / self-hosted remain unaffected — only the literal
/// `github.com` host is gated.
pub(crate) fn enforce_github_push_guard(url: &str) -> Result<()> {
    if !url.contains("github.com") {
        return Ok(());
    }
    if std::env::var("KEI_STORE_ALLOW_GITHUB_PUSH").is_ok() {
        return Ok(());
    }
    bail!(
        "push to github.com blocked by RULE 0.1 (patent-IP protection). \
         Set KEI_STORE_ALLOW_GITHUB_PUSH=1 if this is a public-safe release. \
         See ~/.claude/rules/security.md for banned-project criteria."
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_env::env_lock;

    #[test]
    fn test_github_push_blocked_without_env_var() {
        let _guard = env_lock();
        std::env::remove_var("KEI_STORE_ALLOW_GITHUB_PUSH");
        let err = enforce_github_push_guard("git@github.com:owner/repo.git").unwrap_err();
        let msg = format!("{err:#}");
        assert!(msg.contains("github.com"), "unexpected err: {msg}");
        assert!(msg.contains("RULE 0.1"), "unexpected err: {msg}");
    }

    #[test]
    fn test_github_push_allowed_with_env_var() {
        let _guard = env_lock();
        std::env::set_var("KEI_STORE_ALLOW_GITHUB_PUSH", "1");
        let ok = enforce_github_push_guard("git@github.com:owner/repo.git");
        std::env::remove_var("KEI_STORE_ALLOW_GITHUB_PUSH");
        assert!(ok.is_ok(), "should allow with opt-in env var");
    }

    #[test]
    fn test_non_github_push_always_allowed() {
        // Non-github URLs should always pass regardless of env state, but we
        // still take the lock so we don't observe a half-set var mid-test.
        let _guard = env_lock();
        enforce_github_push_guard("ssh://git@forgejo.local:2222/user/repo.git").unwrap();
        enforce_github_push_guard("https://gitea.example.com/user/repo.git").unwrap();
    }
}
