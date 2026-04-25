//! kei-task CLI — create / update / add-dep / graph / dependency-chain.
//!
//! Pilot refactor (Stream B): `create`, `search`, `add-dependency` now
//! dispatch through `kei_task::atoms::*`. Remaining subcommands call
//! legacy module functions directly — they migrate in a later pass.

use clap::{Parser, Subcommand};
use kei_task::atoms;
use kei_task::deps::dependency_chain;
use kei_task::graph::list_edges;
use kei_task::milestones::{create_milestone, link_task_to_milestone};
use kei_task::run_atom;
use kei_task::{Milestone, Store};
use std::path::PathBuf;
use std::process::ExitCode;

/// Typed error used by the CLI to carry both a message and an exit code.
///
/// Exit-code contract (§Runtime):
/// - 2  — atom rejected input (validation / semantic error)
/// - 1  — usage / IO / storage failure
struct CliError {
    code: u8,
    msg: String,
}

impl CliError {
    fn atom(msg: impl Into<String>) -> Self {
        Self { code: 2, msg: msg.into() }
    }
    fn io(msg: impl Into<String>) -> Self {
        Self { code: 1, msg: msg.into() }
    }
}

impl From<anyhow::Error> for CliError {
    fn from(e: anyhow::Error) -> Self {
        Self::io(format!("{e:#}"))
    }
}

#[derive(Parser)]
#[command(name = "kei-task", version, about = "Task DAG CLI")]
struct Cli {
    #[arg(long)] db: Option<PathBuf>,
    #[command(subcommand)] cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Create { title: String, #[arg(long, default_value = "")] description: String,
             #[arg(long, default_value = "medium")] priority: String },
    Update { id: i64, #[arg(long)] status: Option<String>, #[arg(long)] title: Option<String> },
    AddDependency { from_id: i64, to_id: i64,
                    #[arg(long, default_value = "blocks")] dep_type: String },
    Graph,
    DependencyChain { id: i64 },
    Search { query: String, #[arg(long, default_value_t = 20)] limit: i64 },
    Milestone { name: String, #[arg(long, default_value = "")] description: String },
    LinkMilestone { task_id: i64, milestone_id: i64 },
    /// Machine-facing atom invocation — `run-atom <verb>` reads JSON from
    /// stdin (or `--input`), dispatches to `atoms::<verb>::run`, writes JSON
    /// to stdout. Used by `kei-runtime invoke`.
    RunAtom { verb: String, #[arg(long)] input: Option<String> },
}

fn db_path(cli_db: Option<PathBuf>) -> PathBuf {
    if let Some(p) = cli_db { return p; }
    if let Ok(e) = std::env::var("KEI_TASK_DB") { return PathBuf::from(e); }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/task/task.sqlite")
}

fn run() -> Result<(), CliError> {
    let cli = Cli::parse();
    let s = Store::open(&db_path(cli.db))?;
    dispatch(&s, cli.cmd)
}

fn dispatch(s: &Store, cmd: Cmd) -> Result<(), CliError> {
    match cmd {
        Cmd::Create { title, description, priority } =>
            cmd_create(s, title, description, priority),
        Cmd::Update { id, status, title } => cmd_update(s, id, status, title),
        Cmd::AddDependency { from_id, to_id, dep_type } =>
            cmd_add_dep(s, from_id, to_id, dep_type),
        Cmd::Graph => cmd_graph(s),
        Cmd::DependencyChain { id } => cmd_chain(s, id),
        Cmd::Search { query, limit } => cmd_search(s, query, limit),
        Cmd::Milestone { name, description } => cmd_milestone(s, name, description),
        Cmd::LinkMilestone { task_id, milestone_id } =>
            cmd_link_milestone(s, task_id, milestone_id),
        Cmd::RunAtom { verb, input } => cmd_run_atom(s, verb, input),
    }
}

fn cmd_run_atom(s: &Store, verb: String, input: Option<String>) -> Result<(), CliError> {
    let txt = run_atom::read_input(input).map_err(CliError::io)?;
    let json = run_atom::dispatch(s, &verb, &txt)
        .map_err(|e| CliError { code: run_atom::exit_for_error(&e), msg: format!("{e}") })?;
    println!("{json}");
    Ok(())
}

fn cmd_create(s: &Store, title: String, description: String, priority: String) -> Result<(), CliError> {
    let out = atoms::create::run(s, atoms::create::Input {
        title, description, priority, milestone_id: None,
    }).map_err(|e| classify_dispatch(atoms::DispatchError::Create(e)))?;
    println!("{}", out.id);
    Ok(())
}

/// Classify any kei-task atom error via the shared `run_atom` exit-code table.
fn classify_dispatch(e: atoms::DispatchError) -> CliError {
    CliError { code: run_atom::exit_for_error(&e), msg: format!("{e}") }
}

fn cmd_update(s: &Store, id: i64, status: Option<String>, title: Option<String>) -> Result<(), CliError> {
    let mut t = s.get_task(id)?
        .ok_or_else(|| CliError::atom(format!("TaskNotFound: id {id} not found")))?;
    if let Some(st) = status { t.status = st; }
    if let Some(ti) = title { t.title = ti; }
    s.update_task(&t)?;
    println!("updated {}", id);
    Ok(())
}

fn cmd_add_dep(s: &Store, from_id: i64, to_id: i64, dep_type: String) -> Result<(), CliError> {
    let dep_display = if dep_type.is_empty() { "blocks".to_string() } else { dep_type.clone() };
    atoms::add_dependency::run(s, atoms::add_dependency::Input {
        from: from_id, to: to_id, dep_type,
    }).map_err(|e| classify_dispatch(atoms::DispatchError::AddDep(e)))?;
    println!("dep: {} -> {} ({})", from_id, to_id, dep_display);
    Ok(())
}

fn cmd_graph(s: &Store) -> Result<(), CliError> {
    for e in list_edges(s)? {
        println!("{}\t-[{}]->\t{}", e.task_id, e.dep_type, e.depends_on);
    }
    Ok(())
}

fn cmd_chain(s: &Store, id: i64) -> Result<(), CliError> {
    for did in dependency_chain(s, id)? { println!("{}", did); }
    Ok(())
}

fn cmd_search(s: &Store, query: String, limit: i64) -> Result<(), CliError> {
    let out = atoms::search::run(s, atoms::search::Input {
        query, limit: Some(limit),
    }).map_err(|e| classify_dispatch(atoms::DispatchError::Search(e)))?;
    for t in out.results {
        println!("{}\t{}\t{}", t.id, t.status, t.title);
    }
    Ok(())
}

fn cmd_milestone(s: &Store, name: String, description: String) -> Result<(), CliError> {
    let id = create_milestone(s, &Milestone {
        name, description, ..Default::default() })?;
    println!("{}", id);
    Ok(())
}

fn cmd_link_milestone(s: &Store, task_id: i64, milestone_id: i64) -> Result<(), CliError> {
    link_task_to_milestone(s, task_id, milestone_id)?;
    println!("linked {} -> milestone {}", task_id, milestone_id);
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(CliError { code, msg }) => {
            eprintln!("{msg}");
            ExitCode::from(code)
        }
    }
}
