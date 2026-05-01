//! kei-spawn — CLI dispatcher.
//!
//! Four subcommands:
//!   - `spawn <task.toml>` — prepare invocation + ledger fork, emit JSON
//!   - `drive <task.toml>` — spawn + attempt driver invocation (v0.1: stub,
//!     returns exit 64 NotImplemented after emitting SpawnOutput JSON)
//!   - `verify <agent-id> <worktree>` — run verify pipeline, update ledger
//!   - `list-pending` — forward `kei-ledger list --status running`
//!
//! Exit codes:
//!   0  success (spawn, verify-clean, list-pending)
//!   1  generic failure (any Err from the pipeline)
//!   2  verify-failed (capabilities failed but pipeline ran)
//!   64 drive NotImplemented (v0.1 stub path)

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

use kei_spawn::{
    drive_with, ledger_sh, not_implemented_message, spawn_from_task, spawn_with_pipeline,
    verify_agent, DriveError, ManualDriver, PipelineChain, SpawnOutput,
};
use serde::Serialize;

#[derive(Parser)]
#[command(
    name = "kei-spawn",
    version,
    about = "Automation envelope: prepare + ledger fork + verify (RULE 0.13-compliant)"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Prepare an Agent-tool invocation + register ledger row.
    Spawn {
        /// Path to task.toml.
        task: PathBuf,
        /// kit root (default: cwd).
        #[arg(long)]
        kit_root: Option<PathBuf>,
        /// Also derive downstream handoff chain from role's `[pipeline]`
        /// section + scaffold stub task files for each step.
        #[arg(long)]
        pipeline: bool,
    },
    /// Spawn + invoke driver (v0.1: stub — emits SpawnOutput then exit 64).
    Drive {
        /// Path to task.toml.
        task: PathBuf,
        /// kit root (default: cwd).
        #[arg(long)]
        kit_root: Option<PathBuf>,
    },
    /// Run verify pipeline + update ledger status.
    Verify {
        /// agent-id previously emitted by `kei-spawn spawn`.
        agent_id: String,
        /// Worktree path reported by the Claude harness on agent return.
        worktree: PathBuf,
        #[arg(long)]
        kit_root: Option<PathBuf>,
    },
    /// Show all running ledger rows.
    ListPending,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Spawn { task, kit_root, pipeline } => run_spawn(task, kit_root, pipeline),
        Cmd::Drive { task, kit_root } => run_drive(task, kit_root),
        Cmd::Verify { agent_id, worktree, kit_root } => {
            run_verify(agent_id, worktree, kit_root)
        }
        Cmd::ListPending => run_list_pending(),
    }
}

#[derive(Serialize)]
struct SpawnWithPipelineJson<'a> {
    #[serde(flatten)]
    spawn: &'a SpawnOutput,
    pipeline: &'a PipelineChain,
}

fn run_spawn(task: PathBuf, kit_root: Option<PathBuf>, pipeline: bool) -> ExitCode {
    let kit = kit_root_or_cwd(kit_root);
    if pipeline {
        match spawn_with_pipeline(&task, &kit) {
            Ok((out, chain)) => emit_json(&SpawnWithPipelineJson {
                spawn: &out,
                pipeline: &chain,
            }),
            Err(e) => err("spawn --pipeline", e),
        }
    } else {
        match spawn_from_task(&task, &kit) {
            Ok(out) => emit_json(&out),
            Err(e) => err("spawn", e),
        }
    }
}

fn run_drive(task: PathBuf, kit_root: Option<PathBuf>) -> ExitCode {
    let kit = kit_root_or_cwd(kit_root);
    let out = match spawn_from_task(&task, &kit) {
        Ok(o) => o,
        Err(e) => return err("drive", e),
    };
    // Always emit SpawnOutput JSON first so callers can pipe it regardless
    // of the driver outcome. Drive-only failure modes come via stderr.
    if emit_json(&out) != ExitCode::SUCCESS {
        return ExitCode::from(1);
    }
    dispatch_driver(&out)
}

fn dispatch_driver(out: &SpawnOutput) -> ExitCode {
    let driver = ManualDriver;
    match drive_with(&driver, &out.prompt, &out.subagent_type, out.isolation.as_deref()) {
        Ok(_) => ExitCode::SUCCESS,
        Err(DriveError::NotImplemented { .. }) => {
            eprintln!("kei-spawn drive: {}", not_implemented_message());
            ExitCode::from(64)
        }
        Err(e) => err("drive", e),
    }
}

fn run_verify(agent_id: String, worktree: PathBuf, kit_root: Option<PathBuf>) -> ExitCode {
    let kit = kit_root_or_cwd(kit_root);
    match verify_agent(&agent_id, &worktree, &kit) {
        Ok(out) => {
            let code = if out.is_clean { ExitCode::SUCCESS } else { ExitCode::from(2) };
            let _ = emit_json(&out);
            code
        }
        Err(e) => err("verify", e),
    }
}

fn run_list_pending() -> ExitCode {
    match ledger_sh::list_running() {
        Ok(s) => {
            print!("{s}");
            ExitCode::SUCCESS
        }
        Err(e) => err("list-pending", e),
    }
}

fn emit_json<T: serde::Serialize>(v: &T) -> ExitCode {
    match serde_json::to_string_pretty(v) {
        Ok(s) => {
            println!("{s}");
            ExitCode::SUCCESS
        }
        Err(e) => err("serialize json", e),
    }
}

fn kit_root_or_cwd(arg: Option<PathBuf>) -> PathBuf {
    arg.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}

fn err(stage: &str, e: impl std::fmt::Display) -> ExitCode {
    eprintln!("kei-spawn {stage}: {e}");
    ExitCode::from(1)
}
