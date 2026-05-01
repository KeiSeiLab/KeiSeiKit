//! Integration tests for kei-scheduler. Uses `Store::open_memory` so
//! each test owns a throwaway DB and a deterministic wall clock
//! (`now` passed explicitly where the API allows).
//!
//! `schedule()` + `cancel()` internally read `Utc::now()` once; that's
//! fine because we check relative ordering (`next_run_at` compared to
//! a post-call `Utc::now()` lower bound), not absolute values.

use chrono::Utc;
use kei_scheduler::{
    cancel, compute_next, get_task, list_due, mark_run, open_memory, schedule,
    task_status, ParseError, Store, AT, CRON, INTERVAL,
};

fn store() -> Store {
    open_memory().expect("in-memory store opens clean")
}

#[test]
fn cron_schedule_sets_future_next_run() {
    let s = store();
    let before = Utc::now().timestamp();
    let id = schedule(s.conn(), "cron1", CRON, "*/5 * * * *", "echo").unwrap();
    let t = get_task(s.conn(), id).unwrap().unwrap();
    assert_eq!(t.name, "cron1");
    assert_eq!(t.trigger_kind, "cron");
    assert_eq!(t.status, task_status::PENDING);
    let next = t.next_run_at.expect("cron trigger must produce a next_run_at");
    assert!(next >= before, "next_run_at {next} must be >= launch time {before}");
}

#[test]
fn at_schedule_with_future_ts_matches_iso_parse() {
    let s = store();
    // 2030-01-01T00:00:00Z → unix 1893456000 (verified via
    // chrono::DateTime::parse_from_rfc3339 at test time).
    let id = schedule(s.conn(), "at1", AT, "2030-01-01T00:00:00Z", "echo").unwrap();
    let t = get_task(s.conn(), id).unwrap().unwrap();
    assert_eq!(t.next_run_at, Some(1_893_456_000));
    assert_eq!(t.trigger_kind, "at");
}

#[test]
fn interval_schedule_sets_now_plus_secs() {
    let s = store();
    let before = Utc::now().timestamp();
    let id = schedule(s.conn(), "int1", INTERVAL, "3600", "echo").unwrap();
    let t = get_task(s.conn(), id).unwrap().unwrap();
    let next = t.next_run_at.expect("interval trigger must set next_run_at");
    let after = Utc::now().timestamp();
    assert!(next >= before + 3600);
    assert!(next <= after + 3600);
}

#[test]
fn cancel_sets_status_and_clears_next_run() {
    let s = store();
    let id = schedule(s.conn(), "tcan", INTERVAL, "60", "echo").unwrap();
    cancel(s.conn(), id).unwrap();
    let t = get_task(s.conn(), id).unwrap().unwrap();
    assert_eq!(t.status, task_status::CANCELLED);
    assert_eq!(t.next_run_at, None);
}

#[test]
fn list_due_returns_eligible_pending_tasks() {
    let s = store();
    // An interval with spec=60 produces next_run ≈ now+60. Query with
    // now+120 to make sure it's due.
    schedule(s.conn(), "due1", INTERVAL, "60", "echo").unwrap();
    let query_ts = Utc::now().timestamp() + 120;
    let due = list_due(s.conn(), query_ts).unwrap();
    assert_eq!(due.len(), 1);
    assert_eq!(due[0].name, "due1");
    assert_eq!(due[0].status, task_status::PENDING);
}

#[test]
fn list_due_excludes_cancelled_tasks() {
    let s = store();
    let id = schedule(s.conn(), "cx", INTERVAL, "60", "echo").unwrap();
    cancel(s.conn(), id).unwrap();
    let due = list_due(s.conn(), Utc::now().timestamp() + 100_000).unwrap();
    assert!(due.is_empty(), "cancelled tasks must not appear in list_due");
}

