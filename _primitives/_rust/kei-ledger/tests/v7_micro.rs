//! v7 micro-cents column tests (Wave 44c, 2026-04-24).
//!
//! Constructor Pattern: extracted from `v6_cost.rs` so each test file
//! stays under the 200-LOC ceiling. Loads source modules via `#[path]`
//! to avoid forcing all callers through the public lib API.

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

/// v7-T8: schema reaches at least v7 from a fresh DB; cost_micro_cents
/// column exists with DEFAULT 0 for new rows and backfill SQL is harmless.
/// Uses `>= 7` rather than `== 7` so future migrations (currently at v8)
/// don't break this v7-specific assertion.
#[test]
fn migration_v7_adds_micro_cents_column() {
    let (_d, conn) = open_tmp();
    let v: i64 = conn
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap();
    assert!(v >= 7, "schema must be at least v7, got v{v}");
    ledger::fork(&conn, "v7", "br-v7", None, "sha", None, None, None, None).unwrap();
    let micro: i64 = conn
        .query_row(
            "SELECT cost_micro_cents FROM agents WHERE id = 'v7'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(micro, 0, "fresh row defaults to 0 micro-cents");
}

/// v7-T9: pre-v7 row carrying a non-zero cost_cents value gets backfilled
/// to (cost_cents × 1_000_000) micro-cents on migration.
#[test]
fn migration_v7_backfills_pre_existing_cost_cents() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("ledger.sqlite");
    // Create at v6 manually by invoking only the first 6 migrations,
    // then mass-insert a row with cost_cents set, THEN reopen so v7
    // backfill runs.
    {
        let conn = rusqlite::Connection::open(&db).unwrap();
        for sql in &schema::MIGRATIONS[..6] {
            conn.execute_batch(sql).unwrap();
        }
        conn.pragma_update(None, "user_version", 6_i64).unwrap();
        conn.execute(
            "INSERT INTO agents (id, branch, spec_sha, status, started_ts, cost_cents)
             VALUES ('legacy', 'br-legacy', 'sha', 'done', 1, 250)",
            [],
        )
        .unwrap();
    }
    let conn = ledger::open(&db).unwrap();
    let (cents, micro, _, _) = cost::read_cost_micro(&conn, "legacy").unwrap().unwrap();
    assert_eq!(cents, 250);
    assert_eq!(micro, 250 * 1_000_000, "backfilled to 250M micro-cents");
}

/// v7-T10: compose_micro_cents is exact under integer overflow guards.
/// 1.5M input + 0.5M output @ 100c/MTok / 500c/MTok = 400M micro-cents.
#[test]
fn compose_micro_cents_exact_arithmetic() {
    let m = cost::compose_micro_cents(1_500_000, 500_000, 100, 500);
    // 1.5M × 100 = 150M, 0.5M × 500 = 250M, total = 400M micro-cents.
    assert_eq!(m, 400_000_000);
}

/// v7-T11: 100 micro-turns of 5 tokens each input @ 1c/MTok do NOT
/// round to 1 cent each; the cents accumulator stays 0 while micro
/// accumulates 500.
#[test]
fn compose_micro_cents_micro_turns_no_rounding_loss() {
    let mut total_micro: u64 = 0;
    for _ in 0..100 {
        let m = cost::compose_micro_cents(5, 0, 1, 0);
        total_micro = total_micro.saturating_add(m);
    }
    assert_eq!(total_micro, 500, "100 × 5 tokens × 1 cent/MTok = 500 micro-cents");
    assert_eq!(
        cost::display_cents_from_micro(total_micro),
        0,
        "rounds DOWN to 0 cents — under threshold"
    );
}

/// v7-T12: display_cents_from_micro uses half-up rounding at boundaries.
#[test]
fn display_cents_from_micro_half_up_at_boundary() {
    assert_eq!(cost::display_cents_from_micro(0), 0);
    assert_eq!(cost::display_cents_from_micro(499_999), 0, "below half rounds down");
    assert_eq!(cost::display_cents_from_micro(500_000), 1, "exactly half rounds up");
    assert_eq!(cost::display_cents_from_micro(999_999), 1);
    assert_eq!(cost::display_cents_from_micro(1_000_000), 1);
    assert_eq!(cost::display_cents_from_micro(1_500_000), 2);
}

/// v7-T3c: micro-cents accumulator persists alongside cents. 100 calls
/// of 5 micro-cents each (= 500 micro-cents = 0.0005 cents) round-trip
/// without rounding loss in the micro column.
#[test]
fn record_cost_micro_accumulates_without_rounding_loss() {
    let (_d, conn) = open_tmp();
    ledger::fork(&conn, "mic", "br-mic", None, "sha", None, None, None, None).unwrap();
    for _ in 0..100 {
        cost::record_cost_micro(&conn, "mic", 0, 5, "anthropic", "claude").unwrap();
    }
    let (cents, micro, _, _) = cost::read_cost_micro(&conn, "mic").unwrap().unwrap();
    assert_eq!(cents, 0, "100 × 0 cents truncates to 0");
    assert_eq!(micro, 500, "but micro-cents accumulator is exact");
}
