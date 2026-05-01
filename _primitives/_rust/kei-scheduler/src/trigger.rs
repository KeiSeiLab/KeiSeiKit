//! Trigger parsing + `compute_next` core.
//!
//! Three trigger kinds:
//! - `cron` — 5-field or 6-field cron expression. 5-field inputs get a
//!   leading `"0 "` for seconds (standard cron semantics: trigger at
//!   the start of the minute).
//! - `at` — one-shot ISO-8601 datetime with trailing `Z` (UTC).
//! - `interval` — repeat every N seconds (positive integer).
//!
//! `compute_next(kind, spec, from)` is pure — no DB, no clock read.
//! Callers pass `from` explicitly so tests are deterministic.

use crate::error::ParseError;
use chrono::{DateTime, TimeZone, Utc};
use cron::Schedule;
use std::str::FromStr;

/// Canonical trigger-kind names. Schema stores these as TEXT; this
/// helper keeps the spelling in one place and rejects anything else at
/// the boundary.
pub const CRON: &str = "cron";
pub const AT: &str = "at";
pub const INTERVAL: &str = "interval";

/// Validate a trigger kind string. Returned as `&'static str` so callers
/// can store the canonical form without allocating.
pub fn validate_kind(kind: &str) -> Result<&'static str, ParseError> {
    match kind {
        CRON => Ok(CRON),
        AT => Ok(AT),
        INTERVAL => Ok(INTERVAL),
        other => Err(ParseError::UnknownKind(other.to_string())),
    }
}

/// Compute the next fire time (unix seconds, UTC) for a trigger given a
/// reference `from` timestamp. Returns `Ok(None)` when no future fire
/// exists (e.g. `at` timestamp already in the past, or cron schedule
/// with no upcoming occurrence in `chrono`'s representable range).
pub fn compute_next(
    kind: &str,
    spec: &str,
    from: i64,
) -> Result<Option<i64>, ParseError> {
    match validate_kind(kind)? {
        CRON => next_cron(spec, from),
        AT => next_at(spec, from),
        INTERVAL => next_interval(spec, from),
        _ => unreachable!("validate_kind rejects other variants"),
    }
}

fn next_cron(spec: &str, from: i64) -> Result<Option<i64>, ParseError> {
    let canon = canonicalize_cron(spec);
    let sched = Schedule::from_str(&canon)
        .map_err(|e| ParseError::InvalidCron(spec.to_string(), e.to_string()))?;
    let from_dt = ts_to_utc(from)
        .ok_or_else(|| ParseError::InvalidCron(spec.to_string(), "ref-ts overflow".into()))?;
    Ok(sched.after(&from_dt).next().map(|dt| dt.timestamp()))
}

/// Accept both classic 5-field (`min hour dom mon dow`) and the
/// cron-crate's 6-field form (`sec min hour dom mon dow`). 7-field
/// expressions (with year) pass through untouched.
fn canonicalize_cron(spec: &str) -> String {
    let fields = spec.split_whitespace().count();
    if fields == 5 {
        format!("0 {}", spec)
    } else {
        spec.to_string()
    }
}

fn next_at(spec: &str, from: i64) -> Result<Option<i64>, ParseError> {
    let ts = parse_iso_z(spec)
        .ok_or_else(|| ParseError::InvalidIsoDatetime(spec.to_string()))?;
    if ts > from {
        Ok(Some(ts))
    } else {
        Ok(None)
    }
}

fn next_interval(spec: &str, from: i64) -> Result<Option<i64>, ParseError> {
    let secs: i64 = spec
        .parse::<u64>()
        .map_err(|_| ParseError::InvalidInterval(spec.to_string()))?
        .try_into()
        .map_err(|_| ParseError::InvalidInterval(spec.to_string()))?;
    if secs == 0 {
        return Err(ParseError::InvalidInterval(spec.to_string()));
    }
    Ok(Some(from.saturating_add(secs)))
}

/// Parse `YYYY-MM-DDTHH:MM:SSZ` (no fractional seconds, no offset other
/// than literal `Z`). `DateTime::parse_from_rfc3339` accepts that form.
fn parse_iso_z(spec: &str) -> Option<i64> {
    let dt = DateTime::parse_from_rfc3339(spec).ok()?;
    Some(dt.with_timezone(&Utc).timestamp())
}

fn ts_to_utc(ts: i64) -> Option<DateTime<Utc>> {
    Utc.timestamp_opt(ts, 0).single()
}
