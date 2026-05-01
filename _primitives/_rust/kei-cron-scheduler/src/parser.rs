//! Schedule parser.
//!
//! Port of Hermes `cron/jobs.py:parse_schedule` (102-209). Four input modes:
//!
//! 1. Bare duration  — `30m`, `2h`, `1d`             → [`Schedule::AfterDuration`]
//! 2. Recurring      — `every 30m`, `every 2h`       → [`Schedule::Interval`]
//! 3. Cron expr      — `0 9 * * *`                   → [`Schedule::Cron`]
//! 4. ISO timestamp  — `2026-05-01T14:00:00Z`        → [`Schedule::Once`]

use std::str::FromStr;
use std::time::Duration;

use chrono::{DateTime, Utc};
use thiserror::Error;

use crate::job::Schedule;

/// All parser errors.
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("empty schedule string")]
    Empty,
    #[error("invalid duration {raw:?}: expected like '30m', '2h', '1d'")]
    BadDuration { raw: String },
    #[error("invalid cron expression {raw:?}: {source}")]
    BadCron {
        raw: String,
        #[source]
        source: cron::error::Error,
    },
    #[error("invalid ISO timestamp {raw:?}: {source}")]
    BadTimestamp {
        raw: String,
        #[source]
        source: chrono::ParseError,
    },
    #[error("unrecognised schedule {raw:?}")]
    Unknown { raw: String },
}

/// Top-level entry point.
pub fn parse_schedule(input: &str) -> Result<Schedule, ParseError> {
    let raw = input.trim();
    if raw.is_empty() {
        return Err(ParseError::Empty);
    }

    let lower = raw.to_lowercase();

    if let Some(rest) = lower.strip_prefix("every ") {
        let dur = parse_duration(rest.trim())?;
        return Ok(Schedule::Interval { every: dur });
    }

    if looks_like_cron(raw) {
        return parse_cron(raw);
    }

    if looks_like_iso(raw) {
        return parse_iso(raw);
    }

    // Bare duration → one-shot from now.
    if let Ok(dur) = parse_duration(raw) {
        return Ok(Schedule::AfterDuration { delta: dur });
    }

    Err(ParseError::Unknown { raw: raw.into() })
}

/// Parse `30m`, `2h`, `1d` (and verbose variants).
pub fn parse_duration(raw: &str) -> Result<Duration, ParseError> {
    let s = raw.trim().to_lowercase();
    let (digits, unit) = split_digits_and_unit(&s).ok_or_else(|| ParseError::BadDuration {
        raw: raw.into(),
    })?;
    let value: u64 = digits.parse().map_err(|_| ParseError::BadDuration {
        raw: raw.into(),
    })?;
    let multiplier = match unit.chars().next() {
        Some('m') => 60,
        Some('h') => 3600,
        Some('d') => 86_400,
        _ => return Err(ParseError::BadDuration { raw: raw.into() }),
    };
    Ok(Duration::from_secs(value * multiplier))
}

fn split_digits_and_unit(s: &str) -> Option<(&str, &str)> {
    let pos = s.find(|c: char| !c.is_ascii_digit())?;
    let (head, tail) = s.split_at(pos);
    if head.is_empty() {
        return None;
    }
    let unit = tail.trim();
    if unit.is_empty() {
        return None;
    }
    Some((head, unit))
}

/// Heuristic: ≥5 whitespace-separated tokens, each containing only digits or
/// cron metachars (`* - , /`).
fn looks_like_cron(raw: &str) -> bool {
    let parts: Vec<&str> = raw.split_whitespace().collect();
    if parts.len() < 5 {
        return false;
    }
    parts
        .iter()
        .take(5)
        .all(|p| p.chars().all(|c| c.is_ascii_digit() || matches!(c, '*' | '-' | ',' | '/')))
}

fn parse_cron(raw: &str) -> Result<Schedule, ParseError> {
    // The `cron` crate expects 7 fields (sec min hour dom mon dow year). Hermes
    // uses 5-field POSIX cron — we prepend `0 ` for seconds and accept either.
    let s = if raw.split_whitespace().count() == 5 {
        format!("0 {raw} *")
    } else {
        raw.to_string()
    };
    cron::Schedule::from_str(&s)
        .map(|_| Schedule::Cron { expr: s })
        .map_err(|source| ParseError::BadCron {
            raw: raw.into(),
            source,
        })
}

fn looks_like_iso(raw: &str) -> bool {
    raw.contains('T') || raw.starts_with(|c: char| c.is_ascii_digit()) && raw.contains('-')
}

fn parse_iso(raw: &str) -> Result<Schedule, ParseError> {
    let normalised = raw.replace('Z', "+00:00");
    let dt: DateTime<Utc> = match DateTime::parse_from_rfc3339(&normalised) {
        Ok(d) => d.with_timezone(&Utc),
        Err(e) => {
            // Try naive form: `YYYY-MM-DDTHH:MM:SS`
            if let Ok(naive) =
                chrono::NaiveDateTime::parse_from_str(raw, "%Y-%m-%dT%H:%M:%S")
            {
                naive.and_utc()
            } else if let Ok(naive) =
                chrono::NaiveDateTime::parse_from_str(raw, "%Y-%m-%dT%H:%M")
            {
                naive.and_utc()
            } else {
                return Err(ParseError::BadTimestamp {
                    raw: raw.into(),
                    source: e,
                });
            }
        }
    };
    Ok(Schedule::Once { at: dt })
}
