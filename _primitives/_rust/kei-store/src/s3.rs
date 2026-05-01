//! S3Store — object-storage backend (MVP stub; v0.14.1 local-only).
//!
//! This is a local-manifest-based implementation intended as an offline MVP.
//! Reads/writes go to `cache_path`; `commit` serialises a
//! `manifest-<hash>.json` listing the current file tree + content hash;
//! `push`/`pull` are NO-OPs in stub mode.
//!
//! v0.14.1: because the backend does NOT actually reach S3, the factory
//! now refuses to build an `S3Store` unless `KEI_STORE_ALLOW_S3_STUB=1`
//! is set. Previously users who configured S3 were silently writing to a
//! local cache with no remote push. See `factory.rs` for the guard.
//!
//! v0.14.1 hardening: `full()` rejects absolute paths and `..` components
//! (same CVE class as `filesystem.rs` — user-supplied `rel` could escape
//! the cache root).
//!
//! Production S3/R2/MinIO support is planned via `aws-sdk-s3` behind a
//! feature flag — see README §Store backends. This stub keeps the trait
//! surface honest so downstream code can exercise the full kei-store
//! API without pulling a ~20 MB AWS SDK at install time.

use crate::config::S3Cfg;
use crate::filesystem::safe_join;
use crate::store_trait::MemoryStore;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct S3Store {
    pub cache: PathBuf,
    pub cfg: S3Cfg,
}

impl S3Store {
    pub fn new(cache: PathBuf, cfg: S3Cfg) -> Result<Self> {
        fs::create_dir_all(&cache).with_context(|| format!("mkdir {}", cache.display()))?;
        Ok(Self { cache, cfg })
    }

    fn full(&self, rel: &str) -> Result<PathBuf> {
        safe_join(&self.cache, rel)
    }
}

impl MemoryStore for S3Store {
    fn read(&self, path: &str) -> Result<Vec<u8>> {
        fs::read(self.full(path)?).with_context(|| format!("read {}", path))
    }

    fn write(&self, path: &str, bytes: &[u8]) -> Result<()> {
        let full = self.full(path)?;
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(full, bytes)?;
        Ok(())
    }

    fn list(&self, dir: &str) -> Result<Vec<String>> {
        let full = self.full(dir)?;
        if !full.exists() {
            return Ok(Vec::new());
        }
        let mut out = Vec::new();
        for e in fs::read_dir(&full)? {
            let e = e?;
            if e.file_type()?.is_file() {
                if let Some(n) = e.file_name().to_str() {
                    out.push(n.to_string());
                }
            }
        }
        out.sort();
        Ok(out)
    }

    fn branch(&self, name: &str) -> Result<()> {
        // Logical snapshot namespace — stored under cache/<branch>/.
        // Also guarded against traversal so a malicious branch name cannot
        // escape the cache root.
        let dir = self.full(name)?;
        fs::create_dir_all(dir)?;
        Ok(())
    }

    fn commit(&self, message: &str) -> Result<String> {
        let manifest = build_manifest(&self.cache, message)?;
        let hash = short_hash(&manifest);
        let out = self.cache.join(format!("manifest-{hash}.json"));
        fs::write(&out, manifest)?;
        Ok(hash)
    }

    fn push(&self, _branch: &str) -> Result<()> {
        // Production path: aws-sdk-s3 put_object loop. Stub: no-op.
        Ok(())
    }

    fn pull(&self, _branch: &str) -> Result<()> {
        Ok(())
    }

    fn backend_name(&self) -> &'static str {
        "s3-local-stub"
    }
}

fn build_manifest(root: &PathBuf, message: &str) -> Result<String> {
    let mut entries: Vec<String> = Vec::new();
    if root.exists() {
        for e in fs::read_dir(root)? {
            let e = e?;
            if e.file_type()?.is_file() {
                if let Some(n) = e.file_name().to_str() {
                    entries.push(n.to_string());
                }
            }
        }
    }
    entries.sort();
    let v = serde_json::json!({
        "message": message,
        "entries": entries,
    });
    Ok(v.to_string())
}

fn short_hash(s: &str) -> String {
    // Tiny DJB2 — cheap, deterministic, avoids pulling sha2 just for stub.
    let mut h: u64 = 5381;
    for b in s.bytes() {
        h = h.wrapping_mul(33).wrapping_add(b as u64);
    }
    format!("{:x}", h)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store(root: PathBuf) -> S3Store {
        S3Store::new(root, S3Cfg::default()).unwrap()
    }

    #[test]
    fn test_absolute_path_rejected_s3() {
        let tmp = tempfile::tempdir().unwrap();
        let s = store(tmp.path().join("cache"));
        let err = s.write("/etc/passwd", b"nope").unwrap_err();
        let msg = format!("{err:#}");
        assert!(msg.contains("absolute"), "unexpected err: {msg}");
    }

    #[test]
    fn test_parent_dir_rejected_s3() {
        let tmp = tempfile::tempdir().unwrap();
        let s = store(tmp.path().join("cache"));
        let err = s.write("../../secret", b"nope").unwrap_err();
        let msg = format!("{err:#}");
        assert!(msg.contains("parent-dir"), "unexpected err: {msg}");
    }

    #[test]
    fn test_normal_path_ok_s3() {
        let tmp = tempfile::tempdir().unwrap();
        let s = store(tmp.path().join("cache"));
        s.write("a/b.txt", b"ok").unwrap();
        let bytes = s.read("a/b.txt").unwrap();
        assert_eq!(&bytes, b"ok");
    }
}
