//! CLI dispatch for the `aggregate-skills` subcommand.
//!
//! Constructor Pattern: formatting logic lives here, not in `dispatch.rs`
//! (which stays under 200 LOC). Two output modes: JSON array and Markdown
//! table. Sorted by success_rate ascending (worst first) so the nightly
//! operator sees the most urgent skills at the top.

use crate::skill_aggregator::{aggregate_skills, SkillAggregate, SkillRecommendation};
use rusqlite::Connection;
use std::process::ExitCode;

/// Seconds in 30 days (default lookback window).
const DEFAULT_LOOKBACK_SECS: i64 = 30 * 86_400;

pub fn cmd_aggregate_skills(
    conn: &Connection,
    since: Option<i64>,
    format: &str,
) -> ExitCode {
    let since_ts = since.or_else(|| {
        let now = chrono::Utc::now().timestamp();
        Some(now - DEFAULT_LOOKBACK_SECS)
    });
    match aggregate_skills(conn, since_ts) {
        Ok(rows) => emit(rows, format),
        Err(e) => {
            eprintln!("kei-ledger: aggregate-skills failed: {e}");
            ExitCode::from(1)
        }
    }
}

fn emit(rows: Vec<SkillAggregate>, format: &str) -> ExitCode {
    match format {
        "json" => emit_json(rows),
        "markdown" | "md" => emit_markdown(rows),
        other => {
            eprintln!("kei-ledger: unknown format '{other}'; use json or markdown");
            ExitCode::from(1)
        }
    }
}

fn emit_json(rows: Vec<SkillAggregate>) -> ExitCode {
    match serde_json::to_string_pretty(&rows) {
        Ok(s) => {
            println!("{s}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("kei-ledger: JSON serialisation failed: {e}");
            ExitCode::from(1)
        }
    }
}

fn emit_markdown(rows: Vec<SkillAggregate>) -> ExitCode {
    if rows.is_empty() {
        println!("*(no skill_invocations in window)*");
        return ExitCode::SUCCESS;
    }
    println!(
        "| skill | total | success% | p50_ms | p95_ms | last_ts | recommendation |"
    );
    println!("|-------|-------|----------|--------|--------|---------|----------------|");
    for r in &rows {
        println!(
            "| {} | {} | {:.1}% | {} | {} | {} | {} |",
            r.skill_name,
            r.total_invocations,
            r.success_rate * 100.0,
            r.p50_duration_ms,
            r.p95_duration_ms,
            r.last_invoked_ts,
            recommendation_label(&r.recommendation),
        );
    }
    ExitCode::SUCCESS
}

fn recommendation_label(r: &SkillRecommendation) -> &'static str {
    match r {
        SkillRecommendation::Validated => "validated",
        SkillRecommendation::Archive => "ARCHIVE",
        SkillRecommendation::Reextract => "reextract",
        SkillRecommendation::Insufficient => "insufficient",
    }
}
