//! Constructor-Pattern LOC scanner — see CLEANUP-RUNTIME-SPEC.md §3.2 +
//! Appendix A. Walks workspace, counts file LOC and per-function body
//! LOC; flags violations against `[loc]` config (defaults 200 / 30).
//!
//! Body-line measurement lives in [`super::fn_extract`] to keep this
//! module under the Constructor-Pattern 200-LOC limit (Rule ZERO).

use crate::config::Config;
use crate::error::CleanupError;
use crate::report::{Confidence, Finding, FindingKind, Location, Severity};
use crate::scanners::fn_extract::scan_fn_bodies;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Public scanner entry — see Appendix A row "loc_check".
pub fn scan(workspace: &Path, cfg: &Config) -> Result<Vec<Finding>, CleanupError> {
    let mut out = Vec::new();
    for entry in WalkDir::new(workspace)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_rust_source(e.path(), cfg))
    {
        let path = entry.path().to_path_buf();
        scan_file(&path, workspace, cfg, &mut out)?;
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

fn scan_file(
    path: &Path,
    workspace: &Path,
    cfg: &Config,
    out: &mut Vec<Finding>,
) -> Result<(), CleanupError> {
    let text = std::fs::read_to_string(path)?;
    let total_lines = count_lines(&text);
    let rel = relativize(path, workspace);

    if total_lines > cfg.loc.file_max {
        out.push(file_finding(rel.clone(), total_lines, cfg.loc.file_max));
    }
    for (name, start, body_lines) in scan_fn_bodies(&text) {
        if body_lines > cfg.loc.fn_max {
            out.push(fn_finding(rel.clone(), name, start, body_lines, cfg.loc.fn_max));
        }
    }
    Ok(())
}

fn file_finding(rel: PathBuf, lines: usize, limit: usize) -> Finding {
    Finding {
        kind: FindingKind::LocFile,
        severity: Severity::High,
        location: Location { file: rel, line: Some(1) },
        message: format!("file is {lines} LOC > limit {limit} (Constructor Pattern)"),
        fix_hint: Some("split into 2+ modules".into()),
        rule_ref: Some("Rule ZERO / Constructor Pattern".into()),
        confidence: Confidence::High,
    }
}

fn fn_finding(rel: PathBuf, name: String, line: usize, lines: usize, limit: usize) -> Finding {
    Finding {
        kind: FindingKind::LocFunction,
        severity: Severity::Medium,
        location: Location { file: rel, line: Some(line as u32) },
        message: format!("fn `{name}` body is {lines} LOC > limit {limit} (Constructor Pattern)"),
        fix_hint: Some("split into named sub-functions".into()),
        rule_ref: Some("Rule ZERO / Constructor Pattern".into()),
        confidence: Confidence::High,
    }
}

fn count_lines(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }
    text.lines().count()
}

fn relativize(path: &Path, workspace: &Path) -> PathBuf {
    path.strip_prefix(workspace).map(Path::to_path_buf).unwrap_or_else(|_| path.to_path_buf())
}
