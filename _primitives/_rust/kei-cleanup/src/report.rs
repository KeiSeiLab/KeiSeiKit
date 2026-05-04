//! Report types per CLEANUP-RUNTIME-SPEC.md §3.3.
//!
//! `CleanupReport` is the structured output of a kei-cleanup run.
//! Each scanner contributes zero or more `Finding`s; the runtime
//! aggregates them, computes counts, and emits via TTY or JSON.

use chrono::{DateTime, Utc};
use serde::Serialize;
use std::path::PathBuf;

/// Top-level report — see CLEANUP-RUNTIME-SPEC.md §3.3.
#[derive(Clone, Debug, Serialize)]
pub struct CleanupReport {
    /// JSON schema version. v0.1 = 1.
    pub schema_version: u32,
    /// Workspace root that was scanned.
    pub workspace: PathBuf,
    /// Short git SHA of HEAD (or "unknown" if not a git workspace).
    pub workspace_sha: String,
    /// UTC timestamp when the run started.
    pub timestamp_utc: DateTime<Utc>,
    /// Wall-clock runtime in milliseconds.
    pub runtime_ms: u64,
    /// All findings, in scanner order.
    pub findings: Vec<Finding>,
    /// Aggregate counts by severity.
    pub counts: Counts,
    /// Names of scanners that ran successfully.
    pub scanners_run: Vec<String>,
    /// Names of scanners skipped, with reason (e.g. "tool not found").
    pub scanners_skipped: Vec<(String, String)>,
}

/// Single hygiene finding — see CLEANUP-RUNTIME-SPEC.md §3.3.
#[derive(Clone, Debug, Serialize)]
pub struct Finding {
    /// Category of finding.
    pub kind: FindingKind,
    /// Severity bucket.
    pub severity: Severity,
    /// File + optional line.
    pub location: Location,
    /// Human-readable issue description.
    pub message: String,
    /// Optional textual fix suggestion.
    pub fix_hint: Option<String>,
    /// Optional cross-reference to a rule or spec section.
    pub rule_ref: Option<String>,
    /// Detector confidence (some scanners are heuristic).
    pub confidence: Confidence,
}

/// Finding category — see CLEANUP-RUNTIME-SPEC.md §3.3.
#[derive(Copy, Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// `pub` item with zero workspace callers (machete / udeps).
    DeadCode,
    /// Dependency declared in Cargo.toml, never imported in src/.
    UnusedDep,
    /// Member dep version differs from `[workspace.dependencies]`.
    DepDrift,
    /// File exceeds Constructor Pattern LOC limit.
    LocFile,
    /// Function exceeds Constructor Pattern LOC limit.
    LocFunction,
    /// TODO/FIXME/XXX/HACK comment older than configured threshold.
    StaleTodo,
    /// Derivation marker without matching test marker.
    CoverageGap,
    /// Test references 2+ sibling crates (dev-dep cycle risk).
    WorkspaceTestNeeded,
    /// `cargo doc` warning (broken intra-doc link, etc).
    DocWarning,
    /// Naming-pair drift: two variants of the same constant in use.
    NamingDrift,
}

/// Severity bucket — controls `--fail-on` and HIGH/MEDIUM/LOW grouping.
#[derive(Copy, Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Cosmetic / batch-with-next-refactor.
    Low,
    /// Schedule into next cleanup window (≤7 d).
    Medium,
    /// Block merge to main; needs orchestrator decision.
    High,
}

/// Detector confidence — distinguishes deterministic from heuristic scanners.
#[derive(Copy, Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    /// Heuristic / may have false positives (e.g. cargo-machete).
    Low,
    /// Deterministic (e.g. LOC count, version compare).
    High,
}

/// File + optional line location.
#[derive(Clone, Debug, Serialize)]
pub struct Location {
    /// Path relative to workspace root if possible, else absolute.
    pub file: PathBuf,
    /// 1-based line, or None when not file-line specific.
    pub line: Option<u32>,
}

/// Aggregate severity counts.
#[derive(Copy, Clone, Debug, Default, Serialize)]
pub struct Counts {
    /// HIGH finding count.
    pub high: usize,
    /// MEDIUM finding count.
    pub medium: usize,
    /// LOW finding count.
    pub low: usize,
    /// Total = high + medium + low.
    pub total: usize,
}

impl Counts {
    /// Build from a slice of findings — single pass, O(n).
    pub fn from_findings(findings: &[Finding]) -> Self {
        let mut c = Self::default();
        for f in findings {
            match f.severity {
                Severity::High => c.high += 1,
                Severity::Medium => c.medium += 1,
                Severity::Low => c.low += 1,
            }
        }
        c.total = findings.len();
        c
    }
}

impl Severity {
    /// Parse `--fail-on` argument: "high" / "medium" / "low".
    pub fn parse_min(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "high" => Some(Severity::High),
            "medium" | "med" => Some(Severity::Medium),
            "low" => Some(Severity::Low),
            _ => None,
        }
    }
}
