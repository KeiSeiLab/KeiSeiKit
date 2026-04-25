//! kei-agent-runtime — CLI dispatcher for compose | spawn | verify | run.

use clap::{Parser, Subcommand};
use kei_agent_runtime::capability::RunMode;
use kei_agent_runtime::{compose, prepare, spawn, verify};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "kei-agent-runtime",
    version,
    about = "Agent substrate v1 — compose/spawn/verify gated agent invocations"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Compose prompt from a task.toml and write tasks/<agent-id>/prompt.md.
    Compose {
        task: PathBuf,
        #[arg(long)]
        kit_root: Option<PathBuf>,
    },
    /// Prepare spawn dir (tasks/<agent-id>/) — orchestrator invokes Agent tool.
    Spawn {
        task: PathBuf,
        #[arg(long)]
        kit_root: Option<PathBuf>,
    },
    /// Run every verify capability declared by the task's role.
    Verify {
        task: PathBuf,
        #[arg(long)]
        worktree: PathBuf,
        #[arg(long)]
        kit_root: Option<PathBuf>,
        #[arg(long)]
        main_repo: Option<PathBuf>,
        #[arg(long, default_value = "worktree")]
        mode: String,
    },
    /// One-shot helper: compose + spawn + verify (tests only).
    Run {
        task: PathBuf,
        #[arg(long)]
        worktree: PathBuf,
        #[arg(long)]
        kit_root: Option<PathBuf>,
    },
    /// Assemble everything orchestrator needs to invoke Agent tool.
    /// Does NOT write tasks/ on disk — inspection helper.
    Prepare {
        task: PathBuf,
        #[arg(long)]
        kit_root: Option<PathBuf>,
        /// Output format: human (default) | json | toml
        #[arg(long, default_value = "human")]
        format: String,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Compose { task, kit_root } => run_compose(task, kit_root),
        Cmd::Spawn { task, kit_root } => run_spawn(task, kit_root),
        Cmd::Verify { task, worktree, kit_root, main_repo, mode } => {
            run_verify(task, worktree, kit_root, main_repo, mode)
        }
        Cmd::Run { task, worktree, kit_root } => run_run(task, worktree, kit_root),
        Cmd::Prepare { task, kit_root, format } => run_prepare(task, kit_root, format),
    }
}

fn run_prepare(task_path: PathBuf, kit_root: Option<PathBuf>, format: String) -> ExitCode {
    let kit = kit_root_or_cwd(kit_root);
    let task = match spawn::load_task(&task_path) {
        Ok(t) => t,
        Err(e) => return err("load task", e),
    };
    let inv = match prepare::prepare(&task, &kit) {
        Ok(i) => i,
        Err(e) => return err("prepare", e),
    };
    let rendered = match format.as_str() {
        "human" => Ok(prepare::render_human(&inv)),
        "json" => prepare::render_json(&inv),
        "toml" => prepare::render_toml(&inv),
        other => {
            eprintln!("unknown format '{other}' (expected human|json|toml)");
            return ExitCode::from(2);
        }
    };
    match rendered {
        Ok(s) => {
            print!("{s}");
            ExitCode::SUCCESS
        }
        Err(e) => err("render", e),
    }
}

fn kit_root_or_cwd(arg: Option<PathBuf>) -> PathBuf {
    arg.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}

fn run_compose(task_path: PathBuf, kit_root: Option<PathBuf>) -> ExitCode {
    let kit = kit_root_or_cwd(kit_root);
    let task = match spawn::load_task(&task_path) {
        Ok(t) => t,
        Err(e) => return err("load task", e),
    };
    match compose::compose_prompt(&task, &kit) {
        Ok(p) => {
            println!("{p}");
            ExitCode::SUCCESS
        }
        Err(e) => err("compose", e),
    }
}

fn run_spawn(task_path: PathBuf, kit_root: Option<PathBuf>) -> ExitCode {
    let kit = kit_root_or_cwd(kit_root);
    let task = match spawn::load_task(&task_path) {
        Ok(t) => t,
        Err(e) => return err("load task", e),
    };
    match spawn::prepare_agent(&task, &kit) {
        Ok(p) => {
            println!("agent_id={}", p.agent_id);
            println!("prompt={}", p.prompt_path.display());
            ExitCode::SUCCESS
        }
        Err(e) => err("spawn", e),
    }
}

fn run_verify(
    task_path: PathBuf,
    worktree: PathBuf,
    kit_root: Option<PathBuf>,
    main_repo: Option<PathBuf>,
    mode: String,
) -> ExitCode {
    let kit = kit_root_or_cwd(kit_root);
    let task = match spawn::load_task(&task_path) {
        Ok(t) => t,
        Err(e) => return err("load task", e),
    };
    let caps = match verify::load_role_capabilities(&kit, &task.task.role) {
        Ok(c) => c,
        Err(e) => return err("load role", e),
    };
    let run_mode = match mode.as_str() {
        "worktree" => RunMode::Worktree,
        "simulated-merge" => RunMode::SimulatedMerge,
        "both" => RunMode::Both,
        other => {
            eprintln!("unknown mode '{other}'");
            return ExitCode::from(2);
        }
    };
    let main = main_repo.unwrap_or_else(|| kit.clone());
    let report = match verify::verify_task(
        &task,
        &task.task.agent_id,
        &worktree,
        &main,
        run_mode,
        &caps,
        None,
    ) {
        Ok(r) => r,
        Err(e) => return err("verify", e),
    };
    println!("{}", serde_json::to_string_pretty(&report).unwrap_or_default());
    if report.is_clean() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(2)
    }
}

fn run_run(task_path: PathBuf, worktree: PathBuf, kit_root: Option<PathBuf>) -> ExitCode {
    let code = run_spawn(task_path.clone(), kit_root.clone());
    if code != ExitCode::SUCCESS {
        return code;
    }
    run_verify(task_path, worktree, kit_root, None, "worktree".into())
}

fn err(stage: &str, e: impl std::fmt::Display) -> ExitCode {
    eprintln!("{stage}: {e}");
    ExitCode::from(1)
}
