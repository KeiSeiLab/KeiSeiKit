//! RFC-3339-ish UTC timestamp helpers.
//!
//! Constructor Pattern: single responsibility — compute the current UTC
//! wall-clock as a `"YYYY-MM-DDThh:mm:ssZ"` string without pulling in
//! `chrono` for this one job. Extracted from `config.rs` in v0.22 so the
//! config module stays under the 200-LOC ceiling.
//!
//! Uses Howard Hinnant's civil-from-days algorithm (public domain,
//! <http://howardhinnant.github.io/date_algorithms.html>). Tested against
//! four anchor dates including the century non-leap edge case
//! (`2100-03-01`, NOT Feb 29).

/// Current UTC time as `"YYYY-MM-DDThh:mm:ssZ"`. Falls back to epoch on
/// clock-skew failure — guarantees a round-trippable string at every call.
pub fn now_utc_string() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format_epoch_utc(secs)
}

/// Format a Unix epoch (seconds) as UTC `"YYYY-MM-DDThh:mm:ssZ"`.
pub fn format_epoch_utc(secs: u64) -> String {
    let days = (secs / 86400) as i64;
    let rem = secs % 86400;
    let (h, m, s) = (rem / 3600, (rem % 3600) / 60, rem % 60);
    let (y, mo, d) = civil_from_days(days);
    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", y, mo, d, h, m, s)
}

/// Civil-from-days (Howard Hinnant). `z` is days since 1970-01-01 (may be
/// negative for pre-epoch). Returns `(year, month, day)` in the proleptic
/// Gregorian calendar. Correct for the 400-year cycle including century
/// non-leap years (1900, 2100, 2200, 2300 are NOT leap; 2000, 2400 ARE).
pub fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m as u32, d as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_zero_is_1970_01_01() {
        assert_eq!(format_epoch_utc(0), "1970-01-01T00:00:00Z");
    }

    #[test]
    fn leap_day_2020_02_29() {
        // 2020-02-29T12:00:00Z — 2020 IS a leap year (div by 4, not by 100,
        // and not a century year). 1582977600 = 2020-02-29T12:00:00Z.
        assert_eq!(format_epoch_utc(1582977600), "2020-02-29T12:00:00Z");
    }

    #[test]
    fn century_non_leap_2100_03_01() {
        // 2100 is NOT a leap year (div by 100, not by 400). So
        // "2100-02-29" does not exist — day after 2100-02-28 is
        // 2100-03-01. Test ensures the Hinnant 400-year cycle is correct.
        assert_eq!(format_epoch_utc(4107542400), "2100-03-01T00:00:00Z");
    }

    #[test]
    fn arbitrary_2026_04_22() {
        // 1776877200 = 2026-04-22T17:00:00Z (UTC). Anchor sanity-check
        // for a recent real-world timestamp.
        assert_eq!(format_epoch_utc(1776877200), "2026-04-22T17:00:00Z");
    }

    #[test]
    fn civil_from_days_matches_anchors() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        // 2000-01-01 = day 10957 since 1970-01-01.
        assert_eq!(civil_from_days(10957), (2000, 1, 1));
        // 2000 IS a leap year — Feb 29 exists.
        assert_eq!(civil_from_days(11016), (2000, 2, 29));
    }
}
