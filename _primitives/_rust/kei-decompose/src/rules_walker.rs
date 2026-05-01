//! Directory walker for rule `.md` files.
//!
//! Walks `<rules-dir>/*.md`, `specialty/*.md`, and `projects/*.md` (depth
//! ≤ 2). Skips files starting with `_` and the registry index (`RULES.md`).
//!
//! Constructor Pattern: this cube owns the walk + eligibility filter only.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Collect all eligible rule `.md` files from `rules_dir` and its known
/// subdirectories (`specialty/`, `projects/`), sorted.
pub fn collect_rule_files(rules_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    if !rules_dir.exists() {
        return Ok(out);
    }
    scan_dir(rules_dir, &mut out)?;
    for sub in &["specialty", "projects"] {
        let sub_dir = rules_dir.join(sub);
        if sub_dir.is_dir() {
            scan_dir(&sub_dir, &mut out)?;
        }
    }
    out.sort();
    Ok(out)
}

fn scan_dir(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir).with_context(|| format!("read dir {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if is_eligible(&path) {
            out.push(path);
        }
    }
    Ok(())
}

fn is_eligible(path: &Path) -> bool {
    if !path.is_file() { return false; }
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if ext != "md" { return false; }
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    !stem.starts_with('_') && stem != "RULES"
}
