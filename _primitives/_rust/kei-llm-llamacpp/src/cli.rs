//! CLI — clap structs for the 5 subcommands.
//!
//! Subcommands:
//!   probe                  — discover binaries, emit JSON
//!   models [--dir <path>]  — list .gguf files in directory tree
//!   generate ...           — shell to llama-cli, emit Response (or NDJSON Chunks)
//!   server   ...           — spawn llama-server, emit ServerInfo
//!   version                — emit { llama_cli_version, llama_server_version, kei_wrapper_version }

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "kei-llm-llamacpp",
    version,
    about = "Adapter to llama.cpp via shell-out (no FFI, no daemon)"
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Locate llama-cli / llama-server on PATH and emit JSON BinPaths.
    /// Exit 0 if at least one binary present, 2 if neither found.
    Probe,

    /// List .gguf files. Default: ~/.cache/llama.cpp + macOS app-support.
    Models {
        /// Override search directory (recursive scan).
        #[arg(long)]
        dir: Option<PathBuf>,
    },

    /// Shell to llama-cli. Without --stream emits one Response JSON.
    /// With --stream emits one Chunk JSON per line (NDJSON).
    Generate {
        /// Path to .gguf model file.
        #[arg(long)]
        model: PathBuf,
        /// Prompt text.
        #[arg(long)]
        prompt: String,
        /// Cap on generated tokens.
        #[arg(long, default_value_t = 128)]
        max_tokens: u32,
        /// Sampling temperature (omit to use llama-cli default).
        #[arg(long)]
        temperature: Option<f32>,
        /// Stream tokens line-by-line as NDJSON.
        #[arg(long)]
        stream: bool,
    },

    /// Spawn llama-server and emit JSON ServerInfo.
    /// Default host 127.0.0.1; non-localhost rejected with exit 5.
    Server {
        #[arg(long)]
        model: PathBuf,
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        #[arg(long, default_value_t = 8080)]
        port: u16,
    },

    /// Emit JSON {llama_cli_version, llama_server_version, kei_wrapper_version}.
    Version,
}
