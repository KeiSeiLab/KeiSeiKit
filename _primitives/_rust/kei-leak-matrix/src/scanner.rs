//! Scanner — file / string / tree scan helpers + JSON output + exit codes.
//!
//! Output never contains the rule's pattern string. Match excerpts are
//! redacted to first 12 chars + "…" so the SSoT regex stays in the matrix.

use crate::matrix::{Matrix, Rule, Scope, Severity};
use anyhow::Result;
use serde::Serialize;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize)]
pub struct Violation {
    pub rule_id: String,
    pub severity: String,
    pub file: String,
    pub line: usize,
    pub matched_redacted: String,
}

const REDACT_LIMIT: usize = 12;

fn redact(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i >= REDACT_LIMIT { out.push('…'); break; }
        out.push(ch);
    }
    out
}

fn rule_applies(rule: &Rule, scope: Scope, severity_filter: Option<Severity>) -> bool {
    if !rule.matches_scope(scope) { return false; }
    if matches!(rule.severity, Severity::Exclude) { return false; }
    match severity_filter {
        Some(target) => rule.severity == target,
        None => true,
    }
}

/// Scan one in-memory string. Used by scan_file and scan-cmd.
pub fn scan_string(
    matrix: &Matrix,
    content: &str,
    scope: Scope,
    severity_filter: Option<Severity>,
    file_label: &str,
) -> Vec<Violation> {
    let mut out = Vec::new();
    for (line_idx, line) in content.lines().enumerate() {
        for rule in &matrix.rules {
            if !rule_applies(rule, scope, severity_filter) { continue; }
            for m in rule.regex.find_iter(line) {
                out.push(Violation {
                    rule_id: rule.id.clone(),
                    severity: rule.severity.as_str().to_string(),
                    file: file_label.to_string(),
                    line: line_idx + 1,
                    matched_redacted: redact(m.as_str()),
                });
            }
        }
    }
    out
}

pub fn scan_file(
    matrix: &Matrix,
    path: &Path,
    scope: Scope,
    severity_filter: Option<Severity>,
) -> Result<Vec<Violation>> {
    let content = std::fs::read_to_string(path)?;
    Ok(scan_string(matrix, &content, scope, severity_filter, &path.display().to_string()))
}

/// Allowed extensions for scan-tree (text-ish files only).
const SCAN_EXTS: &[&str] = &["md", "rs", "sh", "toml", "ts", "js", "py", "json"];

fn is_excluded_dir(name: &str) -> bool {
    matches!(name, ".git" | "node_modules" | "target")
}

fn has_scan_ext(p: &Path) -> bool {
    p.extension().and_then(|e| e.to_str())
        .map(|e| SCAN_EXTS.contains(&e)).unwrap_or(false)
}

pub fn scan_tree(
    matrix: &Matrix,
    root: &Path,
    scope: Scope,
    severity_filter: Option<Severity>,
) -> Result<Vec<Violation>> {
    let mut out = Vec::new();
    let walker = WalkDir::new(root).into_iter().filter_entry(|e| {
        e.file_name().to_str().map(|n| !is_excluded_dir(n)).unwrap_or(true)
    });
    for entry in walker.flatten() {
        let path: PathBuf = entry.path().to_path_buf();
        if !path.is_file() || !has_scan_ext(&path) { continue; }
        if let Ok(found) = scan_file(matrix, &path, scope, severity_filter) {
            out.extend(found);
        }
    }
    Ok(out)
}

/// Exit code from a violation set.
/// 0 if empty, 2 if any block, 1 if any warn, 0 otherwise (substitute-only).
pub fn exit_code(violations: &[Violation]) -> i32 {
    let mut has_block = false;
    let mut has_warn = false;
    for v in violations {
        match v.severity.as_str() {
            "block" => has_block = true,
            "warn" => has_warn = true,
            _ => {}
        }
    }
    if has_block { 2 } else if has_warn { 1 } else { 0 }
}

pub fn emit_json(violations: &[Violation]) {
    println!("{}", serde_json::to_string_pretty(violations).unwrap_or_else(|_| "[]".into()));
}

/// Handler: scan one file → JSON + exit code.
pub fn cmd_scan_file(
    matrix: &Matrix, path: &Path, scope: Scope, severity_filter: Option<Severity>,
) -> Result<i32> {
    let v = scan_file(matrix, path, scope, severity_filter)?;
    emit_json(&v);
    Ok(exit_code(&v))
}

/// Handler: recurse dir → JSON + exit code.
pub fn cmd_scan_tree(
    matrix: &Matrix, root: &Path, scope: Scope, severity_filter: Option<Severity>,
) -> Result<i32> {
    let v = scan_tree(matrix, root, scope, severity_filter)?;
    emit_json(&v);
    Ok(exit_code(&v))
}

/// Handler: scan literal command string → JSON + exit code.
pub fn cmd_scan_cmd(matrix: &Matrix, cmd: &str, scope: Scope) -> i32 {
    let v = scan_string(matrix, cmd, scope, None, "<cmd>");
    emit_json(&v);
    exit_code(&v)
}
