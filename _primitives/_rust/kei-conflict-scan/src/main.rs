//! kei-conflict-scan — binary entry point.
//!
//! See lib.rs for overview. CLI spec:
//!   kei-conflict-scan --path <root> [--format json|human] [--only rules|hooks|blocks|orphans|cp]
//!
//! Exit codes:
//!   0 — scan completed (hits or no hits)
//!   1 — usage / I/O error
//!   2 — hits found AND --exit-on-hit set

use clap::{Parser, ValueEnum};
use kei_conflict_scan::scanners::{blocks, cp, hooks, orphans, rules};
use kei_conflict_scan::Conflict;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(name = "kei-conflict-scan", about = "Deep-sleep conflict scanner.")]
struct Cli {
    /// Root directory to scan (e.g. ~/.claude or a cloned memory repo).
    #[arg(long)]
    path: PathBuf,

    /// Output format.
    #[arg(long, value_enum, default_value_t = Format::Json)]
    format: Format,

    /// Only run one category; default = run all.
    #[arg(long, value_enum)]
    only: Option<Only>,

    /// Exit 2 if any conflict is reported.
    #[arg(long)]
    exit_on_hit: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Format {
    Json,
    Human,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Only {
    Rules,
    Hooks,
    Blocks,
    Orphans,
    Cp,
}

fn run_all(root: &std::path::Path, only: Option<Only>) -> Vec<Conflict> {
    let mut out = Vec::new();
    if matches!(only, None | Some(Only::Rules)) {
        out.extend(rules::scan(root));
    }
    if matches!(only, None | Some(Only::Hooks)) {
        out.extend(hooks::scan(root));
    }
    if matches!(only, None | Some(Only::Blocks)) {
        out.extend(blocks::scan(root));
    }
    if matches!(only, None | Some(Only::Orphans)) {
        out.extend(orphans::scan(root));
    }
    if matches!(only, None | Some(Only::Cp)) {
        out.extend(cp::scan(root));
    }
    out
}

fn emit_json(hits: &[Conflict]) {
    let wrapper = serde_json::json!({
        "hit_count": hits.len(),
        "conflicts": hits,
    });
    println!("{}", serde_json::to_string_pretty(&wrapper).unwrap());
}

fn emit_human(hits: &[Conflict]) {
    if hits.is_empty() {
        println!("no conflicts found.");
        return;
    }
    println!("{} conflict(s):", hits.len());
    for h in hits {
        println!(
            "  [{}][{:?}] {} — files: {}",
            h.category.as_str(),
            h.severity,
            h.evidence,
            h.files.join(", ")
        );
        println!("    fix: {}", h.suggested_fix);
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    if !cli.path.exists() {
        eprintln!("kei-conflict-scan: path not found: {}", cli.path.display());
        return ExitCode::from(1);
    }
    let hits = run_all(&cli.path, cli.only);
    match cli.format {
        Format::Json => emit_json(&hits),
        Format::Human => emit_human(&hits),
    }
    if cli.exit_on_hit && !hits.is_empty() {
        ExitCode::from(2)
    } else {
        ExitCode::SUCCESS
    }
}
