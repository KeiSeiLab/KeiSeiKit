use clap::{Parser, Subcommand};
use kei_ledger_sign::cli::{cmd_keygen, cmd_sign, cmd_verify};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-ledger-sign")]
#[command(about = "ed25519 signing of ledger rows", long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Keygen {
        #[arg(long)]
        out: PathBuf,
    },
    Sign {
        #[arg(long)]
        key: PathBuf,
        #[arg(long)]
        dna: String,
        #[arg(long = "spec-sha")]
        spec_sha: String,
        #[arg(long = "creator-id")]
        creator_id: String,
    },
    Verify {
        #[arg(long)]
        pubkey: String,
        #[arg(long)]
        dna: String,
        #[arg(long = "spec-sha")]
        spec_sha: String,
        #[arg(long = "creator-id")]
        creator_id: String,
        #[arg(long)]
        sig: String,
    },
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.cmd {
        Cmd::Keygen { out } => cmd_keygen(&out)?,
        Cmd::Sign {
            key,
            dna,
            spec_sha,
            creator_id,
        } => cmd_sign(&key, &dna, &spec_sha, &creator_id)?,
        Cmd::Verify {
            pubkey,
            dna,
            spec_sha,
            creator_id,
            sig,
        } => cmd_verify(&pubkey, &dna, &spec_sha, &creator_id, &sig)?,
    }
    Ok(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {}", e);
            ExitCode::from(1)
        }
    }
}
