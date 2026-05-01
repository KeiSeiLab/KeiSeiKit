//! Minimal ISO-8601 UTC timestamp without external crate dependencies.
//!
//! Constructor Pattern: one pure function `utc_now()` with helpers.
//! No std-external time crate required — stdlib `SystemTime` only.

/// Return current UTC time as ISO-8601 string (`YYYY-MM-DDTHH:MM:SSZ`).
pub fn utc_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let (y, mo, d, h, mi, sec) = epoch_to_ymdhms(secs);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{mi:02}:{sec:02}Z")
}

fn epoch_to_ymdhms(secs: u64) -> (u64, u64, u64, u64, u64, u64) {
    let sec = secs % 60;
    let total_min = secs / 60;
    let mi = total_min % 60;
    let total_h = total_min / 60;
    let h = total_h % 24;
    let mut days = total_h / 24;
    let mut y = 1970u64;
    loop {
        let dy = if is_leap(y) { 366 } else { 365 };
        if days < dy {
            break;
        }
        days -= dy;
        y += 1;
    }
    let months = if is_leap(y) {
        [31u64, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31u64, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut mo = 1u64;
    for &dm in &months {
        if days < dm {
            break;
        }
        days -= dm;
        mo += 1;
    }
    (y, mo, days + 1, h, mi, sec)
}

fn is_leap(y: u64) -> bool {
    (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_zero_is_unix_epoch() {
        let (y, mo, d, h, mi, s) = epoch_to_ymdhms(0);
        assert_eq!((y, mo, d, h, mi, s), (1970, 1, 1, 0, 0, 0));
    }

    #[test]
    fn known_timestamp_parses_correctly() {
        // 2024-01-15T12:00:00Z = 1705320000
        let (y, mo, d, h, mi, s) = epoch_to_ymdhms(1_705_320_000);
        assert_eq!(y, 2024);
        assert_eq!(mo, 1);
        assert_eq!(d, 15);
        assert_eq!(h, 12);
        assert_eq!(mi, 0);
        assert_eq!(s, 0);
    }
}
