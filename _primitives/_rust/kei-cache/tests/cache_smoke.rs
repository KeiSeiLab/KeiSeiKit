//! cache_smoke — end-to-end integration tests for `wrap_with`.
//!
//! Uses a `MockExecutor` that returns an incrementing counter so "was the
//! executor actually re-invoked?" is observable as a different return
//! value rather than inferred from a side-effect.

use anyhow::{anyhow, Result};
use kei_atom_discovery::AtomKind;
use kei_cache::exec::ensure_cacheable;
use kei_cache::{store, wrap_with, AtomExecutor, Outcome};
use std::cell::Cell;
use tempfile::tempdir;

/// Mock executor: each invocation returns `{"n": <call_count>}`.
/// Simulates a timestamp-like observable so a repeated call with the same
/// input must be a cache-hit to produce the same value.
struct MockExecutor {
    calls: Cell<u32>,
    kind: AtomKind,
}

impl MockExecutor {
    fn new() -> Self {
        Self { calls: Cell::new(0), kind: AtomKind::Query }
    }
    fn with_kind(kind: AtomKind) -> Self {
        Self { calls: Cell::new(0), kind }
    }
}

impl AtomExecutor for MockExecutor {
    fn execute(&self, atom_id: &str, _input_json: &str) -> Result<String> {
        ensure_cacheable(&self.kind, atom_id)?;
        let n = self.calls.get() + 1;
        self.calls.set(n);
        Ok(format!("{{\"n\":{n}}}"))
    }
}

fn open_fresh_cache() -> (tempfile::TempDir, rusqlite::Connection) {
    let d = tempdir().unwrap();
    let p = d.path().join("c.sqlite");
    let c = store::open(&p).unwrap();
    (d, c)
}

#[test]
fn first_call_misses_and_stores() {
    let (_d, conn) = open_fresh_cache();
    let ex = MockExecutor::new();
    let (payload, outcome) = wrap_with(&conn, &ex, "atom:mock", "{\"a\":1}", 60).unwrap();
    assert_eq!(outcome, Outcome::Miss);
    assert_eq!(payload, "{\"n\":1}");
    assert_eq!(ex.calls.get(), 1);
}

#[test]
fn second_call_same_input_is_hit() {
    let (_d, conn) = open_fresh_cache();
    let ex = MockExecutor::new();
    let (p1, o1) = wrap_with(&conn, &ex, "atom:mock", "{\"a\":1}", 60).unwrap();
    let (p2, o2) = wrap_with(&conn, &ex, "atom:mock", "{\"a\":1}", 60).unwrap();
    assert_eq!(o1, Outcome::Miss);
    assert_eq!(o2, Outcome::Hit);
    // Same value both times → executor was NOT re-invoked on the hit.
    assert_eq!(p1, p2);
    assert_eq!(ex.calls.get(), 1);
}

#[test]
fn equivalent_json_is_still_a_hit() {
    // Whitespace + key ordering differ; canonical JSON must hash the same.
    let (_d, conn) = open_fresh_cache();
    let ex = MockExecutor::new();
    let _ = wrap_with(&conn, &ex, "atom:mock", "{\"a\":1,\"b\":2}", 60).unwrap();
    let (_, o2) = wrap_with(&conn, &ex, "atom:mock", "  {\"b\":2,\"a\":1}  ", 60).unwrap();
    assert_eq!(o2, Outcome::Hit);
    assert_eq!(ex.calls.get(), 1);
}

#[test]
fn different_input_misses_with_different_key() {
    let (_d, conn) = open_fresh_cache();
    let ex = MockExecutor::new();
    let (p1, o1) = wrap_with(&conn, &ex, "atom:mock", "{\"a\":1}", 60).unwrap();
    let (p2, o2) = wrap_with(&conn, &ex, "atom:mock", "{\"a\":2}", 60).unwrap();
    assert_eq!(o1, Outcome::Miss);
    assert_eq!(o2, Outcome::Miss);
    // Counter advanced → executor really was re-invoked for the second input.
    assert_ne!(p1, p2);
    assert_eq!(ex.calls.get(), 2);
}

#[test]
fn expired_entry_misses_even_for_same_input() {
    // No sleep: put an entry, then force-expire via direct UPDATE.
    let (_d, conn) = open_fresh_cache();
    let ex = MockExecutor::new();
    let (_, o1) = wrap_with(&conn, &ex, "atom:mock", "{\"a\":1}", 60).unwrap();
    assert_eq!(o1, Outcome::Miss);
    conn.execute("UPDATE cache SET expires_ts = 1", []).unwrap();
    let (_, o2) = wrap_with(&conn, &ex, "atom:mock", "{\"a\":1}", 60).unwrap();
    assert_eq!(o2, Outcome::Miss);
    assert_eq!(ex.calls.get(), 2);
}

#[test]
fn non_cacheable_kind_is_refused() {
    let (_d, conn) = open_fresh_cache();
    let ex = MockExecutor::with_kind(AtomKind::Command);
    let res = wrap_with(&conn, &ex, "atom:danger", "{}", 60);
    assert!(res.is_err(), "command-kind atoms must not be cacheable");
    let msg = format!("{:#}", res.unwrap_err());
    assert!(msg.contains("unsafe to cache"), "unexpected error: {msg}");
    // Nothing stored on rejection.
    let s = store::stats(&conn).unwrap();
    assert_eq!(s.entries, 0);
}

#[test]
fn stream_kind_is_refused() {
    let (_d, conn) = open_fresh_cache();
    let ex = MockExecutor::with_kind(AtomKind::Stream);
    let err = wrap_with(&conn, &ex, "atom:s", "{}", 60).unwrap_err();
    assert!(format!("{err:#}").contains("unsafe to cache"));
}

#[test]
fn invalid_json_input_errors_before_keying() -> Result<()> {
    let (_d, conn) = open_fresh_cache();
    let ex = MockExecutor::new();
    let res = wrap_with(&conn, &ex, "atom:x", "not json", 60);
    if res.is_ok() {
        return Err(anyhow!("malformed JSON must not be accepted"));
    }
    assert_eq!(ex.calls.get(), 0);
    Ok(())
}
