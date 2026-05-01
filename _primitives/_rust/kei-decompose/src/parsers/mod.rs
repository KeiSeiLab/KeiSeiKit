//! FormatParser trait + ordered registry.
//!
//! `Vec<Box<dyn FormatParser>>` (NOT a HashMap) preserves detection order
//! across calls: the first parser to claim wins. Ties resolve by registration
//! order, never by HashMap iteration.

use anyhow::Result;
use std::path::Path;

use crate::normalizer::Action;

pub mod architecture;
pub mod audit;
pub mod new_project;
pub mod research;
pub mod rule;
pub mod sleep;

pub use rule::{parse_rule_file, RuleFragment};

/// Detection confidence — exact-match vs header-only vs ambiguous.
///
/// Values are documented in the spec:
///   1.0   exact-match (multiple structural cues all hit)
///   0.7   header-match (one strong cue)
///   0.5   ambiguous (weak hint)
///   0.0   no signal
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Confidence(pub f64);

impl Confidence {
    pub const NONE: Confidence = Confidence(0.0);
    pub const AMBIGUOUS: Confidence = Confidence(0.5);
    pub const HEADER: Confidence = Confidence(0.7);
    pub const EXACT: Confidence = Confidence(1.0);

    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

/// One adapter per MD output format.
///
/// `name()` is the stable lower-kebab string used in JSON output and as the
/// `--format` flag value.
///
/// `detect()` is fast: scans for headline / table / frontmatter cues without
/// fully parsing.
///
/// `parse()` reads + extracts; result `Vec<Action>` may be empty if the
/// document has the format shape but no actionable rows.
pub trait FormatParser: Send + Sync {
    fn name(&self) -> &str;
    fn detect(&self, md: &str) -> Confidence;
    fn parse(&self, path: &Path) -> Result<Vec<Action>>;
}

/// Standard parser registry — order = detection priority.
///
/// `research` first because its tables have the strictest header signature
/// (`| ... | Action | ... |`); the rest sort by signal strength.
pub fn registry() -> Vec<Box<dyn FormatParser>> {
    vec![
        Box::new(research::ResearchParser),
        Box::new(audit::AuditParser),
        Box::new(architecture::ArchitectureParser),
        Box::new(sleep::SleepParser),
        Box::new(new_project::NewProjectParser),
    ]
}

/// Detection result: best-matching parser plus the full per-parser scoreboard.
#[derive(Debug, Clone)]
pub struct DetectResult {
    pub winner: Option<String>,
    pub confidence: f64,
    pub all_scores: Vec<(String, f64)>,
}

/// Run every parser's `detect`, return best score (ties → first registered).
pub fn detect_format(md: &str) -> DetectResult {
    let reg = registry();
    let mut all_scores = Vec::with_capacity(reg.len());
    let mut best: Option<(String, f64)> = None;
    for p in &reg {
        let c = p.detect(md).as_f64();
        all_scores.push((p.name().to_string(), c));
        if let Some((_, s)) = &best {
            if c > *s {
                best = Some((p.name().to_string(), c));
            }
        } else if c > 0.0 {
            best = Some((p.name().to_string(), c));
        }
    }
    match best {
        Some((name, c)) => DetectResult { winner: Some(name), confidence: c, all_scores },
        None => DetectResult { winner: None, confidence: 0.0, all_scores },
    }
}

/// Lookup parser by lowercase name; returns None if not registered.
pub fn parser_by_name(name: &str) -> Option<Box<dyn FormatParser>> {
    let key = name.to_lowercase();
    let key = key.as_str();
    let mapped = match key {
        "research" => Some(Box::new(research::ResearchParser) as Box<dyn FormatParser>),
        "audit" | "wave-audit" => Some(Box::new(audit::AuditParser) as Box<dyn FormatParser>),
        "sleep" => Some(Box::new(sleep::SleepParser) as Box<dyn FormatParser>),
        "architecture" => {
            Some(Box::new(architecture::ArchitectureParser) as Box<dyn FormatParser>)
        }
        "new-project" | "new_project" => {
            Some(Box::new(new_project::NewProjectParser) as Box<dyn FormatParser>)
        }
        _ => None,
    };
    mapped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_has_five_parsers() {
        let r = registry();
        assert_eq!(r.len(), 5);
        let names: Vec<&str> = r.iter().map(|p| p.name()).collect();
        assert!(names.contains(&"research"));
        assert!(names.contains(&"audit"));
        assert!(names.contains(&"sleep"));
        assert!(names.contains(&"architecture"));
        assert!(names.contains(&"new-project"));
    }

    #[test]
    fn parser_by_name_recognises_aliases() {
        assert!(parser_by_name("audit").is_some());
        assert!(parser_by_name("wave-audit").is_some());
        assert!(parser_by_name("new-project").is_some());
        assert!(parser_by_name("new_project").is_some());
        assert!(parser_by_name("RESEARCH").is_some());
        assert!(parser_by_name("nonsense").is_none());
    }
}
