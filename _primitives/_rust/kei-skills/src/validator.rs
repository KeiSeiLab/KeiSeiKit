//! SKILL.md validation — port of Hermes
//! `tools/skill_manager_tool.py::_validate_frontmatter` + size caps.
//!
//! Returns a `Vec<ValidationIssue>` so a single skill can fail multiple
//! checks in one pass. Empty Vec = valid.

use crate::format::{parse, Skill};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::OnceLock;

/// Hermes parity: ~36k tokens at 2.75 chars/token.
pub const MAX_SKILL_CONTENT_CHARS: usize = 100_000;
/// Hermes parity: 1 MiB ceiling per supporting file (and SKILL.md itself).
pub const MAX_SKILL_FILE_BYTES: usize = 1_048_576;
/// Hermes parity: ≤1024 chars on `description`.
pub const MAX_DESCRIPTION_LENGTH: usize = 1_024;
/// Hermes parity: ≤64 chars on `name`.
pub const MAX_NAME_LENGTH: usize = 64;

/// Slug regex — lowercase letters/digits, then `[a-z0-9._-]*`.
fn name_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^[a-z0-9][a-z0-9._-]*$").expect("static regex compiles"))
}

/// One validation finding. Multiple may stack on one skill (e.g. body
/// missing AND description too long).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationIssue {
    pub kind: IssueKind,
    pub message: String,
}

/// Discriminator on `ValidationIssue`. Stable across versions — callers
/// (Phase D archive policy, agent UI) match on this.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IssueKind {
    MissingOpenFence,
    UnclosedFrontmatter,
    YamlParse,
    NotMapping,
    MissingName,
    MissingDescription,
    NameInvalid,
    NameTooLong,
    DescriptionTooLong,
    BodyEmpty,
    ContentTooLarge,
    FileTooLarge,
}

fn issue(kind: IssueKind, msg: impl Into<String>) -> ValidationIssue {
    ValidationIssue { kind, message: msg.into() }
}

/// Validate raw SKILL.md content. Path is informational (used in
/// messages); pass `Path::new("<inline>")` if no on-disk source.
pub fn validate(content: &str, path: &Path) -> Result<Skill, Vec<ValidationIssue>> {
    let mut issues: Vec<ValidationIssue> = Vec::new();
    if content.len() > MAX_SKILL_FILE_BYTES {
        issues.push(issue(
            IssueKind::FileTooLarge,
            format!("file exceeds {MAX_SKILL_FILE_BYTES} bytes"),
        ));
    }
    if content.chars().count() > MAX_SKILL_CONTENT_CHARS {
        issues.push(issue(
            IssueKind::ContentTooLarge,
            format!("content exceeds {MAX_SKILL_CONTENT_CHARS} chars"),
        ));
    }
    if !content.starts_with("---") {
        issues.push(issue(IssueKind::MissingOpenFence, "must start with `---`"));
        return Err(issues);
    }
    let parsed = match parse(content, path.to_path_buf()) {
        Ok(s) => s,
        Err(e) => {
            // Distinguish unclosed fence from yaml parse via message text.
            let msg = e.to_string();
            let kind = if msg.contains("not closed") {
                IssueKind::UnclosedFrontmatter
            } else {
                IssueKind::YamlParse
            };
            issues.push(issue(kind, msg));
            return Err(issues);
        }
    };
    check_frontmatter(&parsed, &mut issues);
    if parsed.body.trim().is_empty() {
        issues.push(issue(IssueKind::BodyEmpty, "body must be non-empty after frontmatter"));
    }
    if issues.is_empty() {
        Ok(parsed)
    } else {
        Err(issues)
    }
}

fn check_frontmatter(skill: &Skill, issues: &mut Vec<ValidationIssue>) {
    if skill.frontmatter.name.is_empty() {
        issues.push(issue(IssueKind::MissingName, "frontmatter missing `name`"));
    } else if skill.frontmatter.name.len() > MAX_NAME_LENGTH {
        issues.push(issue(
            IssueKind::NameTooLong,
            format!("name exceeds {MAX_NAME_LENGTH} chars"),
        ));
    } else if !name_re().is_match(&skill.frontmatter.name) {
        issues.push(issue(
            IssueKind::NameInvalid,
            format!("name `{}` not slug-form [a-z0-9][a-z0-9._-]*", skill.frontmatter.name),
        ));
    }
    if skill.frontmatter.description.is_empty() {
        issues.push(issue(IssueKind::MissingDescription, "frontmatter missing `description`"));
    } else if skill.frontmatter.description.len() > MAX_DESCRIPTION_LENGTH {
        issues.push(issue(
            IssueKind::DescriptionTooLong,
            format!("description exceeds {MAX_DESCRIPTION_LENGTH} chars"),
        ));
    }
}
