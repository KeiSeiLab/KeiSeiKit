//! Dead-code / unused-dep scanner via external CLI — see
//! CLEANUP-RUNTIME-SPEC.md §3.7 + §8.3 (graceful fallback).
//!
//! Strategy:
//! 1. Try `cargo machete --with-metadata` (stable). Parse stdout.
//! 2. Else try `cargo +nightly udeps --workspace`. Parse output.
//! 3. Else return `ToolNotFound` — runtime records as `scanners_skipped`.
//!
//! v0.1: machete-only parsing implemented; udeps invocation wired but
//! its output parser is conservative (best-effort line scan). Both
//! branches return the same `Finding` shape to keep callers simple.

use crate::config::Config;
use crate::error::CleanupError;
use crate::report::{Confidence, Finding, FindingKind, Location, Severity};
use std::path::{Path, PathBuf};
use std::process::Command;

const TAG_MACHETE: &str = "cargo-machete";
const TAG_UDEPS: &str = "cargo-udeps";

/// Public scanner entry — see Appendix A row "dead_code".
pub fn scan(workspace: &Path, _cfg: &Config) -> Result<Vec<Finding>, CleanupError> {
    if has_tool("cargo-machete") || has_cargo_subcommand("machete") {
        return scan_with_machete(workspace);
    }
    if has_cargo_subcommand("udeps") {
        return scan_with_udeps(workspace);
    }
    Err(CleanupError::ToolNotFound(
        "cargo-machete and cargo-udeps both unavailable".into(),
    ))
}

fn scan_with_machete(workspace: &Path) -> Result<Vec<Finding>, CleanupError> {
    let out = Command::new("cargo")
        .arg("machete")
        .current_dir(workspace)
        .output()?;
    // cargo-machete prints findings to stdout and exits non-zero when
    // findings exist — that is NOT a tool failure for us.
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    if !out.status.success() && stdout.is_empty() && !stderr.is_empty() {
        return Err(CleanupError::ToolFailure {
            tool: TAG_MACHETE.into(),
            detail: stderr.into_owned(),
        });
    }
    Ok(parse_machete_output(&stdout, workspace))
}

fn scan_with_udeps(workspace: &Path) -> Result<Vec<Finding>, CleanupError> {
    let out = Command::new("cargo")
        .args(["+nightly", "udeps", "--workspace"])
        .current_dir(workspace)
        .output()?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let combined = format!("{stdout}\n{stderr}");
    Ok(parse_udeps_output(&combined, workspace))
}

/// Parse cargo-machete stdout. Lines look like:
/// `cratename -- /abs/path/Cargo.toml:` followed by indented dep names.
fn parse_machete_output(text: &str, workspace: &Path) -> Vec<Finding> {
    let mut out = Vec::new();
    let mut current_path: Option<PathBuf> = None;
    for raw in text.lines() {
        let line = raw.trim_end();
        if let Some(p) = extract_manifest_path(line) {
            current_path = Some(p);
            continue;
        }
        if let Some(dep) = extract_dep_name(line) {
            if let Some(ref cp) = current_path {
                out.push(machete_finding(cp, workspace, &dep));
            }
        }
    }
    out
}

fn extract_manifest_path(line: &str) -> Option<PathBuf> {
    if let Some(idx) = line.find("/Cargo.toml") {
        // Walk back to a path-like token start.
        let bytes = line.as_bytes();
        let end = idx + "/Cargo.toml".len();
        let mut start = idx;
        while start > 0 && bytes[start - 1] != b' ' && bytes[start - 1] != b'\t' {
            start -= 1;
        }
        return Some(PathBuf::from(&line[start..end]));
    }
    None
}

fn extract_dep_name(line: &str) -> Option<String> {
    let trim = line.trim_start();
    if trim.is_empty()
        || trim.starts_with("--")
        || trim.contains(':')
        || trim.contains('/')
        || trim.contains(' ')
    {
        return None;
    }
    let first_char = trim.chars().next()?;
    if !first_char.is_alphabetic() {
        return None;
    }
    Some(trim.to_string())
}

fn parse_udeps_output(text: &str, workspace: &Path) -> Vec<Finding> {
    // udeps output format is unstable; conservative best-effort:
    // look for "unused crates in" header + indented `- name` lines.
    let mut out = Vec::new();
    let mut current: Option<PathBuf> = None;
    for raw in text.lines() {
        let line = raw.trim_end();
        if line.contains("unused") && line.contains(":") {
            current = Some(workspace.to_path_buf());
            continue;
        }
        let trim = line.trim_start();
        if let Some(rest) = trim.strip_prefix("- ") {
            if let Some(ref cp) = current {
                let dep = rest.split_whitespace().next().unwrap_or("").to_string();
                if !dep.is_empty() {
                    out.push(udeps_finding(cp, workspace, &dep));
                }
            }
        }
    }
    out
}

fn machete_finding(manifest: &Path, workspace: &Path, dep: &str) -> Finding {
    let rel = manifest
        .strip_prefix(workspace)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| manifest.to_path_buf());
    Finding {
        kind: FindingKind::DeadCode,
        severity: Severity::Medium,
        location: Location { file: rel, line: None },
        message: format!("{TAG_MACHETE} flagged `{dep}` as unused"),
        fix_hint: Some(format!("cargo remove {dep}")),
        rule_ref: Some("CLEANUP §3.2 dead_code".into()),
        confidence: Confidence::Low, // machete is heuristic
    }
}

fn udeps_finding(manifest: &Path, workspace: &Path, dep: &str) -> Finding {
    let rel = manifest
        .strip_prefix(workspace)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| manifest.to_path_buf());
    Finding {
        kind: FindingKind::DeadCode,
        severity: Severity::Medium,
        location: Location { file: rel, line: None },
        message: format!("{TAG_UDEPS} flagged `{dep}` as unused"),
        fix_hint: Some(format!("cargo remove {dep}")),
        rule_ref: Some("CLEANUP §3.2 dead_code".into()),
        confidence: Confidence::High,
    }
}

fn has_tool(name: &str) -> bool {
    Command::new(name)
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn has_cargo_subcommand(sub: &str) -> bool {
    Command::new("cargo")
        .args([sub, "--version"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
