// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use crate::dna::HasDna;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitRemote {
    pub url: String,
    pub branch: String,
    pub auth_kind: GitAuthKind,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GitAuthKind {
    SshKey,
    Pat,
    OAuth,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitMeta {
    pub sha: String,
    pub message: String,
    pub author_email: String,
    pub committed_at_ms: i64,
}

#[async_trait::async_trait]
pub trait GitBackend: HasDna + Send + Sync {
    fn provider_name(&self) -> &'static str;

    async fn ensure_repo(&self, remote: &GitRemote) -> Result<()>;
    async fn clone(&self, remote: &GitRemote, dest: &std::path::Path) -> Result<()>;
    async fn push(&self, dir: &std::path::Path, remote: &GitRemote) -> Result<CommitMeta>;
    async fn mirror(&self, src: &GitRemote, dst: &GitRemote) -> Result<()>;

    /// True if backend supports auto-create of new repos via API
    /// (KeiGit/GitHub: yes; raw SSH/Soft Serve: no).
    fn supports_auto_create(&self) -> bool;
}
