//! Atom-scaffolding generator — pure Rust templating.
//!
//! Reads the five templates in `<repo>/_templates/atom/`, substitutes the
//! six placeholder tokens (`__CRATE__`, `__CRATE_SNAKE__`, `__VERB__`,
//! `__VERB_SNAKE__`, `__KIND__`, `__DESCRIPTION__`), and writes the
//! resulting files into `<repo>/_primitives/_rust/<crate>/`.
//!
//! No shell-out. No sed. The Rust string replace cannot be coerced into
//! executing a secondary expression, so the description-injection attack
//! class defended by `form::validate_description` is structurally gone —
//! the whitelist stays as defence-in-depth, not the primary barrier.
//!
//! Atomicity: every file written is accumulated in a rollback list; on
//! any write failure the accumulator is flushed (files deleted best-
//! effort) before the error surfaces. Matches new-atom.sh's `trap ERR`.

use crate::form::ForgeRequest;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

mod placeholders;
mod paths;
mod rollback;
#[cfg(test)]
mod atom_tests;

use placeholders::Placeholders;
use paths::TargetPaths;
use rollback::Rollback;

/// Structured failure modes returned by the pure-Rust generator.
#[derive(Debug)]
pub enum GenerateError {
    /// `<repo>/_primitives/_rust/<crate>/` does not exist.
    CrateNotFound(PathBuf),
    /// One of the five target files already exists — refuse to overwrite.
    FileExists(PathBuf),
    /// `<repo>/_templates/atom/` missing or a template file unreadable.
    TemplateMissing(PathBuf),
    /// Filesystem I/O failed mid-write.
    Io(std::io::Error, PathBuf),
}

impl std::fmt::Display for GenerateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CrateNotFound(p) => write!(f, "crate directory not found: {}", p.display()),
            Self::FileExists(p) => write!(f, "file already exists: {}", p.display()),
            Self::TemplateMissing(p) => write!(f, "template missing: {}", p.display()),
            Self::Io(e, p) => write!(f, "i/o error on {}: {e}", p.display()),
        }
    }
}

/// Result of a scaffolding attempt — wire-compatible with the previous
/// shell-out implementation.
#[derive(Debug, Serialize)]
pub struct ForgeResult {
    pub success: bool,
    pub files: Vec<String>,
    pub errors: Vec<String>,
}

impl ForgeResult {
    pub fn ok(files: Vec<String>) -> Self {
        Self { success: true, files, errors: Vec::new() }
    }

    pub fn fail(err: impl Into<String>) -> Self {
        Self { success: false, files: Vec::new(), errors: vec![err.into()] }
    }
}

/// Locate the repo root by walking up from CARGO_MANIFEST_DIR until we
/// see `_templates/atom/`. Falls back to CWD if the env var is unset
/// (detached binary) or nothing matches (ship-of-Theseus invariant).
pub fn repo_root() -> PathBuf {
    let start = std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
    let mut cur: &Path = &start;
    loop {
        if cur.join("_templates/atom").is_dir() {
            return cur.to_path_buf();
        }
        match cur.parent() {
            Some(p) => cur = p,
            None => return start,
        }
    }
}

/// Thin wrapper the HTTP layer calls — discovers repo root, invokes the
/// pure-Rust core, projects errors onto the public `ForgeResult` shape.
pub fn forge(req: &ForgeRequest) -> ForgeResult {
    let root = repo_root();
    match generate_atom(req, &root) {
        Ok(files) => ForgeResult::ok(
            files
                .into_iter()
                .map(|p| rel_to_root(&p, &root))
                .collect(),
        ),
        Err(e) => ForgeResult::fail(e.to_string()),
    }
}

/// Core entry point — pure fn over (req, root), exposed for unit tests.
///
/// On success returns the five absolute paths in declaration order. On
/// failure, no partial writes survive (rollback on drop).
pub fn generate_atom(
    req: &ForgeRequest,
    repo_root: &Path,
) -> Result<Vec<PathBuf>, GenerateError> {
    let placeholders = Placeholders::from_request(req);
    let targets = TargetPaths::resolve(repo_root, req)?;
    let template_dir = repo_root.join("_templates/atom");

    if !template_dir.is_dir() {
        return Err(GenerateError::TemplateMissing(template_dir));
    }

    targets.assert_none_exist()?;
    targets.ensure_parent_dirs()?;

    let mut rollback = Rollback::new();
    for (template_rel, dest) in targets.pairs().iter() {
        let src = template_dir.join(template_rel);
        let content = fs::read_to_string(&src)
            .map_err(|_| GenerateError::TemplateMissing(src.clone()))?;
        let rendered = placeholders.substitute(&content);
        write_or_rollback(dest, &rendered, &mut rollback)?;
    }
    Ok(rollback.finish())
}

/// Write one file, register in the rollback list, rollback on error.
fn write_or_rollback(
    dest: &Path,
    content: &str,
    rollback: &mut Rollback,
) -> Result<(), GenerateError> {
    match fs::write(dest, content) {
        Ok(()) => {
            rollback.record(dest.to_path_buf());
            Ok(())
        }
        Err(e) => Err(GenerateError::Io(e, dest.to_path_buf())),
    }
}

/// Render path relative to repo-root for the JSON response.
pub(crate) fn rel_to_root(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.to_string_lossy().into_owned())
}
