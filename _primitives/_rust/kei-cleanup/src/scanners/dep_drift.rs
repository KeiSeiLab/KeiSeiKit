//! Workspace-deps drift scanner — see CLEANUP-RUNTIME-SPEC.md §3.2 +
//! Appendix A. Detects member-crate dependencies that pin a different
//! version than the workspace's `[workspace.dependencies]` table.

use crate::config::Config;
use crate::error::CleanupError;
use crate::report::{Confidence, Finding, FindingKind, Location, Severity};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Public scanner entry — see Appendix A row "dep_drift".
pub fn scan(workspace: &Path, _cfg: &Config) -> Result<Vec<Finding>, CleanupError> {
    let ws_manifest = workspace.join("Cargo.toml");
    if !ws_manifest.exists() {
        return Ok(Vec::new());
    }
    let ws_versions = parse_workspace_versions(&ws_manifest)?;
    if ws_versions.is_empty() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for member in find_member_manifests(workspace) {
        scan_member(&member, workspace, &ws_versions, &mut out)?;
    }
    Ok(out)
}

fn scan_member(
    member: &Path,
    workspace: &Path,
    ws: &BTreeMap<String, String>,
    out: &mut Vec<Finding>,
) -> Result<(), CleanupError> {
    let text = std::fs::read_to_string(member)?;
    let value: toml::Value = toml::from_str(&text).map_err(|e| CleanupError::Manifest {
        path: member.to_path_buf(),
        detail: e.to_string(),
    })?;
    if let Some(table) = value.get("dependencies").and_then(|v| v.as_table()) {
        for (dep, dep_val) in table {
            let Some(declared) = extract_explicit_version(dep_val) else {
                continue; // workspace = true / path = ... / git = ... → skip
            };
            let Some(ws_ver) = ws.get(dep) else {
                continue;
            };
            if &declared != ws_ver {
                let line = grep_line_for_key(&text, dep);
                out.push(drift_finding(member, workspace, dep, &declared, ws_ver, line));
            }
        }
    }
    Ok(())
}

fn drift_finding(
    member: &Path,
    workspace: &Path,
    dep: &str,
    declared: &str,
    ws_ver: &str,
    line: u32,
) -> Finding {
    let rel = member
        .strip_prefix(workspace)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| member.to_path_buf());
    Finding {
        kind: FindingKind::DepDrift,
        severity: Severity::Low,
        location: Location { file: rel, line: Some(line) },
        message: format!("`{dep} {declared}` differs from workspace `{dep} {ws_ver}`"),
        fix_hint: Some(format!("use workspace dep: `{dep} = {{ workspace = true }}`")),
        rule_ref: Some("CLEANUP §3.2 dep_drift".into()),
        confidence: Confidence::High,
    }
}

/// Read `[workspace.dependencies]` from the workspace Cargo.toml.
fn parse_workspace_versions(ws_manifest: &Path) -> Result<BTreeMap<String, String>, CleanupError> {
    let text = std::fs::read_to_string(ws_manifest)?;
    let value: toml::Value = toml::from_str(&text).map_err(|e| CleanupError::Manifest {
        path: ws_manifest.to_path_buf(),
        detail: e.to_string(),
    })?;
    let mut out = BTreeMap::new();
    let Some(ws) = value.get("workspace").and_then(|v| v.as_table()) else {
        return Ok(out);
    };
    let Some(deps) = ws.get("dependencies").and_then(|v| v.as_table()) else {
        return Ok(out);
    };
    for (k, v) in deps {
        if let Some(ver) = extract_explicit_version(v) {
            out.insert(k.clone(), ver);
        }
    }
    Ok(out)
}

/// Pull `version = "x"` out of either string-form dep or table-form.
fn extract_explicit_version(v: &toml::Value) -> Option<String> {
    match v {
        toml::Value::String(s) => Some(s.clone()),
        toml::Value::Table(t) => {
            if t.get("workspace")
                .and_then(|w| w.as_bool())
                .unwrap_or(false)
            {
                return None; // explicitly inheriting → not drift
            }
            t.get("version").and_then(|x| x.as_str()).map(String::from)
        }
        _ => None,
    }
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

fn find_member_manifests(workspace: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in WalkDir::new(workspace)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().file_name().and_then(|f| f.to_str()) != Some("Cargo.toml") {
            continue;
        }
        if entry.path() == workspace.join("Cargo.toml") {
            continue;
        }
        out.push(entry.path().to_path_buf());
    }
    out.sort();
    out
}
