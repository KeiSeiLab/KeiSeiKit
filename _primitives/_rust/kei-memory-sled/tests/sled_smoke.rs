// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Smoke tests for `SledBackend`. Each test gets its own `tempdir` so
//! sled file-locking doesn't cross-contaminate.

use kei_memory_sled::SledBackend;
use kei_runtime_core::traits::memory::{MemoryBackend, MemoryItem, MemoryQuery};
use kei_runtime_core::{DnaBuilder, HasDna};
use tempfile::tempdir;

fn make_item(kind: &str, key: &str, ts_ms: i64, tags: &[&str]) -> MemoryItem {
    let dna = DnaBuilder::new("trace")
        .cap("MM")
        .scope("test")
        .body(key.as_bytes())
        .build()
        .expect("dna");
    MemoryItem {
        dna,
        parent_dna: None,
        kind: kind.into(),
        key: key.into(),
        value: serde_json::json!({"k": key}).to_string(),
        tags: tags.iter().map(|s| (*s).to_string()).collect(),
        created_at_ms: ts_ms,
    }
}

#[tokio::test]
async fn store_and_query_roundtrip() {
    let dir = tempdir().unwrap();
    let backend = SledBackend::from_path(dir.path(), None).unwrap();

    let item = make_item("trace", "session-1", 1_000, &["alpha"]);
    backend.store(&item).await.unwrap();

    let got = backend.query(&MemoryQuery::default()).await.unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].key, "session-1");
    assert_eq!(got[0].kind, "trace");
    assert_eq!(got[0].tags, vec!["alpha"]);
    assert_eq!(backend.backend_name(), "sled");
    // DNA caps include SL marker.
    assert!(backend.dna().caps().contains("SL"));
}

#[tokio::test]
async fn key_prefix_filter() {
    let dir = tempdir().unwrap();
    let backend = SledBackend::from_path(dir.path(), None).unwrap();
    backend
        .store(&make_item("trace", "alpha-1", 100, &[]))
        .await
        .unwrap();
    backend
        .store(&make_item("trace", "alpha-2", 200, &[]))
        .await
        .unwrap();
    backend
        .store(&make_item("trace", "beta-1", 300, &[]))
        .await
        .unwrap();

    let q = MemoryQuery {
        key_prefix: Some("alpha-".into()),
        ..Default::default()
    };
    let got = backend.query(&q).await.unwrap();
    assert_eq!(got.len(), 2);
    assert!(got.iter().all(|it| it.key.starts_with("alpha-")));
}

#[tokio::test]
async fn tag_any_filter() {
    let dir = tempdir().unwrap();
    let backend = SledBackend::from_path(dir.path(), None).unwrap();
    backend
        .store(&make_item("note", "a", 10, &["red"]))
        .await
        .unwrap();
    backend
        .store(&make_item("note", "b", 20, &["green"]))
        .await
        .unwrap();
    backend
        .store(&make_item("note", "c", 30, &["red", "blue"]))
        .await
        .unwrap();

    let q = MemoryQuery {
        tag_any: vec!["red".into()],
        ..Default::default()
    };
    let got = backend.query(&q).await.unwrap();
    assert_eq!(got.len(), 2);
    let keys: Vec<&str> = got.iter().map(|it| it.key.as_str()).collect();
    assert!(keys.contains(&"a"));
    assert!(keys.contains(&"c"));
}

#[tokio::test]
async fn limit_truncates_desc() {
    let dir = tempdir().unwrap();
    let backend = SledBackend::from_path(dir.path(), None).unwrap();
    for i in 0..5 {
        backend
            .store(&make_item("trace", &format!("k{i}"), 1000 + i, &[]))
            .await
            .unwrap();
    }
    let q = MemoryQuery {
        limit: Some(2),
        ..Default::default()
    };
    let got = backend.query(&q).await.unwrap();
    assert_eq!(got.len(), 2);
    // DESC by ts → newest two are k4, k3.
    assert_eq!(got[0].key, "k4");
    assert_eq!(got[1].key, "k3");
}

#[tokio::test]
async fn compact_returns_count_of_older() {
    let dir = tempdir().unwrap();
    let backend = SledBackend::from_path(dir.path(), None).unwrap();
    backend.store(&make_item("trace", "old1", 100, &[])).await.unwrap();
    backend.store(&make_item("trace", "old2", 200, &[])).await.unwrap();
    backend.store(&make_item("trace", "new1", 1000, &[])).await.unwrap();

    let n = backend.compact(500).await.unwrap();
    assert_eq!(n, 2, "two items strictly older than 500 ms");

    // No-op delete: items are still present.
    let all = backend.query(&MemoryQuery::default()).await.unwrap();
    assert_eq!(all.len(), 3);
}

#[tokio::test]
async fn kind_filter_isolates_namespaces() {
    let dir = tempdir().unwrap();
    let backend = SledBackend::from_path(dir.path(), None).unwrap();
    backend.store(&make_item("trace", "x", 1, &[])).await.unwrap();
    backend.store(&make_item("concept", "x", 2, &[])).await.unwrap();
    backend.store(&make_item("report", "x", 3, &[])).await.unwrap();

    let q = MemoryQuery {
        kind: Some("concept".into()),
        ..Default::default()
    };
    let got = backend.query(&q).await.unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].kind, "concept");
}

#[tokio::test]
async fn mirror_to_remote_is_unsupported() {
    let dir = tempdir().unwrap();
    let backend = SledBackend::from_path(dir.path(), None).unwrap();
    let err = backend
        .mirror_to_remote("ssh://example/repo.git")
        .await
        .unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("mirror_to_remote") || msg.contains("RULE 0.15"));
}
