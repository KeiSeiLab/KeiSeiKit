// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! [`RedisBackend`] — `MemoryBackend` impl over [`crate::RedisStore`].
//!
//! Storage layout (see `store.rs`):
//! - `<prefix>:item:<kind>:<ts>:<key>` → JSON-serialized [`MemoryItem`]
//! - `<prefix>:tag:<tag>`              → SET of item-id strings
//!
//! `compact(since_ms)` deletes items strictly older than `since_ms`
//! (i.e. `parsed.ts_ms < since_ms`) and returns the deleted count.
//! Tag-set entries pointing at deleted items are removed in the same
//! pass to keep query-by-tag honest.
//!
//! `mirror_to_remote` is intentionally unimplemented: cross-Redis
//! replication is the operator's responsibility (Redis replication /
//! AOF), not this primitive's. Returns `Provider`.

use crate::error::{Error, Result};
use crate::store::{decode_item_key, RedisStore};
use kei_runtime_core::traits::memory::{MemoryBackend, MemoryItem, MemoryQuery};
use kei_runtime_core::{Dna, DnaBuilder, HasDna};
use redis::AsyncCommands;

pub struct RedisBackend {
    dna: Dna,
    parent: Option<Dna>,
    store: RedisStore,
}

impl RedisBackend {
    pub fn new(store: RedisStore, parent: Option<Dna>) -> Result<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "RD"])
            .scope("keiseikit.dev/primitives/kei-memory-redis")
            .body(b"redis-v7")
            .build()
            .map_err(|e| Error::Dna(e.to_string()))?;
        Ok(Self { dna, parent, store })
    }

    pub fn inner_store(&self) -> &RedisStore {
        &self.store
    }

    /// SCAN every `<prefix>:item:<kind?>:*` key and collect them. Used
    /// by `query` and `compact`. SCAN is cooperative and non-blocking.
    async fn scan_item_keys(&self, kind: Option<&str>) -> Result<Vec<String>> {
        let mut conn = self.store.conn().await?;
        let pattern = self.store.item_match(kind);
        let mut iter: redis::AsyncIter<String> =
            conn.scan_match(pattern.as_str()).await?;
        let mut keys = Vec::new();
        while let Some(k) = iter.next_item().await {
            keys.push(k);
        }
        Ok(keys)
    }
}

impl HasDna for RedisBackend {
    fn dna(&self) -> &Dna {
        &self.dna
    }
    fn parent_dna(&self) -> Option<&Dna> {
        self.parent.as_ref()
    }
}

