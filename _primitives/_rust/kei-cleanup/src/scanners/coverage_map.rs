//! Coverage-map scanner — see CLEANUP-RUNTIME-SPEC.md §3.2 + Appendix A.
//!
//! Walks workspace; collects two sets of marker IDs:
//!   * Derivations:  lines matching cfg.coverage_map.derivation_marker
//!     (default `// derive:`) in src/*.rs and theory/*.md.
//!   * Tests:        lines matching cfg.coverage_map.test_id_marker
//!     (default `// covers:`) in tests/*.rs.
//!
//! Emits CoverageGap (MEDIUM, Confidence::Medium) for every derivation
//! ID with no matching test marker.
//! Emits LOW (Confidence::Low) for tests covering ID with no derivation
//! marker — likely typo or stale fixture.

use crate::config::Config;
use crate::error::CleanupError;
use crate::report::{Confidence, Finding, FindingKind, Location, Severity};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Public scanner entry — see Appendix A row "coverage_map".
pub fn scan(workspace: &Path, cfg: &Config) -> Result<Vec<Finding>, CleanupError> {
    let derive_marker = cfg.coverage_map.derivation_marker.clone();
    let test_marker = cfg.coverage_map.test_id_marker.clone();
    let mut derives: BTreeSet<String> = BTreeSet::new();
    let mut covers: BTreeSet<String> = BTreeSet::new();
    let mut sample_loc: Vec<(String, PathBuf, u32, bool)> = Vec::new();

    for entry in WalkDir::new(workspace).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !is_scannable(path, cfg) {
            continue;
        }
        scan_one_file(path, &derive_marker, &test_marker, &mut derives, &mut covers, &mut sample_loc)?;
    }
    Ok(build_findings(workspace, &derives, &covers, &sample_loc))
}

fn is_scannable(path: &Path, cfg: &Config) -> bool {
    if !path.is_file() {
        return false;
    }
    if cfg.excluded(path) {
        return false;
    }
    matches!(path.extension().and_then(|e| e.to_str()), Some("rs") | Some("md"))
}

fn scan_one_file(
    path: &Path,
    derive_marker: &str,
    test_marker: &str,
    derives: &mut BTreeSet<String>,
    covers: &mut BTreeSet<String>,
    sample_loc: &mut Vec<(String, PathBuf, u32, bool)>,
) -> Result<(), CleanupError> {
    let text = std::fs::read_to_string(path)?;
    for (i, line) in text.lines().enumerate() {
        if let Some(id) = extract_id(line, derive_marker) {
            derives.insert(id.clone());
            sample_loc.push((id, path.to_path_buf(), (i + 1) as u32, true));
        }
        if let Some(id) = extract_id(line, test_marker) {
            covers.insert(id.clone());
            sample_loc.push((id, path.to_path_buf(), (i + 1) as u32, false));
        }
    }
    Ok(())
}

fn extract_id(line: &str, marker: &str) -> Option<String> {
    let idx = line.find(marker)?;
    let rest = line[idx + marker.len()..].trim();
    let id: String = rest
        .chars()
        .take_while(|c| !c.is_whitespace())
        .collect();
    if id.is_empty() {
        None
    } else {
        Some(id)
    }
}

fn build_findings(
    workspace: &Path,
    derives: &BTreeSet<String>,
    covers: &BTreeSet<String>,
    sample_loc: &[(String, PathBuf, u32, bool)],
) -> Vec<Finding> {
    let mut out = Vec::new();
    for d in derives {
        if !covers.contains(d) {
            let (path, line) = first_sample_for(sample_loc, d, true)
                .unwrap_or_else(|| (workspace.to_path_buf(), 1));
            out.push(missing_test_finding(workspace, d, &path, line));
        }
    }
    for c in covers {
        if !derives.contains(c) {
            let (path, line) = first_sample_for(sample_loc, c, false)
                .unwrap_or_else(|| (workspace.to_path_buf(), 1));
            out.push(stale_test_finding(workspace, c, &path, line));
        }
    }
    out
}

fn first_sample_for(
    samples: &[(String, PathBuf, u32, bool)],
    id: &str,
    want_derive: bool,
) -> Option<(PathBuf, u32)> {
    samples
        .iter()
        .find(|(s, _, _, is_derive)| s == id && *is_derive == want_derive)
        .map(|(_, p, l, _)| (p.clone(), *l))
}

fn missing_test_finding(workspace: &Path, id: &str, path: &Path, line: u32) -> Finding {
    let rel = path
        .strip_prefix(workspace)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| path.to_path_buf());
    Finding {
        kind: FindingKind::CoverageGap,
        severity: Severity::Medium,
        location: Location { file: rel, line: Some(line) },
        message: format!("derivation `{id}` has no matching `// covers:` test"),
        fix_hint: Some(format!("add `// covers: {id}` to a test")),
        rule_ref: Some("CLEANUP §3.2 coverage_map".into()),
        confidence: Confidence::Low,
    }
}

fn stale_test_finding(workspace: &Path, id: &str, path: &Path, line: u32) -> Finding {
    let rel = path
        .strip_prefix(workspace)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| path.to_path_buf());
    Finding {
        kind: FindingKind::CoverageGap,
        severity: Severity::Low,
        location: Location { file: rel, line: Some(line) },
        message: format!("test `// covers: {id}` references missing derivation"),
        fix_hint: Some("verify the derivation marker exists or remove the test marker".into()),
        rule_ref: Some("CLEANUP §3.2 coverage_map".into()),
        confidence: Confidence::Low,
    }
}
