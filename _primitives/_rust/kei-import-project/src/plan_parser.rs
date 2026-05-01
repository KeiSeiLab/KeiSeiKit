//! plan_parser — reverse of plan_render: parse plan.md → structured form.
//!
//! Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

use anyhow::{Context, Result};
use regex::Regex;
use std::path::Path;

// ─────────────────────────── public types ──────────────────────────────────

/// One module entry inside a parsed phase.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedModule {
    pub name: String,
    pub confidence: f64,
}

/// One migration phase parsed from the plan.md per-phase detail section.
#[derive(Debug, Clone)]
pub struct ParsedPhase {
    pub id: String,
    pub trait_family: String,
    pub priority: u8,
    pub status: String, // "scaffolding" | "blocked-needs-review"
    pub modules: Vec<ParsedModule>,
}

/// The full parsed plan.
#[derive(Debug, Clone)]
pub struct ParsedPlan {
    pub project_name: String,
    pub source_repo: String,
    pub phases: Vec<ParsedPhase>,
    pub unmatched: Vec<String>,
}

// ─────────────────────────── public API ────────────────────────────────────

/// Parse a plan.md string into a `ParsedPlan`.
pub fn parse_plan(content: &str) -> Result<ParsedPlan> {
    let project_name = extract_project_name(content);
    let source_repo = extract_source_repo(content);
    let phases = extract_phases(content)?;
    let unmatched = extract_unmatched(content);
    Ok(ParsedPlan { project_name, source_repo, phases, unmatched })
}

/// Read a plan.md file and parse it.
pub fn parse_plan_file(path: &Path) -> Result<ParsedPlan> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    parse_plan(&content)
}

// ─────────────────────────── extractors ────────────────────────────────────

fn extract_project_name(content: &str) -> String {
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("# ") {
            if let Some(name) = rest.split(" — ").next() {
                return name.trim().to_owned();
            }
        }
    }
    "unknown-project".to_owned()
}

fn extract_source_repo(content: &str) -> String {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("> Source:") {
            return rest.trim().to_owned();
        }
    }
    String::new()
}

/// Parse all `### Px.y — TraitFamily` blocks from the Per-phase detail section.
pub fn extract_phases(content: &str) -> Result<Vec<ParsedPhase>> {
    let after = match content.split("## Per-phase detail").nth(1) {
        Some(s) => s,
        None => return Ok(vec![]),
    };
    let section = match after.split("\n## ").next() {
        Some(s) => s,
        None => after,
    };
    parse_phase_blocks(section)
}

fn parse_phase_blocks(section: &str) -> Result<Vec<ParsedPhase>> {
    let heading_re = Regex::new(r"(?m)^### (P[\w.]+) — (.+)$")?;
    let module_re = Regex::new(r"- ([\w][\w\-]*) \(confidence (0\.\d+)\)")?;

    let matches: Vec<_> = heading_re.find_iter(section).collect();
    let mut phases = Vec::with_capacity(matches.len());

    for (i, m) in matches.iter().enumerate() {
        let caps = heading_re.captures(m.as_str()).unwrap();
        let id = caps[1].to_owned();
        let trait_family = caps[2].trim().to_owned();
        let priority = priority_from_id(&id);
        let status = if id.starts_with("Pwip") {
            "blocked-needs-review".to_owned()
        } else {
            "scaffolding".to_owned()
        };
        let block_end = matches.get(i + 1).map(|n| n.start()).unwrap_or(section.len());
        let block = &section[m.end()..block_end];
        let modules = module_re
            .captures_iter(block)
            .map(|c| ParsedModule {
                name: c[1].to_owned(),
                confidence: c[2].parse().unwrap_or(0.0),
            })
            .collect();
        phases.push(ParsedPhase { id, trait_family, priority, status, modules });
    }
    Ok(phases)
}

fn priority_from_id(id: &str) -> u8 {
    let mut chars = id.chars();
    if chars.next() != Some('P') {
        return 99;
    }
    match chars.next() {
        Some('0') => 0,
        Some('1') => 1,
        Some('2') => 2,
        Some('3') => 3,
        _ => 99,
    }
}

fn extract_unmatched(content: &str) -> Vec<String> {
    let after = match content.split("## Unmatched modules").nth(1) {
        Some(s) => s,
        None => return vec![],
    };
    let section = match after.split("\n## ").next() {
        Some(s) => s,
        None => after,
    };
    section
        .lines()
        .filter_map(|l| l.trim().strip_prefix("- "))
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty())
        .collect()
}
