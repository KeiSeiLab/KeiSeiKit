//! Sleep-layer report adapter (RULE 0.15 Phase B/C output).
//!
//! Detects:
//!   - Frontmatter / commit refs containing `REM:` or `NREM:`
//!   - `## Patterns` section (cross-session pattern listing)
//!   - `## Backlog` section (open follow-ups)
//!
//! Extracts:
//!   - `- [ ] action` checklist items as Actions
//!   - Pattern rows under `## Patterns` (one Action per pattern)

use anyhow::{Context, Result};
use regex::Regex;
use std::path::Path;

use super::{Confidence, FormatParser};
use crate::normalizer::{Action, Severity};

pub struct SleepParser;

const FORMAT: &str = "sleep";

impl FormatParser for SleepParser {
    fn name(&self) -> &str {
        FORMAT
    }

    fn detect(&self, md: &str) -> Confidence {
        let has_rem = md.contains("REM:") || md.contains("NREM:");
        let has_patterns = section_present(md, "patterns");
        let has_backlog = section_present(md, "backlog");
        let hits = (has_rem as u8) + (has_patterns as u8) + (has_backlog as u8);
        match hits {
            3 => Confidence::EXACT,
            2 => Confidence::HEADER,
            1 => Confidence::AMBIGUOUS,
            _ => Confidence::NONE,
        }
    }

    fn parse(&self, path: &Path) -> Result<Vec<Action>> {
        let body = std::fs::read_to_string(path)
            .with_context(|| format!("read sleep md: {}", path.display()))?;
        let lines: Vec<&str> = body.lines().collect();
        let path_s = path.display().to_string();
        let mut out = Vec::new();
        out.extend(extract_checklist_items(&lines, &path_s));
        out.extend(extract_pattern_rows(&lines, &path_s, out.len()));
        Ok(out)
    }
}

fn section_present(md: &str, name: &str) -> bool {
    let re = Regex::new(&format!(r"(?im)^#{{1,6}}\s+{}\b", regex::escape(name))).unwrap();
    re.is_match(md)
}

fn checklist_regex() -> Regex {
    Regex::new(r"^\s*-\s+\[\s\]\s+(.+)$").unwrap()
}

fn extract_checklist_items(lines: &[&str], source_path: &str) -> Vec<Action> {
    let re = checklist_regex();
    let mut out = Vec::new();
    for (offset, line) in lines.iter().enumerate() {
        if let Some(c) = re.captures(line) {
            let title = c[1].trim().to_string();
            if title.is_empty() {
                continue;
            }
            out.push(build_checklist_action(title, offset + 1, out.len() + 1, source_path));
        }
    }
    out
}

fn build_checklist_action(
    title: String,
    source_line: usize,
    n: usize,
    source_path: &str,
) -> Action {
    let body = format!(
        "Source: {} L{}\n\nSleep-report checklist item: {}",
        source_path, source_line, title
    );
    Action::new(format!("c{}", n), title, FORMAT, source_path, source_line)
        .with_severity(Severity::Medium)
        .with_body(body)
}

fn extract_pattern_rows(lines: &[&str], source_path: &str, base_n: usize) -> Vec<Action> {
    let pattern_heading = Regex::new(r"(?im)^#{1,6}\s+patterns\b").unwrap();
    let mut in_section = false;
    let mut out = Vec::new();
    for (offset, line) in lines.iter().enumerate() {
        if pattern_heading.is_match(line) {
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
        if let Some(act) =
            try_pattern_row(line, offset + 1, base_n + out.len() + 1, source_path)
        {
            out.push(act);
        }
    }
    out
}

fn try_pattern_row(
    line: &str,
    source_line: usize,
    n: usize,
    source_path: &str,
) -> Option<Action> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with('-') && !trimmed.starts_with('|') {
        return None;
    }
    let title = strip_marker(trimmed).trim().to_string();
    if title.is_empty() {
        return None;
    }
    let body = format!(
        "Source: {} L{}\n\nSleep-report pattern: {}",
        source_path, source_line, title
    );
    Some(
        Action::new(format!("p{}", n), title, FORMAT, source_path, source_line)
            .with_severity(Severity::Low)
            .with_body(body),
    )
}

fn strip_marker(line: &str) -> &str {
    let l = line.trim_start();
    if let Some(rest) = l.strip_prefix("- ") {
        return rest;
    }
    if let Some(rest) = l.strip_prefix("|") {
        return rest;
    }
    l
}
