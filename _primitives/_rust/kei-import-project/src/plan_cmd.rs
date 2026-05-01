//! plan_cmd — CLI orchestrator for Phase 4: migration plan generation.
//!
//! Composes map_cmd::build_map → plan_generator::build_plan → render_markdown.
//! Constructor Pattern: one responsibility, ≤100 LOC, ≤30 LOC per fn.

use crate::{map_cmd, plan_generator};
use anyhow::Result;
use std::path::Path;

/// Run the `plan` subcommand end-to-end.
///
/// - Walks `repo_path` and runs the trait matcher.
/// - Clusters results into a numbered migration plan.
/// - Writes markdown to `output` or prints to stdout.
pub fn run_plan(
    repo_path: &Path,
    project_name: &str,
    confidence_threshold: f64,
    output: Option<&Path>,
) -> Result<()> {
    let entries = map_cmd::build_map(repo_path, 0.3)?; // collect low-conf too
    let plan = plan_generator::build_plan(
        project_name,
        &repo_path.display().to_string(),
        &entries,
        confidence_threshold,
    );
    let markdown = plan_generator::render_markdown(&plan);
    write_or_print(output, &markdown)?;
    print_summary(&plan, confidence_threshold);
    Ok(())
}

fn write_or_print(output: Option<&Path>, content: &str) -> Result<()> {
    match output {
        Some(path) => {
            std::fs::write(path, content)?;
            eprintln!("plan written to {}", path.display());
        }
        None => print!("{content}"),
    }
    Ok(())
}

fn print_summary(plan: &plan_generator::MigrationPlan, threshold: f64) {
    let functional = plan.phases.iter()
        .filter(|p| p.initial_status == plan_generator::PhaseStatus::Scaffolding)
        .count();
    let blocked = plan.phases.len() - functional;
    eprintln!(
        "\n{} phase(s) ready | {} blocked | {} unmatched (threshold {:.2}) | avg confidence {:.2}",
        functional,
        blocked,
        plan.unmatched_modules.len(),
        threshold,
        plan.total_confidence_avg,
    );
}
