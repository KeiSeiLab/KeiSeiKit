//! keisei — exobrain attach/status CLI (v0.22 multi-brain + Auto scope).
//!
//! Constructor Pattern: main.rs = clap parse + dispatch only. All
//! subcommand logic lives in sibling modules
//! (`attach.rs`, `status.rs`, `mount.rs`, `detach.rs`, `list.rs`).

mod adapter;
mod adapters;
mod attach;
mod brain;
mod brain_validate;
mod config;
mod config_migrate;
mod detach;
mod display;
mod error;
mod fs_type;
mod fsx;
mod list;
mod mount;
mod paths;
mod scope;
mod status;
mod time;

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use std::process::ExitCode;

use crate::scope::Scope;

#[derive(Parser)]
#[command(
    name = "keisei",
    version,
    about = "Exobrain CLI — mount a portable brain into any supported AI client"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum ScopeArg {
    /// Host-wide config (`~/.claude/...`, `~/.cursor/...`).
    User,
    /// Project-local config (`./.claude/...`, `./.cursor/...`).
    Project,
    /// Let the adapter pick based on CWD (v0.22 default). `./.claude/`
    /// present → project; otherwise user.
    Auto,
}

impl From<ScopeArg> for Scope {
    fn from(value: ScopeArg) -> Self {
        match value {
            ScopeArg::User => Scope::User,
            ScopeArg::Project => Scope::Project,
            ScopeArg::Auto => Scope::Auto,
        }
    }
}

#[derive(Subcommand)]
enum Cmd {
    /// Attach a brain to the single currently detected AI client.
    Attach {
        /// Path to the brain directory (must contain manifest.toml).
        brain_path: PathBuf,
        /// Which client config to write — host-wide (`user`), project-local
        /// (`project`), or `auto` (v0.22 default: inferred from CWD).
        /// Adapters that don't support the requested scope error out cleanly.
        #[arg(long, value_enum, default_value_t = ScopeArg::Auto)]
        scope: ScopeArg,
    },
    /// Attach a brain to EVERY detected AI client in one shot.
    Mount {
        /// Path to the brain directory (must contain manifest.toml).
        brain_path: PathBuf,
    },
    /// Remove the brain from every client recorded in the marker.
    Detach,
    /// Show the currently attached brain + health checks.
    Status,
    /// List every registered adapter + whether it's detected here.
    ListAdapters,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let res = match cli.cmd {
        Cmd::Attach { brain_path, scope } => attach::run(&brain_path, scope.into()),
        Cmd::Mount { brain_path } => mount::run(&brain_path),
        Cmd::Detach => detach::run(),
        Cmd::Status => status::run(),
        Cmd::ListAdapters => list::run(),
    };
    match res {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("keisei: {e}");
            ExitCode::from(1)
        }
    }
}
