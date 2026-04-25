//! Conventional-commit subject parser.
//!
//! Shape: `type(scope)!: subject` — scope and `!` optional.
//! Returns `(kind, scope, subject, breaking)`. Malformed → `Other` kind with
//! the full subject as `subject`.

use crate::commit::CommitKind;
use regex::Regex;
use std::sync::OnceLock;

fn re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| {
        Regex::new(r"^(?P<kind>[a-zA-Z]+)(?:\((?P<scope>[^)]+)\))?(?P<bang>!)?:\s+(?P<subject>.+)$")
            .expect("valid regex")
    })
}

fn kind_from(raw: &str) -> CommitKind {
    match raw.to_ascii_lowercase().as_str() {
        "feat" => CommitKind::Feat,
        "fix" => CommitKind::Fix,
        "refactor" => CommitKind::Refactor,
        "docs" => CommitKind::Docs,
        "test" => CommitKind::Test,
        "chore" => CommitKind::Chore,
        "perf" => CommitKind::Perf,
        "ci" => CommitKind::Ci,
        "build" => CommitKind::Build,
        "checkpoint" => CommitKind::Checkpoint,
        "audit" => CommitKind::Audit,
        other => CommitKind::Other(other.to_string()),
    }
}

/// Parse a commit subject line.
///
/// Returns `(kind, scope, subject, breaking)`. On a non-conventional subject,
/// returns `(Other("_"), None, full_line, false)`.
#[must_use]
pub fn parse_subject(first_line: &str) -> (CommitKind, Option<String>, String, bool) {
    let trimmed = first_line.trim();
    match re().captures(trimmed) {
        Some(c) => {
            let kind = kind_from(&c["kind"]);
            let scope = c.name("scope").map(|m| m.as_str().to_string());
            let subject = c["subject"].to_string();
            let breaking = c.name("bang").is_some();
            (kind, scope, subject, breaking)
        }
        None => (
            CommitKind::Other("_".into()),
            None,
            trimmed.to_string(),
            false,
        ),
    }
}
