//! Integration smoke tests for the kei-discover public API.
//!
//! Covers the 6 behaviours enumerated in task.toml:
//!   1. register returns id + increments count
//!   2. list_available excludes installed
//!   3. mark_installed flips flag
//!   4. search matches slug and description via FTS
//!   5. register rejects duplicate slug
//!   6. stats counts
//!
//! All tests use `open_memory()` so they neither touch nor contend with
//! the on-disk default DB path.

use kei_discover::{
    list_available, mark_installed, open_memory, register, search, stats, DiscoverError,
};

fn fresh() -> kei_discover::Store {
    open_memory().expect("open_memory")
}

#[test]
fn register_returns_id_and_increments_count() {
    let s = fresh();
    let id = register(
        s.conn(),
        "kei-alpha",
        "alice",
        "https://example.com/alpha",
        "first alpha primitive",
    )
    .unwrap();
    assert!(id >= 1, "id must be positive rowid, got {id}");
    let count = stats(s.conn()).unwrap().total;
    assert_eq!(count, 1);

    let id2 = register(s.conn(), "kei-beta", "bob", "", "second").unwrap();
    assert!(id2 > id);
    assert_eq!(stats(s.conn()).unwrap().total, 2);
}

#[test]
fn list_available_excludes_installed() {
    let s = fresh();
    let a = register(s.conn(), "alpha", "alice", "", "a").unwrap();
    let b = register(s.conn(), "beta", "bob", "", "b").unwrap();
    let c = register(s.conn(), "gamma", "carol", "", "c").unwrap();

    // Install one.
    mark_installed(s.conn(), b).unwrap();

    let available = list_available(s.conn()).unwrap();
    assert_eq!(available.len(), 2);
    let ids: Vec<i64> = available.iter().map(|e| e.id).collect();
    assert!(ids.contains(&a));
    assert!(ids.contains(&c));
    assert!(!ids.contains(&b), "installed entry must not appear");
}

#[test]
fn mark_installed_flips_flag() {
    let s = fresh();
    let id = register(s.conn(), "alpha", "alice", "", "primitive alpha").unwrap();
    let before = &list_available(s.conn()).unwrap()[0];
    assert!(!before.installed);

    mark_installed(s.conn(), id).unwrap();

    // No longer appears in list_available.
    assert_eq!(list_available(s.conn()).unwrap().len(), 0);

    // Stats shows 1 installed.
    let st = stats(s.conn()).unwrap();
    assert_eq!(st.total, 1);
    assert_eq!(st.installed, 1);
    assert_eq!(st.available, 0);
}

#[test]
fn mark_installed_not_found_errors() {
    let s = fresh();
    let err = mark_installed(s.conn(), 9999).unwrap_err();
    matches!(err, DiscoverError::NotFound(9999));
    assert_eq!(err.exit_code(), 2);
}

#[test]
fn search_matches_slug_and_desc_via_fts() {
    let s = fresh();
    register(
        s.conn(),
        "refactor-router",
        "alice",
        "",
        "splits monolith into discrete atoms",
    )
    .unwrap();
    register(s.conn(), "kei-unrelated", "bob", "", "completely different").unwrap();

    // Match by slug token.
    let hits = search(s.conn(), "refactor").unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].slug, "refactor-router");

    // Match by description token.
    let hits = search(s.conn(), "monolith").unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].slug, "refactor-router");

    // Miss.
    let hits = search(s.conn(), "nonexistent").unwrap();
    assert_eq!(hits.len(), 0);
}

#[test]
fn register_rejects_duplicate_slug() {
    let s = fresh();
    register(s.conn(), "dup", "alice", "", "first").unwrap();
    let err = register(s.conn(), "dup", "bob", "", "second").unwrap_err();
    match &err {
        DiscoverError::DuplicateSlug(slug) => assert_eq!(slug, "dup"),
        other => panic!("expected DuplicateSlug, got {other:?}"),
    }
    assert_eq!(err.exit_code(), 2);
    // Count must remain 1.
    assert_eq!(stats(s.conn()).unwrap().total, 1);
}

#[test]
fn register_rejects_empty_slug() {
    let s = fresh();
    let err = register(s.conn(), "", "alice", "", "desc").unwrap_err();
    matches!(err, DiscoverError::InvalidInput(_));
    assert_eq!(err.exit_code(), 2);
}

#[test]
fn stats_counts() {
    let s = fresh();
    assert_eq!(
        stats(s.conn()).unwrap(),
        kei_discover::Stats { total: 0, installed: 0, available: 0 }
    );

    register(s.conn(), "a", "alice", "", "").unwrap();
    register(s.conn(), "b", "bob", "", "").unwrap();
    register(s.conn(), "c", "carol", "", "").unwrap();
    let id_b = 2; // second registered => rowid 2

    let st = stats(s.conn()).unwrap();
    assert_eq!(st.total, 3);
    assert_eq!(st.installed, 0);
    assert_eq!(st.available, 3);

    mark_installed(s.conn(), id_b).unwrap();
    let st = stats(s.conn()).unwrap();
    assert_eq!(st.total, 3);
    assert_eq!(st.installed, 1);
    assert_eq!(st.available, 2);
}
