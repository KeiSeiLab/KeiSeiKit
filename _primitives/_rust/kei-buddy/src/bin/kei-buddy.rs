// SPDX-License-Identifier: Apache-2.0
//! kei-buddy binary entry point.
//!
//! Scaffold — the `serve` subcommand is a no-op stub until the
//! Telegram webhook driver and memory layer are wired in.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "kei-buddy",
    about = "KeiBuddy personal-assistant bot (scaffold)",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Start the Telegram webhook listener (not yet implemented).
    Serve,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Serve => {
            println!("kei-buddy serve: not yet implemented, scaffold only");
        }
    }
}
