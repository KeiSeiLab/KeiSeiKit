//! Orphan-reference detector.
//!
//! Finds `[[wikilink]]` and `handoffs: - name` references whose targets
//! do not exist anywhere under the root. Case-insensitive basename match.

use crate::conflict::{Category, Conflict, Severity};
use crate::tree::{read_lossy, rel};
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;

fn all_basenames(root: &Path) -> HashSet<String> {
    let mut out = HashSet::new();
    for e in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if e.file_type().is_file() {
            if let Some(stem) = e.path().file_stem().and_then(|s| s.to_str()) {
                out.insert(stem.to_lowercase());
            }
        }
    }
    out
}

fn extract_wikilinks(content: &str) -> Vec<String> {
    let rx = Regex::new(r"\[\[([^\]\|#]+?)(?:#[^\]]*)?(?:\|[^\]]*)?\]\]").expect("static regex");
    rx.captures_iter(content)
        .map(|c| c[1].trim().to_lowercase())
        .collect()
}

fn extract_handoffs(content: &str) -> Vec<String> {
    let rx = Regex::new(r"(?im)^\s*-\s*\*\*([a-z0-9][a-z0-9_-]{2,})\*\*").expect("static regex");
    rx.captures_iter(content)
        .map(|c| c[1].trim().to_lowercase())
        .collect()
}

pub fn scan(root: &Path) -> Vec<Conflict> {
    let index = all_basenames(root);
    let mut out = Vec::new();
    for e in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if !e.file_type().is_file() {
            continue;
        }
        if e.path().extension().is_none_or(|x| x != "md") {
            continue;
        }
        let content = read_lossy(e.path());
        let file_rel = rel(root, e.path());
        for target in extract_wikilinks(&content) {
            if !index.contains(&target) {
                out.push(orphan(&file_rel, &target, "wikilink"));
            }
        }
        for target in extract_handoffs(&content) {
            if !index.contains(&target) && target.contains('-') {
                out.push(orphan(&file_rel, &target, "handoff"));
            }
        }
    }
    out
}

fn orphan(file: &str, target: &str, kind: &str) -> Conflict {
    Conflict::new(
        Category::Orphans,
        Severity::Low,
        vec![file.to_string()],
        format!("{} target '{}' not found under root", kind, target),
        "either create the target file or remove the stale reference".to_string(),
        true,
    )
}
