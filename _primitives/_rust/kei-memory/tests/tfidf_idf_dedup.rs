//! Regression tests for Wave C TF-IDF dedup + single-JOIN top_similar.
//!
//! Constructor Pattern: each test = one scenario. Uses tempfile per test
//! for sqlite isolation. Imports library crate directly.
//!
//! Coverage:
//!   1. `recompute_idf_if_stale` returns true on first call after indexing,
//!      false on the second call without further indexing.
//!   2. `top_similar` returns the expected top-k by cosine, with synthetic
//!      hand-checked corpus.
//!   3. Indexing many docs (10) does NOT trigger a per-document IDF rebuild
//!      — IDF table stays empty until the first stale-flush.

use kei_memory::{schema, tfidf};
use rusqlite::Connection;
use tempfile::TempDir;

fn open_tmp() -> (TempDir, Connection) {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("kei-memory.sqlite");
    let conn = Connection::open(&db_path).unwrap();
    schema::migrate(&conn).unwrap();
    (dir, conn)
}

#[test]
fn recompute_idf_if_stale_dedups_back_to_back_calls() {
    let (_d, conn) = open_tmp();
    for i in 0..10 {
        tfidf::index_document(&conn, &format!("s{i}"), "rust cargo workspace conflict")
            .unwrap();
    }
    // First call after a batch of inserts: must run.
    let first = tfidf::recompute_idf_if_stale(&conn).unwrap();
    assert!(first, "first call after indexing must recompute");
    // Second call without further indexing: must skip.
    let second = tfidf::recompute_idf_if_stale(&conn).unwrap();
    assert!(!second, "second call without new indexing must skip");
    // Third call after indexing one more doc: must run again.
    tfidf::index_document(&conn, "s10", "swift xcode simulator").unwrap();
    let third = tfidf::recompute_idf_if_stale(&conn).unwrap();
    assert!(third, "indexing a new doc must re-stale the corpus");
}

#[test]
fn index_document_does_not_rebuild_idf_per_call() {
    let (_d, conn) = open_tmp();
    for i in 0..10 {
        tfidf::index_document(&conn, &format!("s{i}"), "alpha beta gamma").unwrap();
    }
    // IDF table must be EMPTY until something flushes the stale flag.
    let idf_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM idf", [], |r| r.get(0))
        .unwrap();
    assert_eq!(
        idf_count, 0,
        "index_document must NOT trigger per-call recompute_idf"
    );
    // Stale flag should be set on every token row.
    let dirty_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM tokens WHERE idf_dirty = 1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert!(dirty_count > 0, "tokens must be marked idf_dirty=1");
    // After a stale-flush, IDF populates and dirty flags clear.
    tfidf::recompute_idf_if_stale(&conn).unwrap();
    let idf_after: i64 = conn
        .query_row("SELECT COUNT(*) FROM idf", [], |r| r.get(0))
        .unwrap();
    assert_eq!(idf_after, 3, "alpha+beta+gamma => 3 IDF rows");
    let dirty_after: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM tokens WHERE idf_dirty = 1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(dirty_after, 0, "all dirty flags cleared after recompute");
}

#[test]
fn top_similar_single_join_returns_expected_topk() {
    let (_d, conn) = open_tmp();
    // Hand-crafted corpus where token overlap with the query is monotone:
    // sA shares 3 tokens, sB shares 2, sC shares 1, sD shares 0.
    tfidf::index_document(&conn, "sA", "rust cargo workspace conflict build").unwrap();
    tfidf::index_document(&conn, "sB", "rust cargo build pipeline").unwrap();
    tfidf::index_document(&conn, "sC", "rust async tokio").unwrap();
    tfidf::index_document(&conn, "sD", "swift xcode simulator audio").unwrap();
    // top_similar must internally flush the dirty flag and rank by cosine.
    let top = tfidf::top_similar(&conn, "rust cargo build", 3).unwrap();
    assert!(!top.is_empty(), "expected at least one match");
    let ids: Vec<&String> = top.iter().map(|(s, _)| s).collect();
    // sD shares zero query tokens — must NOT appear at all (single-JOIN
    // filters by `t.token IN (?)`, so zero-overlap sessions are pruned).
    assert!(!ids.iter().any(|s| s.as_str() == "sD"),
            "sD shares no query tokens, must be pruned, got {ids:?}");
    // sA or sB should rank top.
    let best = top[0].0.as_str();
    assert!(best == "sA" || best == "sB",
            "expected sA or sB first, got {best}");
    // Limit honoured.
    let top1 = tfidf::top_similar(&conn, "rust cargo build", 1).unwrap();
    assert_eq!(top1.len(), 1);
}

#[test]
fn top_similar_empty_query_returns_empty() {
    let (_d, conn) = open_tmp();
    tfidf::index_document(&conn, "s1", "alpha beta gamma").unwrap();
    // Query with no tokenisable content -> empty result, no SQL panic.
    let top = tfidf::top_similar(&conn, "!@#$ %^&*", 5).unwrap();
    assert!(top.is_empty());
}
