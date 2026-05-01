//! kei-decision CLI entry point — clap parse + dispatch only.
//!
//! Each subcommand has a thin `run_*` function declared inline below. Heavy
//! logic lives in the library modules (`parser`, `classifier`, `ranker`,
//! `emitter`, `executor`, `ledger`, `sleep_link`, `graph`).

use clap::Parser;
use kei_decision::cli::{Cli, Cmd};
use kei_decision::{
    classify, emit_task_toml, execute_action, merge_graphs, parse_master_report, pre_fork_ledger,
    rank_actions, ParseError, RankedAction,
};
use serde::Serialize;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Parse { master } => run_parse(master),
        Cmd::Rank { master, limit, markdown } => run_rank(master, limit, markdown),
        Cmd::Plan { master, out } => run_plan(master, out),
        Cmd::Execute { master, dry_run, limit, no_ledger } => {
            run_execute(master, dry_run, limit, no_ledger)
        }
        Cmd::Link { research_dir, graph_out } => run_link(research_dir, graph_out),
    }
}

fn run_parse(master: PathBuf) -> ExitCode {
    match parse_master_report(&master) {
        Ok(actions) => {
            let kinds: Vec<_> = actions.iter().map(classify).collect();
            let with_kind: Vec<_> = actions.iter().zip(kinds.iter()).map(|(a, k)| WithKind { raw: a, kind: *k }).collect();
            print_json_or_err(&with_kind)
        }
        Err(e) => parse_err_to_exit(&e),
    }
}

fn run_rank(master: PathBuf, limit: Option<usize>, markdown: bool) -> ExitCode {
    let ranked = match rank_pipeline(&master, limit) {
        Ok(r) => r,
        Err(e) => return parse_err_to_exit(&e),
    };
    if markdown { print_markdown_table(&ranked); ExitCode::SUCCESS }
    else { print_json_or_err(&ranked) }
}

fn run_plan(master: PathBuf, out: PathBuf) -> ExitCode {
    let ranked = match rank_pipeline(&master, None) {
        Ok(r) => r,
        Err(e) => return parse_err_to_exit(&e),
    };
    let mut paths = Vec::new();
    for action in &ranked {
        match emit_task_toml(action, &out, &master) {
            Ok(emit) => paths.push(emit),
            Err(e) => return generic_err("plan", &e),
        }
    }
    print_json_or_err(&paths)
}

fn run_execute(master: PathBuf, dry_run: bool, limit: Option<usize>, no_ledger: bool) -> ExitCode {
    let ranked = match rank_pipeline(&master, limit) {
        Ok(r) => r,
        Err(e) => return parse_err_to_exit(&e),
    };
    let tmp = std::env::temp_dir().join("kei-decision-execute");
    let mut report: Vec<serde_json::Value> = Vec::new();
    for action in &ranked {
        let emit = match emit_task_toml(action, &tmp, &master) {
            Ok(e) => e,
            Err(e) => return generic_err("execute/emit", &e),
        };
        if dry_run {
            report.push(serde_json::json!({ "action_id": action.raw.id, "task_path": emit.path, "executed": false }));
            continue;
        }
        if let Some(code) = execute_one(action, &emit.path, no_ledger, &mut report) {
            return code;
        }
    }
    print_json_or_err(&report)
}

fn execute_one(
    action: &RankedAction,
    task_path: &std::path::Path,
    no_ledger: bool,
    report: &mut Vec<serde_json::Value>,
) -> Option<ExitCode> {
    let exec_out = match execute_action(&action.raw.id, task_path) {
        Ok(o) => o,
        Err(e) => { eprintln!("kei-decision execute: {}", e); return Some(ExitCode::from(3)); }
    };
    if !no_ledger {
        // best-effort: log the ledger pre-fork attempt; failure is non-fatal here.
        let spec_sha_stub = format!("planned-{}", action.raw.id);
        let _ = pre_fork_ledger(&exec_out.agent_id, &exec_out.branch, &spec_sha_stub);
    }
    report.push(serde_json::to_value(&exec_out).unwrap_or(serde_json::Value::Null));
    None
}

fn run_link(research_dir: PathBuf, graph_out: Option<PathBuf>) -> ExitCode {
    let out = graph_out.unwrap_or_else(default_graph_path);
    match merge_graphs(&research_dir, &out) {
        Ok(o) => print_json_or_err(&o),
        Err(e) => generic_err("link", &e),
    }
}

fn default_graph_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join("Projects/KnowledgeVault/knowledge/research-graph.json")
}

fn rank_pipeline(master: &std::path::Path, limit: Option<usize>) -> Result<Vec<RankedAction>, ParseError> {
    let raws = parse_master_report(master)?;
    let kinds: Vec<_> = raws.iter().map(classify).collect();
    let mut ranked = rank_actions(raws, kinds);
    if let Some(n) = limit { ranked.truncate(n); }
    Ok(ranked)
}

#[derive(Serialize)]
struct WithKind<'a> {
    #[serde(flatten)]
    raw: &'a kei_decision::RawAction,
    kind: kei_decision::ActionKind,
}

fn print_json_or_err<T: Serialize>(v: &T) -> ExitCode {
    match serde_json::to_string_pretty(v) {
        Ok(s) => { println!("{s}"); ExitCode::SUCCESS }
        Err(e) => { eprintln!("kei-decision serialize: {e}"); ExitCode::from(1) }
    }
}

fn print_markdown_table(ranked: &[RankedAction]) {
    println!("| Rank | # | Action | Kind | Severity | Effort | Score |");
    println!("|------|---|--------|------|----------|--------|-------|");
    for r in ranked {
        println!("| {} | {} | {} | {:?} | {} | {} | {:.2} |",
                 r.rank, r.raw.id, r.raw.title, r.kind, r.raw.severity, r.raw.effort, r.score);
    }
}

fn parse_err_to_exit(e: &ParseError) -> ExitCode {
    eprintln!("kei-decision parse: {e}");
    match e {
        ParseError::FileNotFound(_) => ExitCode::from(1),
        ParseError::NoActionsFound => ExitCode::from(2),
        ParseError::Io(_) => ExitCode::from(1),
    }
}

fn generic_err(stage: &str, e: &impl std::fmt::Display) -> ExitCode {
    eprintln!("kei-decision {stage}: {e}");
    ExitCode::from(1)
}
