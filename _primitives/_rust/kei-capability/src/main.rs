//! kei-capability — hook-protocol CLI adapter.
//!
//! Subcommands:
//!   - `check <name>`  — reads tool-use JSON from stdin, runs registry
//!                       gate, emits permissionDecision JSON, exits 0 or 2.
//!   - `verify <name>` — reads env (AGENT_ID, TASK_TOML, WORKTREE_PATH,
//!                       MAIN_REPO, RUN_MODE), runs registry verify,
//!                       exits 0 on pass or non-zero with stderr message.
//!   - `fork <source> --as <new-name> [--kit-root <dir>]` — copy an
//!                       existing capability dir under a new
//!                       `<cat>::<slug>` name and record lineage.

use kei_capability::fork;

use clap::{Parser, Subcommand};
use kei_agent_runtime::capability::{
    GateContext, GateDecision, RunMode, TaskSpec, VerifyContext, VerifyResult,
};
use kei_agent_runtime::registry;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-capability", version, about = "Capability hook adapter")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// PreToolUse gate — stdin holds hook payload JSON.
    Check { name: String },
    /// On-return verify — env carries context.
    Verify { name: String },
    /// Fork a capability: copy dir under a new <cat>::<slug> name with lineage.
    Fork {
        /// Existing `<cat>::<slug>` to clone.
        source: String,
        /// New `<cat>::<slug>` name for the fork.
        #[arg(long = "as")]
        as_name: String,
        /// Kit root (contains `_capabilities/`); defaults to cwd.
        #[arg(long = "kit-root", default_value = ".")]
        kit_root: PathBuf,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Check { name } => run_check(name),
        Cmd::Verify { name } => run_verify(name),
        Cmd::Fork {
            source,
            as_name,
            kit_root,
        } => run_fork_cmd(&source, &as_name, &kit_root),
    }
}

fn run_fork_cmd(source: &str, new_name: &str, kit_root: &Path) -> ExitCode {
    let now = fork::current_iso_utc();
    match fork::run_fork(source, new_name, kit_root, &now) {
        Ok(summary) => {
            println!("forked {} → {}", summary.source, summary.target);
            println!("  dir: {}", summary.target_dir.display());
            println!("  fields rewritten: {}", summary.diff_count);
            println!(
                "  next: edit text.md to reflect fork semantics; ensure \
                 [gate].rust-module and [verify].rust-module match the new slug"
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("fork failed: {e:#}");
            ExitCode::from(2)
        }
    }
}

fn run_check(name: String) -> ExitCode {
    let cap = match registry::get_gate(&name) {
        Some(c) => c,
        None => {
            eprintln!("unknown gate capability: {name}");
            return ExitCode::from(2);
        }
    };
    let payload = read_stdin_json().unwrap_or_else(|| json!({}));
    let tool_name = payload.get("tool_name").and_then(|v| v.as_str()).unwrap_or("");
    let tool_input = payload.get("tool_input").cloned().unwrap_or(json!({}));
    let env: HashMap<String, String> = std::env::vars().collect();
    let task = load_task_from_env().unwrap_or_default();
    let ctx = GateContext {
        tool_name,
        tool_input: &tool_input,
        task: &task,
        env: &env,
    };
    match cap.check(&ctx) {
        GateDecision::Allow | GateDecision::NotApplicable => {
            println!("{}", json!({"permissionDecision": "allow"}));
            ExitCode::SUCCESS
        }
        GateDecision::Deny { reason } => {
            eprintln!("{reason}");
            println!(
                "{}",
                json!({"permissionDecision": "deny", "reason": reason})
            );
            ExitCode::from(2)
        }
    }
}

fn run_verify(name: String) -> ExitCode {
    let cap = match registry::get_verify(&name) {
        Some(c) => c,
        None => {
            eprintln!("unknown verify capability: {name}");
            return ExitCode::from(2);
        }
    };
    let agent_id = std::env::var("AGENT_ID").unwrap_or_default();
    let worktree_path = PathBuf::from(std::env::var("WORKTREE_PATH").unwrap_or_default());
    let main_repo = PathBuf::from(std::env::var("MAIN_REPO").unwrap_or_default());
    let run_mode = match std::env::var("RUN_MODE").unwrap_or_else(|_| "worktree".into()).as_str() {
        "simulated-merge" => RunMode::SimulatedMerge,
        "both" => RunMode::Both,
        _ => RunMode::Worktree,
    };
    let task = load_task_from_env().unwrap_or_default();
    let ctx = VerifyContext {
        agent_id: &agent_id,
        task: &task,
        worktree_path: &worktree_path,
        main_repo: &main_repo,
        run_mode,
        simulated_merge_path: None,
    };
    match cap.verify(&ctx) {
        VerifyResult::Pass => ExitCode::SUCCESS,
        VerifyResult::Fail { reason, detail } => {
            eprintln!("FAIL {name}: {reason}");
            if let Some(d) = detail {
                eprintln!("{d}");
            }
            ExitCode::from(2)
        }
    }
}

fn read_stdin_json() -> Option<Value> {
    let mut buf = String::new();
    if std::io::stdin().read_to_string(&mut buf).is_err() {
        return None;
    }
    if buf.trim().is_empty() {
        return None;
    }
    serde_json::from_str(&buf).ok()
}

fn load_task_from_env() -> Option<TaskSpec> {
    let p = std::env::var("TASK_TOML").ok()?;
    let text = std::fs::read_to_string(&p).ok()?;
    toml::from_str::<TaskSpec>(&text).ok()
}
