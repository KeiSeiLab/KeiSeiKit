//! Filesystem walker helpers — shared across scanners.

use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// True if a path should be excluded from every scanner.
///
/// Skip rules:
/// - `plugins/marketplaces/...` — vendored upstream plugin code; Constructor
///   Pattern thresholds don't apply, refs are external. (backlog #1, 2026-05-12)
/// - `target/`, `node_modules/`, `.git/` — build/vendor noise that should never
///   contribute to architectural conflict counts.
///
/// Public because the standalone `WalkDir::new(root)` callers in
/// `scanners::cp` and `scanners::orphans` also need to apply it.
pub fn should_skip_path(path: &Path) -> bool {
    path.components().any(|c| {
        let s = c.as_os_str().to_string_lossy();
        s == "target" || s == "node_modules" || s == ".git" || s == "marketplaces"
    })
}

pub fn collect_markdown(root: &Path, sub: &str) -> Vec<PathBuf> {
    let base = root.join(sub);
    if !base.exists() {
        return Vec::new();
    }
    WalkDir::new(&base)
        .into_iter()
        .filter_entry(|e| !should_skip_path(e.path()))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        .map(|e| e.into_path())
        .collect()
}

pub fn collect_with_ext(root: &Path, sub: &str, ext: &str) -> Vec<PathBuf> {
    let base = root.join(sub);
    if !base.exists() {
        return Vec::new();
    }
    WalkDir::new(&base)
        .into_iter()
        .filter_entry(|e| !should_skip_path(e.path()))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|e2| e2 == ext))
        .map(|e| e.into_path())
        .collect()
}

pub fn read_lossy(path: &Path) -> String {
    fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default()
}

pub fn rel(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned()
}
