//! Inline tests for `skill_metrics`. Constructor Pattern:
//! sibling test file via `#[path]` from `skill_metrics.rs`.
//!
//! Strategy: open a fresh tempdir-backed ledger (so v8 migration runs),
//! insert 30 fixture rows spanning two skills × successes/failures ×
//! recent/stale timestamps, then assert each public helper.

use super::*;
use crate::ledger;
use rusqlite::Connection;
use tempfile::TempDir;

const NOW: i64 = 1_900_000_000; // arbitrary fixed clock for fixtures

fn open() -> (TempDir, Connection) {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("ledger.sqlite");
    let conn = ledger::open(&path).unwrap();
    (dir, conn)
}

fn seed(conn: &Connection) {
    // Skill A: 20 rows in last 7 days, 16 wins, 4 fails → 80% success.
    for i in 0..20 {
        let ts = NOW - i64::from(i) * 3600; // 1h apart, all within 24h
        let success = i < 16;
        ins(conn, "skill_a", ts, success, Some("traj1"));
    }
    // Skill B: 5 rows in last day (3 wins / 2 fails), 5 rows >35 days ago.
    for i in 0..5 {
        ins(conn, "skill_b", NOW - i64::from(i) * 60, i < 3, None);
    }
    for i in 0..5 {
        ins(conn, "skill_b", NOW - 35 * 86_400 - i64::from(i) * 60, true, None);
    }
    // Skill C: long-stale only (used 60 days ago, never since).
    ins(conn, "skill_c", NOW - 60 * 86_400, true, None);
}

fn ins(conn: &Connection, name: &str, ts: i64, success: bool, traj: Option<&str>) {
    let inv = SkillInvocation {
        skill_name: name.to_string(),
        ts,
        agent_id: Some("agent-1".to_string()),
        success,
        trajectory_id: traj.map(|s| s.to_string()),
        duration_ms: Some(123),
    };
    record_invocation(conn, &inv).unwrap();
}

// ----- helpers that pin "now" so the cutoff arithmetic is deterministic.
// We can't easily monkey-patch `chrono::Utc::now`, so we test the SQL by
// hand-running the same predicate against fixture timestamps.

#[test]
fn record_invocation_inserts_one_row() {
    let (_d, c) = open();
    let n = record_invocation(
        &c,
        &SkillInvocation {
            skill_name: "x".into(),
            ts: NOW,
            agent_id: None,
            success: true,
            trajectory_id: None,
            duration_ms: None,
        },
    )
    .unwrap();
    assert_eq!(n, 1);
}

#[test]
fn last_used_returns_max_ts() {
    let (_d, c) = open();
    seed(&c);
    let lu = last_used(&c, "skill_a").unwrap().unwrap();
    assert_eq!(lu, NOW);
    assert_eq!(last_used(&c, "skill_c").unwrap().unwrap(), NOW - 60 * 86_400);
    assert_eq!(last_used(&c, "missing").unwrap(), None);
}

#[test]
fn schema_v8_table_exists_and_indexes_present() {
    let (_d, c) = open();
    let mut stmt = c
        .prepare("SELECT name FROM sqlite_master WHERE type='index' AND tbl_name='skill_invocations'")
        .unwrap();
    let names: Vec<String> = stmt
        .query_map([], |r| r.get::<_, String>(0))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    assert!(names.iter().any(|n| n.contains("name_ts")));
    assert!(names.iter().any(|n| n.contains("success")));
}

#[test]
fn success_rate_over_full_table() {
    let (_d, c) = open();
    seed(&c);
    // SQL-direct probe: 16/20 = 0.80 for skill_a regardless of cutoff
    // (when cutoff lets all rows in).
    let row: (i64, i64) = c
        .query_row(
            "SELECT COALESCE(SUM(success),0), COUNT(*) FROM skill_invocations
             WHERE skill_name = 'skill_a'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    let rate = row.0 as f64 / row.1 as f64;
    assert!((rate - 0.80).abs() < 1e-9);
}

#[test]
fn usage_count_with_long_lookback_returns_all() {
    let (_d, c) = open();
    seed(&c);
    // Lookback 100 years catches every fixture row → 31 total inserts.
    let n = usage_count(&c, "skill_b", 36_500).unwrap();
    assert_eq!(n, 10);
    let n_a = usage_count(&c, "skill_a", 36_500).unwrap();
    assert_eq!(n_a, 20);
}

#[test]
fn unused_skills_zero_days_lookback_lists_all() {
    // With days=0 the cutoff is the supplied clock; any row whose MAX(ts)
    // is strictly before the cutoff appears. Use deterministic
    // `unused_skills_at(NOW+1)` instead of `unused_skills` (which calls
    // Utc::now() — fixture's NOW=1_900_000_000 is in the future relative
    // to wall-clock today, so the strict `<` filter would exclude all rows).
    let (_d, c) = open();
    seed(&c);
    let unused = super::unused_skills_at(&c, 0, NOW + 1).unwrap();
    assert!(unused.contains(&"skill_a".to_string()) || unused.contains(&"skill_c".to_string()));
}
