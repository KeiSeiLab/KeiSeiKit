//! Workspace-tests scanner — see CLEANUP-RUNTIME-SPEC.md §3.2 + Appendix A.
//!
//! For each workspace member crate, parses each `tests/*.rs` file via
//! `syn` and looks for `extern crate <kei_*>` and `use <kei_*>` items
//! referencing 2+ DIFFERENT sibling crates. Such tests create dev-dep
//! cycles and should live in a dedicated `tests` crate (the K2K
//! workspace-tests pattern).
//!
//! Severity: MEDIUM, Confidence::High (syn AST is deterministic).

use crate::config::Config;
use crate::error::CleanupError;
use crate::report::{Confidence, Finding, FindingKind, Location, Severity};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use syn::{Item, UseTree};
use walkdir::WalkDir;

const KEI_PREFIX: &str = "kei_";

/// Public scanner entry — see Appendix A row "workspace_tests".
pub fn scan(workspace: &Path, _cfg: &Config) -> Result<Vec<Finding>, CleanupError> {
    let mut out = Vec::new();
    for member in find_test_dirs(workspace) {
        scan_member_tests(&member, workspace, &mut out)?;
    }
    Ok(out)
}

fn scan_member_tests(
    tests_dir: &Path,
    workspace: &Path,
    out: &mut Vec<Finding>,
) -> Result<(), CleanupError> {
    for entry in WalkDir::new(tests_dir)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !is_rust_file(path) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        let crates = collect_kei_refs(&text);
        if crates.len() >= 2 {
            out.push(make_finding(path, workspace, &crates));
        }
    }
    Ok(())
}

fn is_rust_file(path: &Path) -> bool {
    path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("rs")
}

fn collect_kei_refs(text: &str) -> BTreeSet<String> {
    let mut crates = BTreeSet::new();
    let Ok(file) = syn::parse_file(text) else {
        return collect_kei_refs_textual(text);
    };
    for item in &file.items {
        match item {
            Item::ExternCrate(e) => {
                let name = e.ident.to_string();
                if name.starts_with(KEI_PREFIX) {
                    crates.insert(name);
                }
            }
            Item::Use(u) => collect_use_first_seg(&u.tree, &mut crates),
            _ => {}
        }
    }
    crates
}

fn collect_use_first_seg(tree: &UseTree, crates: &mut BTreeSet<String>) {
    match tree {
        UseTree::Path(p) => {
            let name = p.ident.to_string();
            if name.starts_with(KEI_PREFIX) {
                crates.insert(name);
            }
        }
        UseTree::Name(n) => {
            let name = n.ident.to_string();
            if name.starts_with(KEI_PREFIX) {
                crates.insert(name);
            }
        }
        UseTree::Group(g) => {
            for t in &g.items {
                collect_use_first_seg(t, crates);
            }
        }
        _ => {}
    }
}

/// Fallback when syn fails (e.g. macro-generated tests):
/// scan first segment of `use kei_X` / `extern crate kei_X` lines textually.
fn collect_kei_refs_textual(text: &str) -> BTreeSet<String> {
    let mut crates = BTreeSet::new();
    for line in text.lines() {
        let trim = line.trim_start();
        let rest = trim
            .strip_prefix("use ")
            .or_else(|| trim.strip_prefix("extern crate "));
        let Some(rest) = rest else { continue };
        let first: String = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if first.starts_with(KEI_PREFIX) {
            crates.insert(first);
        }
    }
    crates
}

fn make_finding(path: &Path, workspace: &Path, crates: &BTreeSet<String>) -> Finding {
    let rel = path
        .strip_prefix(workspace)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| path.to_path_buf());
    let list = crates.iter().cloned().collect::<Vec<_>>().join(", ");
    Finding {
        kind: FindingKind::WorkspaceTestNeeded,
        severity: Severity::Medium,
        location: Location { file: rel, line: Some(1) },
        message: format!("test references {} sibling crates: {list}", crates.len()),
        fix_hint: Some("move into a dedicated workspace-tests crate".into()),
        rule_ref: Some("CLEANUP §3.2 workspace_tests".into()),
        confidence: Confidence::High,
    }
}

fn find_test_dirs(workspace: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in WalkDir::new(workspace)
        .max_depth(4)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !entry.file_type().is_dir() {
            continue;
        }
        if entry.path().file_name().and_then(|f| f.to_str()) != Some("tests") {
            continue;
        }
        if entry
            .path()
            .components()
            .any(|c| c.as_os_str() == "target")
        {
            continue;
        }
        out.push(entry.path().to_path_buf());
    }
    out.sort();
    out.dedup();
    out
}
