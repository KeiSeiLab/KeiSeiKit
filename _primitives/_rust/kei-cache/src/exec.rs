//! Atom invocation on cache miss.
//!
//! Constructor Pattern: `AtomExecutor` trait = one-method contract
//! (atom_id + canonical input → JSON payload string). `SubprocessExecutor`
//! is the production impl — mirrors the kei-runtime binary-resolution
//! rules (`KEI_RUNTIME_BIN_DIR` → `$PATH`) and spawns
//! `<crate> run-atom <verb>` with the input on stdin.
//!
//! Kind-safety: before invoking we consult `kei-atom-discovery` to obtain
//! `AtomKind`. `command` and `stream` are refused ("unsafe to cache");
//! `query` and `transform` pass through.

use anyhow::{anyhow, Context, Result};
use kei_atom_discovery::{discover_atoms, AtomKind, AtomMeta};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Strategy for invoking an atom after a cache miss.
///
/// Implementations MUST return the atom's raw JSON stdout as a String.
/// They MUST NOT perform any caching themselves.
pub trait AtomExecutor {
    fn execute(&self, atom_id: &str, input_json: &str) -> Result<String>;
}

/// Production executor: resolves atom metadata via kei-atom-discovery,
/// refuses non-cacheable kinds, and spawns `<crate> run-atom <verb>`.
pub struct SubprocessExecutor {
    atoms_root: PathBuf,
}

impl SubprocessExecutor {
    pub fn new(atoms_root: impl Into<PathBuf>) -> Self {
        Self { atoms_root: atoms_root.into() }
    }

    fn find_meta(&self, atom_id: &str) -> Result<AtomMeta> {
        discover_atoms(&self.atoms_root)
            .into_iter()
            .find(|a| a.full_id == atom_id)
            .ok_or_else(|| anyhow!("no atom matching `{atom_id}` under {}", self.atoms_root.display()))
    }
}

impl AtomExecutor for SubprocessExecutor {
    fn execute(&self, atom_id: &str, input_json: &str) -> Result<String> {
        let meta = self.find_meta(atom_id)?;
        ensure_cacheable(&meta.kind, atom_id)?;
        run_subprocess(&meta, input_json)
    }
}

/// Gate: only pure kinds may be cached. Command has side effects; stream is
/// incremental so caching the first frame would be misleading.
pub fn ensure_cacheable(kind: &AtomKind, atom_id: &str) -> Result<()> {
    match kind {
        AtomKind::Query | AtomKind::Transform => Ok(()),
        AtomKind::Command => Err(anyhow!(
            "atom `{atom_id}` has kind=command (side effects); unsafe to cache"
        )),
        AtomKind::Stream => Err(anyhow!(
            "atom `{atom_id}` has kind=stream (incremental); unsafe to cache"
        )),
    }
}

/// Spawn `<crate> run-atom <verb>` with `input_json` on stdin; return stdout.
fn run_subprocess(meta: &AtomMeta, input_json: &str) -> Result<String> {
    let bin = resolve_binary(&meta.crate_name)
        .ok_or_else(|| anyhow!("binary `{}` not on PATH or KEI_RUNTIME_BIN_DIR", meta.crate_name))?;
    let mut child = Command::new(&bin)
        .arg("run-atom")
        .arg(&meta.verb)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("spawn {}", bin.display()))?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input_json.as_bytes())
            .context("write stdin to atom subprocess")?;
    }
    let out = child.wait_with_output().context("wait on atom subprocess")?;
    if !out.status.success() {
        let code = out.status.code().unwrap_or(-1);
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        return Err(anyhow!("atom `{}` exited {code}: {stderr}", meta.full_id));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Resolve binary by name:
///   1. `$KEI_RUNTIME_BIN_DIR/<crate>` when env var is set and file exists
///   2. Walk `$PATH`, return first `<dir>/<crate>` that exists
pub fn resolve_binary(crate_name: &str) -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("KEI_RUNTIME_BIN_DIR") {
        let candidate = PathBuf::from(dir).join(crate_name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    let path = std::env::var("PATH").ok()?;
    for dir in std::env::split_paths(&path) {
        let candidate: PathBuf = Path::new(&dir).join(crate_name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_command_kind() {
        let err = ensure_cacheable(&AtomKind::Command, "atom:x").unwrap_err();
        assert!(err.to_string().contains("unsafe to cache"));
    }

    #[test]
    fn rejects_stream_kind() {
        let err = ensure_cacheable(&AtomKind::Stream, "atom:x").unwrap_err();
        assert!(err.to_string().contains("unsafe to cache"));
    }

    #[test]
    fn accepts_query_kind() {
        ensure_cacheable(&AtomKind::Query, "atom:x").unwrap();
    }

    #[test]
    fn accepts_transform_kind() {
        ensure_cacheable(&AtomKind::Transform, "atom:x").unwrap();
    }
}
