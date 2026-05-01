//! map_cmd — build an architecture map of a repo by running the matcher per module.
//!
//! Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

use crate::{identifier, matcher, matcher::MatchScore, module_source::ModuleSource, walker};
use anyhow::Result;
use serde::Serialize;
use std::path::Path;

/// One row in the architecture map.
#[derive(Debug, Clone, Serialize)]
pub struct MapEntry {
    pub module: String,
    pub kind: String,
    pub source_files: usize,
    pub best_match: Option<MatchScore>,
    pub all_matches: Vec<MatchScore>,
}

// Implement Serialize for MatchScore so MapEntry can derive it.
// MatchScore lives in matcher.rs — we add a manual impl here to avoid
// modifying a file not in scope (Surgical Changes rule).

// Walk → identify → match per module, filter by threshold, sort desc by confidence.
pub fn build_map(repo_path: &Path, threshold: f64) -> Result<Vec<MapEntry>> {
    let walk = walker::walk_repo(repo_path)?;
    let modules = identifier::identify_modules(&walk)?;
    let mut entries = Vec::new();
    for m in modules {
        if matches!(m.kind, identifier::ModuleKind::RustCrate) {
            let abs_dir = walk.root.join(&m.root_dir);
            let source = ModuleSource::from_dir(&m.name, &abs_dir)?;
            let all_matches: Vec<MatchScore> = matcher::match_module(&source)
                .into_iter()
                .filter(|ms| ms.confidence >= threshold)
                .collect();
            let best_match = all_matches.first().cloned();
            entries.push(MapEntry {
                module: m.name,
                kind: format!("{:?}", m.kind),
                source_files: m.source_files.len(),
                best_match,
                all_matches,
            });
        } else {
            // Non-Rust: include in map with no matches (matcher is Rust-only).
            entries.push(MapEntry {
                module: m.name,
                kind: format!("{:?}", m.kind),
                source_files: m.source_files.len(),
                best_match: None,
                all_matches: vec![],
            });
        }
    }
    entries.sort_by(|a, b| {
        let ca = a.best_match.as_ref().map(|m| m.confidence).unwrap_or(0.0);
        let cb = b.best_match.as_ref().map(|m| m.confidence).unwrap_or(0.0);
        cb.partial_cmp(&ca).unwrap()
    });
    Ok(entries)
}

/// Render entries as a markdown table.
pub fn render_markdown(entries: &[MapEntry], threshold: f64, repo_name: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {repo_name} — architecture map\n\n"));
    out.push_str("| Module | Kind | Files | Suggested trait | Confidence | Matched methods |\n");
    out.push_str("|---|---|---:|---|---:|---|\n");

    let mut below: Vec<&MapEntry> = Vec::new();
    for entry in entries {
        match &entry.best_match {
            Some(ms) => {
                let methods = ms.matched_methods.join(", ");
                out.push_str(&format!(
                    "| {} | {} | {} | {:?} | {:.2} | {} |\n",
                    entry.module, entry.kind, entry.source_files, ms.kind, ms.confidence, methods
                ));
            }
            None => below.push(entry),
        }
    }

    if !below.is_empty() {
        out.push_str(&format!("\n## Modules below threshold ({} total)\n\n", below.len()));
        for entry in below {
            out.push_str(&format!(
                "- {} ({}, {} files): no trait at threshold ≥ {:.2}\n",
                entry.module, entry.kind, entry.source_files, threshold
            ));
        }
    }
    out
}

/// Render entries as a JSON array.
pub fn render_json(entries: &[MapEntry]) -> Result<String> {
    Ok(serde_json::to_string_pretty(entries)?)
}
