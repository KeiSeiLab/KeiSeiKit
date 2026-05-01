//! Phase D skill-invocation metrics (HERMES-MIGRATION-PLAN P3.4).
//!
//! Constructor Pattern: one cube = one read/write surface for the
//! `skill_invocations` table introduced by schema v8. Phase D's nightly
//! self-improvement loop uses these helpers to decide
//!   - archive (skill not used in N days)
//!   - re-extract (success_rate < 60% over last M days)
//!   - mark validated (usage_count > 20 AND success_rate > 90%)
//!
//! Times are unix-seconds (i64), matching the rest of the ledger
//! (`agents.started_ts`, etc.). The task spec calls for unix-ms; we keep
//! seconds for ledger-wide consistency — Phase D thresholds are "days",
//! and millisecond resolution buys nothing while breaking SUM/MAX
//! comparisons against existing columns.

use rusqlite::{params, Connection, OptionalExtension, Result as SqlResult};
use serde::{Deserialize, Serialize};

/// One invocation record. `success` is the agent's review.md verdict
/// (boolean). `duration_ms` captures wall-time even though `ts` is
/// seconds — Phase D plots latency distribution per skill.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInvocation {
    pub skill_name: String,
    pub ts: i64,
    pub agent_id: Option<String>,
    pub success: bool,
    pub trajectory_id: Option<String>,
    pub duration_ms: Option<i64>,
}

/// Append a row. Returns rows-inserted (always 1 on success).
pub fn record_invocation(conn: &Connection, inv: &SkillInvocation) -> SqlResult<usize> {
    conn.execute(
        "INSERT INTO skill_invocations
            (skill_name, ts, agent_id, success, trajectory_id, duration_ms)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            inv.skill_name,
            inv.ts,
            inv.agent_id,
            i64::from(inv.success),
            inv.trajectory_id,
            inv.duration_ms,
        ],
    )
}

/// Success rate over `lookback_days` for `skill_name`. Returns NaN when
/// there are zero invocations in the window — caller decides archive vs
/// stays-quiet.
pub fn success_rate(conn: &Connection, skill_name: &str, lookback_days: u32) -> SqlResult<f64> {
    let cutoff = cutoff_ts(lookback_days);
    let row: Option<(i64, i64)> = conn
        .query_row(
            "SELECT
                COALESCE(SUM(success), 0) AS wins,
                COUNT(*) AS total
             FROM skill_invocations
             WHERE skill_name = ?1 AND ts >= ?2",
            params![skill_name, cutoff],
            |r| Ok((r.get::<_, i64>(0)?, r.get::<_, i64>(1)?)),
        )
        .optional()?;
    Ok(match row {
        Some((_, 0)) | None => f64::NAN,
        Some((wins, total)) => wins as f64 / total as f64,
    })
}

/// Count invocations of `skill_name` in the last `lookback_days`.
pub fn usage_count(conn: &Connection, skill_name: &str, lookback_days: u32) -> SqlResult<u64> {
    let cutoff = cutoff_ts(lookback_days);
    let n: i64 = conn.query_row(
        "SELECT COUNT(*) FROM skill_invocations
         WHERE skill_name = ?1 AND ts >= ?2",
        params![skill_name, cutoff],
        |r| r.get(0),
    )?;
    Ok(n.max(0) as u64)
}

/// Most-recent `ts` for `skill_name`. Returns `None` if never invoked.
pub fn last_used(conn: &Connection, skill_name: &str) -> SqlResult<Option<i64>> {
    conn.query_row(
        "SELECT MAX(ts) FROM skill_invocations WHERE skill_name = ?1",
        params![skill_name],
        |r| r.get::<_, Option<i64>>(0),
    )
}

/// Distinct skill names whose most-recent invocation is older than
/// `days` (or that have NO invocations at all but a row exists somehow).
/// Phase D archive policy reads this list at 03:00.
///
/// Production wrapper using wall-clock `chrono::Utc::now()`. Tests that need
/// to pin the clock against synthetic future-epoch fixtures should call
/// [`unused_skills_at`].
pub fn unused_skills(conn: &Connection, days: u32) -> SqlResult<Vec<String>> {
    unused_skills_at(conn, days, chrono::Utc::now().timestamp())
}

/// Test-injectable variant of [`unused_skills`].
///
/// HAZARD: production fixtures historically used a synthetic
/// `NOW=1_900_000_000` (~year 2030) epoch that is FUTURE relative to
/// real wall-clock. With the closure form of `unused_skills` (clock from
/// `Utc::now()`), every fixture row landed AFTER cutoff and the strict
/// `<` HAVING predicate returned empty — masking archive-policy bugs.
/// This variant lets tests pin `now` to the same epoch their fixtures use.
pub fn unused_skills_at(conn: &Connection, days: u32, now: i64) -> SqlResult<Vec<String>> {
    let cutoff = cutoff_ts_at(days, now);
    let mut stmt = conn.prepare(
        "SELECT skill_name FROM skill_invocations
         GROUP BY skill_name
         HAVING MAX(ts) < ?1
         ORDER BY skill_name",
    )?;
    let rows = stmt.query_map(params![cutoff], |r| r.get::<_, String>(0))?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

/// `now - days*86400`. Centralised so tests and helpers agree on the
/// cutoff arithmetic (one place to fix if we move to ms).
fn cutoff_ts(days: u32) -> i64 {
    cutoff_ts_at(days, chrono::Utc::now().timestamp())
}

/// Test-injectable: `cutoff_ts(days)` reduces to this with `now` from clock.
pub(crate) fn cutoff_ts_at(days: u32, now: i64) -> i64 {
    now.saturating_sub(i64::from(days) * 86_400)
}

#[cfg(test)]
#[path = "skill_metrics_test.rs"]
mod tests;
