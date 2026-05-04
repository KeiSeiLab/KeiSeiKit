use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

// Bin uses local sibling modules for orchestration (`runner`, `plan`, `render`)
// and pulls evidence checkers + schema from the library facade. This avoids
// double-compilation of `evidence/` and `schema.rs`.
mod plan;
mod plan_io;
mod render;
mod runner;

#[derive(Parser)]
#[command(name = "kei-arch-map", about = "Self-validating architecture map")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Verify {
        #[arg(long, default_value = "arch/PLAN.toml")]
        plan: PathBuf,
    },
    Render {
        #[arg(long, default_value = "arch/PLAN.toml")]
        plan: PathBuf,
        #[arg(long, default_value = "arch/ARCH.md")]
        out: PathBuf,
    },
    Plan {
        #[arg(long, default_value = "arch/PLAN.toml")]
        plan: PathBuf,
        #[arg(long, default_value = "arch/CLAIMS.md")]
        out: PathBuf,
    },
    AddClaim {
        #[arg(long, default_value = "arch/PLAN.toml")]
        plan: PathBuf,
        #[arg(long)]
        module: String,
        #[arg(long)]
        id: String,
        #[arg(long)]
        description: String,
        #[arg(long)]
        evidence_json: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Verify { plan } => runner::run(&plan),
        Cmd::Render { plan, out } => render::run(&plan, &out),
        Cmd::Plan { plan, out } => plan::run(&plan, &out),
        Cmd::AddClaim {
            plan,
            module,
            id,
            description,
            evidence_json,
        } => plan::add_claim(&plan, &module, &id, &description, &evidence_json),
    }
}
