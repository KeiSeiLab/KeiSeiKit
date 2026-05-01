//! Clap structs + subcommand handlers for the four-verb surface.
//!
//! Constructor Pattern: ONE responsibility — define the user-facing CLI
//! and own its handlers. The `handlers` submodule keeps the actual I/O
//! (probe, route, list-backends, which) close to its argument structs
//! while leaving `main.rs` ≤30 LOC.
//!
//! Subcommands (per task spec):
//!   1. `probe`           — passthrough to `kei_machine_probe::probe`.
//!   2. `route`           — full route decision; supports `--require-local`.
//!   3. `list-backends`   — health-check all 3 backends, JSON table out.
//!   4. `which --model X` — pure discovery, no health probe.

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "kei-llm-router",
    version,
    about = "Universal local-LLM backend selector (Wave 60)"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Run the W56 machine probe and emit JSON.
    Probe(ProbeArgs),
    /// Resolve the best local backend for a given model id.
    Route(RouteArgs),
    /// Emit a JSON list of `BackendHealth` entries for every backend.
    ListBackends,
    /// Pure discovery — which backends claim to have `<model_id>`?
    Which(WhichArgs),
}

#[derive(Debug, Parser)]
pub struct ProbeArgs {
    /// Skip the four `which` + `--version` shell-outs (CI / fast path).
    #[arg(long)]
    pub skip_tooling: bool,
}

#[derive(Debug, Parser)]
pub struct RouteArgs {
    /// Canonical model id (e.g. `llama-3-70b-local`, `qwen3:4b`).
    #[arg(long)]
    pub model: String,

    /// Optional role tag for budget / role-default lookup.
    #[arg(long)]
    pub role: Option<String>,

    /// Optional micro-cents budget (used for registry-side filtering).
    #[arg(long)]
    pub budget_micro: Option<u64>,

    /// Refuse any non-local backend (no Anthropic / OpenAI fallback).
    #[arg(long)]
    pub require_local: bool,
}

#[derive(Debug, Parser)]
pub struct WhichArgs {
    /// Model id to query — no health check, just discovery.
    #[arg(long)]
    pub model: String,
}

pub mod handlers {
    use super::{ProbeArgs, RouteArgs, WhichArgs};
    use crate::discovery::discover_models;
    use crate::error::Error;
    use crate::health::check_all;
    use crate::router::{route, RouteOpts};

    pub fn run_probe(args: ProbeArgs) -> i32 {
        let runner = kei_machine_probe::SystemRunner;
        let machine = kei_machine_probe::probe(&runner, args.skip_tooling);
        emit_json(&machine);
        0
    }

    pub async fn run_list_backends() -> i32 {
        let report = check_all().await;
        emit_json(&report);
        0
    }

    pub async fn run_which(args: WhichArgs) -> i32 {
        let machine = probe_default();
        let matches = discover_models(&machine, &args.model).await;
        emit_json(&matches);
        0
    }

    pub async fn run_route(args: RouteArgs) -> i32 {
        let machine = probe_default();
        let opts = RouteOpts {
            require_local: args.require_local,
            role: args.role,
            budget_micro: args.budget_micro,
        };
        match route(&machine, &args.model, &opts, None).await {
            Ok(decision) => {
                emit_json(&decision);
                0
            }
            Err(e) => emit_error(&e),
        }
    }

    fn probe_default() -> kei_machine_probe::Machine {
        let runner = kei_machine_probe::SystemRunner;
        kei_machine_probe::probe(&runner, true)
    }

    fn emit_json<T: serde::Serialize>(v: &T) {
        match serde_json::to_string_pretty(v) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("serialise error: {e}"),
        }
    }

    fn emit_error(e: &Error) -> i32 {
        let payload = serde_json::json!({
            "error": e.kind(),
            "message": e.to_string(),
        });
        eprintln!("{payload}");
        e.exit_code()
    }
}
