//! kei-store — binary entry.
//!
//! Subcommands: init / read / write / list / branch / commit / push / status.

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use kei_store::config::{expand_tilde, Config};
use kei_store::{build_store, MemoryStore};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(name = "kei-store", about = "Memory-repo backend abstraction.")]
struct Cli {
    /// Config file path (default: ~/.claude/agents/_primitives/store-config.toml).
    #[arg(long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    Init { backend: String, #[arg(long)] url: Option<String> },
    Read { path: String },
    Write { path: String, file: PathBuf },
    List { dir: String },
    Branch { name: String },
    Commit { #[arg(long, short)] message: String },
    Push { branch: String },
    Pull { branch: String },
    Status,
}

fn default_config_path() -> PathBuf {
    PathBuf::from(expand_tilde(
        "~/.claude/agents/_primitives/store-config.toml",
    ))
}

fn load_config(cli: &Cli) -> Result<Config> {
    let path = cli.config.clone().unwrap_or_else(default_config_path);
    if !path.exists() {
        return Err(anyhow!("config not found: {}", path.display()));
    }
    Config::load(&path)
}

fn cmd_init(backend: &str, url: Option<&str>, target: &PathBuf) -> Result<()> {
    if target.exists() {
        return Err(anyhow!("config already exists: {}", target.display()));
    }
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(target, render_init(backend, url))?;
    eprintln!("kei-store: wrote {}", target.display());
    Ok(())
}

fn render_init(backend: &str, url: Option<&str>) -> String {
    let u = url.unwrap_or("<set-me>");
    format!(
        "[active]\nbackend = \"{b}\"\nlocal_path = \"~/.claude/memory/sync-repo\"\n\n\
         [{b}]\nurl = \"{u}\"\nssh_key_env = \"KEI_MEMORY_SSH_KEY\"\npat_env = \"KEI_MEMORY_PAT\"\n",
        b = backend,
        u = u
    )
}

fn run(cli: &Cli) -> Result<()> {
    if let Cmd::Init { backend, url } = &cli.cmd {
        let target = cli.config.clone().unwrap_or_else(default_config_path);
        return cmd_init(backend, url.as_deref(), &target);
    }
    let cfg = load_config(cli)?;
    let store = build_store(&cfg)?;
    dispatch(&*store, &cli.cmd)
}

fn dispatch(store: &dyn MemoryStore, cmd: &Cmd) -> Result<()> {
    match cmd {
        Cmd::Read { path } => {
            let bytes = store.read(path)?;
            std::io::Write::write_all(&mut std::io::stdout(), &bytes).context("write stdout")?;
        }
        Cmd::Write { path, file } => {
            let bytes = fs::read(file)?;
            store.write(path, &bytes)?;
        }
        Cmd::List { dir } => {
            for name in store.list(dir)? {
                println!("{}", name);
            }
        }
        Cmd::Branch { name } => store.branch(name)?,
        Cmd::Commit { message } => println!("{}", store.commit(message)?),
        Cmd::Push { branch } => store.push(branch)?,
        Cmd::Pull { branch } => store.pull(branch)?,
        Cmd::Status => println!("backend: {}", store.backend_name()),
        Cmd::Init { .. } => unreachable!(),
    }
    Ok(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(&cli) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("kei-store: {e:#}");
            ExitCode::from(1)
        }
    }
}
