//! /research MASTER-REPORT.md adapter.
//!
//! Mirrors the kei-decision parser shape (Wave 51): scans for an
//! `## Actionable plan` / `## Backlog` / `## Action items` section, then
//! parses the markdown table that follows. The trait wrapper lets the
//! registry treat it as one adapter among many.

use anyhow::{Context, Result};
use regex::Regex;
use std::path::Path;

use super::{Confidence, FormatParser};
use crate::normalizer::{Action, Severity};

pub struct ResearchParser;

const FORMAT: &str = "research";

impl FormatParser for ResearchParser {
    fn name(&self) -> &str {
        FORMAT
    }

    fn detect(&self, md: &str) -> Confidence {
        let has_heading = heading_regex().is_match(md);
        let has_table = md.contains("| Action") || md.contains("|Action") || md.contains("| action");
        if has_heading && has_table {
            Confidence::EXACT
        } else if has_heading || has_table {
            Confidence::HEADER
        } else {
            Confidence::NONE
        }
    }

    fn parse(&self, path: &Path) -> Result<Vec<Action>> {
        let body = std::fs::read_to_string(path)
            .with_context(|| format!("read research md: {}", path.display()))?;
        let lines: Vec<&str> = body.lines().collect();
        let table = match find_action_table(&lines) {
            Some(t) => t,
            None => return Ok(Vec::new()),
        };
        let path_s = path.display().to_string();
        Ok(extract_rows(&lines, table, &path_s))
    }
}

#[derive(Clone, Copy, Debug)]
struct ColumnMap {
    id: Option<usize>,
    action: usize,
    effort: Option<usize>,
    risk: Option<usize>,
}

struct TableLocation {
    start_line: usize,
    cols: ColumnMap,
}

fn heading_regex() -> Regex {
    Regex::new(r"(?im)^#{1,6}\s+(actionable\s+plan|backlog|action\s+items)\b").unwrap()
}

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
            in_section = false;
            continue;
        }
        if !line.trim_start().starts_with('|') {
            continue;
        }
        if let Some(cols) = parse_header_row(line) {
            return Some(TableLocation { start_line: i, cols });
        }
    }
    None
}

fn parse_header_row(line: &str) -> Option<ColumnMap> {
    let cells = split_pipes(line);
    if cells.is_empty() {
        return None;
    }
    let lower: Vec<String> = cells.iter().map(|c| c.to_lowercase()).collect();
    let action = lower.iter().position(|c| c.contains("action"))?;
    let id = lower
        .iter()
        .position(|c| c.trim() == "#" || c.contains("id"));
    let effort = lower
        .iter()
        .position(|c| c.contains("effort") || c.contains("hours") || c.contains("time"));
    let risk = lower
        .iter()
        .position(|c| c.contains("risk") || c.contains("severity") || c.contains("priority"));
    Some(ColumnMap { id, action, effort, risk })
}

fn extract_rows(lines: &[&str], table: TableLocation, source_path: &str) -> Vec<Action> {
    let mut out = Vec::new();
    for (offset, line) in lines.iter().enumerate().skip(table.start_line + 1) {
        let trimmed = line.trim_start();
        if !trimmed.starts_with('|') {
            break;
        }
        if is_divider(line) {
            continue;
        }
        if let Some(act) = build_action(line, table.cols, offset + 1, out.len() + 1, source_path) {
            out.push(act);
        }
    }
    out
}

fn is_divider(line: &str) -> bool {
    let t = line.trim();
    t.starts_with('|') && t.chars().all(|c| matches!(c, '|' | '-' | ':' | ' '))
}

fn build_action(
    line: &str,
    cols: ColumnMap,
    source_line: usize,
    fallback_n: usize,
    source_path: &str,
) -> Option<Action> {
    let cells = split_pipes(line);
    let title = cells.get(cols.action)?.trim().to_string();
    if title.is_empty() {
        return None;
    }
    let id = cell_or_fallback(&cells, cols.id, fallback_n);
    let effort = cell_or_default(&cells, cols.effort);
    let severity_text = cell_or_default(&cells, cols.risk);
    let deps = parse_deps_hint(&title);
    let body = format!(
        "Source: {} L{}\n\n{}\nSeverity hint: {}\nEffort hint: {}",
        source_path, source_line, title, severity_text, effort
    );
    Some(
        Action::new(id, title, FORMAT, source_path, source_line)
            .with_effort(effort)
            .with_severity(Severity::from_text(&severity_text))
            .with_deps(deps)
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
