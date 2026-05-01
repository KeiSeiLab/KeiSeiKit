//! kei-llm-llamacpp — CLI dispatcher (thin).
//!
//! Each subcommand maps to one helper in `dispatch.rs`. Errors flow
//! through `Error::exit_code()` so the harness sees the canonical
//! exit-code surface.

mod dispatch;

use clap::Parser;
use kei_llm_llamacpp::cli::Cli;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    dispatch::run(cli.cmd).await
}
