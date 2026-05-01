//! kei-discover CLI — register / list / search / install / stats.
//!
//! Metadata-only: `install` flips the local `installed` flag but does
//! NOT fetch anything. Real federation (remote index, fetch, signature
//! verify) arrives in a future wave.
//!
//! Exit-code contract: 2 for validation / duplicate / not-found, 1 for
//! storage / IO, 0 on success (matches kei-entity-store convention).

use clap::{Parser, Subcommand};
use kei_discover::{
    list_available, mark_installed, open, register, search, stats, DiscoverError, Entry,
};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-discover", version, about = "Federated primitive discovery (stub)")]
struct Cli {
    #[arg(long)]
    db: Option<PathBuf>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Register {
        #[arg(long)]
        slug: String,
        #[arg(long)]
        author: String,
        #[arg(long)]
        url: String,
        #[arg(long, default_value = "")]
        description: String,
    },
    List,
    Search {
        query: String,
    },
    Install {
        #[arg(long)]
        id: i64,
    },
    Stats,
}

fn db_path(cli_db: Option<PathBuf>) -> PathBuf {
    if let Some(p) = cli_db {
        return p;
    }
    if let Ok(e) = std::env::var("KEI_DISCOVER_DB") {
        return PathBuf::from(e);
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/discover/index.sqlite")
}

fn run() -> Result<(), DiscoverError> {
    let cli = Cli::parse();
    let store = open(&db_path(cli.db))?;
    dispatch(&store, cli.cmd)
}

fn dispatch(store: &kei_discover::Store, cmd: Cmd) -> Result<(), DiscoverError> {
    match cmd {
        Cmd::Register { slug, author, url, description } => {
            cmd_register(store, &slug, &author, &url, &description)
        }
        Cmd::List => cmd_list(store),
        Cmd::Search { query } => cmd_search(store, &query),
        Cmd::Install { id } => cmd_install(store, id),
        Cmd::Stats => cmd_stats(store),
    }
}

fn cmd_register(
    store: &kei_discover::Store,
    slug: &str,
    author: &str,
    url: &str,
    desc: &str,
) -> Result<(), DiscoverError> {
    let id = register(store.conn(), slug, author, url, desc)?;
    println!("{id}");
    Ok(())
}

fn cmd_list(store: &kei_discover::Store) -> Result<(), DiscoverError> {
    for e in list_available(store.conn())? {
        print_entry(&e);
    }
    Ok(())
}

fn cmd_search(store: &kei_discover::Store, query: &str) -> Result<(), DiscoverError> {
    for e in search(store.conn(), query)? {
        print_entry(&e);
    }
    Ok(())
}

fn cmd_install(store: &kei_discover::Store, id: i64) -> Result<(), DiscoverError> {
    mark_installed(store.conn(), id)?;
    println!("installed {id}");
    Ok(())
}

fn cmd_stats(store: &kei_discover::Store) -> Result<(), DiscoverError> {
    let s = stats(store.conn())?;
    println!("total={} installed={} available={}", s.total, s.installed, s.available);
    Ok(())
}

fn print_entry(e: &Entry) {
    let flag = if e.installed { "I" } else { "-" };
    println!("{}\t{}\t{}\t{}\t{}", e.id, flag, e.slug, e.author, e.description);
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(e.exit_code())
        }
    }
}
