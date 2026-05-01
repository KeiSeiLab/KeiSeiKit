//! /wave-audit adapter.
//!
//! Detects the Wave-audit MD shape:
//!   - `## Wave N` heading (or "Audit Report" / similar)
//!   - `## Priority Matrix` section (table of findings)
//!   - `## Apply Plan` section (actionable next steps)
//!
//! Extracts each Priority Matrix row as one `Action`. Header columns:
//!   `| # | Severity | Finding | Fix | Complexity | Blast | Score | [E] |`
//! plus tolerated header variants.

use anyhow::{Context, Result};
use regex::Regex;
use std::path::Path;

use super::{Confidence, FormatParser};
use crate::normalizer::{Action, Severity};

pub struct AuditParser;

const FORMAT: &str = "audit";

impl FormatParser for AuditParser {
    fn name(&self) -> &str {
        FORMAT
    }

    fn detect(&self, md: &str) -> Confidence {
        let has_wave = wave_heading_regex().is_match(md);
        let has_matrix = md.to_lowercase().contains("priority matrix");
        let has_apply = md.to_lowercase().contains("## apply plan");
        let hits = (has_wave as u8) + (has_matrix as u8) + (has_apply as u8);
        match hits {
            3 => Confidence::EXACT,
            2 => Confidence::HEADER,
            1 => Confidence::AMBIGUOUS,
            _ => Confidence::NONE,
        }
    }

    fn parse(&self, path: &Path) -> Result<Vec<Action>> {
        let body = std::fs::read_to_string(path)
            .with_context(|| format!("read audit md: {}", path.display()))?;
        let lines: Vec<&str> = body.lines().collect();
        let path_s = path.display().to_string();
        let table_start = match find_priority_matrix(&lines) {
            Some(i) => i,
            None => return Ok(Vec::new()),
        };
        Ok(extract_findings(&lines, table_start, &path_s))
    }
}

fn wave_heading_regex() -> Regex {
    Regex::new(r"(?im)^#{1,6}\s+(wave\s+\d+|audit\s+report)\b").unwrap()
}

/// Locate the `## Priority Matrix` heading, then return the index of the
/// header row of the table that follows.
fn find_priority_matrix(lines: &[&str]) -> Option<usize> {
    let heading_re = Regex::new(r"(?im)^#{1,6}\s+priority\s+matrix\b").unwrap();
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
            in_section = false;
            continue;
        }
        if line.trim_start().starts_with('|') && header_has_finding(line) {
            return Some(i);
        }
    }
    None
}

fn header_has_finding(line: &str) -> bool {
    let lower = line.to_lowercase();
    lower.contains("finding")
}

fn extract_findings(lines: &[&str], header_line: usize, source_path: &str) -> Vec<Action> {
    let mut out = Vec::new();
    let cols = match parse_audit_header(lines[header_line]) {
        Some(c) => c,
        None => return out,
    };
    for (offset, line) in lines.iter().enumerate().skip(header_line + 1) {
        if !line.trim_start().starts_with('|') {
            break;
        }
        if is_divider(line) {
            continue;
        }
        if let Some(act) = build_finding(line, &cols, offset + 1, out.len() + 1, source_path) {
            out.push(act);
        }
    }
    out
}

#[derive(Debug, Clone)]
struct AuditCols {
    id: Option<usize>,
    severity: Option<usize>,
    finding: usize,
    fix: Option<usize>,
    complexity: Option<usize>,
}

fn parse_audit_header(line: &str) -> Option<AuditCols> {
    let cells = split_pipes(line);
    let lower: Vec<String> = cells.iter().map(|c| c.to_lowercase()).collect();
    let finding = lower.iter().position(|c| c.contains("finding"))?;
    let id = lower.iter().position(|c| c.trim() == "#" || c.contains("id"));
    let severity = lower
        .iter()
        .position(|c| c.contains("severity") || c.contains("priority") || c.contains("risk"));
    let fix = lower.iter().position(|c| c.contains("fix") || c.contains("action"));
    let complexity = lower
        .iter()
        .position(|c| c.contains("complex") || c.contains("effort") || c.contains("hours"));
    Some(AuditCols { id, severity, finding, fix, complexity })
}

fn build_finding(
    line: &str,
    cols: &AuditCols,
    source_line: usize,
    fallback_n: usize,
    source_path: &str,
) -> Option<Action> {
    let cells = split_pipes(line);
    let title = cells.get(cols.finding)?.trim().to_string();
    if title.is_empty() {
        return None;
    }
    let id = cell_or_fallback(&cells, cols.id, fallback_n);
    let severity_text = cell_or_default(&cells, cols.severity);
    let effort = cell_or_default(&cells, cols.complexity);
    let fix = cell_or_default(&cells, cols.fix);
    let body = format!(
        "Source: {} L{}\n\nFinding: {}\nFix: {}\nSeverity: {}\nComplexity: {}",
        source_path, source_line, title, fix, severity_text, effort
    );
    Some(
        Action::new(id, title, FORMAT, source_path, source_line)
            .with_effort(effort)
            .with_severity(Severity::from_text(&severity_text))
            .with_body(body),
    )
}

fn cell_or_fallback(cells: &[String], col: Option<usize>, fallback: usize) -> String {
    col.and_then(|i| cells.get(i))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| fallback.to_string())
}

fn cell_or_default(cells: &[String], col: Option<usize>) -> String {
    col.and_then(|i| cells.get(i))
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

fn split_pipes(line: &str) -> Vec<String> {
    line.trim()
        .trim_start_matches('|')
        .trim_end_matches('|')
        .split('|')
        .map(|s| s.to_string())
        .collect()
}

fn is_divider(line: &str) -> bool {
    let t = line.trim();
    t.starts_with('|') && t.chars().all(|c| matches!(c, '|' | '-' | ':' | ' '))
}
