//! clap CLI shapes — five subcommands.
//!
//! Constructor Pattern: this cube holds parser structs + the dispatch
//! table only. Per-subcommand bodies live in sibling cubes
//! (`platform`, `discovery`, `models`, `generate`, `server`). Every
//! handler checks the platform gate FIRST and exits with code 2 + a
//! stable JSON payload when unsupported.

use crate::discovery::{discover, MlxBins};
use crate::error::{exit_code_for, Error};
use crate::generate::{generate, GenerateOpts};
use crate::models::{default_cache_dir, list_models};
use crate::platform::{host_arch_label, is_supported};
use crate::runner::{Runner, SystemRunner};
use crate::server::{build_spec, openai_compat_url, DEFAULT_HOST, DEFAULT_PORT};
use clap::{Parser, Subcommand};
use serde_json::json;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "kei-llm-mlx",
    version,
    about = "Wave 59 — Apple MLX adapter (mlx_lm shell-out, macOS Apple Silicon only)"
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Probe: platform gate + discover mlx_lm binaries.
    Probe,
    /// List MLX-quantised models cached under HuggingFace hub.
    Models {
        #[arg(long)]
        cache_dir: Option<PathBuf>,
    },
    /// Run a single non-streaming generation.
    Generate {
        #[arg(long)]
        model: String,
        #[arg(long)]
        prompt: String,
        #[arg(long)]
        max_tokens: Option<u32>,
        #[arg(long)]
        temperature: Option<f32>,
        #[arg(long, default_value_t = false)]
        stream: bool,
    },
    /// Spawn `mlx_lm.server` for an OpenAI-compat local HTTP endpoint.
    Server {
        #[arg(long)]
        model: String,
        #[arg(long, default_value_t = DEFAULT_PORT)]
        port: u16,
        #[arg(long, default_value = DEFAULT_HOST)]
        host: String,
    },
    /// Print version metadata for both the wrapper and discovered mlx_lm.
    Version,
}

pub fn dispatch(cli: Cli) -> ExitCode {
    let runner = SystemRunner;
    match cli.cmd {
        Cmd::Probe => cmd_probe(&runner),
        Cmd::Models { cache_dir } => cmd_models(cache_dir),
        Cmd::Generate { model, prompt, max_tokens, temperature, stream } => {
            cmd_generate(&runner, &model, &prompt, max_tokens, temperature, stream)
        }
        Cmd::Server { model, port, host } => cmd_server(&model, port, &host),
        Cmd::Version => cmd_version(&runner),
    }
}

fn cmd_probe(runner: &dyn Runner) -> ExitCode {
    let support = is_supported();
    if !support.supported {
        let body = json!({
            "supported": false,
            "reason": support.reason,
            "host_arch": support.host_arch,
            "host_os": support.host_os,
        });
        println!("{}", body);
        return ExitCode::from(2);
    }
    let bins = discover(runner);
    print_probe(&support.host_arch, &support.host_os, &bins);
    if bins.any_present() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(3)
    }
}

fn print_probe(host_arch: &str, host_os: &str, bins: &MlxBins) {
    let body = json!({
        "supported": true,
        "host_arch": host_arch,
        "host_os": host_os,
        "mlx_lm_version": bins.version,
        "generate_path": bins.generate,
        "server_path": bins.server,
    });
    println!("{}", body);
}

fn cmd_models(cache_dir: Option<PathBuf>) -> ExitCode {
    let support = is_supported();
    if !support.supported {
        return print_unsupported(&support.reason);
    }
    let dir = cache_dir.or_else(default_cache_dir);
    let entries = match dir.as_deref() {
        Some(d) => list_models(d),
        None => Vec::new(),
    };
    println!("{}", serde_json::to_string(&entries).unwrap_or_else(|_| "[]".into()));
    ExitCode::SUCCESS
}

fn cmd_generate(
    runner: &dyn Runner,
    model: &str,
    prompt: &str,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    _stream: bool,
) -> ExitCode {
    let support = is_supported();
    if !support.supported {
        return print_unsupported(&support.reason);
    }
    let bins = discover(runner);
    let bin = match bins.generate.as_ref() {
        Some(p) => p.to_string_lossy().to_string(),
        None => {
            return emit_error(&Error::BinaryNotFound("mlx_lm.generate".into()));
        }
    };
    let opts = GenerateOpts { max_tokens, temperature };
    match generate(runner, &bin, model, prompt, &opts) {
        Ok(r) => {
            println!("{}", serde_json::to_string(&r).unwrap_or_default());
            ExitCode::SUCCESS
        }
        Err(e) => emit_error(&e),
    }
}

fn cmd_server(model: &str, port: u16, host: &str) -> ExitCode {
    match build_spec(model, host, port) {
        Ok(spec) => {
            let body = json!({
                "spec": spec,
                "openai_compat_url": openai_compat_url(&spec),
                "note": "spec built; spawn happens via Runner in caller",
            });
            println!("{}", body);
            ExitCode::SUCCESS
        }
        Err(e) => emit_error(&e),
    }
}

fn cmd_version(runner: &dyn Runner) -> ExitCode {
    let support = is_supported();
    let bins = if support.supported { discover(runner) } else { MlxBins::default() };
    let body = json!({
        "kei_wrapper_version": env!("CARGO_PKG_VERSION"),
        "mlx_lm_version": bins.version,
        "supported": support.supported,
        "host_arch": host_arch_label(),
        "reason": support.reason,
    });
    println!("{}", body);
    if support.supported { ExitCode::SUCCESS } else { ExitCode::from(2) }
}

fn print_unsupported(reason: &Option<String>) -> ExitCode {
    let body = json!({"supported": false, "reason": reason});
    println!("{}", body);
    ExitCode::from(2)
}

fn emit_error(e: &Error) -> ExitCode {
    let body = json!({"error": e.to_string()});
    eprintln!("{}", body);
    ExitCode::from(exit_code_for(e))
}
