//! CLI entry — dispatches to one of four subcommand handlers.
//!
//! Constructor Pattern: this file stays ≤30 LOC. Per-subcommand handlers
//! live in `cli.rs::handlers`.

use clap::Parser;

use kei_llm_router::cli::{handlers, Cli, Command};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let exit = match cli.command {
        Command::Probe(args) => handlers::run_probe(args),
        Command::Route(args) => handlers::run_route(args).await,
        Command::ListBackends => handlers::run_list_backends().await,
        Command::Which(args) => handlers::run_which(args).await,
    };
    std::process::exit(exit);
}
