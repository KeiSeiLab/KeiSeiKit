//! Constructor-Pattern violation reporter (read-only).
//!
//! Flags files >200 LOC (physical lines) and Rust/Python functions >30 LOC.
//! Read-only: we do NOT propose a refactor here; refactor-engine decides.

use crate::conflict::{Category, Conflict, Severity};
use crate::tree::{read_lossy, rel, should_skip_path};
use regex::Regex;
use std::path::Path;
use walkdir::WalkDir;

const FILE_LIMIT: usize = 200;
const FN_LIMIT: usize = 30;

pub fn scan(root: &Path) -> Vec<Conflict> {
    let mut out = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if skip_dir(path) {
            continue;
        }
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if !["rs", "py", "sh", "ts", "js", "go", "swift"].contains(&ext) {
            continue;
        }
        let content = read_lossy(path);
        let line_count = content.lines().count();
        let file_rel = rel(root, path);
        if line_count > FILE_LIMIT {
            out.push(file_violation(&file_rel, line_count));
        }
        for (name, len) in long_fns(&content, ext) {
            if len > FN_LIMIT {
                out.push(fn_violation(&file_rel, &name, len));
            }
        }
    }
    out
}

fn skip_dir(path: &Path) -> bool {
    should_skip_path(path)
}

fn long_fns(content: &str, ext: &str) -> Vec<(String, usize)> {
    let pattern = match ext {
        "rs" => r"(?m)^\s*(?:pub\s+)?(?:async\s+)?fn\s+([a-zA-Z0-9_]+)",
        "py" => r"(?m)^\s*def\s+([a-zA-Z0-9_]+)",
        _ => return Vec::new(),
    };
    let rx = Regex::new(pattern).expect("static regex");
    let starts: Vec<(usize, String)> = rx
        .captures_iter(content)
        .filter_map(|c| {
            let name = c.get(1)?.as_str().to_string();
            let pos = c.get(0)?.start();
            let line = content[..pos].lines().count();
            Some((line, name))
        })
        .collect();
    let total = content.lines().count();
    starts
        .iter()
        .enumerate()
        .map(|(i, (line, name))| {
            let next = starts.get(i + 1).map(|(l, _)| *l).unwrap_or(total);
            (name.clone(), next.saturating_sub(*line))
        })
        .collect()
}

fn file_violation(file: &str, loc: usize) -> Conflict {
    Conflict::new(
        Category::Cp,
        Severity::Medium,
        vec![file.to_string()],
        format!("file is {} LOC (limit 200)", loc),
        "split into smaller cubes; one file = one class = one responsibility".to_string(),
        false,
    )
}

fn fn_violation(file: &str, name: &str, loc: usize) -> Conflict {
    Conflict::new(
        Category::Cp,
        Severity::Low,
        vec![file.to_string()],
        format!("function '{}' is {} LOC (limit 30)", name, loc),
        "split into helper subfunctions".to_string(),
        false,
    )
}
