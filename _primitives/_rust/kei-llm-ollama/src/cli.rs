//! Clap definitions for the 5 subcommands.

use clap::{Parser, Subcommand};

use crate::client::DEFAULT_BASE_URL;

#[derive(Parser, Debug)]
#[command(
    name = "kei-llm-ollama",
    version,
    about = "HTTP adapter for the Ollama daemon (localhost:11434)"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// List installed models (GET /api/tags).
    Tags(BaseUrlOpt),

    /// Single-prompt completion (POST /api/generate).
    Generate(GenerateOpt),

    /// Multi-turn chat (POST /api/chat).
    Chat(ChatOpt),

    /// Download or update a model (POST /api/pull).
    Pull(PullOpt),

    /// Health probe — returns {running, version, models_count}.
    Health(BaseUrlOpt),
}

#[derive(Parser, Debug)]
pub struct BaseUrlOpt {
    /// Ollama daemon URL. Local-only by default for security.
    #[arg(long, default_value = DEFAULT_BASE_URL)]
    pub base_url: String,

    /// Per-call timeout in milliseconds. Ignored for streaming flows.
    #[arg(long)]
    pub timeout_ms: Option<u64>,
}

#[derive(Parser, Debug)]
pub struct GenerateOpt {
    /// Model name (e.g. `qwen3:4b`).
    #[arg(long)]
    pub model: String,

    /// User prompt.
    #[arg(long)]
    pub prompt: String,

    /// Stream NDJSON chunks one per line (instead of full JSON).
    #[arg(long)]
    pub stream: bool,

    /// Cap response tokens (`options.num_predict`).
    #[arg(long)]
    pub max_tokens: Option<u32>,

    /// Sampling temperature (`options.temperature`).
    #[arg(long)]
    pub temperature: Option<f32>,

    #[command(flatten)]
    pub base: BaseUrlOpt,
}

#[derive(Parser, Debug)]
pub struct ChatOpt {
    /// Model name.
    #[arg(long)]
    pub model: String,

    /// Inline JSON array of `{role, content}` OR `@path/to/file.json`.
    #[arg(long)]
    pub messages: String,

    /// Stream NDJSON chunks one per line.
    #[arg(long)]
    pub stream: bool,

    /// Cap response tokens (`options.num_predict`).
    #[arg(long)]
    pub max_tokens: Option<u32>,

    /// Sampling temperature.
    #[arg(long)]
    pub temperature: Option<f32>,

    #[command(flatten)]
    pub base: BaseUrlOpt,
}

#[derive(Parser, Debug)]
pub struct PullOpt {
    /// Model to pull (e.g. `qwen3:4b`).
    #[arg(long)]
    pub model: String,

    #[command(flatten)]
    pub base: BaseUrlOpt,
}
