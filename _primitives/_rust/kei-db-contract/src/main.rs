//! kei-db-contract — CLI entrypoint.
//!
//! Diffs SQL migrations against TypeScript types in a project root.
//! Exit 0 when --strict not set OR no drift; exit 1 in --strict with drift; exit 2 on I/O error.

use clap::{Parser, ValueEnum};
use kei_db_contract::diff::diff_project;
use kei_db_contract::output::{render_json, render_text};
use kei_db_contract::sql_parse::parse_migrations_dir;
use kei_db_contract::ts_parse::parse_ts_glob;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "kei-db-contract",
    about = "Diff SQL migrations against TypeScript types to catch drift."
)]
struct Cli {
    /// Project root.
    project_root: PathBuf,
    /// Migrations directory (relative to project root).
    #[arg(long, default_value = "migrations")]
    migrations_dir: PathBuf,
    /// TS source root (relative to project root). Walked recursively for `.ts`/`.tsx`.
    #[arg(long, default_value = "src")]
    types_dir: PathBuf,
    /// Output format.
    #[arg(long, value_enum, default_value_t = Format::Text)]
    output: Format,
    /// Exit 1 if drift_count > 0.
    #[arg(long)]
    strict: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Format {
    Text,
    Json,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("kei-db-contract: error: {:?}", err);
            ExitCode::from(2)
        }
    }
}

fn run(cli: Cli) -> anyhow::Result<ExitCode> {
    let mig_dir = cli.project_root.join(&cli.migrations_dir);
    let ts_dir = cli.project_root.join(&cli.types_dir);
    let tables = parse_migrations_dir(&mig_dir)?;
    let ts_types = parse_ts_glob(&[ts_dir.as_path()])?;
    let report = diff_project(&tables, &ts_types);
    let text = match cli.output {
        Format::Text => render_text(&report),
        Format::Json => render_json(&report),
    };
    println!("{}", text);
    if cli.strict && report.drift_count > 0 {
        return Ok(ExitCode::from(1));
    }
    Ok(ExitCode::SUCCESS)
}
