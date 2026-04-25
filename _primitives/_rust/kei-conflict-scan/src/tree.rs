//! Filesystem walker helpers — shared across scanners.

use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn collect_markdown(root: &Path, sub: &str) -> Vec<PathBuf> {
    let base = root.join(sub);
    if !base.exists() {
        return Vec::new();
    }
    WalkDir::new(&base)
        .into_iter()
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
