//! Inline unit tests for `usage.rs`. Uses `tempfile::NamedTempFile` to
//! seed an extended-schema `agents` table (with provider/model/cost_cents)
//! and exercises the aggregation directly via `load_usage`.
//!
//! F-MED-3 note: today/week/month boundaries are CALENDAR ANCHORS in local
//! time (not sliding windows). Tests that need to drive specific window
//! membership therefore anchor relative to the boundaries themselves
//! (`bounds.today_start_ts + 1`, etc.) rather than `now - N hours`.

use super::calendar::CalendarBoundaries;
use super::load_usage;
use rusqlite::{params, Connection};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::NamedTempFile;

fn now_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

/// Create the v1 agents schema PLUS the cost-tracking columns the
/// usage handler reads. Mirrors a future v6 migration shape.
fn seed_with_cost(path: &Path, rows: &[(i64, i64, &str, &str)]) {
    let conn = Connection::open(path).unwrap();
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS agents (
            id TEXT PRIMARY KEY,
            branch TEXT NOT NULL,
            parent_branch TEXT,
            spec_sha TEXT NOT NULL,
            status TEXT NOT NULL,
            started_ts INTEGER NOT NULL,
            finished_ts INTEGER,
            summary TEXT,
            worktree_path TEXT,
            provider TEXT,
            model TEXT,
            cost_cents INTEGER
        )",
    )
    .unwrap();
    for (i, (started_ts, cost_cents, provider, model)) in rows.iter().enumerate() {
        conn.execute(
            "INSERT INTO agents (id, branch, spec_sha, status, started_ts, provider, model, cost_cents)
             VALUES (?1, 'feat/test', 'sha', 'done', ?2, ?3, ?4, ?5)",
            params![format!("a{i}"), started_ts, provider, model, cost_cents],
        )
        .unwrap();
    }
}

/// Seed the v1 schema WITHOUT cost_cents — the 404 path.
fn seed_without_cost(path: &Path) {
    let conn = Connection::open(path).unwrap();
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS agents (
            id TEXT PRIMARY KEY,
            branch TEXT NOT NULL,
            parent_branch TEXT,
            spec_sha TEXT NOT NULL,
            status TEXT NOT NULL,
            started_ts INTEGER NOT NULL,
            finished_ts INTEGER,
            summary TEXT,
            worktree_path TEXT
        )",
    )
    .unwrap();
}

#[test]
fn returns_none_when_db_missing() {
    let result = load_usage(Path::new("/tmp/does-not-exist-usage.sqlite")).unwrap();
    assert!(result.is_none(), "missing DB should yield 404 path");
}

#[test]
fn returns_none_when_cost_column_absent() {
    let f = NamedTempFile::new().unwrap();
    seed_without_cost(f.path());
    let result = load_usage(f.path()).unwrap();
    assert!(result.is_none(), "no cost_cents column → 404 path");
}

#[test]
fn returns_none_when_provider_or_model_column_missing() {
    let f = NamedTempFile::new().unwrap();
    let conn = Connection::open(f.path()).unwrap();
    // cost_cents present but provider/model NOT — partial migration path.
    conn.execute_batch(
        "CREATE TABLE agents (
            id TEXT PRIMARY KEY,
            branch TEXT NOT NULL,
            spec_sha TEXT NOT NULL,
            status TEXT NOT NULL,
            started_ts INTEGER NOT NULL,
            cost_cents INTEGER
        )",
    )
    .unwrap();
    drop(conn);
    let result = load_usage(f.path()).unwrap();
    assert!(result.is_none(), "partial migration → 404 path");
}

