//! S3AsyncBackend — `AsyncBackend` impl over `aws-sdk-s3::Client`.
//!
//! v0.22 Track B. Holds only the S3-specific pieces: the `aws-sdk-s3`
//! client, the bucket name, and the ListObjectsV2 paginator. Branch-prefix
//! + path-safety + commit-manifest semantics live in
//! `crate::async_backend::AsyncBackendStore<S3AsyncBackend>`.

use super::client;
use crate::async_backend::AsyncBackend;
use crate::config::S3Cfg;
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;

pub struct S3AsyncBackend {
    client: Client,
    bucket: String,
}

impl S3AsyncBackend {
    /// Build the backend. Requires `cfg.bucket` to be set.
    ///
    /// This is async because `aws_config::load()` is async; the thin
    /// `S3CloudStore::new` wrapper in `mod.rs` drives it via the shared
    /// runtime so callers keep the sync signature they already have.
    pub async fn new(cfg: S3Cfg) -> Result<Self> {
        let bucket = cfg
            .bucket
            .clone()
            .ok_or_else(|| anyhow!("s3 backend requires s3.bucket in config"))?;
        let client = client::build_client(&cfg).await?;
        Ok(Self { client, bucket })
    }

    /// Shared ListObjectsV2 paginator. `delim_slash=true` → delimiter="/"
    /// (single-level). `false` → recursive over every key under prefix.
    async fn list_inner(&self, prefix: &str, delim_slash: bool) -> Result<Vec<String>> {
        let mut out = Vec::new();
        let mut token: Option<String> = None;
        let tag = if delim_slash { "s3 list" } else { "s3 list-recursive" };
        loop {
            let mut req = self
                .client
                .list_objects_v2()
                .bucket(&self.bucket)
                .prefix(prefix);
            if delim_slash {
                req = req.delimiter("/");
            }
            if let Some(t) = token.as_ref() {
                req = req.continuation_token(t);
            }
            let resp = req
                .send()
                .await
                .with_context(|| format!("{tag} {prefix}"))?;
            for obj in resp.contents() {
                if let Some(k) = obj.key() {
                    if let Some(name) = k.strip_prefix(prefix) {
                        if !name.is_empty() {
                            out.push(name.to_string());
                        }
                    }
                }
            }
            if resp.is_truncated().unwrap_or(false) {
                token = resp.next_continuation_token().map(|s| s.to_string());
            } else {
                break;
            }
        }
        out.sort();
        Ok(out)
    }
}

#[async_trait]
impl AsyncBackend for S3AsyncBackend {
    async fn get(&self, key: &str) -> Result<Vec<u8>> {
        let resp = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .with_context(|| format!("s3 get_object {key}"))?;
        let body = resp
            .body
            .collect()
            .await
            .with_context(|| format!("s3 read body {key}"))?;
        Ok(body.into_bytes().to_vec())
    }

    async fn put(&self, key: &str, bytes: Vec<u8>) -> Result<()> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(ByteStream::from(bytes))
            .send()
            .await
            .with_context(|| format!("s3 put_object {key}"))?;
        Ok(())
    }

    async fn list(&self, prefix: &str) -> Result<Vec<String>> {
        self.list_inner(prefix, true).await
    }

    async fn list_recursive(&self, prefix: &str) -> Result<Vec<String>> {
        self.list_inner(prefix, false).await
    }

    fn label(&self) -> &'static str {
        "s3-cloud"
    }
}
