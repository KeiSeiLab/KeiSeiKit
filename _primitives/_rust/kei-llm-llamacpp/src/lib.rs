//! kei-llm-llamacpp — adapter to llama.cpp via shell-out.
//!
//! Wave 58 of the local-LLM stack. NO FFI, NO daemon. Each subcommand
//! is one Constructor-Pattern module:
//!   discovery — find llama-cli / llama-server + version
//!   models    — scan dirs for .gguf, detect quant
//!   generate  — non-streaming inference
//!   stream    — streaming token chunks
//!   server    — spawn llama-server (loopback only)
//!   runner    — Runner trait + RealRunner + ServerHandle
//!   error     — Error enum + exit-code mapping
//!   cli       — clap entry structs

pub mod cli;
pub mod discovery;
pub mod error;
pub mod generate;
pub mod models;
pub mod runner;
pub mod server;
pub mod stream;

pub use discovery::{discover, BinPaths};
pub use error::{Error, Result};
pub use generate::{generate, GenerateOpts, Response};
pub use models::{detect_quant, list_models, ModelEntry};
pub use runner::{bin_in_path, RealRunner, RunOutput, Runner, ServerHandle};
pub use server::{start_server, validate_host, ServerInfo, ServerOpts};
pub use stream::{generate_stream, lines_to_chunks, Chunk};

/// Wrapper version — surfaced by the `version` subcommand.
pub const KEI_WRAPPER_VERSION: &str = env!("CARGO_PKG_VERSION");
