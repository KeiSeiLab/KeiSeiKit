//! Inline unit tests for the schema-migration runner.
//!
//! Constructor Pattern: kept in a sibling file via `#[path]` so
//! `schema.rs` itself stays a focused DDL list + apply loop.

use super::*;
use rusqlite::{params, Connection};
use tempfile::TempDir;

fn open_with_migrations() -> (TempDir, Connection) {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("ledger.sqlite");
    let conn = Connection::open(&path).unwrap();
    migrate(&conn).unwrap();
    (dir, conn)
}

#[test]
fn schema_version_constant_matches_migration_count() {
    // Sanity check: SCHEMA_VERSION constant must equal MIGRATIONS.len().
    // A future contributor adding a migration without bumping the constant
    // would silently break the user_version assertion in tests.
    assert_eq!(SCHEMA_VERSION as usize, MIGRATIONS.len());
}

#[test]
fn fresh_db_lands_at_latest_with_all_cost_columns() {
    let (_d, conn) = open_with_migrations();
    let v: i64 = conn
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap();
    assert_eq!(v, SCHEMA_VERSION as i64);
    // All v6 + v7 cost-tracking columns must be present.
    let mut stmt = conn.prepare("PRAGMA table_info(agents)").unwrap();
    let cols: Vec<String> = stmt
        .query_map([], |r| r.get::<_, String>(1))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    assert!(cols.contains(&"cost_cents".to_string()));
    assert!(cols.contains(&"cost_micro_cents".to_string()), "v7 column missing");
    assert!(cols.contains(&"provider".to_string()));
    assert!(cols.contains(&"model".to_string()));
}

#[test]
fn migration_is_idempotent_on_already_migrated_db() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("ledger.sqlite");
    // First open applies v1..vN where N = SCHEMA_VERSION.
    {
        let conn = Connection::open(&path).unwrap();
        migrate(&conn).unwrap();
    }
    // Second open re-runs migrate() — must NOT error with "duplicate column".
    let conn = Connection::open(&path).unwrap();
    migrate(&conn).unwrap();
    let v: i64 = conn
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap();
    assert_eq!(v, SCHEMA_VERSION as i64);
}

#[test]
fn cost_cents_defaults_to_zero_for_pre_v6_rows() {
    // Simulate a row that existed before v6 by INSERTing on a fresh DB then
    // checking the column default kicks in. The DEFAULT 0 clause means we
    // never see NULL even if a writer omits the column.
    let (_d, conn) = open_with_migrations();
    let now = chrono::Utc::now().timestamp();
    conn.execute(
        "INSERT INTO agents (id, branch, spec_sha, status, started_ts)
         VALUES ('preexisting', 'br-x', 'sha', 'done', ?1)",
        params![now],
    )
    .unwrap();
    let cost: i64 = conn
        .query_row(
            "SELECT cost_cents FROM agents WHERE id = 'preexisting'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(cost, 0);
}

#[test]
fn provider_and_model_default_to_empty_string() {
    let (_d, conn) = open_with_migrations();
    let now = chrono::Utc::now().timestamp();
    conn.execute(
        "INSERT INTO agents (id, branch, spec_sha, status, started_ts)
         VALUES ('blanks', 'br-y', 'sha', 'running', ?1)",
        params![now],
    )
    .unwrap();
    let (p, m): (String, String) = conn
        .query_row(
            "SELECT provider, model FROM agents WHERE id = 'blanks'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(p, "");
    assert_eq!(m, "");
}

// Cross-module test (`record_cost_lib_call_roundtrips`) lives in
// `tests/v6_cost.rs` so it isn't pulled into integration.rs's test binary
// (which doesn't carry a `mod cost;` declaration). Keeping it out here
// avoids forcing every consumer of `schema.rs` (binary, lib, integration
// test crate) to also load `cost.rs` just to satisfy a private-module ref.
