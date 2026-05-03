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
use crate::invoke_io::{capture_with_cap, Captured};
use crate::validate::validate_input;
use serde::Serialize;
use serde_json::Value;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub use crate::invoke_error::InvokeError;

/// Max bytes we read from subprocess stdout/stderr to guard against
/// runaway output. Streamed reads enforce this DURING capture (not
/// post-hoc) — see `invoke_io.rs`. The constant is kept here as the
/// public name documented in `lib.rs`.
pub const OUTPUT_CAP: usize = 16 * 1024 * 1024; // 16 MiB

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
    write_stdin(&mut child, input_json)?;
    let captured = capture_with_cap(child)
        .map_err(|e| InvokeError::SubprocessError(format!("wait: {e}")))?;
    handle_captured(meta, captured)
}

/// Write the atom's input JSON to the child's stdin, dropping the handle
/// so the writer side closes (otherwise the child would block on EOF).
fn write_stdin(child: &mut std::process::Child, input_json: &str) -> Result<(), InvokeError> {
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input_json.as_bytes())
            .map_err(|e| InvokeError::SubprocessError(format!("write stdin: {e}")))?;
    }
    Ok(())
}

/// Map a `Captured` (potentially-truncated) child result into our typed
/// `Output` / `InvokeError`. Truncation is surfaced in `SubprocessError`
/// when the child was killed for exceeding the cap; the parent decides
/// what exit code to return.
fn handle_captured(meta: &AtomMeta, c: Captured) -> Result<Output, InvokeError> {
    if c.truncated {
        return Err(InvokeError::SubprocessError(format!(
            "atom `{}` killed: stdout/stderr exceeded {OUTPUT_CAP} byte cap",
            meta.full_id
        )));
    }
    if c.status_code != 0 {
        let stderr = String::from_utf8_lossy(&c.stderr).trim().to_string();
        return Err(InvokeError::AtomFailed {
            atom: meta.full_id.clone(),
            code: c.status_code,
            stderr,
        });
    }
    let stdout = String::from_utf8_lossy(&c.stdout);
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