#[test]
fn aggregates_against_calendar_boundaries() {
    // Anchor rows to the actual calendar boundaries (not `now - N hours`)
    // so the test stays valid regardless of when it runs.
    //
    // Boundary edge case (1st-5th of month, when ISO-week-start Monday
    // falls in the PREVIOUS calendar month): `inside_week_only` lands
    // BEFORE `month_start_ts`, so it's in the week-window but NOT in
    // the month-window. Assertion below adapts to both cases.
    let f = NamedTempFile::new().unwrap();
    let bounds = CalendarBoundaries::for_now_local();
    let inside_today = bounds.today_start_ts + 1;
    let inside_week_only = bounds.week_start_ts + 1;
    let inside_month_only = bounds.month_start_ts + 1;
    let outside = bounds.month_start_ts - 24 * 3600;

    seed_with_cost(
        f.path(),
        &[
            (inside_today, 50, "anthropic", "claude-3-5-sonnet"),
            (inside_week_only, 200, "openai", "gpt-4o-mini"),
            (inside_month_only, 1000, "anthropic", "claude-3-5-sonnet"),
            (outside, 9999, "kimi", "moonshot-v1-8k"),
        ],
    );
    let r = load_usage(f.path()).unwrap().expect("rows present");

    // Expected month_cents minimum: today (50) + month_only (1000) +
    // optionally week_only (200) IF Monday-anchor falls in the same
    // calendar month. On the first few days of a month, week starts
    // in the previous month → week_only is excluded from month.
    let week_anchored_in_same_month = bounds.week_start_ts >= bounds.month_start_ts;
    let expected_month_min: i64 =
        50 + 1000 + if week_anchored_in_same_month { 200 } else { 0 };

    assert!(r.today_cents >= 50, "today must include post-midnight row");
    assert!(r.week_cents >= 250, "week must include today + Monday-anchor");
    assert!(
        r.month_cents >= expected_month_min,
        "month must include in-month rows (expected_min={expected_month_min}, got={got}, week_in_month={w})",
        got = r.month_cents,
        w = week_anchored_in_same_month
    );
    assert!(r.month_cents < 9999, "out-of-month row leaked into month");
}

#[test]
fn empty_table_returns_zeroes_and_blank_top() {
    let f = NamedTempFile::new().unwrap();
    seed_with_cost(f.path(), &[]);
    let r = load_usage(f.path()).unwrap().expect("schema present");
    assert_eq!(r.today_cents, 0);
    assert_eq!(r.week_cents, 0);
    assert_eq!(r.month_cents, 0);
    assert_eq!(r.top_provider, "");
    assert_eq!(r.top_model, "");
}

#[test]
fn top_provider_breaks_ties_by_summed_cost() {
    let f = NamedTempFile::new().unwrap();
    let now = now_secs();
    seed_with_cost(
        f.path(),
        &[
            (now - 100, 100, "openai", "gpt-4o"),
            (now - 200, 300, "kimi", "moonshot-v1-32k"),
            (now - 300, 250, "kimi", "moonshot-v1-32k"),
        ],
    );
    let r = load_usage(f.path()).unwrap().expect("rows present");
    assert_eq!(r.top_provider, "kimi");
    assert_eq!(r.top_model, "moonshot-v1-32k");
}

/// F-MED-3 verify-criterion: a row stamped 1s BEFORE today's midnight does
/// NOT count toward today_cents, even though it's well within a sliding-24h
/// window. This is what the bug looked like at 00:01 the next morning.
#[test]
fn row_just_before_midnight_does_not_count_today() {
    let f = NamedTempFile::new().unwrap();
    let bounds = CalendarBoundaries::for_now_local();
    let yesterday_late = bounds.today_start_ts - 1;
    seed_with_cost(
        f.path(),
        &[(yesterday_late, 777, "anthropic", "claude-3-5-sonnet")],
    );
    let r = load_usage(f.path()).unwrap().expect("rows present");
    assert_eq!(
        r.today_cents, 0,
        "yesterday's row leaked into today (sliding-window bug): got {}",
        r.today_cents
    );
    // But it must still be inside the month (assuming today is not the 1st);
    // skip month-side assertion to avoid an end-of-month edge case here.
}
