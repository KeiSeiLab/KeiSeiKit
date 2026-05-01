//! Top-level walker for `~/Projects/`.
//!
//! Constructor Pattern: one cube = directory enumeration. Returns a flat
//! list of `ProjectEntry` for each top-level dir under the supplied root.
//! Hidden dirs (leading `.`) and `_archive` are skipped — they are never
//! active project workspaces.

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// One enumerated project candidate. `has_git` is a quick precondition
/// check: a `.git` entry exists at the project root. Detailed git state
/// is delegated to `git_state.rs`.
#[derive(Debug, Clone)]
pub struct ProjectEntry {
    /// Absolute path to the project directory.
    pub path: PathBuf,
    /// Basename of `path` (final component).
    pub name: String,
    /// True if `path/.git` exists (file or dir). Does NOT validate the
    /// repo — that lives in `git_state::detect_git_state`.
    pub has_git: bool,
}

/// Returns true if a directory entry should be skipped during the walk.
/// We skip dot-prefixed names (hidden / IDE / claude metadata) and the
/// `_archive` convention used across our portfolio for retired work.
fn is_excluded(name: &str) -> bool {
    name.starts_with('.') || name == "_archive"
}

/// Detect `.git` (file or dir) at `project_root`. Both shapes are valid:
/// a regular `.git` directory (standard repo) or a `.git` file (git
/// worktree / submodule pointer).
fn has_git_marker(project_root: &Path) -> bool {
    project_root.join(".git").exists()
}

/// Build a `ProjectEntry` from one walkdir entry. Returns `None` if the
/// entry is not a directory or its name is excluded.
fn entry_from(dirent: &walkdir::DirEntry) -> Option<ProjectEntry> {
    if !dirent.file_type().is_dir() {
        return None;
    }
    let name = dirent.file_name().to_str()?.to_string();
    if is_excluded(&name) {
        return None;
    }
    let path = dirent.path().to_path_buf();
    let has_git = has_git_marker(&path);
    Some(ProjectEntry { path, name, has_git })
}

/// Walks `projects_root` exactly one level deep and returns one
/// `ProjectEntry` per top-level subdirectory (skipping hidden +
/// `_archive`).
///
/// The root itself is `min_depth(1)` so we never include
/// `projects_root` as a project. `max_depth(1)` keeps the walk shallow —
/// nested repos / monorepos are NOT enumerated as separate projects;
/// that's a deliberate choice to mirror the `~/Projects/` convention of
/// "one repo per top-level dir".
///
/// Returns `Ok(empty)` if `projects_root` does not exist — callers
/// (CLI / library) are expected to handle a "no projects yet" state
/// rather than treating it as a hard error.
pub fn walk_projects_root(projects_root: &Path) -> std::io::Result<Vec<ProjectEntry>> {
    if !projects_root.exists() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    let walker = WalkDir::new(projects_root)
        .min_depth(1)
        .max_depth(1)
        .follow_links(false)
        .into_iter();
    for dirent in walker.flatten() {
        if let Some(entry) = entry_from(&dirent) {
            out.push(entry);
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}
