//! SKILL.md parser/serializer.
//!
//! On-disk shape:
//! ```text
//! ---
//! name: <slug>
//! description: <≤1024 chars>
//! category: <optional>
//! stability: <optional — experimental | validated>
//! ---
//!
//! <markdown body>
//! ```
//!
//! Round-trip rule: `serialize(parse(s)) == s` byte-for-byte for any
//! Hermes / agentskills.io conformant SKILL.md. Tested in
//! `tests/format_roundtrip.rs`.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Frontmatter required by Hermes / agentskills.io. Optional fields kept
/// as `Option<String>` so a missing key serializes back to absence (not
/// `null`) — preserves byte-equality for skills that omit them.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SkillFrontmatter {
    /// Slug-form skill name. Default empty so a missing `name` key in
    /// YAML produces a typed `MissingName` validator issue instead of
    /// surfacing as a generic YAML parse error.
    #[serde(default)]
    pub name: String,
    /// Human-readable summary, ≤1024 chars (validator-enforced). Default
    /// empty for the same reason as `name`.
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub stability: Option<String>,
    /// Catch-all for vendor extensions (`metadata`, `argument-hint`, etc.).
    /// Stored verbatim in YAML order to preserve serialize round-trip.
    #[serde(flatten)]
    pub extra: serde_yaml::Mapping,
}

/// Parsed skill: typed frontmatter + raw markdown body + originating path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub frontmatter: SkillFrontmatter,
    pub body: String,
    #[serde(skip)]
    pub source_path: PathBuf,
}

/// Locate the closing `---` after the opening one. Returns the offset of
/// the newline that terminates the closing fence (so callers slice
/// `&content[3..idx]` to get the YAML and `&content[idx + 4..]` to get
/// the body). `None` when the fence is unbalanced.
fn find_close_fence(content: &str) -> Option<usize> {
    if !content.starts_with("---") {
        return None;
    }
    let after_open = &content[3..];
    let after_open_offset = 3usize;
    // Looking for "\n---\n" or "\n---" at end-of-file (rare but legal).
    let mut search_start = 0usize;
    while let Some(rel) = after_open[search_start..].find("\n---") {
        let abs = after_open_offset + search_start + rel;
        // Must be followed by '\n' or EOF.
        let tail = &content[abs + 4..];
        if tail.is_empty() || tail.starts_with('\n') {
            return Some(abs);
        }
        search_start += rel + 4;
    }
    None
}

/// Parse a SKILL.md string. Errors wrap `anyhow` messages with the
/// originating path included by callers (loader / registry) for context.
pub fn parse(content: &str, source: PathBuf) -> Result<Skill> {
    let close = find_close_fence(content)
        .ok_or_else(|| anyhow!("SKILL.md frontmatter not closed (missing trailing ---)"))?;
    let yaml_str = &content[3..close];
    let frontmatter: SkillFrontmatter = serde_yaml::from_str(yaml_str.trim())
        .map_err(|e| anyhow!("YAML frontmatter parse error: {e}"))?;
    // Body starts after the closing fence's "\n---" plus its own newline if any.
    let body_start = if content[close + 4..].starts_with('\n') {
        close + 5
    } else {
        close + 4
    };
    let body = content[body_start..].to_string();
    Ok(Skill { frontmatter, body, source_path: source })
}

/// Serialize back to canonical SKILL.md form. Layout:
/// `---\n<yaml>---\n<body>` — matches Hermes writer (`skill_manager_tool.py`).
pub fn serialize(skill: &Skill) -> Result<String> {
    let yaml = serde_yaml::to_string(&skill.frontmatter)
        .map_err(|e| anyhow!("YAML serialize: {e}"))?;
    let mut out = String::with_capacity(yaml.len() + skill.body.len() + 16);
    out.push_str("---\n");
    out.push_str(&yaml);
    out.push_str("---\n");
    out.push_str(&skill.body);
    Ok(out)
}
