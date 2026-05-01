//! 30-input corpus exercising every parser branch (port of Hermes
//! `cron/jobs.py:parse_schedule` 102-209 + edge cases).

use std::time::Duration;

use kei_cron_scheduler::job::Schedule;
use kei_cron_scheduler::parser::{parse_duration, parse_schedule, ParseError};

#[test]
fn duration_30m() {
    matches_after_duration(parse_schedule("30m").unwrap(), 30 * 60);
}

#[test]
fn duration_2h() {
    matches_after_duration(parse_schedule("2h").unwrap(), 2 * 3600);
}

#[test]
fn duration_1d() {
    matches_after_duration(parse_schedule("1d").unwrap(), 86_400);
}

#[test]
fn duration_with_whitespace() {
    matches_after_duration(parse_schedule("  45m  ").unwrap(), 45 * 60);
}

#[test]
fn interval_every_30m() {
    matches_interval(parse_schedule("every 30m").unwrap(), 30 * 60);
}

#[test]
fn interval_every_2h() {
    matches_interval(parse_schedule("every 2h").unwrap(), 2 * 3600);
}

#[test]
fn interval_every_1d() {
    matches_interval(parse_schedule("every 1d").unwrap(), 86_400);
}

#[test]
fn interval_case_insensitive() {
    matches_interval(parse_schedule("Every 5m").unwrap(), 5 * 60);
}

#[test]
fn cron_daily_9am() {
    matches_cron(parse_schedule("0 9 * * *").unwrap(), "0 9 * * *");
}

#[test]
fn cron_weekday_business_hours() {
    matches_cron(parse_schedule("0 9 * * 1-5").unwrap(), "0 9 * * 1-5");
}

#[test]
fn cron_every_15_minutes() {
    matches_cron(parse_schedule("*/15 * * * *").unwrap(), "*/15 * * * *");
}

#[test]
fn cron_top_of_every_hour() {
    matches_cron(parse_schedule("0 * * * *").unwrap(), "0 * * * *");
}

#[test]
fn iso_with_z_suffix() {
    let s = parse_schedule("2026-05-01T14:00:00Z").unwrap();
    assert!(matches!(s, Schedule::Once { .. }), "expected Once, got {s:?}");
}

#[test]
fn iso_with_offset() {
    let s = parse_schedule("2026-05-01T14:00:00+02:00").unwrap();
    assert!(matches!(s, Schedule::Once { .. }), "expected Once, got {s:?}");
}

#[test]
fn iso_naive_full() {
    let s = parse_schedule("2026-05-01T14:00:00").unwrap();
    assert!(matches!(s, Schedule::Once { .. }), "expected Once, got {s:?}");
}

#[test]
fn iso_naive_no_seconds() {
    let s = parse_schedule("2026-05-01T14:00").unwrap();
    assert!(matches!(s, Schedule::Once { .. }), "expected Once, got {s:?}");
}

#[test]
fn empty_string_errors() {
    assert!(matches!(parse_schedule(""), Err(ParseError::Empty)));
    assert!(matches!(parse_schedule("   "), Err(ParseError::Empty)));
}

#[test]
fn nonsense_errors() {
    assert!(matches!(
        parse_schedule("blarg"),
        Err(ParseError::Unknown { .. })
    ));
}

#[test]
fn bad_duration_unit_errors() {
    assert!(matches!(
        parse_schedule("30x"),
        Err(ParseError::Unknown { .. })
    ));
}

#[test]
fn bad_cron_too_few_fields() {
    // 4 fields parses as ISO/duration/unknown — never as cron.
    let r = parse_schedule("0 9 * *");
    assert!(matches!(r, Err(ParseError::Unknown { .. })));
}

#[test]
fn duration_helper_30m() {
    assert_eq!(parse_duration("30m").unwrap(), Duration::from_secs(30 * 60));
}

#[test]
fn duration_helper_4h() {
    assert_eq!(parse_duration("4h").unwrap(), Duration::from_secs(4 * 3600));
}

#[test]
fn duration_helper_3d() {
    assert_eq!(parse_duration("3d").unwrap(), Duration::from_secs(3 * 86_400));
}

#[test]
fn duration_helper_rejects_no_unit() {
    assert!(matches!(parse_duration("30"), Err(ParseError::BadDuration { .. })));
}

#[test]
fn duration_helper_rejects_no_digits() {
    assert!(matches!(parse_duration("h"), Err(ParseError::BadDuration { .. })));
}

#[test]
fn duration_helper_rejects_bad_unit() {
    assert!(matches!(parse_duration("5y"), Err(ParseError::BadDuration { .. })));
}

#[test]
fn next_after_for_interval() {
    let s = parse_schedule("every 1m").unwrap();
    let now = chrono::Utc::now();
    let nxt = s.next_after(now).unwrap();
    assert!(nxt > now);
    assert!(nxt <= now + chrono::Duration::seconds(61));
}

#[test]
fn next_after_for_after_duration() {
    let s = parse_schedule("2h").unwrap();
    let now = chrono::Utc::now();
    let nxt = s.next_after(now).unwrap();
    let diff = (nxt - now).num_seconds();
    assert_eq!(diff, 7200);
}

#[test]
fn next_after_past_once_returns_none() {
    let past = chrono::Utc::now() - chrono::Duration::hours(1);
    let s = Schedule::Once { at: past };
    assert!(s.next_after(chrono::Utc::now()).is_none());
}

// ---- helpers ----
fn matches_after_duration(s: Schedule, expected_secs: u64) {
    match s {
        Schedule::AfterDuration { delta } => assert_eq!(delta, Duration::from_secs(expected_secs)),
        other => panic!("expected AfterDuration, got {other:?}"),
    }
}
fn matches_interval(s: Schedule, expected_secs: u64) {
    match s {
        Schedule::Interval { every } => assert_eq!(every, Duration::from_secs(expected_secs)),
        other => panic!("expected Interval, got {other:?}"),
    }
}
fn matches_cron(s: Schedule, contains: &str) {
    match s {
        Schedule::Cron { expr } => assert!(expr.contains(contains), "expr {expr:?} missing {contains:?}"),
        other => panic!("expected Cron, got {other:?}"),
    }
}
