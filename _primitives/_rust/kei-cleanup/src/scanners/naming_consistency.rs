//! Naming-consistency scanner — see CLEANUP-RUNTIME-SPEC.md §3.2 +
//! Appendix A.
//!
//! Reads `[naming_pairs] pairs = [["A", "B"], ...]` from cleanup.toml.
//! For each synonym group, counts whole-word occurrences across
//! workspace src/. If 2+ variants from the same group are present in
//! the workspace, emits a Severity::Low NamingDrift finding.

use crate::config::Config;
use crate::error::CleanupError;
use crate::report::{Confidence, Finding, FindingKind, Location, Severity};
use regex::Regex;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Public scanner entry — see Appendix A row "naming_consistency".
pub fn scan(workspace: &Path, cfg: &Config) -> Result<Vec<Finding>, CleanupError> {
    if cfg.naming_pairs.pairs.is_empty() {
        return Ok(Vec::new());
    }
    let counts = count_occurrences(workspace, cfg)?;
    Ok(emit_findings(&cfg.naming_pairs.pairs, &counts, workspace))
}

/// Map of variant → (file_count, first-seen path) across workspace.
fn count_occurrences(
    workspace: &Path,
    cfg: &Config,
) -> Result<BTreeMap<String, (usize, PathBuf)>, CleanupError> {
    let variants = collect_variants(&cfg.naming_pairs.pairs);
    let mut counts = init_counts(&variants);
    let regexes = build_regexes(&variants)?;
    for entry in WalkDir::new(workspace).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !is_rust_source(path, cfg) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        tally_file(&text, path, &regexes, &mut counts);
    }
    Ok(counts)
}

fn init_counts(variants: &[String]) -> BTreeMap<String, (usize, PathBuf)> {
    let mut counts: BTreeMap<String, (usize, PathBuf)> = BTreeMap::new();
    for v in variants {
        counts.insert(v.clone(), (0, PathBuf::new()));
    }
    counts
}

fn tally_file(
    text: &str,
    path: &Path,
    regexes: &[(String, Regex)],
    counts: &mut BTreeMap<String, (usize, PathBuf)>,
) {
    for (variant, re) in regexes {
        if re.is_match(text) {
            let e = counts.entry(variant.clone()).or_insert((0, PathBuf::new()));
            e.0 += 1;
            if e.1.as_os_str().is_empty() {
                e.1 = path.to_path_buf();
            }
        }
    }
}

fn collect_variants(pairs: &[Vec<String>]) -> Vec<String> {
    let mut v = Vec::new();
    for grp in pairs {
        for s in grp {
            v.push(s.clone());
        }
    }
    v
}

fn build_regexes(variants: &[String]) -> Result<Vec<(String, Regex)>, CleanupError> {
    let mut out = Vec::new();
    for v in variants {
        let pat = format!(r"\b{}\b", regex::escape(v));
        let re = Regex::new(&pat).map_err(|e| CleanupError::Walk(format!("regex {v}: {e}")))?;
        out.push((v.clone(), re));
    }
    Ok(out)
}

fn is_rust_source(path: &Path, cfg: &Config) -> bool {
    if !path.is_file() {
        return false;
    }
    if path.extension().and_then(|e| e.to_str()) != Some("rs") {
        return false;
    }
    !cfg.excluded(path)
}

fn emit_findings(
    pairs: &[Vec<String>],
    counts: &BTreeMap<String, (usize, PathBuf)>,
    workspace: &Path,
) -> Vec<Finding> {
    let mut out = Vec::new();
    for grp in pairs {
        let present: Vec<&String> = grp
            .iter()
            .filter(|v| counts.get(*v).map(|(c, _)| *c > 0).unwrap_or(false))
            .collect();
        if present.len() < 2 {
            continue;
        }
        let first_path = grp
            .iter()
            .find_map(|v| {
                counts
                    .get(v)
                    .filter(|(c, p)| *c > 0 && !p.as_os_str().is_empty())
                    .map(|(_, p)| p.clone())
            })
            .unwrap_or_else(|| workspace.to_path_buf());
        out.push(make_finding(workspace, &first_path, &present));
    }
    out
}

fn make_finding(workspace: &Path, path: &Path, present: &[&String]) -> Finding {
    let rel = path
        .strip_prefix(workspace)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| path.to_path_buf());
    let list = present
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    Finding {
        kind: FindingKind::NamingDrift,
        severity: Severity::Low,
        location: Location { file: rel, line: Some(1) },
        message: format!("naming-pair drift: {list}"),
        fix_hint: Some("pick one variant + replace the others workspace-wide".into()),
        rule_ref: Some("CLEANUP §3.2 naming_consistency".into()),
        confidence: Confidence::High,
    }
}
