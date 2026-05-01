//! /architecture decisions adapter.
//!
//! Detects:
//!   - `## Decision` heading (single architectural decision file)
//!   - `## Recommendation` / `## Recommendations` section
//!
//! Extracts:
//!   - Numbered recommendations under the recommendations heading
//!     (e.g. `1. Adopt X`, `2. Refactor Y`) — one Action per item.

use anyhow::{Context, Result};
use regex::Regex;
use std::path::Path;

use super::{Confidence, FormatParser};
use crate::normalizer::{Action, Severity};

pub struct ArchitectureParser;

const FORMAT: &str = "architecture";

impl FormatParser for ArchitectureParser {
    fn name(&self) -> &str {
        FORMAT
    }

    fn detect(&self, md: &str) -> Confidence {
        let has_decision = decision_heading_regex().is_match(md);
        let has_reco = recommendation_heading_regex().is_match(md);
        let has_impl = md.to_lowercase().contains("## implementation");
        let hits = (has_decision as u8) + (has_reco as u8) + (has_impl as u8);
        match hits {
            3 => Confidence::EXACT,
            2 => Confidence::HEADER,
            1 => Confidence::AMBIGUOUS,
            _ => Confidence::NONE,
        }
    }

    fn parse(&self, path: &Path) -> Result<Vec<Action>> {
        let body = std::fs::read_to_string(path)
            .with_context(|| format!("read architecture md: {}", path.display()))?;
        let lines: Vec<&str> = body.lines().collect();
        let path_s = path.display().to_string();
        Ok(extract_recommendations(&lines, &path_s))
    }
}

fn decision_heading_regex() -> Regex {
    Regex::new(r"(?im)^#{1,6}\s+decision\b").unwrap()
}

fn recommendation_heading_regex() -> Regex {
    Regex::new(r"(?im)^#{1,6}\s+recommendation(s)?\b").unwrap()
}

fn numbered_item_regex() -> Regex {
    Regex::new(r"^\s*(\d+)\.\s+(.+)$").unwrap()
}

fn extract_recommendations(lines: &[&str], source_path: &str) -> Vec<Action> {
    let heading = recommendation_heading_regex();
    let item = numbered_item_regex();
    let mut in_section = false;
    let mut out = Vec::new();
    for (offset, line) in lines.iter().enumerate() {
        if heading.is_match(line) {
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
        if let Some(c) = item.captures(line) {
            let id = c[1].to_string();
            let title = c[2].trim().to_string();
            if title.is_empty() {
                continue;
            }
            out.push(build_recommendation(id, title, offset + 1, source_path));
        }
    }
    out
}

fn build_recommendation(
    id: String,
    title: String,
    source_line: usize,
    source_path: &str,
) -> Action {
    let body = format!(
        "Source: {} L{}\n\nArchitecture recommendation #{}: {}",
        source_path, source_line, id, title
    );
    Action::new(id, title, FORMAT, source_path, source_line)
        .with_severity(Severity::Medium)
        .with_body(body)
}
