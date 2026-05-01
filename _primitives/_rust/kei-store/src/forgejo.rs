//! ForgejoStore — thin alias of GitHubStore with a different display name.
//!
//! Forgejo is a hard fork of Gitea — git wire protocol identical. Only the
//! base URL and token env var differ; those are resolved from config.

use crate::config::GitRemoteCfg;
use crate::github::GitHubStore;
use anyhow::Result;
use std::path::PathBuf;

pub struct ForgejoStore {
    inner: GitHubStore,
}

impl ForgejoStore {
    pub fn new(local: PathBuf, remote: GitRemoteCfg) -> Result<Self> {
        let inner = GitHubStore::with_name(local, remote, "forgejo")?;
        Ok(Self { inner })
    }
}

impl crate::store_trait::MemoryStore for ForgejoStore {
    fn read(&self, path: &str) -> Result<Vec<u8>> { self.inner.read(path) }
    fn write(&self, path: &str, bytes: &[u8]) -> Result<()> { self.inner.write(path, bytes) }
    fn list(&self, dir: &str) -> Result<Vec<String>> { self.inner.list(dir) }
    fn branch(&self, name: &str) -> Result<()> { self.inner.branch(name) }
    fn commit(&self, message: &str) -> Result<String> { self.inner.commit(message) }
    fn push(&self, branch: &str) -> Result<()> { self.inner.push(branch) }
    fn pull(&self, branch: &str) -> Result<()> { self.inner.pull(branch) }
    fn backend_name(&self) -> &'static str { "forgejo" }
}