#[async_trait::async_trait]
impl MemoryBackend for RedisBackend {
    fn backend_name(&self) -> &'static str {
        "redis"
    }

    async fn store(&self, item: &MemoryItem) -> kei_runtime_core::Result<()> {
        let item_key = self
            .store
            .item_key(&item.kind, item.created_at_ms, &item.key);
        let payload = serde_json::to_string(item).map_err(Error::from)?;
        let mut conn = self.store.conn().await.map_err(Into::<Error>::into)?;
        // SET the JSON payload (no TTL — retention is operator policy).
        let _: () = conn
            .set(&item_key, payload)
            .await
            .map_err(Error::from)?;
        // Add this item-id to every tag set.
        for tag in &item.tags {
            let tag_key = self.store.tag_key(tag);
            let _: () = conn
                .sadd(&tag_key, &item_key)
                .await
                .map_err(Error::from)?;
        }
        Ok(())
    }

    async fn query(&self, q: &MemoryQuery) -> kei_runtime_core::Result<Vec<MemoryItem>> {
        let keys = self
            .scan_item_keys(q.kind.as_deref())
            .await
            .map_err(Into::<kei_runtime_core::Error>::into)?;

        // Pre-filter on key components (cheap parse) before we GET the
        // payload — saves bandwidth on large key spaces.
        let mut hits: Vec<String> = Vec::new();
        for k in keys {
            let p = match decode_item_key(&k) {
                Some(v) => v,
                None => continue,
            };
            if let Some(prefix) = &q.key_prefix {
                if !p.key.starts_with(prefix.as_str()) {
                    continue;
                }
            }
            if let Some(since) = q.since_ms {
                if p.ts_ms < since {
                    continue;
                }
            }
            hits.push(k);
        }

        // Tag filter: intersect with SMEMBERS of any requested tag.
        if !q.tag_any.is_empty() {
            let mut conn = self.store.conn().await.map_err(Into::<Error>::into)?;
            let mut tag_union: std::collections::HashSet<String> = Default::default();
            for tag in &q.tag_any {
                let members: Vec<String> = conn
                    .smembers(self.store.tag_key(tag))
                    .await
                    .map_err(Error::from)?;
                tag_union.extend(members);
            }
            hits.retain(|k| tag_union.contains(k));
        }

        // GET payloads, decode, sort by ts desc, apply limit.
        let mut conn = self.store.conn().await.map_err(Into::<Error>::into)?;
        let mut items: Vec<MemoryItem> = Vec::with_capacity(hits.len());
        for k in &hits {
            let raw: Option<String> =
                conn.get(k).await.map_err(Error::from)?;
            if let Some(s) = raw {
                let it: MemoryItem = serde_json::from_str(&s).map_err(Error::from)?;
                items.push(it);
            }
        }
        items.sort_by(|a, b| b.created_at_ms.cmp(&a.created_at_ms));
        if let Some(lim) = q.limit {
            items.truncate(lim as usize);
        }
        Ok(items)
    }

    async fn compact(&self, since_ms: i64) -> kei_runtime_core::Result<usize> {
        let keys = self
            .scan_item_keys(None)
            .await
            .map_err(Into::<kei_runtime_core::Error>::into)?;
        let mut to_delete: Vec<String> = Vec::new();
        for k in keys {
            if let Some(p) = decode_item_key(&k) {
                if p.ts_ms < since_ms {
                    to_delete.push(k);
                }
            }
        }
        if to_delete.is_empty() {
            return Ok(0);
        }
        let mut conn = self.store.conn().await.map_err(Into::<Error>::into)?;

        // Drop tag-set membership for every deleted item-id. We don't
        // know the tag list at this point, so SCAN tag keys and SREM
        // each. Cheap relative to compact's overall cost.
        let tag_pattern = format!("{}:tag:*", self.store.prefix());
        let tag_keys: Vec<String> = {
            let mut iter: redis::AsyncIter<String> =
                conn.scan_match(tag_pattern.as_str()).await.map_err(Error::from)?;
            let mut acc: Vec<String> = Vec::new();
            while let Some(t) = iter.next_item().await {
                acc.push(t);
            }
            acc
        };
        for tk in tag_keys {
            for ik in &to_delete {
                let _: i64 = conn.srem(&tk, ik).await.map_err(Error::from)?;
            }
        }

        let n = to_delete.len();
        let _: () = conn.del(&to_delete).await.map_err(Error::from)?;
        Ok(n)
    }

    async fn mirror_to_remote(&self, _dest_url: &str) -> kei_runtime_core::Result<()> {
        Err(kei_runtime_core::Error::Provider(
            "kei-memory-redis: mirror_to_remote is delegated to Redis replication / AOF".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dna_has_rd_cap_and_correct_role() {
        // Use a dummy URL — we never connect; constructor only opens
        // the client (no IO until first command).
        let store = RedisStore::from_url("redis://127.0.0.1:65500", "kei-test").unwrap();
        let b = RedisBackend::new(store, None).unwrap();
        assert_eq!(b.backend_name(), "redis");
        assert_eq!(b.dna().role(), "primitive");
        assert!(b.dna().caps().contains("RD"));
        assert!(b.dna().caps().contains("PR"));
        assert!(b.dna().caps().contains("AP"));
    }

    #[test]
    fn parent_dna_threaded_through() {
        let parent = DnaBuilder::new("vm-managed")
            .cap("RD")
            .scope("test")
            .body("p")
            .build()
            .unwrap();
        let store = RedisStore::from_url("redis://127.0.0.1:65500", "kei-test").unwrap();
        let b = RedisBackend::new(store, Some(parent.clone())).unwrap();
        assert_eq!(b.parent_dna().map(|d| d.as_str()), Some(parent.as_str()));
    }
}
