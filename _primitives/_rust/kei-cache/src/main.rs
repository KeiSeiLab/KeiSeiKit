//! kei-cache CLI dispatcher.
//!
//! Constructor Pattern: single cube = arg parsing + dispatch + formatting.
//! Storage: `~/.claude/cache/cache.sqlite` (or `$KEI_CACHE_DB` override).

use clap::{Parser, Subcommand};
use kei_cache::{store, wrap_with, Outcome, SubprocessExecutor};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-cache", version, about = "Atom result cache")]
struct Cli {
    /// Override cache DB path (default: $KEI_CACHE_DB or ~/.claude/cache/cache.sqlite)
    #[arg(long)]
    db: Option<PathBuf>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Wrap an atom invocation with deterministic caching.
    Wrap {
        /// Atom id (e.g. `kei-router:route`).
        atom_id: String,
        /// JSON-string input to hash + forward on miss.
        #[arg(long)]
        input: String,
        /// TTL in seconds (default: 3600).
        #[arg(long, default_value_t = 3600)]
        ttl: i64,
        /// Atoms-root for discovery (default: $KEI_ATOMS_ROOT or cwd).
        #[arg(long)]
        atoms_root: Option<PathBuf>,
    },
    /// Print hit/miss + live entry counts.
    Stats,
    /// Evict all expired entries.
    Purge,
    /// Wipe cache + counters.
    Clear,
}

fn db_path(cli_db: Option<PathBuf>) -> PathBuf {
    if let Some(p) = cli_db {
        return p;
    }
    if let Ok(env) = std::env::var("KEI_CACHE_DB") {
        return PathBuf::from(env);
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/cache/cache.sqlite")
}

fn atoms_root(flag: Option<PathBuf>) -> PathBuf {
    flag.or_else(|| std::env::var("KEI_ATOMS_ROOT").ok().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."))
}

fn err(msg: &str) -> ExitCode {
    eprintln!("kei-cache: {msg}");
    ExitCode::from(1)
}

fn cmd_wrap(
    conn: &rusqlite::Connection,
    atom_id: &str,
    input: &str,
    ttl: i64,
    atoms_root: PathBuf,
) -> ExitCode {
    let executor = SubprocessExecutor::new(atoms_root);
    match wrap_with(conn, &executor, atom_id, input, ttl) {
        Ok((payload, Outcome::Hit)) => {
            eprintln!("cache=hit");
            println!("{payload}");
            ExitCode::SUCCESS
        }
        Ok((payload, Outcome::Miss)) => {
            eprintln!("cache=miss");
            println!("{payload}");
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("wrap failed: {e:#}")),
    }
}

fn cmd_stats(conn: &rusqlite::Connection) -> ExitCode {
    match store::stats(conn) {
        Ok(s) => {
            println!(
                "hits={} misses={} entries={} bytes={}",
                s.hits, s.misses, s.entries, s.bytes
            );
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("stats failed: {e:#}")),
    }
}

fn cmd_purge(conn: &rusqlite::Connection) -> ExitCode {
    match store::purge(conn) {
        Ok(n) => {
            println!("purged={n}");
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("purge failed: {e:#}")),
    }
}

fn cmd_clear(conn: &rusqlite::Connection) -> ExitCode {
    match store::clear(conn) {
        Ok(n) => {
            println!("cleared={n}");
            ExitCode::SUCCESS
        }
        Err(e) => err(&format!("clear failed: {e:#}")),
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let path = db_path(cli.db);
    let conn = match store::open(&path) {
        Ok(c) => c,
        Err(e) => return err(&format!("open {}: {e:#}", path.display())),
    };
    match cli.cmd {
        Cmd::Wrap { atom_id, input, ttl, atoms_root: ar } => {
            cmd_wrap(&conn, &atom_id, &input, ttl, atoms_root(ar))
        }
        Cmd::Stats => cmd_stats(&conn),
        Cmd::Purge => cmd_purge(&conn),
        Cmd::Clear => cmd_clear(&conn),
    }
}
