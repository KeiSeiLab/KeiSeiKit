//! Factory — construct a `Box<dyn MemoryStore>` from a Config.
//!
//! v0.14.1: the S3 backend is gated behind `KEI_STORE_ALLOW_S3_STUB=1`
//! because the default build has no real S3 push — it's a local-manifest
//! stub. Previous behaviour silently stored data locally, confusing users
//! who thought their traces were uploaded.
//!
//! v0.21.0: when the crate is built with `--features s3` AND
//! `s3.bucket` is configured, the real `S3CloudStore` is used (no
//! KEI_STORE_ALLOW_S3_STUB gate needed — data really is uploaded).
//! The stub path remains available for users who don't want the AWS SDK
//! in their binary: omit `s3.bucket` and set the stub opt-in env.

use crate::config::{expand_tilde, Config};
use crate::{filesystem::FilesystemStore, forgejo::ForgejoStore, gitea::GiteaStore,
            github::GitHubStore, s3::S3Store};
use crate::store_trait::MemoryStore;
use anyhow::{anyhow, bail, Context, Result};
use std::path::PathBuf;

pub fn build_store(cfg: &Config) -> Result<Box<dyn MemoryStore>> {
    let local = PathBuf::from(cfg.expanded_local_path());
    match cfg.active.backend.as_str() {
        "filesystem" => {
            let p = cfg.filesystem.path.as_deref().map(expand_tilde);
            let path = p.map(PathBuf::from).unwrap_or(local);
            Ok(Box::new(FilesystemStore::new(path)?))
        }
        "github" => Ok(Box::new(GitHubStore::new(local, cfg.github.clone())?)),
        "forgejo" => Ok(Box::new(ForgejoStore::new(local, cfg.forgejo.clone())?)),
        "gitea" => Ok(Box::new(GiteaStore::new(local, cfg.gitea.clone())?)),
        "s3" => build_s3(cfg),
        other => Err(anyhow!("unknown backend: {other}"))
            .context("supported: filesystem | github | forgejo | gitea | s3"),
    }
}

#[cfg(feature = "s3")]
fn build_s3(cfg: &Config) -> Result<Box<dyn MemoryStore>> {
    // Cloud path: real S3 round-trips when bucket is configured.
    if cfg.s3.bucket.is_some() {
        return Ok(Box::new(
            crate::s3_cloud::S3CloudStore::new(cfg.s3.clone())?,
        ));
    }
    // Fallback: local stub (legacy behaviour, requires opt-in).
    build_s3_stub(cfg)
}

#[cfg(not(feature = "s3"))]
fn build_s3(cfg: &Config) -> Result<Box<dyn MemoryStore>> {
    build_s3_stub(cfg)
}

fn build_s3_stub(cfg: &Config) -> Result<Box<dyn MemoryStore>> {
    if std::env::var("KEI_STORE_ALLOW_S3_STUB").is_err() {
        bail!(
            "S3 backend is a local-only MVP stub (no upload to S3/R2/MinIO yet). \
             Set KEI_STORE_ALLOW_S3_STUB=1 to proceed; data will be stored in the \
             configured cache_path only. For real S3 push, build with \
             `--features s3` AND set s3.bucket in config."
        );
    }
    eprintln!(
        "[kei-store] WARNING: S3 backend is a local-only stub — data stored \
         at cache_path only, not pushed to any object store."
    );
    let cache = cfg
        .s3
        .cache_path
        .as_deref()
        .map(expand_tilde)
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("s3 backend requires s3.cache_path"))?;
    Ok(Box::new(S3Store::new(cache, cfg.s3.clone())?))
}
