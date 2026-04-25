//! kei-artifact CLI — register-schema / emit / get / list / validate / chain.
//!
//! Constructor Pattern: main.rs = dispatch only. Each `cmd_*` fn < 30 LOC.
//! Artifact-CRUD command bodies live in the `cli_cmds` sibling module.

mod cli_cmds;

use clap::{Parser, Subcommand};
use kei_artifact::artifact::{list_schemas, register_schema, validate_by_id};
use kei_artifact::export::write as export_write;
use kei_artifact::schemas::register_builtins;
use kei_artifact::Store;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-artifact", version, about = "Typed artifact handoff store")]
struct Cli {
    #[arg(long)]
    db: Option<PathBuf>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Initialise the db and register the 5 built-in schemas.
    Init,
    /// Register a JSON Schema file under a name. Also refreshes the export
    /// consumed by the assembler (see `ExportSchemas`).
    RegisterSchema {
        #[arg(long)] name: String,
        #[arg(long)] path: PathBuf,
    },
    /// List all registered schema names, one per line (plain text).
    ListSchemas,
    /// v0.16: write the current schema-name list as JSON so the assembler's
    /// manifest validator can accept custom-registered schemas without a
    /// rebuild. Default path: `~/.claude/agents/artifacts/schemas.json`.
    ExportSchemas {
        #[arg(long)] path: Option<PathBuf>,
    },
    /// Emit an artifact. Content file must be JSON.
    Emit {
        #[arg(long)] schema: String,
        #[arg(long)] from: String,
        #[arg(long)] content: PathBuf,
        #[arg(long)] meta: Vec<String>,
        #[arg(long)] parent: Option<String>,
    },
    /// Fetch an artifact by id.
    Get {
        id: String,
        #[arg(long, default_value = "typed")] format: String,
    },
    /// List artifacts; filter by schema / source / since-seconds.
    List {
        #[arg(long)] schema: Option<String>,
        #[arg(long)] from: Option<String>,
        #[arg(long)] since: Option<String>,
    },
    /// Re-validate a stored artifact against its schema.
    Validate { id: String },
    /// Walk the parent-handoff chain.
    Chain { id: String },
}

fn db_path(o: Option<PathBuf>) -> PathBuf {
    if let Some(p) = o {
        return p;
    }
    if let Ok(e) = std::env::var("KEI_ARTIFACT_DB") {
        return PathBuf::from(e);
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/artifacts/artifacts.sqlite")
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let store = Store::open(&db_path(cli.db))?;
    dispatch(&store, cli.cmd)
}

fn dispatch(store: &Store, cmd: Cmd) -> anyhow::Result<()> {
    match cmd {
        Cmd::Init => cmd_init(store),
        Cmd::RegisterSchema { name, path } => cmd_register(store, &name, &path),
        Cmd::ListSchemas => cmd_list_schemas(store),
        Cmd::ExportSchemas { path } => cmd_export_schemas(store, path.as_deref()),
        Cmd::Emit { schema, from, content, meta, parent } => {
            cli_cmds::cmd_emit(store, &schema, &from, &content, &meta, parent.as_deref())
        }
        Cmd::Get { id, format } => cli_cmds::cmd_get(store, &id, &format),
        Cmd::List { schema, from, since } => {
            cli_cmds::cmd_list(store, schema.as_deref(), from.as_deref(), since.as_deref())
        }
        Cmd::Validate { id } => validate_by_id(store, &id),
        Cmd::Chain { id } => cli_cmds::cmd_chain(store, &id),
    }
}

fn cmd_init(store: &Store) -> anyhow::Result<()> {
    register_builtins(store)?;
    println!("registered 5 built-in schemas: spec, plan, patch, review, research");
    Ok(())
}

fn cmd_register(store: &Store, name: &str, path: &std::path::Path) -> anyhow::Result<()> {
    let text = std::fs::read_to_string(path)?;
    register_schema(store, name, &text)?;
    println!("registered schema '{name}'");
    // Best-effort auto-refresh the export so the assembler sees the new
    // schema without a manual `export-schemas` call. A write failure
    // (perm / missing parent) is non-fatal: register succeeded.
    if let Err(e) = cmd_export_schemas(store, None) {
        eprintln!("warning: export refresh failed: {e}");
    }
    Ok(())
}

fn cmd_list_schemas(store: &Store) -> anyhow::Result<()> {
    for name in list_schemas(store)? {
        println!("{name}");
    }
    Ok(())
}

fn cmd_export_schemas(store: &Store, override_path: Option<&std::path::Path>) -> anyhow::Result<()> {
    let (count, target) = export_write(store, override_path)?;
    println!("exported {count} schemas → {}", target.display());
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("kei-artifact: {e:#}");
            ExitCode::from(1)
        }
    }
}
