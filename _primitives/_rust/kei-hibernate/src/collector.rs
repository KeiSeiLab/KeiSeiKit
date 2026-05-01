//! File collector — enumerates what goes into a hibernate bundle.
//!
//! Rules:
//! 1. All `*.sqlite` under `~/.claude/agents/` and `~/.claude/memory/`
//!    (when those paths live inside `kit_root`).
//! 2. Entire trees under `_capabilities/`, `_roles/`, `_blocks/`,
//!    `_agents/`, `skills/`, `hooks/` at `kit_root`.
//!
//! Anything else is excluded. Symlinks are not followed.

use std::path::{Path, PathBuf};

/// Top-level directories inside `kit_root` that are included wholesale.
pub const KIT_SUBTREES: &[&str] = &[
    "_capabilities",
    "_roles",
    "_blocks",
    "_agents",
    "skills",
    "hooks",
];

/// Sub-paths inside `~/.claude/` that contribute `*.sqlite` files.
pub const SQLITE_SUBPATHS: &[&str] = &[".claude/agents", ".claude/memory"];

#[derive(Debug, Clone)]
pub struct Found {
    pub abs: PathBuf,
    pub rel: String,
}

/// Walk `kit_root`, returning every file eligible for the bundle.
///
/// Each `Found` carries the absolute path and a forward-slash
/// bundle-relative path (the archive entry name). The list is
/// sorted lexicographically for reproducible bundles.
pub fn collect(kit_root: &Path) -> std::io::Result<Vec<Found>> {
    let mut out: Vec<Found> = Vec::new();
    for sub in KIT_SUBTREES {
        walk_subtree(kit_root, sub, &mut out)?;
    }
    for sub in SQLITE_SUBPATHS {
        walk_sqlite(kit_root, sub, &mut out)?;
    }
    out.sort_by(|a, b| a.rel.cmp(&b.rel));
    Ok(out)
}

/// Recursively collect every file under `kit_root/subtree_name`.
fn walk_subtree(kit_root: &Path, sub: &str, out: &mut Vec<Found>) -> std::io::Result<()> {
    let root = kit_root.join(sub);
    if !root.is_dir() {
        return Ok(());
    }
    walk_any(&root, kit_root, out, |_p| true)
}

/// Collect only `*.sqlite` files under `kit_root/sub`.
fn walk_sqlite(kit_root: &Path, sub: &str, out: &mut Vec<Found>) -> std::io::Result<()> {
    let root = kit_root.join(sub);
    if !root.is_dir() {
        return Ok(());
    }
    walk_any(&root, kit_root, out, |p| {
        p.extension().map(|x| x == "sqlite").unwrap_or(false)
    })
}

/// Generic recursive walker. `filter` decides inclusion per file.
/// Skips symlinks to avoid exporter / importer confusion.
fn walk_any<F>(dir: &Path, kit_root: &Path, out: &mut Vec<Found>, filter: F) -> std::io::Result<()>
where
    F: Fn(&Path) -> bool + Copy,
{
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        if meta.file_type().is_symlink() {
            continue;
        }
        let path = entry.path();
        if meta.is_dir() {
            walk_any(&path, kit_root, out, filter)?;
        } else if meta.is_file() && filter(&path) {
            push_found(&path, kit_root, out);
        }
    }
    Ok(())
}

/// Convert absolute path → bundle-relative (forward slash) and push.
fn push_found(abs: &Path, kit_root: &Path, out: &mut Vec<Found>) {
    if let Ok(rel) = abs.strip_prefix(kit_root) {
        let rel_s = rel.to_string_lossy().replace('\\', "/");
        out.push(Found { abs: abs.to_path_buf(), rel: rel_s });
    }
}
