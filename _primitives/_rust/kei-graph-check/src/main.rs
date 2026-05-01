//! kei-graph-check — binary entry.
//!
//! Exit 0 if all refs resolve; exit 2 if any broken. Useful as a gate
//! BEFORE the orchestrator commits the deep-sleep fork branch.

use clap::Parser;
use kei_graph_check::{graph::Graph, patch_advisory};
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(name = "kei-graph-check", about = "Post-refactor graph-integrity gate.")]
struct Cli {
    /// Root directory (e.g. memory-repo clone).
    #[arg(long)]
    path: PathBuf,

    /// Optional patch file — any `+++ /dev/null` removal or `# removed: <p>`
    /// header is treated as a phantom-removed file for the check.
    #[arg(long)]
    after_diff: Option<PathBuf>,

    /// JSON output (default is human).
    #[arg(long)]
    json: bool,
}

fn emit_human(broken: &[kei_graph_check::graph::BrokenRef]) {
    if broken.is_empty() {
        println!("kei-graph-check: graph ok (no broken references).");
        return;
    }
    println!("kei-graph-check: {} broken reference(s):", broken.len());
    for b in broken {
        println!("  {}:{}  [{}]  -> '{}'", b.source, b.line, b.kind, b.target);
    }
}

fn emit_json(broken: &[kei_graph_check::graph::BrokenRef]) {
    let v = serde_json::json!({ "broken_count": broken.len(), "broken": broken });
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    if !cli.path.exists() {
        eprintln!("kei-graph-check: path not found: {}", cli.path.display());
        return ExitCode::from(1);
    }
    let removed: HashSet<String> = match cli.after_diff.as_ref() {
        Some(p) if p.exists() => patch_advisory::parse_removals(p),
        _ => HashSet::new(),
    };
    let graph = Graph::index(&cli.path);
    let broken = graph.check(&cli.path, &removed);

    if cli.json {
        emit_json(&broken);
    } else {
        emit_human(&broken);
    }
    if broken.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(2)
    }
}