#[test]
fn mark_run_on_interval_advances_next_run() {
    let s = store();
    let id = schedule(s.conn(), "mrint", INTERVAL, "3600", "echo").unwrap();
    let now = 2_000_000_000;
    mark_run(s.conn(), id, 0, now).unwrap();
    let t = get_task(s.conn(), id).unwrap().unwrap();
    assert_eq!(t.last_run_at, Some(now));
    assert_eq!(t.last_exit_code, Some(0));
    assert_eq!(t.next_run_at, Some(now + 3600));
    assert_eq!(t.status, task_status::SCHEDULED);
}

#[test]
fn mark_run_on_at_completes_task() {
    let s = store();
    let id = schedule(s.conn(), "mrat", AT, "2030-01-01T00:00:00Z", "echo").unwrap();
    mark_run(s.conn(), id, 0, 1_893_456_005).unwrap();
    let t = get_task(s.conn(), id).unwrap().unwrap();
    assert_eq!(t.status, task_status::DONE);
    assert_eq!(t.next_run_at, None);
    assert_eq!(t.last_exit_code, Some(0));
}

#[test]
fn mark_run_on_cron_recomputes_next() {
    let s = store();
    let id = schedule(s.conn(), "mrcron", CRON, "*/5 * * * *", "echo").unwrap();
    let now: i64 = 2_000_000_000;
    mark_run(s.conn(), id, 0, now).unwrap();
    let t = get_task(s.conn(), id).unwrap().unwrap();
    let next = t.next_run_at.expect("cron mark_run must recompute next_run_at");
    // `*/5 * * * *` = every 5 minutes at second-0; next must be within
    // 300 seconds of `now` and strictly greater.
    assert!(next > now);
    assert!(next <= now + 300, "next {next} must be within 5m of now {now}");
    assert_eq!(t.status, task_status::SCHEDULED);
}

#[test]
fn mark_run_with_nonzero_exit_at_sets_failed() {
    let s = store();
    let id = schedule(s.conn(), "failat", AT, "2030-06-15T12:00:00Z", "echo").unwrap();
    mark_run(s.conn(), id, 17, 1_910_000_000).unwrap();
    let t = get_task(s.conn(), id).unwrap().unwrap();
    assert_eq!(t.status, task_status::FAILED);
    assert_eq!(t.last_exit_code, Some(17));
    assert_eq!(t.next_run_at, None);
}

#[test]
fn invalid_cron_returns_parse_error() {
    let err = compute_next(CRON, "not a cron expression", 0).unwrap_err();
    assert!(
        matches!(err, ParseError::InvalidCron(_, _)),
        "expected InvalidCron, got {err:?}"
    );
}

#[test]
fn invalid_iso_datetime_returns_parse_error() {
    let err = compute_next(AT, "not-a-date", 0).unwrap_err();
    assert!(
        matches!(err, ParseError::InvalidIsoDatetime(_)),
        "expected InvalidIsoDatetime, got {err:?}"
    );
}

#[test]
fn malformed_trigger_kind_is_rejected() {
    let s = store();
    let err = schedule(s.conn(), "bad", "weekly", "whatever", "echo")
        .expect_err("unknown kind must fail");
    assert!(
        matches!(err, kei_scheduler::Error::Parse(ParseError::UnknownKind(_))),
        "expected UnknownKind, got {err:?}"
    );
}

#[test]
fn duplicate_name_is_rejected_typed() {
    let s = store();
    schedule(s.conn(), "dup", INTERVAL, "60", "echo").unwrap();
    let err = schedule(s.conn(), "dup", INTERVAL, "120", "echo")
        .expect_err("unique-name collision must fail");
    assert!(
        matches!(err, kei_scheduler::Error::NameExists(ref n) if n == "dup"),
        "expected NameExists(dup), got {err:?}"
    );
}

#[test]
fn zero_interval_is_rejected() {
    let err = compute_next(INTERVAL, "0", 100).unwrap_err();
    assert!(matches!(err, ParseError::InvalidInterval(_)));
}

#[test]
fn at_in_the_past_returns_none() {
    // 2020-01-01 with `from = 2030-era` → no future fire.
    let next = compute_next(AT, "2020-01-01T00:00:00Z", 1_893_456_000).unwrap();
    assert_eq!(next, None);
}
