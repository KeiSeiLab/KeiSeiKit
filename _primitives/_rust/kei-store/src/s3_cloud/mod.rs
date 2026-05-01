//! S3CloudStore — real object-storage backend via `aws-sdk-s3`.
//!
//! v0.22 Track B refactor: this module now contains ONLY the S3-specific
//! construction and re-exports. The sync-over-async runtime bridge,
//! branch-prefix handling, path validation, and commit-manifest semantics
//! all live in `crate::async_backend::AsyncBackendStore`, which is a
//! generic wrapper over any `AsyncBackend` impl.
//!
//! Extension seam: to add a new cloud backend (GCS, Azure Blob, Bunny),
//!
//!   1. Create `src/gcs_cloud/backend.rs` with a struct that impls
//!      `crate::async_backend::AsyncBackend` (4 async methods + `label`).
//!   2. Add `pub type GcsCloudStore = AsyncBackendStore<GcsAsyncBackend>;`.
//!   3. Wire a constructor that calls `AsyncBackendStore::wrap(backend)`.
//!   4. `factory::build_store` dispatches on `cfg.active.backend`.
//!
//! The shared tokio runtime + MemoryStore impl are free.

mod backend;
mod client;
mod keys;

pub use backend::S3AsyncBackend;

use crate::async_backend::{shared_runtime, AsyncBackendStore};
use crate::config::S3Cfg;
use anyhow::Result;

/// Public API: unchanged from v0.21 — `S3CloudStore::new(cfg)` still works.
/// Internally it is `AsyncBackendStore<S3AsyncBackend>`, which solves the
/// N=2-Store runtime footgun of the previous per-instance design.
pub type S3CloudStore = AsyncBackendStore<S3AsyncBackend>;

impl S3CloudStore {
    /// Build a cloud-S3 backend. `bucket` MUST be configured.
    pub fn new(cfg: S3Cfg) -> Result<Self> {
        let backend = shared_runtime().block_on(S3AsyncBackend::new(cfg))?;
        Ok(AsyncBackendStore::wrap(backend))
    }
}

#[cfg(test)]
mod tests;
