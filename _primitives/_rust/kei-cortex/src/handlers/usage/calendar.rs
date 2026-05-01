//! Calendar-day boundary computation in local time (F-MED-3).
//!
//! Three anchors:
//!   - `today_start_ts`  — 00:00 of the current local day
//!   - `week_start_ts`   — 00:00 Monday of the current ISO week
//!   - `month_start_ts`  — 00:00 of the 1st of the current month
//!
//! All exposed as unix seconds (UTC), suitable for `WHERE started_ts >= ?`
//! against `started_ts INTEGER` in the ledger DB.
//!
//! The pure helper `boundaries_for(local_dt)` is split out so unit tests
//! can drive a deterministic `NaiveDateTime` instead of `Local::now()`.

use chrono::{Datelike, Days, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Weekday};

/// Three calendar anchors as unix seconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalendarBoundaries {
    pub today_start_ts: i64,
    pub week_start_ts: i64,
    pub month_start_ts: i64,
}

impl CalendarBoundaries {
    /// Compute the three boundaries from the system clock in local time.
    pub fn for_now_local() -> Self {
        let now_local = Local::now().naive_local();
        boundaries_for(now_local)
    }
}

/// Pure boundary computation given a naive local datetime. Folds three
/// dates (today / monday / first-of-month) back to UTC unix seconds via
/// `Local`. If a local instant is ambiguous (DST gap), we take the
/// earliest valid mapping — usage rollups never need sub-hour precision.
pub fn boundaries_for(local_dt: NaiveDateTime) -> CalendarBoundaries {
    let date = local_dt.date();
    CalendarBoundaries {
        today_start_ts: midnight_local_to_utc_ts(date),
        week_start_ts: midnight_local_to_utc_ts(start_of_iso_week(date)),
        month_start_ts: midnight_local_to_utc_ts(start_of_month(date)),
    }
}

/// 00:00 local of `date` → unix seconds (UTC). DST-ambiguous instant
/// resolves to the earlier offset (consistent for monthly rollups).
fn midnight_local_to_utc_ts(date: NaiveDate) -> i64 {
    let naive = NaiveDateTime::new(date, NaiveTime::from_hms_opt(0, 0, 0).expect("00:00:00"));
    Local
        .from_local_datetime(&naive)
        .earliest()
        .map(|dt| dt.timestamp())
        .unwrap_or(0)
}

/// Monday of the ISO week containing `date`. ISO weeks are Monday-anchored.
fn start_of_iso_week(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday();
    let back: u64 = match weekday {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6,
    };
    date.checked_sub_days(Days::new(back)).unwrap_or(date)
}

/// 1st-of-month for `date`.
fn start_of_month(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap_or(date)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Wednesday 2026-04-22 14:30 → today=22, week=20 (Mon), month=01.
    #[test]
    fn wednesday_anchors_to_monday_and_month_start() {
        let dt = NaiveDate::from_ymd_opt(2026, 4, 22)
            .unwrap()
            .and_hms_opt(14, 30, 0)
            .unwrap();
        let b = boundaries_for(dt);
        let (today, week, month) = (
            unix_to_naive_local(b.today_start_ts).date(),
            unix_to_naive_local(b.week_start_ts).date(),
            unix_to_naive_local(b.month_start_ts).date(),
        );
        assert_eq!(today, NaiveDate::from_ymd_opt(2026, 4, 22).unwrap());
        assert_eq!(week, NaiveDate::from_ymd_opt(2026, 4, 20).unwrap());
        assert_eq!(month, NaiveDate::from_ymd_opt(2026, 4, 1).unwrap());
    }

    /// Sunday 2026-04-26 23:59 → today=26, week=20 (Mon prior), month=01.
    #[test]
    fn sunday_anchors_to_prior_monday() {
        let dt = NaiveDate::from_ymd_opt(2026, 4, 26)
            .unwrap()
            .and_hms_opt(23, 59, 0)
            .unwrap();
        let b = boundaries_for(dt);
        let week_date = unix_to_naive_local(b.week_start_ts).date();
        assert_eq!(week_date, NaiveDate::from_ymd_opt(2026, 4, 20).unwrap());
    }

    /// 23:59 today and 00:01 tomorrow give DIFFERENT today_start_ts —
    /// the sliding-window pre-fix would have given (close enough to) the
    /// same answer. This is the heart of F-MED-3.
    #[test]
    fn midnight_rollover_changes_today_start() {
        let evening = NaiveDate::from_ymd_opt(2026, 4, 22)
            .unwrap()
            .and_hms_opt(23, 59, 0)
            .unwrap();
        let next_morning = NaiveDate::from_ymd_opt(2026, 4, 23)
            .unwrap()
            .and_hms_opt(0, 1, 0)
            .unwrap();
        let b1 = boundaries_for(evening);
        let b2 = boundaries_for(next_morning);
        assert_ne!(
            b1.today_start_ts, b2.today_start_ts,
            "calendar today must roll over at local midnight"
        );
        // The new start is at LEAST 23h*3600 later (DST-tolerant lower bound).
        assert!(b2.today_start_ts - b1.today_start_ts >= 23 * 3600);
    }

    fn unix_to_naive_local(ts: i64) -> NaiveDateTime {
        Local
            .timestamp_opt(ts, 0)
            .single()
            .expect("ts maps")
            .naive_local()
    }
}
