//! Spawn an atom subprocess and return its JSON output.
//!
//! Atom IDs are `<crate-name>::<verb>` — e.g. `kei-task::create`. The
//! crate name resolves to an executable using the same contract as
//! `kei-runtime`: first `$KEI_RUNTIME_BIN_DIR/<crate>`, then walk `PATH`.
//!
//! The atom is invoked as `<crate> run-atom <verb>`, JSON on stdin, JSON
//! on stdout. Exit code 0 = ok; anything else = `AtomFailed`. Tests can
//! substitute a mock binary by pointing `KEI_RUNTIME_BIN_DIR` at a temp
//! dir whose `<crate>` is a shell script that echoes its stdin (see
//! `tests/pipe_smoke.rs`).

use serde_json::Value;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug, thiserror::Error)]
pub enum ExecError {
    #[error("atom id `{0}` must be `<crate>::<verb>`")]
    BadAtomId(String),
    #[error("binary `{0}` not found on PATH or KEI_RUNTIME_BIN_DIR")]
    BinaryNotFound(String),
    #[error("atom `{atom}` exited {code}: {stderr}")]
    AtomFailed { atom: String, code: i32, stderr: String },
    #[error("subprocess {0}: {1}")]
    Subprocess(String, std::io::Error),
    #[error("atom `{atom}` stdout not JSON: {err}; stdout was: {stdout}")]
    OutputParse { atom: String, err: String, stdout: String },
    #[error("serialize input: {0}")]
    InputSerialize(String),
    #[error("cache error: {0}")]
    Cache(String),
}

/// Parse an atom id into `(crate, verb)`. Rejects empty halves.
pub fn split_atom(atom: &str) -> Result<(&str, &str), ExecError> {
    match atom.split_once("::") {
        Some((c, v)) if !c.is_empty() && !v.is_empty() => Ok((c, v)),
        _ => Err(ExecError::BadAtomId(atom.into())),
    }
}

/// Invoke an atom, returning the parsed JSON result (the atom's own
/// stdout — callers decide how to slot it under `{"atom":..., "result":...}`).
pub fn run_atom(atom: &str, input: &Value) -> Result<Value, ExecError> {
    let (crate_name, verb) = split_atom(atom)?;
    let bin = resolve_binary(crate_name)
        .ok_or_else(|| ExecError::BinaryNotFound(crate_name.into()))?;
    let stdin_bytes = serde_json::to_vec(input)
        .map_err(|e| ExecError::InputSerialize(e.to_string()))?;
    let output = spawn_and_wait(&bin, verb, &stdin_bytes, atom)?;
    parse_output(atom, output)
}

/// Outcome label accompanying a cache-aware invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheOutcome {
    /// Returned from the cache; atom was NOT invoked.
    Hit,
    /// Cache miss; atom was invoked and the result stored.
    Fresh,
}

impl CacheOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            CacheOutcome::Hit => "cache",
            CacheOutcome::Fresh => "fresh",
        }
    }
}

/// Cache-aware atom invocation. On hit returns cached JSON; on miss calls
/// [`run_atom`], stores the serialised result under the computed key with
/// `ttl_sec`, and returns `Fresh`. Cache I/O errors are surfaced via
/// [`ExecError::Cache`] so the caller can distinguish from atom failures.
pub fn run_atom_cached(
    conn: &rusqlite::Connection,
    atom: &str,
    input: &Value,
    ttl_sec: i64,
) -> Result<(Value, CacheOutcome), ExecError> {
    let key = kei_cache::key::cache_key(atom, input);
    let hit = kei_cache::store::get(conn, &key).map_err(|e| ExecError::Cache(e.to_string()))?;
    match hit {
        Some(payload) => load_hit(conn, atom, payload),
        None => load_miss(conn, atom, input, &key, ttl_sec),
    }
}

fn load_hit(
    conn: &rusqlite::Connection,
    atom: &str,
    payload: String,
) -> Result<(Value, CacheOutcome), ExecError> {
    let _ = kei_cache::store::bump(conn, "hits");
    let value: Value =
        serde_json::from_str(&payload).map_err(|e| ExecError::OutputParse {
            atom: atom.into(),
            err: e.to_string(),
            stdout: payload,
        })?;
    Ok((value, CacheOutcome::Hit))
}

fn load_miss(
    conn: &rusqlite::Connection,
    atom: &str,
    input: &Value,
    key: &str,
    ttl_sec: i64,
) -> Result<(Value, CacheOutcome), ExecError> {
    let result = run_atom(atom, input)?;
    let payload =
        serde_json::to_string(&result).map_err(|e| ExecError::InputSerialize(e.to_string()))?;
    kei_cache::store::put(conn, key, atom, &payload, ttl_sec)
        .map_err(|e| ExecError::Cache(e.to_string()))?;
    let _ = kei_cache::store::bump(conn, "misses");
    Ok((result, CacheOutcome::Fresh))
}

fn spawn_and_wait(
    bin: &PathBuf,
    verb: &str,
    input_bytes: &[u8],
    atom: &str,
) -> Result<std::process::Output, ExecError> {
    let mut child = Command::new(bin)
        .arg("run-atom")
        .arg(verb)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| ExecError::Subprocess(format!("spawn {atom}"), e))?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input_bytes)
            .map_err(|e| ExecError::Subprocess(format!("stdin {atom}"), e))?;
    }
    child
        .wait_with_output()
        .map_err(|e| ExecError::Subprocess(format!("wait {atom}"), e))
}

fn parse_output(atom: &str, out: std::process::Output) -> Result<Value, ExecError> {
    if !out.status.success() {
        return Err(ExecError::AtomFailed {
            atom: atom.into(),
            code: out.status.code().unwrap_or(-1),
            stderr: String::from_utf8_lossy(&out.stderr).trim().into(),
        });
    }
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    serde_json::from_str(stdout.trim()).map_err(|e| ExecError::OutputParse {
        atom: atom.into(),
        err: e.to_string(),
        stdout,
    })
}

/// Resolve `<crate>` as an executable. Mirrors `kei-runtime::invoke`.
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
