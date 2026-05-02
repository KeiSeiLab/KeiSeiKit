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

/// Max bytes we read from subprocess stdout/stderr to guard against runaway output.
const OUTPUT_CAP: usize = 16 * 1024 * 1024; // 16 MiB

#[derive(Debug)]
pub enum InvokeError {
    AtomNotFound(String),
    InputParse(String),
    InputInvalid(String),
    MissingInputSchema(String),
    /// `crate_name` in atom YAML failed the `kei-*` allowlist check.
    InvalidAtom(String),
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
            Self::InvalidAtom(msg) => write!(f, "invalid atom crate_name: {msg}"),
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

/// Validate `name` matches `^kei-[a-z][a-z0-9-]+$`; rejects path traversal and injection chars.
fn is_safe_crate_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 128 {
        return false;
    }
    // Forbidden substrings — absolute path indicators, separators, injection chars.
    for bad in &["/", "\\", "..", ":", "@", " "] {
        if name.contains(bad) {
            return false;
        }
    }
    // Must match kei-[a-z][a-z0-9-]+
    let bytes = name.as_bytes();
    if !name.starts_with("kei-") || bytes.len() < 5 {
        return false;
    }
    let after_prefix = &bytes[4..];
    if !after_prefix[0].is_ascii_lowercase() {
        return false;
    }
    after_prefix[1..].iter().all(|&b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

fn exec_atom(meta: &AtomMeta, input_json: &str) -> Result<Output, InvokeError> {
    if !is_safe_crate_name(&meta.crate_name) {
        return Err(InvokeError::InvalidAtom(format!(
            "crate_name '{}' fails kei-* allowlist",
            meta.crate_name
        )));
    }
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

fn cap_bytes(data: Vec<u8>, label: &str) -> Vec<u8> {
    if data.len() > OUTPUT_CAP {
        let mut v = data;
        v.truncate(OUTPUT_CAP);
        eprintln!("[kei-runtime] {label} truncated at {OUTPUT_CAP} bytes");
        v
    } else {
        data
    }
}

fn handle_subprocess_output(
    meta: &AtomMeta,
    mut out: std::process::Output,
) -> Result<Output, InvokeError> {
    out.stdout = cap_bytes(out.stdout, "stdout");
    out.stderr = cap_bytes(out.stderr, "stderr");
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

/// Resolve `<crate_name>` as binary: first `$KEI_RUNTIME_BIN_DIR/<name>`, then `$PATH`.
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
