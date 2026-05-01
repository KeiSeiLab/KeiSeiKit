//! v6 cost-tracking column tests (Wave 40, 2026-04-24).
//!
//! Constructor Pattern: extracted from `integration.rs` so each test
//! file stays focused. Like `integration.rs`, loads source modules via
//! `#[path]` to avoid forcing all callers through the public lib API.

#[path = "../src/migrations_list.rs"]
mod migrations_list;
#[path = "../src/schema.rs"]
mod schema;
#[path = "../src/error.rs"]
mod error;
#[path = "../src/row.rs"]
mod row;
#[path = "../src/ledger.rs"]
mod ledger;
#[path = "../src/descendants.rs"]
mod descendants;
#[path = "../src/cost.rs"]
mod cost;

use rusqlite::Connection;
use tempfile::TempDir;

fn open_tmp() -> (TempDir, Connection) {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("ledger.sqlite");
    let conn = ledger::open(&db).unwrap();
    (dir, conn)
}

/// v6-T0: schema migrations bring the ledger to v6 from a fresh DB and
/// `cost::record_cost` round-trips a full agent row. Cross-module test
/// originally drafted in `src/schema_test.rs` — moved here so the inline
/// schema tests don't force every test binary to load `mod cost;`.
#[test]
fn schema_v6_cost_record_lib_call_roundtrips() {
    let (_d, conn) = open_tmp();
    let now = chrono::Utc::now().timestamp();
    conn.execute(
        "INSERT INTO agents (id, branch, spec_sha, status, started_ts)
         VALUES ('a-rc', 'br-rc', 'sha', 'done', ?1)",
        rusqlite::params![now],
    )
    .unwrap();
    let updated =
        cost::record_cost(&conn, "a-rc", 250, "anthropic", "claude-haiku").unwrap();
    assert_eq!(updated, 1);
    let (c, p, m) = cost::read_cost(&conn, "a-rc").unwrap().expect("row present");
    assert_eq!(c, 250);
    assert_eq!(p, "anthropic");
    assert_eq!(m, "claude-haiku");
}

/// v6-T1: a fresh ledger has all three cost columns reachable via record_cost.
#[test]
fn record_cost_writes_all_three_columns() {
    let (_d, conn) = open_tmp();
    ledger::fork(&conn, "vc1", "br-vc1", None, "sha", None, None, None, None).unwrap();
    let updated =
        cost::record_cost(&conn, "vc1", 1234, "anthropic", "claude-haiku-4-5-20251001").unwrap();
    assert_eq!(updated, 1, "exactly one row should match");
    let (c, p, m) = cost::read_cost(&conn, "vc1").unwrap().expect("row present");
    assert_eq!(c, 1234);
    assert_eq!(p, "anthropic");
    assert_eq!(m, "claude-haiku-4-5-20251001");
}

/// v6-T2: record_cost on a missing agent_id yields zero rows updated.
#[test]
fn record_cost_on_missing_agent_returns_zero() {
    let (_d, conn) = open_tmp();
    let updated = cost::record_cost(&conn, "ghost", 50, "anthropic", "claude").unwrap();
    assert_eq!(updated, 0);
    assert!(cost::read_cost(&conn, "ghost").unwrap().is_none());
}

/// v7-T3 (Wave 44c, replaces v6-T3): record_cost is ADDITIVE; provider
/// and model land last-write-wins. Three turns under the same agent_id
/// previously billed only the third turn — silent under-charge.
#[test]
fn record_cost_accumulates_across_calls() {
    let (_d, conn) = open_tmp();
    ledger::fork(&conn, "acc", "br-acc", None, "sha", None, None, None, None).unwrap();
    cost::record_cost(&conn, "acc", 10, "anthropic", "claude-haiku").unwrap();
    cost::record_cost(&conn, "acc", 999, "openai", "gpt-4o").unwrap();
    cost::record_cost(&conn, "acc", 42, "kimi", "moonshot").unwrap();
    let (c, p, m) = cost::read_cost(&conn, "acc").unwrap().unwrap();
    assert_eq!(c, 10 + 999 + 42, "cents accumulate across calls");
    assert_eq!(p, "kimi", "provider is last-write");
    assert_eq!(m, "moonshot", "model is last-write");
}

/// v7-T3b: explicit `replace_cost` overrides the running total. Used
/// by retry / amend flows that must NOT add the prior partial estimate.
#[test]
fn replace_cost_overrides_running_total() {
    let (_d, conn) = open_tmp();
    ledger::fork(&conn, "rep", "br-rep", None, "sha", None, None, None, None).unwrap();
    cost::record_cost(&conn, "rep", 100, "anthropic", "claude-haiku").unwrap();
    cost::record_cost(&conn, "rep", 200, "anthropic", "claude-haiku").unwrap();
    cost::replace_cost(&conn, "rep", 50, "openai", "gpt-4o").unwrap();
    let (c, p, _) = cost::read_cost(&conn, "rep").unwrap().unwrap();
    assert_eq!(c, 50, "replace overrides accumulated 300, not adds to it");
    assert_eq!(p, "openai");
}

// v7-T3c micro-cents accumulator test moved to `tests/v7_micro.rs`.

/// v6-T4: legacy pre-v6 row gets cost_cents = 0 default (DEFAULT clause).
#[test]
fn pre_existing_row_defaults_cost_to_zero() {
    let (_d, conn) = open_tmp();
    ledger::fork(&conn, "old", "br-old", None, "sha", None, None, None, None).unwrap();
    let cost: i64 = conn
        .query_row(
            "SELECT cost_cents FROM agents WHERE id = 'old'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(cost, 0);
}

/// v6-T5: migration is idempotent across reopens (no "duplicate column").
#[test]
fn migration_idempotent_across_reopens() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("ledger.sqlite");
    for _ in 0..3 {
        let conn = ledger::open(&db).unwrap();
        let v: i64 = conn
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .unwrap();
        assert_eq!(v, schema::MIGRATIONS.len() as i64);
    }
}

// v7 micro-cents tests live in `tests/v7_micro.rs`.
// CLI binary tests live in `tests/v7_cost_cli.rs`.
