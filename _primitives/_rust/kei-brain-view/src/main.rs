//! kei-brain-view CLI entrypoint.
//!
//! Constructor Pattern: main.rs = argument parsing + dispatch. Each
//! subcommand calls into a library fn. No business logic inline.

use clap::{Parser, Subcommand};
use kei_brain_view::{
    build_graph, compute_stats, render_ascii, render_clusters, render_lineage,
    render_stats, render_summary, resolve_dna, BrainViewError, Result,
};
use kei_dna_index::ClusterBy;
use rusqlite::Connection;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "kei-brain-view",
    about = "Read-only visualizer of kei-ledger taxonomy + agent lineage (Wave 14)"
)]
struct Cli {
    /// Path to ledger.sqlite. Defaults to ~/.claude/agents/ledger.sqlite.
    #[arg(long, global = true)]
    db: Option<PathBuf>,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Print the full fork-tree as indented text (roots first, BFS).
    Tree,
    /// Bucket counts by status + has-dna.
    Stats,
    /// Print ancestors + descendants of the node matching a DNA prefix.
    Lineage {
        #[arg(long)]
        dna: String,
    },
    /// Group DNAs by scope / body / role+caps and print the cluster tree.
    Clusters {
        /// One of: scope | body | role
        #[arg(long)]
        by: String,
    },
    /// One-shot aggregate summary over the ledger DNAs.
    Summary,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("kei-brain-view: {e}");
            ExitCode::from(1)
        }
    }
}

fn run(cli: Cli) -> Result<()> {
    let db_path = cli.db.unwrap_or_else(default_db_path);
    let conn = Connection::open(&db_path)?;
    match cli.cmd {
        Cmd::Tree => {
            let graph = build_graph(&conn)?;
            print!("{}", render_ascii(&graph));
        }
        Cmd::Stats => {
            let graph = build_graph(&conn)?;
            print!("{}", render_stats(&compute_stats(&graph)));
        }
        Cmd::Lineage { dna } => {
            let graph = build_graph(&conn)?;
            let focus = resolve_dna(&graph, &dna)?.clone();
            let colored = std::env::var_os("NO_COLOR").is_none();
            print!("{}", render_lineage(&graph, &focus, colored)?);
        }
        Cmd::Clusters { by } => {
            let by = parse_cluster_by(&by)?;
            print!("{}", render_clusters(&conn, by)?);
        }
        Cmd::Summary => print!("{}", render_summary(&conn)?),
    }
    Ok(())
}

fn parse_cluster_by(raw: &str) -> Result<ClusterBy> {
    match raw.to_ascii_lowercase().as_str() {
        "scope" => Ok(ClusterBy::Scope),
        "body" => Ok(ClusterBy::Body),
        "role" | "rolecaps" | "role-caps" => Ok(ClusterBy::RoleCaps),
        other => Err(BrainViewError::DnaIndex(
            kei_dna_index::Error::MalformedDna(format!(
                "cluster key must be one of scope|body|role, got: {other}"
            )),
        )),
    }
}

fn default_db_path() -> PathBuf {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    home.join(".claude").join("agents").join("ledger.sqlite")
}
