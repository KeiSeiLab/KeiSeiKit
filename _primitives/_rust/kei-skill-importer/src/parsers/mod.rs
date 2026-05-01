//! Format parsers — one module per source dialect.
//!
//! Each parser exposes `pub fn parse(path: &Path) -> Result<ImportedSkill>`
//! and is side-effect-free (read-only file access).

pub mod claude;
pub mod cline;
pub mod cursor;
pub mod kimi;
pub mod openclaw;

use crate::canonical::SourceFormat;
use std::path::Path;

/// Detect format from extension + content sniffing.
///
/// Resolution order:
/// 1. `.mdc` → Cursor.
/// 2. `.yaml` / `.yml` → Kimi (agent spec).
/// 3. Filename `AGENTS.md` (any case) → OpenClaw.
/// 4. Filename `SKILL.md` (any case) → ClaudeCode.
/// 5. Filename starts with `cline-` or path contains `.clinerules` → Cline.
/// 6. Default → ClaudeCode (our native format).
pub fn detect_format(path: &Path) -> SourceFormat {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if ext == "mdc" {
        return SourceFormat::Cursor;
    }
    if ext == "yaml" || ext == "yml" {
        return SourceFormat::Kimi;
    }
    let fname = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if fname == "agents.md" {
        return SourceFormat::OpenClaw;
    }
    if fname == "skill.md" {
        return SourceFormat::ClaudeCode;
    }
    let path_str = path.to_string_lossy().to_ascii_lowercase();
    if fname.starts_with("cline-") || path_str.contains(".clinerules") {
        return SourceFormat::Cline;
    }
    if fname.starts_with("kimi-") {
        return SourceFormat::Kimi;
    }
    if fname.starts_with("openclaw-") {
        return SourceFormat::OpenClaw;
    }
    if fname.starts_with("cursor-") {
        return SourceFormat::Cursor;
    }
    SourceFormat::ClaudeCode
}

/// Shared utility: split a markdown file into `(frontmatter_yaml, body)`.
/// Returns `("", &full)` when no frontmatter is present.
pub(crate) fn split_frontmatter(text: &str) -> (&str, &str) {
    let stripped = text
        .strip_prefix("---\n")
        .or_else(|| text.strip_prefix("---\r\n"));
    let Some(rest) = stripped else {
        return ("", text);
    };
    let mut idx = 0usize;
    for line in rest.split_inclusive('\n') {
        let trimmed = line.trim_end_matches(&['\n', '\r'][..]);
        if trimmed == "---" {
            let fm = &rest[..idx];
            let body = rest.get(idx + line.len()..).unwrap_or("");
            return (fm, body);
        }
        idx += line.len();
    }
    ("", text)
}

/// Heuristic language detector: counts Cyrillic codepoints in the
/// first 4 KiB of body and tags `Some("ru")` if `>= 5%`. Otherwise
/// `Some("en")`. Returns `None` when body is empty.
pub(crate) fn detect_language(body: &str) -> Option<String> {
    if body.is_empty() {
        return None;
    }
    let sample: String = body.chars().take(4096).collect();
    if sample.is_empty() {
        return None;
    }
    let cyr = sample
        .chars()
        .filter(|c| matches!(*c, '\u{0400}'..='\u{04FF}'))
        .count();
    let total = sample.chars().count().max(1);
    if cyr * 100 / total >= 5 {
        Some("ru".into())
    } else {
        Some("en".into())
    }
}
