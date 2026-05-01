//! Walk a directory and load every valid `SKILL.md`.
//!
//! Used by [`crate::registry::SkillRegistry::new`] at daemon start. Lossy
//! by default — invalid skills surface as `LoadOutcome::Invalid` so the
//! daemon can log them without crashing the boot path.

use crate::format::Skill;
use crate::validator::{validate, ValidationIssue};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Per-file outcome of `load_all`.
#[derive(Debug)]
pub enum LoadOutcome {
    Loaded(Skill),
    Invalid { path: PathBuf, issues: Vec<ValidationIssue> },
    Io { path: PathBuf, error: io::Error },
}

/// Walk `dir` recursively for `SKILL.md` files. Each is read, validated,
/// and bucketed into a `LoadOutcome`. The loader never fails the whole
/// directory — bad eggs surface as `Invalid`/`Io` for caller logging.
///
/// Skips files whose path contains a `_archive` segment (Hermes /
/// agentskills archive convention) so retired skills don't get re-loaded.
pub fn load_all(dir: &Path) -> Vec<LoadOutcome> {
    let mut out = Vec::new();
    if !dir.exists() {
        return out;
    }
    for entry in WalkDir::new(dir).follow_links(false).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }
        let p = entry.path();
        if p.file_name().and_then(|s| s.to_str()) != Some("SKILL.md") {
            continue;
        }
        if is_archived(p) {
            continue;
        }
        out.push(load_one(p));
    }
    out
}

fn is_archived(path: &Path) -> bool {
    path.components()
        .any(|c| c.as_os_str().to_str().is_some_and(|s| s == "_archive"))
}

fn load_one(path: &Path) -> LoadOutcome {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return LoadOutcome::Io { path: path.to_path_buf(), error: e },
    };
    match validate(&content, path) {
        Ok(skill) => LoadOutcome::Loaded(skill),
        Err(issues) => LoadOutcome::Invalid { path: path.to_path_buf(), issues },
    }
}

/// Shorthand for callers that only want the valid skills (drops Invalid/Io).
pub fn loaded_only(outcomes: Vec<LoadOutcome>) -> Vec<Skill> {
    outcomes
        .into_iter()
        .filter_map(|o| match o {
            LoadOutcome::Loaded(s) => Some(s),
            _ => None,
        })
        .collect()
}

/// Count outcomes by kind for diagnostics. Returns `(loaded, invalid, io)`.
pub fn tally(outcomes: &[LoadOutcome]) -> (usize, usize, usize) {
    let mut l = 0usize;
    let mut i = 0usize;
    let mut io = 0usize;
    for o in outcomes {
        match o {
            LoadOutcome::Loaded(_) => l += 1,
            LoadOutcome::Invalid { .. } => i += 1,
            LoadOutcome::Io { .. } => io += 1,
        }
    }
    (l, i, io)
}
