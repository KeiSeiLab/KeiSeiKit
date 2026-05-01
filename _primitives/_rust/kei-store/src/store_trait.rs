//! MemoryStore trait — single point of truth for every backend.

use anyhow::Result;

pub trait MemoryStore: Send + Sync {
    /// Read a byte blob at a relative path.
    fn read(&self, path: &str) -> Result<Vec<u8>>;

    /// Write a byte blob at a relative path. Creates parents.
    fn write(&self, path: &str, bytes: &[u8]) -> Result<()>;

    /// List regular files under a relative directory (non-recursive).
    fn list(&self, dir: &str) -> Result<Vec<String>>;

    /// Create a branch (git) or a logical "snapshot namespace" (S3).
    fn branch(&self, name: &str) -> Result<()>;

    /// Commit staged changes; returns the object id / manifest hash.
    fn commit(&self, message: &str) -> Result<String>;

    /// Push a branch to the remote (no-op for FilesystemStore).
    fn push(&self, branch: &str) -> Result<()>;

    /// Pull a branch from the remote (no-op for FilesystemStore).
    fn pull(&self, branch: &str) -> Result<()>;

    /// Human-readable backend name for `status` reporting.
    fn backend_name(&self) -> &'static str;
}
