//! Walk up from `cwd` to the user's `$HOME` (or `/`) and collect every
//! `CLAUDE.md`, `AGENTS.md`, `SOUL.md` encountered at each directory level.
//!
//! Returned order is **nearest-first**: index 0 is the file in `cwd`
//! itself, index 1 is in `cwd.parent()`, and so on. The outer caller
//! (`inject::build_system_prompt`) relies on this ordering to make sure
//! the most-specific context wins when truncation is required.
//!
//! Hard stops:
//!   - At `$HOME` (after processing it).
//!   - At `/` (after processing it).
//!   - On the first directory whose name is `node_modules`, `.git`, or
//!     `_archive` (we still process the level above; we just don't
//!     descend into those).
//!
//! Safety:
//!   - Symlinks are NOT followed (`WalkDir::follow_links(false)`).
//!   - Each file is capped at 1 MiB; oversize content is truncated with a
//!     trailing `\n[truncated]` marker.

use super::types::{ContextKind, DiscoveredFile};
use std::path::{Path, PathBuf};

/// Hard read cap per file. Anything larger is truncated.
const MAX_FILE_BYTES: usize = 1024 * 1024;

/// Directory names we never descend into when listing.
const SKIP_DIRS: &[&str] = &["node_modules", ".git", "_archive"];

/// Walk up from `cwd`, collecting context files at every level.
pub fn discover(cwd: &Path) -> Vec<DiscoveredFile> {
    let stop_at = home_dir();
    let mut out: Vec<DiscoveredFile> = Vec::new();
    let mut current: Option<PathBuf> = Some(cwd.to_path_buf());
    while let Some(dir) = current {
        if !is_safe_dir(&dir) {
            break;
        }
        for f in scan_level(&dir) {
            out.push(f);
        }
        if reached_stop(&dir, stop_at.as_deref()) {
            break;
        }
        current = dir.parent().map(Path::to_path_buf);
    }
    out
}

/// Resolve `$HOME` once. Treated as a stop boundary; we still process the
/// home directory itself, then break out of the walk.
fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

/// True when `dir` is not one of the explicitly-skipped names. We still
/// scan it; the skip semantics apply to descent only, but as a defence
/// against accidental cwd inside `node_modules` we also bail here.
fn is_safe_dir(dir: &Path) -> bool {
    let Some(name) = dir.file_name() else { return true };
    let n = name.to_string_lossy();
    !SKIP_DIRS.iter().any(|s| *s == n.as_ref())
}

/// True when we should stop after processing `dir` (reached `$HOME` or `/`).
fn reached_stop(dir: &Path, stop_at: Option<&Path>) -> bool {
    if dir.parent().is_none() {
        return true;
    }
    matches!(stop_at, Some(home) if dir == home)
}

/// Read all known context files at a single directory level.
fn scan_level(dir: &Path) -> Vec<DiscoveredFile> {
    let mut hits = Vec::new();
    for (name, kind) in candidates() {
        let p = dir.join(name);
        if let Some(file) = read_capped(&p, *kind) {
            hits.push(file);
        }
    }
    hits
}

/// Filenames + their classification. Order here determines the within-level
/// order in the returned vector.
fn candidates() -> &'static [(&'static str, ContextKind)] {
    &[
        ("CLAUDE.md", ContextKind::ClaudeMd),
        ("AGENTS.md", ContextKind::AgentsMd),
        ("SOUL.md", ContextKind::SoulMd),
    ]
}

/// Read `path` if it exists and is a regular file (not a symlink), capped
/// at `MAX_FILE_BYTES`. Returns `None` on missing/unreadable/symlink.
fn read_capped(path: &Path, kind: ContextKind) -> Option<DiscoveredFile> {
    let meta = std::fs::symlink_metadata(path).ok()?;
    if meta.file_type().is_symlink() || !meta.file_type().is_file() {
        return None;
    }
    let raw = std::fs::read_to_string(path).ok()?;
    let content = truncate_with_marker(raw);
    Some(DiscoveredFile {
        path: path.to_path_buf(),
        content,
        kind,
    })
}

/// Cut `s` at `MAX_FILE_BYTES` (UTF-8-safe) and append a `[truncated]`
/// marker. Returns `s` unchanged when it already fits.
fn truncate_with_marker(s: String) -> String {
    if s.len() <= MAX_FILE_BYTES {
        return s;
    }
    let mut cut = MAX_FILE_BYTES;
    while cut > 0 && !s.is_char_boundary(cut) {
        cut -= 1;
    }
    let mut out = s[..cut].to_owned();
    out.push_str("\n[truncated]");
    out
}
