//! Integration tests covering schema migration, aggregation, and on-disk
//! persistence. The `store` and `sleep_report` modules carry per-module
//! unit tests; this file focuses on cross-module + filesystem behaviour.

use kei_token_tracker::aggregate::format_usd;
use kei_token_tracker::sleep_report;
use kei_token_tracker::{Store, TokenEvent};
use rusqlite::Connection;

fn ev(ts: i64, agent: &str, model: &str, in_tok: u32, out_tok: u32, micro: u64) -> TokenEvent {
    TokenEvent::chat_turn(ts, agent, model, "assistant", in_tok, out_tok, micro)
}

#[test]
fn schema_migration_creates_tables_and_indexes() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("t.sqlite");
    {
        let _store = Store::open(&path).unwrap();
    }
    let conn = Connection::open(&path).unwrap();
    let user_version: i64 = conn
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap();
    assert_eq!(user_version, 1);
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='token_events'")
        .unwrap();
    let row: Option<String> = stmt
        .query_row([], |r| r.get(0))
        .ok();
    assert_eq!(row.as_deref(), Some("token_events"));
    let idx_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master
             WHERE type='index' AND name LIKE 'idx_token_events_%'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(idx_count, 3);
}

#[test]
fn aggregate_by_model_sums_correctly() {
    let s = Store::open_in_memory().unwrap();
    s.record_event(&ev(100, "a", "claude-haiku-4-5", 10, 5, 1_000)).unwrap();
    s.record_event(&ev(200, "a", "claude-haiku-4-5", 30, 10, 4_000)).unwrap();
    s.record_event(&ev(300, "b", "gpt-4o", 50, 20, 9_000)).unwrap();
    let rows = s.aggregate_by_model(0).unwrap();
    assert_eq!(rows.len(), 2);
    let haiku = rows.iter().find(|r| r.model == "claude-haiku-4-5").unwrap();
    assert_eq!(haiku.events, 2);
    assert_eq!(haiku.input_tokens, 40);
    assert_eq!(haiku.output_tokens, 15);
    assert_eq!(haiku.micro_cents, 5_000);
    let gpt = rows.iter().find(|r| r.model == "gpt-4o").unwrap();
    assert_eq!(gpt.events, 1);
    assert_eq!(gpt.input_tokens, 50);
    assert_eq!(gpt.output_tokens, 20);
}

#[test]
fn aggregate_respects_since_lower_bound() {
    let s = Store::open_in_memory().unwrap();
    s.record_event(&ev(100, "a", "m1", 10, 5, 1_000)).unwrap();
    s.record_event(&ev(500, "a", "m1", 20, 10, 2_000)).unwrap();
    let rows = s.aggregate_by_model(300).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].events, 1);
    assert_eq!(rows[0].input_tokens, 20);
}

#[test]
fn list_recent_orders_newest_first() {
    let s = Store::open_in_memory().unwrap();
    s.record_event(&ev(100, "a", "m", 1, 1, 1)).unwrap();
    s.record_event(&ev(200, "a", "m", 1, 1, 1)).unwrap();
    s.record_event(&ev(150, "a", "m", 1, 1, 1)).unwrap();
    let rows = s.list_recent(10).unwrap();
    let timestamps: Vec<i64> = rows.iter().map(|r| r.ts).collect();
    assert_eq!(timestamps, vec![200, 150, 100]);
}

#[test]
fn sleep_report_renders_aggregated_store() {
    let s = Store::open_in_memory().unwrap();
    s.record_event(&ev(10, "a", "claude-haiku-4-5", 100, 50, 150_000_000))
        .unwrap();
    s.record_event(&ev(20, "a", "claude-haiku-4-5", 200, 100, 300_000_000))
        .unwrap();
    s.record_event(&ev(30, "a", "gpt-4o", 50, 25, 75_000_000))
        .unwrap();
    let rows = s.aggregate_by_model(0).unwrap();
    let md = sleep_report::render("2026-05-01", &rows);
    assert!(md.contains("# Token usage report — 2026-05-01"));
    assert!(md.contains("- Total events: 3"));
    assert!(md.contains("- Total tokens: 350 in / 175 out"));
    assert!(md.contains("- Total cost: $5.25"));
    assert!(md.contains("| claude-haiku-4-5 | 2 | 300 | 150 | $4.50 |"));
    assert!(md.contains("| gpt-4o | 1 | 50 | 25 | $0.75 |"));
}

#[test]
fn open_in_memory_persists_within_handle() {
    let s = Store::open_in_memory().unwrap();
    let id = s.record_event(&ev(1, "a", "m", 1, 1, 1)).unwrap();
    assert!(id >= 1);
    assert_eq!(s.count().unwrap(), 1);
}

#[test]
fn open_in_memory_isolated_per_handle() {
    let s1 = Store::open_in_memory().unwrap();
    let s2 = Store::open_in_memory().unwrap();
    s1.record_event(&ev(1, "a", "m", 1, 1, 1)).unwrap();
    assert_eq!(s1.count().unwrap(), 1);
    assert_eq!(s2.count().unwrap(), 0);
}

#[test]
fn format_usd_basic_cases() {
    assert_eq!(format_usd(0), "$0.00");
    assert_eq!(format_usd(1_000_000), "$0.01");
    assert_eq!(format_usd(123_456_789), "$1.23");
    assert_eq!(format_usd(1_000_000_000), "$10.00");
}
