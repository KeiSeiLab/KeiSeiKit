// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! `SledBackend` — async `MemoryBackend` impl that wraps the sync
//! `SledStore` via `tokio::task::spawn_blocking`.

use crate::error::{Error, Result as SlResult};
use crate::store::SledStore;
use kei_runtime_core::traits::memory::{MemoryBackend, MemoryItem, MemoryQuery};
use kei_runtime_core::{Dna, DnaBuilder, HasDna};
use std::path::Path;

pub struct SledBackend {
    dna: Dna,
    parent: Option<Dna>,
    store: SledStore,
}

impl SledBackend {
    /// Open a sled DB at `path` and stamp this backend with a fresh DNA.
    pub fn from_path(path: impl AsRef<Path>, parent: Option<Dna>) -> SlResult<Self> {
        let store = SledStore::from_path(path)?;
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "SL"])
            .scope("keiseikit.dev/primitives/kei-memory-sled")
            .body(b"sled-v0.34")
            .build()
            .map_err(|e| Error::Provider(format!("dna build: {e}")))?;
        Ok(Self { dna, parent, store })
    }

    /// Borrow the inner store (mostly for tests / advanced callers).
    pub fn inner_store(&self) -> &SledStore {
        &self.store
    }
}

impl HasDna for SledBackend {
    fn dna(&self) -> &Dna {
        &self.dna
    }
    fn parent_dna(&self) -> Option<&Dna> {
        self.parent.as_ref()
    }
}

#[async_trait::async_trait]
impl MemoryBackend for SledBackend {
    fn backend_name(&self) -> &'static str {
        "sled"
    }

    async fn store(&self, item: &MemoryItem) -> kei_runtime_core::Result<()> {
        let store = self.store.clone();
        let item = item.clone();
        tokio::task::spawn_blocking(move || store.put_item(&item))
            .await
            .map_err(|e| Error::Join(e.to_string()))?
            .map_err(Into::into)
    }

    async fn query(&self, q: &MemoryQuery) -> kei_runtime_core::Result<Vec<MemoryItem>> {
        let store = self.store.clone();
        let q = q.clone();
        let items: Vec<MemoryItem> = tokio::task::spawn_blocking(move || -> SlResult<_> {
            let raw = store.scan(q.kind.as_deref())?;
            Ok(filter_items(raw, &q))
        })
        .await
        .map_err(|e| Error::Join(e.to_string()))??;
        Ok(items)
    }

    async fn compact(&self, since_ms: i64) -> kei_runtime_core::Result<usize> {
        let store = self.store.clone();
        tokio::task::spawn_blocking(move || store.count_older_than(None, since_ms))
            .await
            .map_err(|e| Error::Join(e.to_string()))?
            .map_err(Into::into)
    }

    async fn mirror_to_remote(&self, _dest_url: &str) -> kei_runtime_core::Result<()> {
        Err(Error::Provider(
            "sled backend does not implement mirror_to_remote; use kei-sleep-sync.sh per RULE 0.15"
                .into(),
        )
        .into())
    }
}

/// Apply post-scan filters (key_prefix, tag_any, since_ms, limit).
/// Sled has no native secondary index; we scan-then-filter in memory.
fn filter_items(raw: Vec<MemoryItem>, q: &MemoryQuery) -> Vec<MemoryItem> {
    let mut out: Vec<MemoryItem> = raw
        .into_iter()
        .filter(|it| match &q.key_prefix {
            Some(p) => it.key.starts_with(p.as_str()),
            None => true,
        })
        .filter(|it| {
            if q.tag_any.is_empty() {
                true
            } else {
                q.tag_any.iter().any(|t| it.tags.iter().any(|x| x == t))
            }
        })
        .filter(|it| match q.since_ms {
            Some(s) => it.created_at_ms >= s,
            None => true,
        })
        .collect();
    if let Some(n) = q.limit {
        out.truncate(n as usize);
    }
    out
}
