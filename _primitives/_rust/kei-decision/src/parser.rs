//! Markdown action-table parser.
//!
//! Looks for a section heading whose text matches one of:
//!   - "Actionable plan"
//!   - "Backlog"
//!   - "Action items"
//! and extracts the markdown table that follows. Each table row becomes one
//! [`RawAction`]. Effort and severity are inferred from the row cells; deps
//! are parsed from a free-text "deps:" hint inside the action cell when
//! present.
//!
//! No md crate — table format is well-defined: `| col | col | ... |`.

use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawAction {
    pub id: String,
    pub title: String,
    pub severity: String,
    pub effort: String,
    pub deps: Vec<String>,
    pub source_line: usize,
}

mod thiserror_lite {
    /// Local error enum — avoids pulling thiserror as new dep (RULE: no new deps).
    #[derive(Debug)]
    pub enum ParseError {
        FileNotFound(String),
        NoActionsFound,
        Io(String),
    }
    impl std::fmt::Display for ParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::FileNotFound(p) => write!(f, "file not found: {p}"),
                Self::NoActionsFound => write!(f, "no Actionable plan / Backlog / Action items table found"),
                Self::Io(s) => write!(f, "io error: {s}"),
            }
        }
    }
    impl std::error::Error for ParseError {}
}

pub use thiserror_lite::ParseError;

/// Read MASTER-REPORT.md, locate first action-style section, return rows.
pub fn parse_master_report(path: &Path) -> Result<Vec<RawAction>, ParseError> {
    if !path.exists() {
        return Err(ParseError::FileNotFound(path.display().to_string()));
    }
    let body = std::fs::read_to_string(path).map_err(|e| ParseError::Io(e.to_string()))?;
    let lines: Vec<&str> = body.lines().collect();
    let table = find_action_table(&lines).ok_or(ParseError::NoActionsFound)?;
    let actions = extract_rows(&lines, table.start_line, table.column_indices);
    if actions.is_empty() {
        return Err(ParseError::NoActionsFound);
    }
    Ok(actions)
}

struct TableLocation {
    start_line: usize,
    column_indices: ColumnMap,
}

#[derive(Clone, Copy, Debug)]
struct ColumnMap {
    id: Option<usize>,
    action: usize,
    effort: Option<usize>,
    risk: Option<usize>,
}

/// Walk the doc, find a heading line that names an action section, then the
/// next markdown table whose header includes "Action".
fn find_action_table(lines: &[&str]) -> Option<TableLocation> {
    let heading_re = heading_regex();
    let mut in_section = false;
    for (i, line) in lines.iter().enumerate() {
        if heading_re.is_match(line) {
            in_section = true;
            continue;
        }
        if !in_section {
            continue;
        }
        if line.trim_start().starts_with('#') {
            in_section = false; // moved into a different section
            continue;
        }
        if !line.trim_start().starts_with('|') {
            continue;
        }
        if let Some(map) = parse_header_row(line) {
            // Header found — the body rows start two lines below (after the
            // separator row). Caller skips separator in extract_rows.
            return Some(TableLocation { start_line: i, column_indices: map });
        }
    }
    None
}

fn heading_regex() -> Regex {
    // Matches `## Actionable plan`, `### Backlog`, `## Action items` (any depth).
    Regex::new(r"(?i)^#{1,6}\s+(actionable\s+plan|backlog|action\s+items)\b").unwrap()
}

/// Inspect the table-header pipe row and locate the columns we care about.
fn parse_header_row(line: &str) -> Option<ColumnMap> {
    let cells = split_pipes(line);
    if cells.is_empty() {
        return None;
    }
    let lower: Vec<String> = cells.iter().map(|c| c.to_lowercase()).collect();
    let action_idx = lower.iter().position(|c| c.contains("action"))?;
    let id_idx = lower.iter().position(|c| c == "#" || c.contains("id"));
    let effort_idx = lower.iter().position(|c| c.contains("effort") || c.contains("hours") || c.contains("time"));
    let risk_idx = lower.iter().position(|c| c.contains("risk") || c.contains("severity") || c.contains("priority"));
    Some(ColumnMap { id: id_idx, action: action_idx, effort: effort_idx, risk: risk_idx })
}

/// Walk the body rows below the separator, build [`RawAction`] per row.
fn extract_rows(lines: &[&str], header_line: usize, cols: ColumnMap) -> Vec<RawAction> {
    let mut out = Vec::new();
    // Skip header and the divider line `|---|---|...`
    for (offset, line) in lines.iter().enumerate().skip(header_line + 1) {
        if offset == header_line + 1 && is_divider(line) {
            continue;
        }
        if !line.trim_start().starts_with('|') {
            break;
        }
        if is_divider(line) {
            continue;
        }
        if let Some(act) = build_raw_action(line, cols, offset + 1, out.len() + 1) {
            out.push(act);
        }
    }
    out
}

fn is_divider(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('|')
        && trimmed.chars().all(|c| matches!(c, '|' | '-' | ':' | ' '))
}

fn build_raw_action(line: &str, cols: ColumnMap, source_line: usize, fallback_n: usize) -> Option<RawAction> {
    let cells = split_pipes(line);
    let title = cells.get(cols.action)?.trim().to_string();
    if title.is_empty() {
        return None;
    }
    let id = cols.id
        .and_then(|i| cells.get(i))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| fallback_n.to_string());
    let effort = cols.effort.and_then(|i| cells.get(i)).map(|s| s.trim().to_string()).unwrap_or_default();
    let severity = cols.risk.and_then(|i| cells.get(i)).map(|s| s.trim().to_string()).unwrap_or_default();
    let deps = parse_deps_hint(&title);
    Some(RawAction { id, title, severity, effort, deps, source_line })
}

/// Split a pipe-row into its inner cells (drop empty leading/trailing).
fn split_pipes(line: &str) -> Vec<String> {
    line.trim()
        .trim_start_matches('|')
        .trim_end_matches('|')
        .split('|')
        .map(|s| s.to_string())
        .collect()
}

/// `deps: 1, 2` or `(after #3)` → vec of id strings.
fn parse_deps_hint(text: &str) -> Vec<String> {
    let re = Regex::new(r"(?i)\b(?:deps|after)\s*[:#]?\s*([0-9, ]+)").unwrap();
    if let Some(c) = re.captures(text) {
        return c[1]
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
    }
    Vec::new()
}
