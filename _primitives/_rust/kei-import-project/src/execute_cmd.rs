//! execute_cmd — CLI orchestrator for Phase 5: migration plan execution.
//!
//! Composes plan_parser → executor → phase_prompt → output.
//! Constructor Pattern: one responsibility, ≤100 LOC, ≤30 LOC per fn.

use crate::{executor, phase_prompt, plan_parser};
use anyhow::Result;
use std::path::Path;

/// Run the `execute` subcommand end-to-end.
///
/// Parses `plan_path`, builds the executor plan, optionally pre-registers
/// phases in kei-ledger, and prints a summary table (markdown or JSON).
pub fn run_execute(
    plan_path: &Path,
    ledger_db: Option<&Path>,
    prereg: bool,
    format: &str,
) -> Result<()> {
    let parsed = plan_parser::parse_plan_file(plan_path)?;
    eprintln!(
        "parsed {} phase(s) from {}",
        parsed.phases.len(),
        plan_path.display()
    );

    let exec_plan = executor::build_executor_plan(&parsed, ledger_db)?;

    if prereg {
        let db = ledger_db
            .map(|p| p.to_path_buf())
            .or_else(|| std::env::var("KEI_LEDGER_DB").ok().map(std::path::PathBuf::from))
            .unwrap_or_else(default_ledger_path);
        executor::prereg_phases(&exec_plan, &db)?;
        eprintln!("pre-registered {} phase(s) in {}", exec_plan.records.len(), db.display());
    }

    match format {
        "json" => print_json(&exec_plan.prompts),
        _ => print_markdown(&exec_plan),
    }
    Ok(())
}

fn print_json(prompts: &[phase_prompt::PhasePrompt]) -> () {
    match phase_prompt::render_json(prompts) {
        Ok(s) => println!("{s}"),
        Err(e) => eprintln!("json render failed: {e}"),
    }
}

fn print_markdown(plan: &executor::ExecutorPlan) -> () {
    println!("| Phase | Trait | Modules | Agent type | Status |");
    println!("|---|---|---:|---|---|");
    for (rec, prompt) in plan.records.iter().zip(plan.prompts.iter()) {
        println!(
            "| {} | {} | {} | {} | {:?} |",
            rec.phase_id,
            prompt.trait_family,
            prompt.modules.len(),
            prompt.agent_type,
            rec.status,
        );
    }
    println!();
    println!("To spawn agents:");
    println!("- Pipe each phase's `prompt_text` to `Agent({{subagent_type: ..., prompt: ...}})`");
    println!("- After each agent returns: `kei-ledger done <row-id>` (or `fail`)");
}

fn default_ledger_path() -> std::path::PathBuf {
    std::env::var("HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join(".claude")
        .join("agents")
        .join("ledger.sqlite")
}
