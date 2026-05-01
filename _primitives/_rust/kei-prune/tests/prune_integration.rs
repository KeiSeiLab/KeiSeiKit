//! Integration tests for kei-prune. In-memory SQLite with a minimal
//! `agents` table mirroring kei-ledger's shape; sidecar installed via
//! `ensure_schema`. No kei-ledger dep — synthetic rows inserted directly.

use kei_prune::{candidates, ensure_schema, mark_retired, stats, PruneError};
use rusqlite::{params, Connection};

/// Seconds per day — must match `prune.rs`.
const SECONDS_PER_DAY: i64 = 86_400;

/// Fixed "now" used by every test so age arithmetic is deterministic.
/// 2026-04-23 00:00 UTC-ish — any constant works; only deltas matter.
const FIXED_NOW: i64 = 1_745_366_400;

/// Minimal `agents` DDL — the column set kei-prune actually reads.
/// No kei-ledger CHECK on `status` — we test the library filter.
const AGENTS_DDL: &str = "\
CREATE TABLE agents (
    id TEXT PRIMARY KEY,
    branch TEXT NOT NULL,
    parent_branch TEXT,
    spec_sha TEXT NOT NULL,
    status TEXT NOT NULL,
    started_ts INTEGER NOT NULL,
    finished_ts INTEGER,
    summary TEXT,
    worktree_path TEXT,
    dna TEXT
);
";

/// Build an in-memory DB with the ledger shape + sidecar installed.
fn setup() -> Connection {
    let conn = Connection::open_in_memory().expect("open in-memory");
    conn.execute_batch(AGENTS_DDL).expect("agents DDL");
    ensure_schema(&conn).expect("ensure_schema");
    conn
}

/// Helper: insert a synthetic agent row.
fn insert_agent(
    conn: &Connection,
    id: &str,
    status: &str,
    started_ts: i64,
    finished_ts: Option<i64>,
    dna: Option<&str>,
) {
    conn.execute(
        "INSERT INTO agents (id, branch, spec_sha, status, started_ts, finished_ts, dna)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        params![
            id,
            format!("agent/{id}"),
            "deadbeef",
            status,
            started_ts,
            finished_ts,
            dna,
        ],
    )
    .expect("insert agent");
}

/// Helper: timestamp `days` days before FIXED_NOW.
fn days_ago(days: i64) -> i64 {
    FIXED_NOW - days * SECONDS_PER_DAY
}

// --- tests ------------------------------------------------------------

#[test]
fn candidates_returns_empty_on_fresh_db() {
    let conn = setup();
    let out = candidates(&conn, FIXED_NOW, 90).expect("candidates");
    assert!(out.is_empty(), "fresh DB must yield zero candidates");
}

#[test]
fn candidates_excludes_active_rows() {
    let conn = setup();
    // Row started 3 days ago — far below 90-day threshold.
    insert_agent(&conn, "a1", "running", days_ago(3), None, Some("dna-a1"));
    let out = candidates(&conn, FIXED_NOW, 90).expect("candidates");
    assert!(out.is_empty(), "recent row must not be a candidate");
}

#[test]
fn candidates_returns_idle_over_threshold() {
    let conn = setup();
    insert_agent(&conn, "old1", "done", days_ago(120), Some(days_ago(119)), Some("dna-old1"));
    insert_agent(&conn, "young", "running", days_ago(5), None, Some("dna-young"));
    let out = candidates(&conn, FIXED_NOW, 90).expect("candidates");
    assert_eq!(out.len(), 1, "only the idle row should surface");
    assert_eq!(out[0].id, "old1");
    assert_eq!(out[0].dna, "dna-old1");
    assert_eq!(out[0].age_days, 120);
    assert_eq!(out[0].last_used_ts, days_ago(119));
}

