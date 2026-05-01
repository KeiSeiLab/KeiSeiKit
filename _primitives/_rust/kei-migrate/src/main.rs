//! kei-migrate CLI entry. Dispatches to the library surface in `lib.rs`.

use anyhow::Result;
use clap::Parser;
use kei_migrate::cli::{Cli, Command};
use kei_migrate::{cmd_create, do_down, do_status, do_up};
use std::path::Path;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let dir = Path::new(&cli.dir);
    match cli.command {
        Command::Up => {
            let n = do_up(&cli.database_url, dir).await?;
            println!("up: {} migration(s) applied", n);
        }
        Command::Down { n } => {
            let r = do_down(&cli.database_url, dir, n).await?;
            println!("down: {} migration(s) reverted", r);
        }
        Command::Status => {
            do_status(&cli.database_url, dir).await?;
        }
        Command::Create { name } => {
            cmd_create::run(dir, &name)?;
        }
    }
    Ok(())
}
