//! kei-search-core CLI.

use clap::{Parser, Subcommand, ValueEnum};
use kei_search_core::export::{export, Format};
use kei_search_core::fetch::StubFetcher;
use kei_search_core::pipeline::run_research;
use kei_search_core::ResearchStore;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-search-core", version)]
struct Cli {
    #[arg(long)] db: Option<PathBuf>,
    #[command(subcommand)] cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Run { prompt: String,
          #[arg(long, default_value_t = 1_000_000)] budget: i64 }, // 1 USD
    Stop { id: i64 },
    Export { id: i64, #[arg(long, value_enum, default_value_t = Fmt::Md)] format: Fmt },
}

#[derive(Clone, Copy, ValueEnum)]
enum Fmt { Md, Json }

fn db_path(o: Option<PathBuf>) -> PathBuf {
    if let Some(p) = o { return p; }
    if let Ok(e) = std::env::var("KEI_SEARCH_DB") { return PathBuf::from(e); }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/search/research.sqlite")
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let s = ResearchStore::open(&db_path(cli.db))?;
    match cli.cmd {
        Cmd::Run { prompt, budget } => {
            let id = run_research(&s, &StubFetcher, &prompt, budget)?;
            println!("{}", id);
        }
        Cmd::Stop { id } => {
            s.set_status(id, "stopped")?;
            println!("stopped {}", id);
        }
        Cmd::Export { id, format } => {
            let f = match format { Fmt::Md => Format::Markdown, Fmt::Json => Format::Json };
            println!("{}", export(&s, id, f)?);
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => { eprintln!("kei-search-core: {e:#}"); ExitCode::from(1) }
    }
}
