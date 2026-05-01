use clap::{Parser, Subcommand};
use kei_crossdomain::auto_link::auto_link;
use kei_crossdomain::bfs::bfs;
use kei_crossdomain::edges::{count_by_type, link, query_edges, unlink};
use kei_crossdomain::Store;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-crossdomain", version)]
struct Cli {
    #[arg(long)] db: Option<PathBuf>,
    #[command(subcommand)] cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Link { from: String, to: String,
           #[arg(long, default_value = "related")] edge_type: String,
           #[arg(long, default_value_t = 1.0)] weight: f64,
           #[arg(long, default_value = "E4")] evidence: String },
    Unlink { from: String, to: String,
             #[arg(long, default_value = "related")] edge_type: String },
    Query { node: String },
    Graph { start: String, #[arg(long, default_value_t = 2)] depth: i64 },
    AutoLink { node: String },
    Stats,
}

fn db_path(o: Option<PathBuf>) -> PathBuf {
    if let Some(p) = o { return p; }
    if let Ok(e) = std::env::var("KEI_CROSS_DB") { return PathBuf::from(e); }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/cross/cross.sqlite")
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let s = Store::open(&db_path(cli.db))?;
    dispatch(&s, cli.cmd)
}

fn dispatch(s: &Store, cmd: Cmd) -> anyhow::Result<()> {
    match cmd {
        Cmd::Link { from, to, edge_type, weight, evidence } =>
            cmd_link(s, &from, &to, &edge_type, weight, &evidence),
        Cmd::Unlink { from, to, edge_type } => cmd_unlink(s, &from, &to, &edge_type),
        Cmd::Query { node } => cmd_query(s, &node),
        Cmd::Graph { start, depth } => cmd_graph(s, &start, depth),
        Cmd::AutoLink { node } => cmd_auto(s, &node),
        Cmd::Stats => cmd_stats(s),
    }
}

fn cmd_link(s: &Store, from: &str, to: &str, et: &str, w: f64, ev: &str) -> anyhow::Result<()> {
    link(s, from, to, et, w, ev)?;
    println!("linked {} -> {}", from, to);
    Ok(())
}

fn cmd_unlink(s: &Store, from: &str, to: &str, et: &str) -> anyhow::Result<()> {
    let n = unlink(s, from, to, et)?;
    println!("removed {} edge(s)", n);
    Ok(())
}

fn cmd_query(s: &Store, node: &str) -> anyhow::Result<()> {
    for e in query_edges(s, node)? {
        println!("{}\t{} -[{}]-> {}", e.id, e.from_uri, e.edge_type, e.to_uri);
    }
    Ok(())
}

fn cmd_graph(s: &Store, start: &str, depth: i64) -> anyhow::Result<()> {
    for r in bfs(s, start, depth)? {
        println!("{}\t(depth {})\tvia {}", r.uri, r.depth, r.edge_type);
    }
    Ok(())
}

fn cmd_auto(s: &Store, node: &str) -> anyhow::Result<()> {
    let n = auto_link(s, node)?;
    println!("proposed+added {} edges", n);
    Ok(())
}

fn cmd_stats(s: &Store) -> anyhow::Result<()> {
    for (et, n) in count_by_type(s)? { println!("{}\t{}", n, et); }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => { eprintln!("kei-crossdomain: {e:#}"); ExitCode::from(1) }
    }
}
