//! kei-dna-index CLI — JSON stdout for all subcommands.

use clap::{Parser, Subcommand, ValueEnum};
use kei_dna_index::{
    adjacent, cluster_by, open_read_only, precedent, stats, AdjacencyKind, ClusterBy, Result,
};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "kei-dna-index", about = "Read-only adjacency/cluster/precedent over kei-ledger DNAs")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    Adjacent {
        #[arg(long)]
        dna: String,
        #[arg(long, value_enum, default_value_t = ByKind::All)]
        by: ByKind,
        #[arg(long, default_value_t = 10)]
        limit: usize,
        #[arg(long)]
        db: Option<PathBuf>,
    },
    Cluster {
        #[arg(long, value_enum)]
        by: ByCluster,
        #[arg(long)]
        db: Option<PathBuf>,
    },
    Precedent {
        #[arg(long)]
        body: String,
        #[arg(long, default_value = "all")]
        status: String,
        #[arg(long)]
        db: Option<PathBuf>,
    },
    Stats {
        #[arg(long)]
        db: Option<PathBuf>,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum ByKind {
    Scope,
    Body,
    Role,
    Temporal,
    All,
}

#[derive(ValueEnum, Clone, Debug)]
enum ByCluster {
    Scope,
    Body,
    Role,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Adjacent {
            dna,
            by,
            limit,
            db,
        } => run_adjacent(dna, by, limit, db),
        Cmd::Cluster { by, db } => run_cluster(by, db),
        Cmd::Precedent { body, status, db } => run_precedent(body, status, db),
        Cmd::Stats { db } => run_stats(db),
    }
}

fn run_adjacent(dna: String, by: ByKind, limit: usize, db: Option<PathBuf>) -> Result<()> {
    let conn = open_read_only(resolve_db(db))?;
    let kind = match by {
        ByKind::Scope => AdjacencyKind::Scope,
        ByKind::Body => AdjacencyKind::Body,
        ByKind::Role => AdjacencyKind::Role,
        ByKind::Temporal => AdjacencyKind::Temporal,
        ByKind::All => AdjacencyKind::All,
    };
    let out = adjacent(&conn, &dna, kind, limit)?;
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}

fn run_cluster(by: ByCluster, db: Option<PathBuf>) -> Result<()> {
    let conn = open_read_only(resolve_db(db))?;
    let by = match by {
        ByCluster::Scope => ClusterBy::Scope,
        ByCluster::Body => ClusterBy::Body,
        ByCluster::Role => ClusterBy::RoleCaps,
    };
    let out = cluster_by(&conn, by)?;
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}

fn run_precedent(body: String, status: String, db: Option<PathBuf>) -> Result<()> {
    let conn = open_read_only(resolve_db(db))?;
    let filter = if status == "all" { None } else { Some(status.as_str()) };
    let out = precedent(&conn, &body, filter)?;
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}

fn run_stats(db: Option<PathBuf>) -> Result<()> {
    let conn = open_read_only(resolve_db(db))?;
    let out = stats(&conn)?;
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}

fn resolve_db(explicit: Option<PathBuf>) -> PathBuf {
    if let Some(p) = explicit {
        return p;
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude").join("agents").join("ledger.sqlite")
}
