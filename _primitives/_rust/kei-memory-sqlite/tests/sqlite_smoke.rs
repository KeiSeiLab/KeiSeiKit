// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! End-to-end smoke tests for [`kei_memory_sqlite::SqliteBackend`].
//! All tests use `SqliteStore::from_memory()` so no tempfile is needed
//! and the suite has no external dependencies.

use kei_memory_sqlite::{SqliteBackend, SqliteStore};
use kei_runtime_core::dna::{Dna, DnaBuilder};
use kei_runtime_core::traits::memory::{MemoryBackend, MemoryItem, MemoryQuery};
use std::sync::Arc;

fn fresh_dna(role: &str) -> Dna {
    DnaBuilder::new(role)
        .caps(["PR", "AP", "SQ"])
        .scope("keiseikit.dev/primitives/kei-memory-sqlite")
        .body(b"sqlite-v3")
        .build()
        .expect("dna build")
}

fn fresh_backend() -> SqliteBackend {
    let store = Arc::new(SqliteStore::from_memory().expect("open"));
    SqliteBackend::new(fresh_dna("primitive"), None, store)
}

fn make_item(kind: &str, key: &str, ts: i64, tags: &[&str]) -> MemoryItem {
    MemoryItem {
        dna: fresh_dna("trace"),
        parent_dna: None,
        kind: kind.to_string(),
        key: key.to_string(),
        value: serde_json::json!({"k": key}).to_string(),
        tags: tags.iter().map(|s| s.to_string()).collect(),
        created_at_ms: ts,
    }
}

#[tokio::test]
async fn store_and_query_roundtrip() {
    let b = fresh_backend();
    let item = make_item("trace", "session-1", 1000, &["sleep", "rem"]);
    b.store(&item).await.expect("store");

    let q = MemoryQuery::default();
    let got = b.query(&q).await.expect("query");
    assert_eq!(got.len(), 1, "single insert must return one row");
    assert_eq!(got[0].key, "session-1");
    assert_eq!(got[0].kind, "trace");
    assert_eq!(got[0].tags, vec!["sleep".to_string(), "rem".to_string()]);
}

#[tokio::test]
async fn key_prefix_filter() {
    let b = fresh_backend();
    b.store(&make_item("trace", "alpha-1", 100, &[])).await.unwrap();
    b.store(&make_item("trace", "alpha-2", 200, &[])).await.unwrap();
    b.store(&make_item("trace", "beta-1", 300, &[])).await.unwrap();

    let q = MemoryQuery {
        key_prefix: Some("alpha-".into()),
        ..Default::default()
    };
    let got = b.query(&q).await.unwrap();
    assert_eq!(got.len(), 2, "alpha- prefix selects 2 of 3");
    // DESC by created_at_ms.
    assert_eq!(got[0].key, "alpha-2");
    assert_eq!(got[1].key, "alpha-1");
}

#[tokio::test]
async fn tag_any_exact_token_filter() {
    let b = fresh_backend();
    // "rem" must NOT match "remix" — exact-token boundary.
    b.store(&make_item("trace", "a", 100, &["rem"])).await.unwrap();
    b.store(&make_item("trace", "b", 200, &["remix"])).await.unwrap();
    b.store(&make_item("trace", "c", 300, &["sleep", "rem"])).await.unwrap();

    let q = MemoryQuery {
        tag_any: vec!["rem".into()],
        ..Default::default()
    };
    let got = b.query(&q).await.unwrap();
    assert_eq!(got.len(), 2, "tag 'rem' matches a and c, NOT b (remix)");
    let keys: Vec<_> = got.iter().map(|i| i.key.clone()).collect();
    assert!(keys.contains(&"a".to_string()));
    assert!(keys.contains(&"c".to_string()));
    assert!(!keys.contains(&"b".to_string()));
}

#[tokio::test]
async fn limit_clamps_result_count() {
    let b = fresh_backend();
    for i in 0..5 {
        b.store(&make_item("trace", &format!("k-{i}"), 100 + i, &[]))
            .await
            .unwrap();
    }
    let q = MemoryQuery {
        limit: Some(2),
        ..Default::default()
    };
    let got = b.query(&q).await.unwrap();
    assert_eq!(got.len(), 2);
    // DESC: newest first.
    assert_eq!(got[0].key, "k-4");
    assert_eq!(got[1].key, "k-3");
}

#[tokio::test]
async fn compact_returns_deleted_count() {
    let b = fresh_backend();
    b.store(&make_item("trace", "old-1", 100, &[])).await.unwrap();
    b.store(&make_item("trace", "old-2", 200, &[])).await.unwrap();
    b.store(&make_item("trace", "new-1", 1000, &[])).await.unwrap();

    let n = b.compact(500).await.expect("compact");
    assert_eq!(n, 2, "two items strictly older than 500 must be removed");
    let remaining = b.query(&MemoryQuery::default()).await.unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].key, "new-1");
}

#[tokio::test]
async fn kind_filter_isolates_namespace() {
    let b = fresh_backend();
    b.store(&make_item("trace", "x", 100, &[])).await.unwrap();
    b.store(&make_item("concept", "y", 200, &[])).await.unwrap();
    b.store(&make_item("report", "z", 300, &[])).await.unwrap();

    let q = MemoryQuery {
        kind: Some("concept".into()),
        ..Default::default()
    };
    let got = b.query(&q).await.unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].kind, "concept");
    assert_eq!(got[0].key, "y");
}

#[tokio::test]
async fn since_ms_filter_inclusive_lower_bound() {
    let b = fresh_backend();
    b.store(&make_item("trace", "old", 100, &[])).await.unwrap();
    b.store(&make_item("trace", "boundary", 500, &[])).await.unwrap();
    b.store(&make_item("trace", "new", 999, &[])).await.unwrap();

    let q = MemoryQuery {
        since_ms: Some(500),
        ..Default::default()
    };
    let got = b.query(&q).await.unwrap();
    assert_eq!(got.len(), 2, "since_ms is inclusive lower bound");
    let keys: Vec<_> = got.iter().map(|i| i.key.clone()).collect();
    assert!(keys.contains(&"boundary".to_string()));
    assert!(keys.contains(&"new".to_string()));
}

#[tokio::test]
async fn mirror_to_remote_returns_provider_error() {
    let b = fresh_backend();
    let r = b.mirror_to_remote("ssh://example/path.git").await;
    assert!(r.is_err(), "mirror_to_remote must surface a Provider error");
}
