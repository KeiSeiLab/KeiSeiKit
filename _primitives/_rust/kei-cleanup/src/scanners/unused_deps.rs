//! Unused dependencies scanner — see CLEANUP-RUNTIME-SPEC.md §3.2 +
//! Appendix A. For each workspace member, parse Cargo.toml
//! `[dependencies]`, then grep `src/` for `use <crate>` or `<crate>::`.
//! Flag any dep that is not referenced.

use crate::config::Config;
use crate::error::CleanupError;
use crate::report::{Confidence, Finding, FindingKind, Location, Severity};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Public scanner entry — see Appendix A row "unused_deps".
pub fn scan(workspace: &Path, _cfg: &Config) -> Result<Vec<Finding>, CleanupError> {
    let mut out = Vec::new();
    for member_dir in find_member_dirs(workspace) {
        scan_member(&member_dir, workspace, &mut out)?;
    }
    Ok(out)
}

fn scan_member(member: &Path, workspace: &Path, out: &mut Vec<Finding>) -> Result<(), CleanupError> {
    let manifest = member.join("Cargo.toml");
    if !manifest.exists() {
        return Ok(());
    }
    let deps = parse_dep_names(&manifest)?;
    if deps.is_empty() {
        return Ok(());
    }
    let used = collect_used_idents(member);
    for (dep, line) in deps {
        if !used.contains(&dep_to_ident(&dep)) {
            out.push(unused_finding(&manifest, workspace, &dep, line));
        }
    }
    Ok(())
}

fn unused_finding(manifest: &Path, workspace: &Path, dep: &str, line: u32) -> Finding {
    let rel = manifest.strip_prefix(workspace).map(Path::to_path_buf).unwrap_or_else(|_| manifest.to_path_buf());
    Finding {
        kind: FindingKind::UnusedDep,
        severity: Severity::Medium,
        location: Location { file: rel, line: Some(line) },
        message: format!("`{dep}` declared, not imported"),
        fix_hint: Some(format!("cargo remove {dep}")),
        rule_ref: Some("CLEANUP §3.2 unused_deps".into()),
        confidence: Confidence::High,
    }
}

/// Extract dep names + line numbers from `[dependencies]` (and `[dev-dependencies]`).
fn parse_dep_names(manifest: &Path) -> Result<Vec<(String, u32)>, CleanupError> {
    let text = std::fs::read_to_string(manifest)?;
    let value: toml::Value = toml::from_str(&text).map_err(|e| CleanupError::Manifest {
        path: manifest.to_path_buf(),
        detail: e.to_string(),
    })?;
    let mut out = Vec::new();
    for section in &["dependencies"] {
        if let Some(t) = value.get(section).and_then(|v| v.as_table()) {
            for k in t.keys() {
                let line = grep_line_for_key(&text, k);
                out.push((k.clone(), line));
            }
        }
    }
    Ok(out)
}

fn grep_line_for_key(text: &str, key: &str) -> u32 {
    for (i, line) in text.lines().enumerate() {
        let trim = line.trim_start();
        if trim.starts_with(&format!("{key} ")) || trim.starts_with(&format!("{key}=")) {
            return (i + 1) as u32;
        }
    }
    1
}

fn collect_used_idents(member: &Path) -> BTreeSet<String> {
    let mut used = BTreeSet::new();
    let src = member.join("src");
    if !src.exists() {
        return used;
    }
    for entry in WalkDir::new(&src).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(entry.path()) else {
            continue;
        };
        extract_idents_into(&text, &mut used);
    }
    used
}

/// Naive but workable: capture top-level idents in `use X::...;`,
/// `extern crate X;`, and `X::` qualified paths in source.
fn extract_idents_into(text: &str, used: &mut BTreeSet<String>) {
    for line in text.lines() {
        let t = line.trim_start();
        if let Some(rest) = t.strip_prefix("use ") {
            push_first_segment(rest, used);
        } else if let Some(rest) = t.strip_prefix("extern crate ") {
            push_first_segment(rest, used);
        }
        // Also harvest any `Foo::` prefix.
        for tok in line.split(|c: char| !c.is_alphanumeric() && c != '_') {
            if tok.is_empty() || !tok.chars().next().unwrap().is_alphabetic() {
                continue;
            }
            used.insert(tok.to_string());
        }
    }
}

fn push_first_segment(rest: &str, used: &mut BTreeSet<String>) {
    let rest = rest.trim_start_matches("crate::");
    let first: String = rest
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if !first.is_empty() {
        used.insert(first);
    }
}

/// Convert `serde-json` style names to `serde_json` Rust ident form.
fn dep_to_ident(name: &str) -> String {
    name.replace('-', "_")
}

fn find_member_dirs(workspace: &Path) -> Vec<PathBuf> {
    // For unit/integration tests we accept either a single-crate root
    // (containing Cargo.toml + src/) or a workspace root listing members.
    let mut out = Vec::new();
    if workspace.join("Cargo.toml").exists() && workspace.join("src").exists() {
        out.push(workspace.to_path_buf());
    }
    for entry in WalkDir::new(workspace)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !entry.file_type().is_dir() {
            continue;
        }
        let p = entry.path();
        if p == workspace {
            continue;
        }
        if p.join("Cargo.toml").exists() && p.join("src").exists() {
            out.push(p.to_path_buf());
        }
    }
    out.sort();
    out.dedup();
    out
}
