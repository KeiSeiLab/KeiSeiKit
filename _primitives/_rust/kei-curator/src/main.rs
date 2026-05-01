use clap::{Parser, Subcommand};
use kei_curator::{decay_edges, prune_orphans, Config};
use rusqlite::Connection;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-curator", version)]
struct Cli {
    #[arg(long)] db: PathBuf,
    #[command(subcommand)] cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Decay { #[arg(long, default_value_t = 0.05)] default_lambda: f64,
            #[arg(long, default_value_t = 0.1)] threshold: f64 },
    PruneOrphans,
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let conn = Connection::open(&cli.db)?;
    match cli.cmd {
        Cmd::Decay { default_lambda, threshold } => {
            let mut cfg = Config::default();
            cfg.default_lambda = default_lambda;
            cfg.prune_threshold = threshold;
            let r = decay_edges(&conn, &cfg)?;
            println!("updated={} pruned={}", r.updated, r.pruned);
        }
        Cmd::PruneOrphans => {
            let n = prune_orphans(&conn)?;
            println!("removed {} orphan edges", n);
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => { eprintln!("kei-curator: {e:#}"); ExitCode::from(1) }
    }
}
