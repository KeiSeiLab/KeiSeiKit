//! kei-runtime — CLI dispatcher.
//!
//! Subcommands: list-atoms | invoke | schema-lint | pipe (stub).
//! Default --root: `~/.claude/agents/_primitives/_rust`.

use clap::{Parser, Subcommand};
use kei_runtime::invoke::InvokeError;
use kei_runtime::{discover, invoke, lint};
use std::path::PathBuf;
use std::process::ExitCode;

/// Exit code returned when `invoke` lands on a not-yet-wired atom.
/// Distinct from exit 2 (atom rejected input) so CI can branch.
/// Chosen in the EX_USAGE range per `sysexits.h` convention.
const EXIT_INVOKE_NOT_IMPLEMENTED: u8 = 64;

#[derive(Parser)]
#[command(name = "kei-runtime", version, about = "Atom invocation runtime + schema linter")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// List atoms discovered under --root.
    ListAtoms {
        #[arg(long)]
        root: Option<PathBuf>,
        #[arg(long = "crate")]
        crate_name: Option<String>,
        #[arg(long)]
        kind: Option<String>,
    },
    /// Invoke one atom (MVP stub — see docs).
    Invoke {
        atom_id: String,
        #[arg(long)]
        input: String,
        #[arg(long)]
        root: Option<PathBuf>,
    },
    /// Lint every `atoms/*.md` under --root for schema correctness.
    SchemaLint {
        #[arg(long)]
        root: Option<PathBuf>,
        #[arg(long = "crate")]
        crate_name: Option<String>,
    },
    /// Execute a pipeline (not yet implemented).
    Pipe { dag: PathBuf },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::ListAtoms { root, crate_name, kind } => {
            run_list_atoms(resolve_root(root), crate_name, kind)
        }
        Cmd::Invoke { atom_id, input, root } => run_invoke(resolve_root(root), atom_id, input),
        Cmd::SchemaLint { root, crate_name } => run_lint(resolve_root(root), crate_name),
        Cmd::Pipe { dag: _ } => {
            println!("pipe: not yet implemented");
            ExitCode::SUCCESS
        }
    }
}

fn resolve_root(arg: Option<PathBuf>) -> PathBuf {
    if let Some(p) = arg {
        return p;
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/agents/_primitives/_rust")
}

fn run_list_atoms(root: PathBuf, crate_name: Option<String>, kind: Option<String>) -> ExitCode {
    let atoms = discover::walk_atoms(&root);
    for a in atoms {
        if let Some(c) = &crate_name {
            if a.crate_name != *c {
                continue;
            }
        }
        if let Some(k) = &kind {
            if a.kind.as_str() != k.as_str() {
                continue;
            }
        }
        println!("{}\t{}\t{}", a.full_id, a.kind.as_str(), a.md_path.display());
    }
    ExitCode::SUCCESS
}

fn run_invoke(root: PathBuf, atom_id: String, input_arg: String) -> ExitCode {
    let input_text = match load_input(&input_arg) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("input: {e}");
            return ExitCode::from(1);
        }
    };
    match invoke::invoke(&root, &atom_id, &input_text) {
        Ok(out) => {
            println!("{}", serde_json::to_string(&out).unwrap_or_default());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(invoke_exit_code(&e))
        }
    }
}

/// Map typed invoke errors to exit codes per locked §Runtime schema.
///
/// - `AtomNotFound|InputParse|InputInvalid|MissingInputSchema|InvalidAtom` → 2 (atom error)
/// - `AtomFailed { code, .. }`  → passthrough child exit code
/// - `SubprocessError|OutputParse` → 1 (IO / malformed output)
/// - `BinaryNotFound` → 127 (POSIX command-not-found)
/// - `NotImplemented` → 64 (legacy escape)
fn invoke_exit_code(err: &InvokeError) -> u8 {
    match err {
        InvokeError::AtomNotFound(_)
        | InvokeError::InputParse(_)
        | InvokeError::InputInvalid(_)
        | InvokeError::MissingInputSchema(_)
        | InvokeError::InvalidAtom(_) => 2,
        InvokeError::AtomFailed { code, .. } => {
            let c = *code;
            if (0..=255).contains(&c) { c as u8 } else { 1 }
        }
        InvokeError::SubprocessError(_) | InvokeError::OutputParse(_) => 1,
        InvokeError::BinaryNotFound { .. } => 127,
        InvokeError::NotImplemented { .. } => EXIT_INVOKE_NOT_IMPLEMENTED,
    }
}

fn load_input(arg: &str) -> Result<String, String> {
    if let Some(path) = arg.strip_prefix('@') {
        std::fs::read_to_string(path).map_err(|e| format!("read {path}: {e}"))
    } else {
        Ok(arg.to_string())
    }
}

fn run_lint(root: PathBuf, crate_filter: Option<String>) -> ExitCode {
    let report = lint::schema_lint(&root);
    print_lint(&report, crate_filter.as_deref());
    if report.failed.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(2)
    }
}

fn print_lint(report: &lint::LintReport, crate_filter: Option<&str>) {
    let keep = |label: &str| crate_filter.is_none_or(|f| label.contains(f));
    for label in report.passed.iter().filter(|l| keep(l)) {
        println!("PASS\t{label}");
    }
    for (label, errs) in report.failed.iter().filter(|(l, _)| keep(l)) {
        println!("FAIL\t{label}\t{}", errs.join(" | "));
    }
}
