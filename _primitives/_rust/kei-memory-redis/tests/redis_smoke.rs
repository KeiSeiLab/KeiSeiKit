// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Live smoke tests. Compile-only by default. Enable with:
//!
//! ```text
//! cargo test -p kei-memory-redis --features live
//! ```
//!
//! The tests assume a reachable Redis 7+ on `REDIS_URL`
//! (default `redis://127.0.0.1:6379`). They isolate themselves under a
//! per-test prefix and clean up at the end.

#![cfg(feature = "live")]

use kei_memory_redis::{RedisBackend, RedisStore};
use kei_runtime_core::traits::memory::{MemoryBackend, MemoryItem, MemoryQuery};
use kei_runtime_core::DnaBuilder;

fn redis_url() -> String {
    std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".into())
}

fn fresh_prefix(suffix: &str) -> String {
    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("kei-test:{suffix}:{pid}:{nanos}")
}

fn dummy_item(prefix: &str, kind: &str, key: &str, ts: i64, tags: Vec<String>) -> MemoryItem {
    let dna = DnaBuilder::new("primitive")
        .cap("RD")
        .scope(prefix)
        .body(key)
        .build()
        .unwrap();
    MemoryItem {
        dna,
        parent_dna: None,
        kind: kind.into(),
        key: key.into(),
        value: format!(r#"{{"k":"{key}"}}"#),
        tags,
        created_at_ms: ts,
    }
}

#[tokio::test]
async fn store_then_query_roundtrip() {
    let prefix = fresh_prefix("roundtrip");
    let store = RedisStore::from_url(&redis_url(), prefix.clone()).unwrap();
    let backend = RedisBackend::new(store, None).unwrap();

    let it = dummy_item(&prefix, "trace", "session-1", 1_000, vec!["sleep".into()]);
    backend.store(&it).await.expect("store");

    let q = MemoryQuery {
        kind: Some("trace".into()),
        ..Default::default()
    };
    let out = backend.query(&q).await.expect("query");
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].key, "session-1");

    // Cleanup: compact past everything.
    let _ = backend.compact(i64::MAX).await;
}

#[tokio::test]
async fn compact_drops_old_items() {
    let prefix = fresh_prefix("compact");
    let store = RedisStore::from_url(&redis_url(), prefix.clone()).unwrap();
    let backend = RedisBackend::new(store, None).unwrap();

    let old = dummy_item(&prefix, "trace", "old", 100, vec![]);
    let new = dummy_item(&prefix, "trace", "new", 5_000, vec![]);
    backend.store(&old).await.unwrap();
    backend.store(&new).await.unwrap();

    let n = backend.compact(1_000).await.expect("compact");
    assert_eq!(n, 1, "exactly the old item should be removed");

    let q = MemoryQuery::default();
    let remaining = backend.query(&q).await.unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].key, "new");

    let _ = backend.compact(i64::MAX).await;
}

#[tokio::test]
async fn mirror_returns_provider_error() {
    let prefix = fresh_prefix("mirror");
    let store = RedisStore::from_url(&redis_url(), prefix).unwrap();
    let backend = RedisBackend::new(store, None).unwrap();
    let r = backend.mirror_to_remote("redis://elsewhere").await;
    assert!(r.is_err());
}
