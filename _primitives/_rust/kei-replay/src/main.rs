//! kei-replay — CLI dispatcher.
//!
//! Commands:
//!   kei-replay <dna>            — reconstruct; print task.toml + prompt
//!   kei-replay <dna> --verify   — also fail non-zero on body-hash drift
//!   kei-replay diff <a> <b>     — compare two DNAs, print facet report

use clap::{Parser, Subcommand};
use kei_replay::{diff, ledger_lookup, replay};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "kei-replay",
    version,
    about = "Reconstruct agent spawn from DNA — replay / verify / diff"
)]
struct Cli {
    /// Override ledger DB path (default: $KEI_LEDGER_DB or ~/.claude/agents/ledger.sqlite)
    #[arg(long, global = true)]
    db: Option<PathBuf>,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Reconstruct the spawn for a DNA string.
    Replay {
        /// DNA string: role::caps::scope::body-nonce
        dna: String,
        /// Repo root holding _roles/ and _capabilities/ (default: cwd)
        #[arg(long)]
        kit_root: Option<PathBuf>,
        /// Explicit task.toml path (skips ledger lookup for the file path)
        #[arg(long)]
        task: Option<PathBuf>,
        /// Fail with exit 2 when recomputed body hash differs from DNA.
        #[arg(long)]
        verify: bool,
    },
    /// Diff two DNA strings facet-by-facet.
    Diff { left: String, right: String },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Replay { dna, kit_root, task, verify } => {
            run_replay(cli.db, dna, kit_root, task, verify)
        }
        Cmd::Diff { left, right } => run_diff(left, right),
    }
}

fn run_replay(
    db_cli: Option<PathBuf>,
    dna: String,
    kit_root: Option<PathBuf>,
    task: Option<PathBuf>,
    verify: bool,
) -> ExitCode {
    let db = db_cli.unwrap_or_else(ledger_lookup::default_db_path);
    let kit = kit_root.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| ".".into()));
    let result = replay::replay(&db, &dna, task.as_deref(), &kit);
    let r = match result {
        Ok(r) => r,
        Err(e) => {
            eprintln!("replay failed: {e}");
            return ExitCode::from(1);
        }
    };
    print_replay(&r);
    if verify && !r.body_hash_matches {
        eprintln!(
            "DRIFT: DNA body_hash={} but recomputed={} — task.toml differs from original spawn",
            r.dna.body_hash, r.recomputed_body_hash
        );
        return ExitCode::from(2);
    }
    ExitCode::SUCCESS
}

fn print_replay(r: &replay::Replay) {
    println!("=== task.toml ===");
    print!("{}", r.task_toml_text);
    if !r.task_toml_text.ends_with('\n') {
        println!();
    }
    println!("=== composed prompt ===");
    println!("{}", r.composed_prompt);
    println!("=== integrity ===");
    println!("dna.body_hash        = {}", r.dna.body_hash);
    println!("recomputed body_hash = {}", r.recomputed_body_hash);
    println!(
        "match                = {}",
        if r.body_hash_matches { "yes" } else { "NO (drift)" }
    );
}

fn run_diff(left: String, right: String) -> ExitCode {
    match diff::diff(&left, &right) {
        Ok(d) => {
            println!("{}", d.render());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("diff failed: {e}");
            ExitCode::from(1)
        }
    }
}
