// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use crate::dna::{Dna, HasDna};
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub dna: Dna,
    pub parent_dna: Dna,
    pub key: String,                 // remote key / path
    pub bytes: u64,
    pub sha256: String,
    pub created_at_ms: i64,
}

#[async_trait::async_trait]
pub trait Backup: HasDna + Send + Sync {
    fn destination_name(&self) -> &'static str;

    async fn push(
        &self,
        local_path: &std::path::Path,
        parent_dna: &Dna,
    ) -> Result<Snapshot>;

    async fn list(&self, prefix: &str) -> Result<Vec<Snapshot>>;
    async fn restore(&self, snap: &Snapshot, dest: &std::path::Path) -> Result<()>;
    async fn prune_older_than(&self, ms: i64) -> Result<usize>;
}
