//! AsyncBackend — cloud-storage sub-trait + sync-over-async adapter.
//!
//! v0.22 Track B. Adding a new cloud backend (GCS, Azure, Bunny, …) =
//! implement 4 async methods on `AsyncBackend` + `label()`. Runtime glue +
//! branch-prefix + path-validation + commit-manifest are all free.
//!
//! Multi-thread shared runtime (2 workers, IO + time) avoids the N=2-Store
//! footgun of the previous per-instance `current_thread` runtimes — two
//! `AsyncBackendStore` instances in one process no longer risk `block_on`
//! panics when one instance's call runs on the other's runtime thread.

use crate::store_trait::MemoryStore;
use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use std::sync::{Mutex, OnceLock};
use tokio::runtime::{Builder, Runtime};

pub const DEFAULT_BRANCH: &str = "main";

static SHARED_RT: OnceLock<Runtime> = OnceLock::new();

/// Process-global multi-thread tokio runtime.
pub(crate) fn shared_runtime() -> &'static Runtime {
    SHARED_RT.get_or_init(|| {
        Builder::new_multi_thread()
            .worker_threads(2)
            .enable_io()
            .enable_time()
            .thread_name("kei-store-rt")
            .build()
            .expect("kei-store: failed to build shared tokio runtime")
    })
}

/// Reject absolute paths and `..` components. Keeps branch prefix
/// unescapable (CVE-class guard, same contract as `filesystem::safe_join`).
pub fn validate_rel(rel: &str) -> Result<()> {
    if rel.starts_with('/') {
        bail!("path traversal rejected: absolute path {:?}", rel);
    }
    for part in rel.split('/') {
        if part == ".." {
            bail!("path traversal rejected: parent-dir component in {:?}", rel);
        }
    }
    Ok(())
}

/// Tiny DJB2 — deterministic, avoids pulling sha2 just for manifest names.
pub fn short_hash(s: &str) -> String {
    let mut h: u64 = 5381;
    for b in s.bytes() {
        h = h.wrapping_mul(33).wrapping_add(b as u64);
    }
    format!("{:x}", h)
}

/// `manifest-<hex>.json` — format produced by `commit()` below.
pub fn is_manifest_key(k: &str) -> bool {
    k.starts_with("manifest-") && k.ends_with(".json")
}

/// Cloud-storage backend trait. Impls deal with keys only.
#[async_trait]
pub trait AsyncBackend: Send + Sync {
    async fn get(&self, key: &str) -> Result<Vec<u8>>;
    async fn put(&self, key: &str, bytes: Vec<u8>) -> Result<()>;
    /// Single-level list — keys directly under `prefix`, no recursion.
    async fn list(&self, prefix: &str) -> Result<Vec<String>>;
    /// Full recursive list under `prefix`.
    async fn list_recursive(&self, prefix: &str) -> Result<Vec<String>>;
    /// Backend-specific label used by `MemoryStore::backend_name`.
    fn label(&self) -> &'static str;
}

/// Sync wrapper: `MemoryStore` on top of any `AsyncBackend`.
pub struct AsyncBackendStore<B: AsyncBackend> {
    backend: B,
    branch: Mutex<String>,
}

impl<B: AsyncBackend> AsyncBackendStore<B> {
    /// Wrap an already-constructed backend. Renamed from `new` to avoid a
    /// multiple-`new` collision with specialised inherent impls on the
    /// `pub type XyzCloudStore = AsyncBackendStore<XyzAsyncBackend>` sugar.
    pub fn wrap(backend: B) -> Self {
        Self {
            backend,
            branch: Mutex::new(DEFAULT_BRANCH.to_string()),
        }
    }

    pub fn current_branch(&self) -> String {
        self.branch
            .lock()
            .map(|g| g.clone())
            .unwrap_or_else(|p| p.into_inner().clone())
    }

    pub fn key(&self, rel: &str) -> Result<String> {
        validate_rel(rel)?;
        Ok(format!("{}/{}", self.current_branch(), rel))
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }
}

impl<B: AsyncBackend> MemoryStore for AsyncBackendStore<B> {
    fn read(&self, path: &str) -> Result<Vec<u8>> {
        let key = self.key(path)?;
        shared_runtime().block_on(self.backend.get(&key))
    }

    fn write(&self, path: &str, bytes: &[u8]) -> Result<()> {
        let key = self.key(path)?;
        shared_runtime().block_on(self.backend.put(&key, bytes.to_vec()))
    }

    fn list(&self, dir: &str) -> Result<Vec<String>> {
        let raw = self.key(dir)?;
        let prefix = if raw.ends_with('/') { raw } else { format!("{raw}/") };
        shared_runtime().block_on(self.backend.list(&prefix))
    }

    fn branch(&self, name: &str) -> Result<()> {
        validate_rel(name)?;
        let mut g = self.branch.lock().map_err(|_| anyhow!("branch lock poisoned"))?;
        *g = name.to_string();
        Ok(())
    }

    fn commit(&self, message: &str) -> Result<String> {
        let branch_prefix = format!("{}/", self.current_branch());
        let all = shared_runtime()
            .block_on(self.backend.list_recursive(&branch_prefix))
            .with_context(|| format!("list_recursive for commit on {branch_prefix}"))?;
        let mut entries: Vec<String> = all.into_iter().filter(|k| !is_manifest_key(k)).collect();
        entries.sort();
        let manifest = serde_json::json!({
            "message": message,
            "branch": self.current_branch(),
            "entries": entries,
        })
        .to_string();
        let hash = short_hash(&manifest);
        self.write(&format!("manifest-{hash}.json"), manifest.as_bytes())?;
        Ok(hash)
    }

    fn push(&self, _branch: &str) -> Result<()> { Ok(()) }
    fn pull(&self, _branch: &str) -> Result<()> { Ok(()) }
    fn backend_name(&self) -> &'static str { self.backend.label() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_runtime_is_singleton() {
        let a = shared_runtime() as *const Runtime;
        let b = shared_runtime() as *const Runtime;
        assert_eq!(a, b);
    }

    #[test]
    fn validate_rel_rejects_absolute() {
        assert!(format!("{:#}", validate_rel("/etc/passwd").unwrap_err()).contains("absolute"));
    }

    #[test]
    fn validate_rel_rejects_parent() {
        assert!(format!("{:#}", validate_rel("a/../b").unwrap_err()).contains("parent-dir"));
    }

    #[test]
    fn short_hash_deterministic() {
        assert_eq!(short_hash("abc"), short_hash("abc"));
        assert_ne!(short_hash("abc"), short_hash("abd"));
    }

    #[test]
    fn is_manifest_key_matches_format() {
        assert!(is_manifest_key("manifest-deadbeef.json"));
        assert!(!is_manifest_key("traces/a.jsonl"));
    }
}
