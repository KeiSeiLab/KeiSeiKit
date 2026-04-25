//! Atom invocation — executes atoms by spawning `<crate> run-atom <verb>`.
//!
//! Flow:
//!   1. Discover atom → get `crate_name` + `verb` from `AtomMeta`
//!   2. Validate input JSON against the atom's `input_schema`
//!   3. Resolve the binary at `<KEI_RUNTIME_BIN_DIR>/<crate>` or `PATH`
//!   4. Spawn `<crate> run-atom <verb>` with input on stdin
//!   5. Parse stdout as JSON → `Output { atom, result }`
//!   6. Propagate exit codes: 0 ok, 2 atom-error, 127 not-found, 1 IO
//!
//! `NotImplemented` is retained as a rare corner-case escape (e.g. an atom
//! whose crate has not yet been migrated to the `run-atom` protocol).

use crate::discover::{walk_atoms, AtomMeta};
use crate::validate::validate_input;
use serde::Serialize;
use serde_json::Value;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug)]
pub enum InvokeError {
    AtomNotFound(String),
    InputParse(String),
    InputInvalid(String),
    MissingInputSchema(String),
    /// Crate binary is missing from both `KEI_RUNTIME_BIN_DIR` and `PATH`.
    BinaryNotFound { crate_name: String },
    /// Subprocess exited non-zero — propagate the atom's own exit code.
    AtomFailed { atom: String, code: i32, stderr: String },
    /// IO / spawn failure (not a non-zero exit from the child).
    SubprocessError(String),
    /// Atom's stdout was not parseable as JSON.
    OutputParse(String),
    /// Legacy escape — atom not yet migrated to `run-atom` protocol.
    NotImplemented { atom: String },
}

impl std::fmt::Display for InvokeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AtomNotFound(id) => write!(f, "no atom matching {id}"),
            Self::InputParse(e) => write!(f, "input rejected: {e}"),
            Self::InputInvalid(e) => write!(f, "input rejected: {e}"),
            Self::MissingInputSchema(id) => write!(f, "atom `{id}` declares no input schema"),
            Self::BinaryNotFound { crate_name } => write!(
                f,
                "binary `{crate_name}` not found on PATH or KEI_RUNTIME_BIN_DIR"
            ),
            Self::AtomFailed { atom, code, stderr } => {
                write!(f, "atom `{atom}` exited {code}: {stderr}")
            }
            Self::SubprocessError(e) => write!(f, "subprocess: {e}"),
            Self::OutputParse(e) => write!(f, "atom stdout not JSON: {e}"),
            Self::NotImplemented { atom } => write!(
                f,
                "invoke not yet wired for this atom ({atom}); use the underlying CLI directly"
            ),
        }
    }
}

impl std::error::Error for InvokeError {}

/// Parsed output of an invoked atom. `result` is the raw JSON the atom wrote.
#[derive(Debug, Serialize)]
pub struct Output {
    pub atom: String,
    pub result: Value,
}

/// Invoke an atom by full ID with a JSON input string.
///
/// Contract: discover atom → validate input against schema → spawn
/// `<crate> run-atom <verb>` with stdin=input → parse stdout as JSON.
pub fn invoke(root: &Path, atom_id: &str, input_json: &str) -> Result<Output, InvokeError> {
    let meta = find_atom(root, atom_id)?;
    let input: Value =
        serde_json::from_str(input_json).map_err(|e| InvokeError::InputParse(e.to_string()))?;
    let schema = meta
        .input_schema
        .as_ref()
        .ok_or_else(|| InvokeError::MissingInputSchema(atom_id.to_string()))?;
    validate_input(schema, &input).map_err(|e| InvokeError::InputInvalid(e.to_string()))?;
    exec_atom(&meta, input_json)
}

fn find_atom(root: &Path, atom_id: &str) -> Result<AtomMeta, InvokeError> {
    walk_atoms(root)
        .into_iter()
        .find(|a| a.full_id == atom_id)
        .ok_or_else(|| InvokeError::AtomNotFound(atom_id.to_string()))
}

fn exec_atom(meta: &AtomMeta, input_json: &str) -> Result<Output, InvokeError> {
    let bin = resolve_binary(&meta.crate_name)
        .ok_or_else(|| InvokeError::BinaryNotFound { crate_name: meta.crate_name.clone() })?;
    let mut child = Command::new(&bin)
        .arg("run-atom")
        .arg(&meta.verb)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| InvokeError::SubprocessError(format!("spawn {}: {e}", bin.display())))?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input_json.as_bytes())
            .map_err(|e| InvokeError::SubprocessError(format!("write stdin: {e}")))?;
    }
    let out = child
        .wait_with_output()
        .map_err(|e| InvokeError::SubprocessError(format!("wait: {e}")))?;
    handle_subprocess_output(meta, out)
}

fn handle_subprocess_output(
    meta: &AtomMeta,
    out: std::process::Output,
) -> Result<Output, InvokeError> {
    let code = out.status.code().unwrap_or(-1);
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        return Err(InvokeError::AtomFailed { atom: meta.full_id.clone(), code, stderr });
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let result: Value = serde_json::from_str(stdout.trim())
        .map_err(|e| InvokeError::OutputParse(format!("{e}; stdout was: {stdout}")))?;
    Ok(Output { atom: meta.full_id.clone(), result })
}

/// Resolve `<crate_name>` as an executable:
///   1. `$KEI_RUNTIME_BIN_DIR/<crate_name>` if env is set and file exists
///   2. Walk `$PATH`, return first `<dir>/<crate_name>` that exists
fn resolve_binary(crate_name: &str) -> Option<PathBuf> {
    if let Ok(bin_dir) = std::env::var("KEI_RUNTIME_BIN_DIR") {
        let candidate = PathBuf::from(bin_dir).join(crate_name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    let path = std::env::var("PATH").ok()?;
    for dir in std::env::split_paths(&path) {
        let candidate = dir.join(crate_name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}
