//! kei-refactor-engine — binary entry.
//!
//! Usage:
//!   kei-refactor-engine --input conflicts.json --plan-only > plan.md
//!   kei-refactor-engine --input conflicts.json --apply-to-branch deep-sleep/2026-04-22 \
//!                       --plan-out plan.md --patch-out plan-autoresolve.md
//!
//! NOTE (v0.14.1): `--patch-out` writes a MARKDOWN review file, NOT a
//! unified diff. The old claim "git apply-ready patch" was retracted —
//! see `patch.rs` header. The flag name is kept for backwards-compat.

use anyhow::Result;
use clap::Parser;
use kei_refactor_engine::input::{read_conflicts, read_from_stdin};
use kei_refactor_engine::plan::Plan;
use kei_refactor_engine::{patch, render};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(name = "kei-refactor-engine", about = "Deep-sleep refactor-plan generator.")]
struct Cli {
    /// Input JSON file (output of kei-conflict-scan). Use `-` for stdin.
    #[arg(long)]
    input: Option<PathBuf>,

    /// Plan-only mode (default). Prints markdown to stdout if no --plan-out.
    #[arg(long, default_value_t = true)]
    plan_only: bool,

    /// Apply mode — also write an auto-resolve review file; takes the branch name.
    #[arg(long)]
    apply_to_branch: Option<String>,

    /// Optional explicit path for the markdown plan.
    #[arg(long)]
    plan_out: Option<PathBuf>,

    /// Optional explicit path for the auto-resolve review markdown
    /// (NOT a unified diff — see patch.rs header).
    #[arg(long)]
    patch_out: Option<PathBuf>,
}

fn load(cli: &Cli) -> Result<Vec<kei_refactor_engine::input::Conflict>> {
    match cli.input.as_deref() {
        None => read_from_stdin(),
        Some(p) if p.to_string_lossy() == "-" => read_from_stdin(),
        Some(p) => read_conflicts(p),
    }
}

fn write_plan(plan: &Plan, branch: Option<&str>, out: Option<&PathBuf>) -> Result<()> {
    let md = render::render(plan, branch);
    match out {
        Some(p) => std::fs::write(p, md)?,
        None => print!("{}", md),
    }
    Ok(())
}

fn maybe_write_autoresolve(
    plan: &Plan,
    branch: &str,
    out: Option<&PathBuf>,
) -> Result<usize> {
    let default = PathBuf::from("plan-autoresolve.md");
    let target = out.unwrap_or(&default);
    patch::write_autoresolve(plan, branch, target)
}

fn run(cli: &Cli) -> Result<ExitCode> {
    let conflicts = load(cli)?;
    let plan = Plan::from_conflicts(&conflicts);
    let branch = cli.apply_to_branch.as_deref();

    write_plan(&plan, branch, cli.plan_out.as_ref())?;

    if let Some(br) = branch {
        let applied = maybe_write_autoresolve(&plan, br, cli.patch_out.as_ref())?;
        eprintln!(
            "kei-refactor-engine: wrote auto-resolve review with {} auto-apply item(s); \
             {} human-decision item(s) excluded. Review manually — this is NOT a unified diff.",
            applied,
            plan.manual_items().len(),
        );
    }
    Ok(ExitCode::SUCCESS)
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(&cli) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("kei-refactor-engine: {e}");
            ExitCode::from(1)
        }
    }
}
