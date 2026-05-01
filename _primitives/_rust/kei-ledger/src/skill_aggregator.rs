//! Phase D nightly aggregation cube for `skill_invocations`.
//!
//! Constructor Pattern: one cube = aggregate-read surface. The write side
//! lives in `skill_metrics.rs`. This file stays at ≤200 LOC.
//!
//! Decision rules (thresholds per task spec):
//!   - Validated  : total ≥ 10 AND success_rate ≥ 0.90
//!   - Archive    : total ≥ 10 AND success_rate < 0.30
//!   - Reextract  : total ≥ 10 AND success_rate ∈ [0.30, 0.70)
//!   - Insufficient: total < 10
//!
//! Times: unix-seconds (consistent with rest of ledger).

use rusqlite::{params, Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};

/// Recommendation tier for a skill based on aggregated metrics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillRecommendation {
    /// ≥10 invocations AND success_rate ≥ 0.90 → mark stable.
    Validated,
    /// ≥10 invocations AND success_rate < 0.30 → archive.
    Archive,
    /// ≥10 invocations AND success_rate ∈ [0.30, 0.70) → re-derive from corpus.
    Reextract,
    /// < 10 invocations → wait for more data.
    Insufficient,
}

/// Aggregated per-skill metrics for Phase D decision-making.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillAggregate {
    pub skill_name: String,
    pub total_invocations: u64,
    /// Success rate in [0.0, 1.0]. `0.0` when total_invocations == 0.
    pub success_rate: f64,
    /// Median duration (p50) in milliseconds; 0 when no duration data.
    pub p50_duration_ms: u64,
    /// 95th-percentile duration in milliseconds; 0 when no duration data.
    pub p95_duration_ms: u64,
    /// Unix-second timestamp of the most-recent invocation.
    pub last_invoked_ts: i64,
    pub recommendation: SkillRecommendation,
}

/// Derive the recommendation tier from counts and rate.
fn recommend(total: u64, success_rate: f64) -> SkillRecommendation {
    if total < 10 {
        return SkillRecommendation::Insufficient;
    }
    if success_rate >= 0.90 {
        SkillRecommendation::Validated
    } else if success_rate < 0.30 {
        SkillRecommendation::Archive
    } else if success_rate < 0.70 {
        SkillRecommendation::Reextract
    } else {
        // [0.70, 0.90) — not yet stable enough to validate, not bad enough to reextract
        SkillRecommendation::Insufficient
    }
}

/// Compute p50 and p95 for a single skill via a percentile sub-query.
///
/// SQLite lacks a native percentile aggregate, so we use NTILE-compatible
/// ORDER-BY row selection. Rows without duration_ms are excluded.
fn percentiles(
    conn: &Connection,
    skill_name: &str,
    since_ts: Option<i64>,
) -> SqlResult<(u64, u64)> {
    let cutoff = since_ts.unwrap_or(0);
    let mut stmt = conn.prepare(
        "SELECT duration_ms FROM skill_invocations
         WHERE skill_name = ?1 AND duration_ms IS NOT NULL AND ts >= ?2
         ORDER BY duration_ms ASC",
    )?;
    let durations: Vec<u64> = stmt
        .query_map(params![skill_name, cutoff], |r| r.get::<_, i64>(0))?
        .filter_map(|r| r.ok())
        .map(|v| v.max(0) as u64)
        .collect();
    Ok(compute_percentiles(&durations))
}

/// Pure fn: index-based p50/p95 from a sorted slice.
fn compute_percentiles(sorted: &[u64]) -> (u64, u64) {
    if sorted.is_empty() {
        return (0, 0);
    }
    let n = sorted.len();
    let p50 = sorted[(n - 1) / 2];
    let p95_idx = ((n as f64 * 0.95).ceil() as usize).saturating_sub(1).min(n - 1);
    (p50, sorted[p95_idx])
}

/// Aggregate all skills from `skill_invocations`.
///
/// `since_ts`: optional unix-second lower bound; pass `None` to include all rows.
/// Returns one `SkillAggregate` per distinct `skill_name`, sorted by
/// `success_rate` ascending (worst first — matching the markdown format).
pub fn aggregate_skills(
    conn: &Connection,
    since_ts: Option<i64>,
) -> SqlResult<Vec<SkillAggregate>> {
    let cutoff = since_ts.unwrap_or(0);
    let mut stmt = conn.prepare(
        "SELECT skill_name,
                COUNT(*)                             AS total,
                COALESCE(SUM(success), 0)            AS wins,
                MAX(ts)                              AS last_ts
         FROM skill_invocations
         WHERE ts >= ?1
         GROUP BY skill_name
         ORDER BY CAST(COALESCE(SUM(success), 0) AS REAL) / COUNT(*) ASC,
                  skill_name ASC",
    )?;
    let rows = stmt.query_map(params![cutoff], |r| {
        let skill_name: String = r.get(0)?;
        let total: i64 = r.get(1)?;
        let wins: i64 = r.get(2)?;
        let last_ts: i64 = r.get(3)?;
        Ok((skill_name, total, wins, last_ts))
    })?;

    let mut out = Vec::new();
    for row in rows {
        let (skill_name, total, wins, last_invoked_ts) = row?;
        let total_u64 = total.max(0) as u64;
        let rate = if total == 0 { 0.0 } else { wins as f64 / total as f64 };
        let (p50, p95) = percentiles(conn, &skill_name, since_ts)?;
        out.push(SkillAggregate {
            skill_name,
            total_invocations: total_u64,
            success_rate: rate,
            p50_duration_ms: p50,
            p95_duration_ms: p95,
            last_invoked_ts,
            recommendation: recommend(total_u64, rate),
        });
    }
    Ok(out)
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn recommend_tiers() {
        assert_eq!(recommend(5, 0.95), SkillRecommendation::Insufficient);
        assert_eq!(recommend(10, 0.95), SkillRecommendation::Validated);
        assert_eq!(recommend(20, 0.25), SkillRecommendation::Archive);
        assert_eq!(recommend(20, 0.55), SkillRecommendation::Reextract);
        assert_eq!(recommend(20, 0.80), SkillRecommendation::Insufficient);
    }

    #[test]
    fn percentiles_empty_slice() {
        assert_eq!(compute_percentiles(&[]), (0, 0));
    }

    #[test]
    fn percentiles_single() {
        assert_eq!(compute_percentiles(&[42]), (42, 42));
    }

    #[test]
    fn percentiles_known_values() {
        // 10 values: [10,20,30,40,50,60,70,80,90,100]
        let v: Vec<u64> = (1..=10).map(|i| i * 10).collect();
        let (p50, p95) = compute_percentiles(&v);
        // p50 index = (10-1)/2 = 4 → 50
        assert_eq!(p50, 50);
        // p95 index = ceil(10*0.95)-1 = 10-1 = 9 → 100
        assert_eq!(p95, 100);
    }
}
