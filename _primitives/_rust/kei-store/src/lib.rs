//! kei-store — memory-repo backend abstraction.
//!
//! Trait `MemoryStore` + 5 implementations:
//!   - `GitHubStore`, `ForgejoStore`, `GiteaStore` — git-over-SSH/HTTPS
//!   - `FilesystemStore` — local `.git` only; never pushes
//!   - `S3Store` — object-storage with manifest.json (MVP local stub)
//!   - `S3CloudStore` — real S3 / R2 / MinIO via `aws-sdk-s3`
//!     (behind `s3` feature; v0.21+)
//!
//! Config loaded from `~/.claude/agents/_primitives/store-config.toml`
//! by default; overridable via `--config`.
//!
//! RULE 0.8 — this crate reads secret references from env vars only
//! (`KEI_MEMORY_SSH_KEY`, `KEI_MEMORY_PAT`, `AWS_SECRET_ACCESS_KEY`, ...).

pub mod config;
pub mod factory;
pub mod filesystem;
pub mod forgejo;
pub mod gitea;
pub mod github;
pub mod s3;
/// Cloud-backend sub-trait + shared tokio runtime + sync-over-async adapter.
/// Extension seam for future GCS / Azure Blob / Bunny backends (v0.22+).
/// Gated behind `s3` for now — promoted to default once a second cloud
/// backend exists.
#[cfg(feature = "s3")]
pub mod async_backend;
#[cfg(feature = "s3")]
pub mod s3_cloud;
pub mod store_trait;

/// Test hygiene — shared ENV_LOCK for tests that mutate process env.
/// Exposed under `cfg(any(test, feature = "s3"))` so cross-crate smoke
/// tests (which run behind the `s3` feature) can take the same lock.
#[cfg(any(test, feature = "s3"))]
pub mod test_env;

pub use config::Config;
pub use factory::build_store;
pub use store_trait::MemoryStore;
