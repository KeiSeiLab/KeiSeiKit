//! Job + Schedule types.
//!
//! Hermes equivalent: `cron/jobs.py` (Job / parse_schedule output dict).

use std::str::FromStr;
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Stable job identifier (12-char hex per Hermes convention; we keep the
/// caller's choice though).
pub type JobId = String;

/// All supported schedule shapes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Schedule {
    /// One-shot at an absolute instant.
    Once { at: DateTime<Utc> },
    /// Recurring every `every` (Duration in seconds).
    Interval {
        #[serde(with = "duration_secs")]
        every: Duration,
    },
    /// Cron expression (5-field: minute hour day month weekday).
    Cron { expr: String },
    /// One-shot delta-from-creation (resolved to `Once { at }` at insertion).
    AfterDuration {
        #[serde(with = "duration_secs")]
        delta: Duration,
    },
}

impl Schedule {
    /// Compute the next firing instant after `now`, given the schedule.
    ///
    /// Returns `None` when the schedule is exhausted (e.g. `Once` already past).
    pub fn next_after(&self, now: DateTime<Utc>) -> Option<DateTime<Utc>> {
        match self {
            Schedule::Once { at } => {
                if *at > now {
                    Some(*at)
                } else {
                    None
                }
            }
            Schedule::Interval { every } => {
                let secs = every.as_secs() as i64;
                if secs <= 0 {
                    return None;
                }
                Some(now + chrono::Duration::seconds(secs))
            }
            Schedule::Cron { expr } => cron::Schedule::from_str(expr)
                .ok()
                .and_then(|s| s.after(&now).next()),
            Schedule::AfterDuration { delta } => {
                Some(now + chrono::Duration::seconds(delta.as_secs() as i64))
            }
        }
    }
}

/// Persisted job record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: JobId,
    pub prompt: String,
    pub schedule: Schedule,
    /// Optional Hermes-style toolset gating.
    #[serde(default)]
    pub enabled_toolsets: Vec<String>,
    pub created_at: DateTime<Utc>,
    /// When the runner last fired this job.
    pub last_run_at: Option<DateTime<Utc>>,
    /// Cumulative successful executions.
    #[serde(default)]
    pub run_count: u64,
    /// Pre-computed next firing instant (so the runner can sort cheaply).
    pub next_run_at: Option<DateTime<Utc>>,
}

impl Job {
    pub fn new(id: impl Into<JobId>, prompt: impl Into<String>, schedule: Schedule) -> Self {
        let now = Utc::now();
        let next = schedule.next_after(now);
        Self {
            id: id.into(),
            prompt: prompt.into(),
            schedule,
            enabled_toolsets: Vec::new(),
            created_at: now,
            last_run_at: None,
            run_count: 0,
            next_run_at: next,
        }
    }

    /// True if `now >= next_run_at`.
    pub fn is_due(&self, now: DateTime<Utc>) -> bool {
        matches!(self.next_run_at, Some(t) if now >= t)
    }

    /// Mark the job as just-fired and recompute `next_run_at`.
    pub fn mark_fired(&mut self, fired_at: DateTime<Utc>) {
        self.last_run_at = Some(fired_at);
        self.run_count = self.run_count.saturating_add(1);
        self.next_run_at = self.schedule.next_after(fired_at);
    }
}

/// Helper module for `serde(with = ...)` to serialise Duration as integer
/// seconds (matches Hermes' minutes-as-int convention closely enough).
mod duration_secs {
    use std::time::Duration;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(d: &Duration, s: S) -> Result<S::Ok, S::Error> {
        d.as_secs().serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
        let secs = u64::deserialize(d)?;
        Ok(Duration::from_secs(secs))
    }
}
