//! /new-project phases adapter.
//!
//! Detects:
//!   - One or more `## Phase N` headings (`## Phase 1: scaffold`, etc.)
//!   - Often combined with `## Verification` / `## Output` per phase.
//!
//! Extracts:
//!   - One Action per `## Phase N` heading. The Action title is the phase
//!     summary; the body is the phase content up to the next phase or EOF.

use anyhow::{Context, Result};
use regex::Regex;
use std::path::Path;

use super::{Confidence, FormatParser};
use crate::normalizer::{Action, Severity};

pub struct NewProjectParser;

const FORMAT: &str = "new-project";

impl FormatParser for NewProjectParser {
    fn name(&self) -> &str {
        FORMAT
    }

    fn detect(&self, md: &str) -> Confidence {
        let phases: Vec<_> = phase_heading_regex().find_iter(md).collect();
        match phases.len() {
            0 => Confidence::NONE,
            1 => Confidence::AMBIGUOUS,
            2 => Confidence::HEADER,
            _ => Confidence::EXACT,
        }
    }

    fn parse(&self, path: &Path) -> Result<Vec<Action>> {
        let body = std::fs::read_to_string(path)
            .with_context(|| format!("read new-project md: {}", path.display()))?;
        let lines: Vec<&str> = body.lines().collect();
        let path_s = path.display().to_string();
        Ok(extract_phases(&lines, &path_s))
    }
}

fn phase_heading_regex() -> Regex {
    Regex::new(r"(?im)^#{1,6}\s+phase\s+(\d+)(?:\s*[:\-]\s*(.+))?$").unwrap()
}

fn extract_phases(lines: &[&str], source_path: &str) -> Vec<Action> {
    let re = phase_heading_regex();
    let positions = collect_phase_positions(lines, &re);
    if positions.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(positions.len());
    for (idx, info) in positions.iter().enumerate() {
        let next_line = positions
            .get(idx + 1)
            .map(|n| n.line_idx)
            .unwrap_or(lines.len());
        let body_lines = &lines[info.line_idx + 1..next_line];
        out.push(build_phase_action(info, body_lines, source_path));
    }
    out
}

#[derive(Debug, Clone)]
struct PhaseInfo {
    line_idx: usize,
    number: String,
    title: String,
}

fn collect_phase_positions(lines: &[&str], re: &Regex) -> Vec<PhaseInfo> {
    let mut out = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if let Some(c) = re.captures(line) {
            let number = c.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            let title = c
                .get(2)
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_else(|| format!("Phase {}", number));
            out.push(PhaseInfo { line_idx: i, number, title });
        }
    }
    out
}

fn build_phase_action(info: &PhaseInfo, body_lines: &[&str], source_path: &str) -> Action {
    let body_text: String = body_lines
        .iter()
        .take(40)
        .copied()
        .collect::<Vec<&str>>()
        .join("\n");
    let body = format!(
        "Source: {} L{}\n\nPhase {}: {}\n\n{}",
        source_path, info.line_idx + 1, info.number, info.title, body_text
    );
    Action::new(
        info.number.clone(),
        info.title.clone(),
        FORMAT,
        source_path,
        info.line_idx + 1,
    )
    .with_severity(Severity::Medium)
    .with_body(body)
}
