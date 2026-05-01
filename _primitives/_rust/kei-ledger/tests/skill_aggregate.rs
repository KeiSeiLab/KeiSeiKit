//! Integration tests for Phase D skill aggregation (P3.4.b).
//!
//! 4 skills exercise all 4 recommendation tiers.
//! p50 / p95 are hand-computed and verified.

use kei_ledger::{
    aggregate_skills, ledger, record_invocation, SkillInvocation, SkillRecommendation,
};
use tempfile::TempDir;

const NOW: i64 = 1_900_000_000; // fixed clock matching skill_metrics_test

fn open_db() -> (TempDir, rusqlite::Connection) {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("ledger.sqlite");
    let conn = ledger::open(&path).unwrap();
    (dir, conn)
}

fn ins(conn: &rusqlite::Connection, name: &str, success: bool, duration_ms: Option<i64>) {
    record_invocation(
        conn,
        &SkillInvocation {
            skill_name: name.to_string(),
            ts: NOW,
            agent_id: None,
            success,
            trajectory_id: None,
            duration_ms,
        },
    )
    .unwrap();
}

/// Insert `n` rows for `name` with `wins` successes and known durations.
fn ins_batch(conn: &rusqlite::Connection, name: &str, n: u32, wins: u32, base_dur: i64) {
    for i in 0..n {
        let success = i < wins;
        let dur = base_dur + i64::from(i) * 10;
        ins(conn, name, success, Some(dur));
    }
}

/// Seed 4 skills:
///  - "validated"  : 10 rows, 10 wins  → 100% → Validated
///  - "archive"    : 10 rows, 2 wins   → 20%  → Archive
///  - "reextract"  : 10 rows, 5 wins   → 50%  → Reextract
///  - "few"        :  5 rows, 5 wins   → 100% → Insufficient (< 10 invocations)
fn seed(conn: &rusqlite::Connection) {
    ins_batch(conn, "validated", 10, 10, 100);
    ins_batch(conn, "archive", 10, 2, 200);
    ins_batch(conn, "reextract", 10, 5, 300);
    ins_batch(conn, "few", 5, 5, 50);
}

// ---- test 1: all four tiers are produced ----

#[test]
fn four_tiers_all_present() {
    let (_d, conn) = open_db();
    seed(&conn);
    let aggs = aggregate_skills(&conn, Some(0)).unwrap();
    assert_eq!(aggs.len(), 4);

    let tier = |name: &str| {
        aggs.iter()
            .find(|a| a.skill_name == name)
            .map(|a| a.recommendation.clone())
            .unwrap()
    };
    assert_eq!(tier("validated"), SkillRecommendation::Validated);
    assert_eq!(tier("archive"), SkillRecommendation::Archive);
    assert_eq!(tier("reextract"), SkillRecommendation::Reextract);
    assert_eq!(tier("few"), SkillRecommendation::Insufficient);
}

// ---- test 2: success rates are computed correctly ----

#[test]
fn success_rates_correct() {
    let (_d, conn) = open_db();
    seed(&conn);
    let aggs = aggregate_skills(&conn, Some(0)).unwrap();

    let rate = |name: &str| {
        aggs.iter()
            .find(|a| a.skill_name == name)
            .map(|a| a.success_rate)
            .unwrap()
    };
    assert!((rate("validated") - 1.0).abs() < 1e-9);
    assert!((rate("archive") - 0.20).abs() < 1e-9);
    assert!((rate("reextract") - 0.50).abs() < 1e-9);
    assert!((rate("few") - 1.0).abs() < 1e-9);
}

// ---- test 3: total_invocations counts are correct ----

#[test]
fn total_invocations_correct() {
    let (_d, conn) = open_db();
    seed(&conn);
    let aggs = aggregate_skills(&conn, Some(0)).unwrap();

    let total = |name: &str| {
        aggs.iter()
            .find(|a| a.skill_name == name)
            .map(|a| a.total_invocations)
            .unwrap()
    };
    assert_eq!(total("validated"), 10);
    assert_eq!(total("archive"), 10);
    assert_eq!(total("reextract"), 10);
    assert_eq!(total("few"), 5);
}

// ---- test 4: p50 and p95 for "validated" hand-computed ----
// durations for "validated": [100, 110, 120, 130, 140, 150, 160, 170, 180, 190]
// sorted, n=10: p50 index = (10-1)/2 = 4 → 140
// p95 index = ceil(10*0.95)-1 = 10-1 = 9 → 190

#[test]
fn percentiles_validated_hand_computed() {
    let (_d, conn) = open_db();
    seed(&conn);
    let aggs = aggregate_skills(&conn, Some(0)).unwrap();
    let v = aggs.iter().find(|a| a.skill_name == "validated").unwrap();
    assert_eq!(v.p50_duration_ms, 140);
    assert_eq!(v.p95_duration_ms, 190);
}

// ---- test 5: since_ts filter excludes rows before cutoff ----

#[test]
fn since_ts_filter_excludes_old_rows() {
    let (_d, conn) = open_db();
    // Insert old rows then new rows for the same skill.
    for _ in 0..10 {
        record_invocation(
            &conn,
            &SkillInvocation {
                skill_name: "filtered".to_string(),
                ts: NOW - 100_000,
                agent_id: None,
                success: false,
                trajectory_id: None,
                duration_ms: None,
            },
        )
        .unwrap();
    }
    // 5 recent successes only.
    for _ in 0..5 {
        record_invocation(
            &conn,
            &SkillInvocation {
                skill_name: "filtered".to_string(),
                ts: NOW,
                agent_id: None,
                success: true,
                trajectory_id: None,
                duration_ms: None,
            },
        )
        .unwrap();
    }
    // With cutoff=NOW-1, only the 5 recent rows are visible.
    let aggs = aggregate_skills(&conn, Some(NOW - 1)).unwrap();
    let f = aggs.iter().find(|a| a.skill_name == "filtered").unwrap();
    assert_eq!(f.total_invocations, 5);
    assert_eq!(f.recommendation, SkillRecommendation::Insufficient);
}

// ---- test 6: empty DB returns empty vec (no panic) ----

#[test]
fn empty_db_returns_empty_vec() {
    let (_d, conn) = open_db();
    let aggs = aggregate_skills(&conn, None).unwrap();
    assert!(aggs.is_empty());
}

// ---- test 7: last_invoked_ts reflects max ts in window ----

#[test]
fn last_invoked_ts_is_max_ts_in_window() {
    let (_d, conn) = open_db();
    let older = NOW - 5_000;
    for &ts in &[older, NOW] {
        record_invocation(
            &conn,
            &SkillInvocation {
                skill_name: "ts_check".to_string(),
                ts,
                agent_id: None,
                success: true,
                trajectory_id: None,
                duration_ms: Some(1),
            },
        )
        .unwrap();
    }
    let aggs = aggregate_skills(&conn, Some(0)).unwrap();
    let a = aggs.iter().find(|a| a.skill_name == "ts_check").unwrap();
    assert_eq!(a.last_invoked_ts, NOW);
}
