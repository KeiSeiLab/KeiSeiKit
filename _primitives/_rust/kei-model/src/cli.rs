//! clap structs for the 5 subcommands.
//!
//! `Cli` is the top-level parser; `Cmd` is the subcommand enum. Concrete
//! arg-bags live next to their variant so each one is a self-contained struct.

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "kei-model", version, about = "Universal model registry + selector")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// List models matching all supplied filters.
    List(ListArgs),
    /// Pick the cheapest active model for a role + budget + caps triple.
    Resolve(ResolveArgs),
    /// Estimate cost in micro-cents for a token budget.
    Price(PriceArgs),
    /// List distinct providers + active/deprecated counts.
    Providers(ProvidersArgs),
    /// Walk a fallback chain until None or cycle.
    Fallback(FallbackArgs),
}

#[derive(Debug, Args)]
pub struct ListArgs {
    #[arg(long)]
    pub provider: Option<String>,
    #[arg(long)]
    pub cap: Option<String>,
    #[arg(long)]
    pub status: Option<String>,
    #[arg(long)]
    pub role: Option<String>,
    #[arg(long = "models-toml")]
    pub models_toml: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct ResolveArgs {
    #[arg(long)]
    pub role: String,
    #[arg(long = "budget-micro")]
    pub budget_micro: Option<u64>,
    /// Comma-separated capabilities, e.g. "code,vision".
    #[arg(long)]
    pub cap: Option<String>,
    #[arg(long = "models-toml")]
    pub models_toml: Option<PathBuf>,
    #[arg(long = "selectors-toml")]
    pub selectors_toml: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct PriceArgs {
    /// Positional model id.
    pub model_id: String,
    #[arg(long = "input-tokens")]
    pub input_tokens: u64,
    #[arg(long = "output-tokens")]
    pub output_tokens: u64,
    #[arg(long = "models-toml")]
    pub models_toml: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct ProvidersArgs {
    #[arg(long = "models-toml")]
    pub models_toml: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct FallbackArgs {
    #[arg(long)]
    pub primary: String,
    #[arg(long = "models-toml")]
    pub models_toml: Option<PathBuf>,
}