#[test]
fn candidates_respects_min_idle_days() {
    let conn = setup();
    // Row age exactly 90 days — boundary inclusive per spec (>=).
    insert_agent(&conn, "edge", "merged", days_ago(90), Some(days_ago(90)), None);
    // Row age 89 days — below threshold.
    insert_agent(&conn, "below", "done", days_ago(89), Some(days_ago(89)), None);

    let out_90 = candidates(&conn, FIXED_NOW, 90).expect("at 90");
    assert_eq!(out_90.len(), 1, "90d-old row must match at threshold=90");
    assert_eq!(out_90[0].id, "edge");

    let out_91 = candidates(&conn, FIXED_NOW, 91).expect("at 91");
    assert!(out_91.is_empty(), "90d-old row must NOT match at threshold=91");
}

#[test]
fn mark_retired_inserts_sidecar_row() {
    let conn = setup();
    insert_agent(&conn, "victim", "done", days_ago(200), Some(days_ago(199)), None);
    mark_retired(&conn, "victim", FIXED_NOW).expect("mark_retired");
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM prune_retirements WHERE agent_id = ?",
            params!["victim"],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 1, "retirement row must be present exactly once");
}

#[test]
fn mark_retired_idempotent() {
    let conn = setup();
    insert_agent(&conn, "idempot", "done", days_ago(200), None, None);
    mark_retired(&conn, "idempot", FIXED_NOW).expect("first mark");
    let first_ts: i64 = conn
        .query_row(
            "SELECT retired_ts FROM prune_retirements WHERE agent_id = ?",
            params!["idempot"],
            |r| r.get(0),
        )
        .unwrap();
    // Second call at a later "now" — must NOT overwrite.
    mark_retired(&conn, "idempot", FIXED_NOW + 10_000).expect("second mark");
    let second_ts: i64 = conn
        .query_row(
            "SELECT retired_ts FROM prune_retirements WHERE agent_id = ?",
            params!["idempot"],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(first_ts, second_ts, "repeat mark must preserve original ts");
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM prune_retirements", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 1, "no duplicate rows");
}

#[test]
fn mark_retired_rejects_unknown_agent() {
    let conn = setup();
    let err = mark_retired(&conn, "ghost", FIXED_NOW).expect_err("unknown id must error");
    match err {
        PruneError::UnknownAgent(id) => assert_eq!(id, "ghost"),
        other => panic!("expected UnknownAgent, got {other:?}"),
    }
}

#[test]
fn stats_counts_buckets() {
    let conn = setup();
    // 2 active (running/done not yet retired) + 1 to-be-retired + 1 failed.
    insert_agent(&conn, "act1", "running", days_ago(1), None, None);
    insert_agent(&conn, "act2", "done", days_ago(10), Some(days_ago(9)), None);
    insert_agent(&conn, "tired", "merged", days_ago(300), Some(days_ago(299)), None);
    insert_agent(&conn, "fail1", "failed", days_ago(5), Some(days_ago(4)), None);
    mark_retired(&conn, "tired", FIXED_NOW).expect("mark tired");

    let s = stats(&conn).expect("stats");
    assert_eq!(s.total, 4, "total = all rows");
    assert_eq!(s.active, 2, "active = running/done/merged not retired");
    assert_eq!(s.retired, 1, "retired = sidecar row count");
    assert_eq!(s.idle, 0, "idle bucket is placeholder (candidates() is authoritative)");
}

#[test]
fn retired_rows_excluded_from_candidates() {
    let conn = setup();
    insert_agent(&conn, "stillhere", "done", days_ago(200), Some(days_ago(199)), None);
    insert_agent(&conn, "gone", "done", days_ago(300), Some(days_ago(299)), None);
    mark_retired(&conn, "gone", FIXED_NOW).expect("mark gone");
    let out = candidates(&conn, FIXED_NOW, 90).expect("candidates");
    assert_eq!(out.len(), 1, "only the non-retired row surfaces");
    assert_eq!(out[0].id, "stillhere");
}
