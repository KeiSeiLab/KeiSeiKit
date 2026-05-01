// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

use crate::dna::{Dna, HasDna};
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryItem {
    pub dna: Dna,
    pub parent_dna: Option<Dna>,
    pub kind: String,         // "trace" | "concept" | "report" | ...
    pub key: String,
    pub value: String,        // JSON-encoded payload
    pub tags: Vec<String>,
    pub created_at_ms: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryQuery {
    pub kind: Option<String>,
    pub key_prefix: Option<String>,
    pub tag_any: Vec<String>,
    pub limit: Option<u32>,
    pub since_ms: Option<i64>,
}

#[async_trait::async_trait]
pub trait MemoryBackend: HasDna + Send + Sync {
    fn backend_name(&self) -> &'static str;

    async fn store(&self, item: &MemoryItem) -> Result<()>;
    async fn query(&self, q: &MemoryQuery) -> Result<Vec<MemoryItem>>;

    /// Phase B REM consolidation entry-point. Implementations decide
    /// what compaction means (kei-memory: pattern extraction; Zep:
    /// summary edge addition; mem0: profile update).
    async fn compact(&self, since_ms: i64) -> Result<usize>;

    /// Mirror to a remote — used by per-user VM to push memory diffs
    /// to the user's git host (RULE 0.15 sleep-sync).
    async fn mirror_to_remote(&self, dest_url: &str) -> Result<()>;
}
