//! Unified Action struct + severity helpers.
//!
//! Every parser in the registry yields `Action` regardless of source format.
//! Downstream (emitter, dispatcher) consumes only `Action`, so adding a new
//! format means adding a parser — never touching the consumers.

use serde::{Deserialize, Serialize};

/// Source-format tag carried alongside each Action.
///
/// Lowercase string preserved for forward-compat (new formats added later
/// don't require enum bumps in older clients).
pub type SourceFormat = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    High,
    Medium,
    Low,
    Unknown,
}

impl Severity {
    pub fn from_text(s: &str) -> Self {
        let t = s.to_lowercase();
        if t.contains("high") || t.contains("critical") || t.contains("p0") {
            Self::High
        } else if t.contains("med") || t.contains("p1") {
            Self::Medium
        } else if t.contains("low") || t.contains("p2") || t.contains("p3") {
            Self::Low
        } else {
            Self::Unknown
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::Unknown => "unknown",
        }
    }
}

/// Single canonical shape across all formats.
///
/// Fields:
///   id              — stable per-source identifier (numeric or slug)
///   title           — short action title (one line)
///   source_format   — "research" / "audit" / "sleep" / "architecture" / "new-project"
///   source_path     — absolute path to MD file
///   source_line     — 1-based line number where the action originates
///   effort_hint     — free-text effort hint as parsed (e.g. "1-2h")
///   severity        — normalized severity
///   deps            — referenced upstream action ids (best-effort)
///   body            — long-form description for the task body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub title: String,
    pub source_format: SourceFormat,
    pub source_path: String,
    pub source_line: usize,
    pub effort_hint: String,
    pub severity: Severity,
    pub deps: Vec<String>,
    pub body: String,
}

impl Action {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        source_format: impl Into<String>,
        source_path: impl Into<String>,
        source_line: usize,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            source_format: source_format.into(),
            source_path: source_path.into(),
            source_line,
            effort_hint: String::new(),
            severity: Severity::Unknown,
            deps: Vec::new(),
            body: String::new(),
        }
    }

    pub fn with_effort(mut self, e: impl Into<String>) -> Self {
        self.effort_hint = e.into();
        self
    }

    pub fn with_severity(mut self, s: Severity) -> Self {
        self.severity = s;
        self
    }

    pub fn with_deps(mut self, deps: Vec<String>) -> Self {
        self.deps = deps;
        self
    }

    pub fn with_body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_from_text_classifies_keywords() {
        assert_eq!(Severity::from_text("HIGH"), Severity::High);
        assert_eq!(Severity::from_text("Critical"), Severity::High);
        assert_eq!(Severity::from_text("medium"), Severity::Medium);
        assert_eq!(Severity::from_text("Low"), Severity::Low);
        assert_eq!(Severity::from_text("P0"), Severity::High);
        assert_eq!(Severity::from_text(""), Severity::Unknown);
    }

    #[test]
    fn action_builder_sets_fields() {
        let a = Action::new("3", "Refactor X", "research", "/tmp/x.md", 42)
            .with_effort("1-2h")
            .with_severity(Severity::High)
            .with_deps(vec!["1".into(), "2".into()]);
        assert_eq!(a.id, "3");
        assert_eq!(a.severity.as_str(), "high");
        assert_eq!(a.deps.len(), 2);
        assert_eq!(a.source_format, "research");
    }
}
