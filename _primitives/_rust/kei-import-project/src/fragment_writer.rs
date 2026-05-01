//! Write and manage canonical SKILL.md fragment files on disk.
//!
//! One fragment file per extracted skill. Format: YAML frontmatter
//! (name + description) followed by the verbatim section body.
//! Idempotent: if content is unchanged the file is not rewritten.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Outcome of a single write attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteOutcome {
    Written,
    Unchanged,
}

/// Build the canonical fragment file path.
///
/// Pattern: `<fragments_dir>/<project_slug>__<source_stem>__<section_slug>.md`
pub fn fragment_path(fragments_dir: &Path, project_slug: &str, source_stem: &str, section_slug: &str) -> PathBuf {
    let filename = format!(
        "{}__{}__{}.md",
        sanitize(project_slug),
        sanitize(source_stem),
        sanitize(section_slug)
    );
    fragments_dir.join(filename)
}

/// Render SKILL.md content (frontmatter + body).
pub fn render_skill_md(name: &str, description: &str, body: &str) -> String {
    format!(
        "---\nname: {name}\ndescription: {desc}\n---\n\n{body}",
        name = name,
        desc = description,
        body = body.trim_end()
    )
}

/// Write a fragment file. Returns Unchanged if the existing content matches.
pub fn write_fragment(path: &Path, content: &str) -> Result<WriteOutcome> {
    if path.exists() {
        let existing = std::fs::read_to_string(path)
            .with_context(|| format!("read existing fragment {}", path.display()))?;
        if existing == content {
            return Ok(WriteOutcome::Unchanged);
        }
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create fragments dir {}", parent.display()))?;
    }
    std::fs::write(path, content)
        .with_context(|| format!("write fragment {}", path.display()))?;
    Ok(WriteOutcome::Written)
}

/// Slugify a string: lowercase, replace non-alnum with `-`, collapse runs.
pub fn sanitize(s: &str) -> String {
    let lower = s.to_lowercase();
    let mut out = String::with_capacity(lower.len());
    let mut prev_dash = false;
    for c in lower.chars() {
        if c.is_alphanumeric() || c == '-' || c == '_' {
            out.push(c);
            prev_dash = false;
        } else {
            if !prev_dash && !out.is_empty() {
                out.push('-');
            }
            prev_dash = true;
        }
    }
    out.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn render_has_frontmatter() {
        let md = render_skill_md("proj::setup", "Quick start guide.", "## Setup\nDo this.");
        assert!(md.starts_with("---\nname:"));
        assert!(md.contains("description: Quick start guide."));
        assert!(md.contains("---\n\n"));
    }

    #[test]
    fn write_unchanged_on_same_content() {
        let dir = TempDir::new().unwrap();
        let p = dir.path().join("test.md");
        let content = "hello world";
        assert_eq!(write_fragment(&p, content).unwrap(), WriteOutcome::Written);
        assert_eq!(write_fragment(&p, content).unwrap(), WriteOutcome::Unchanged);
    }

    #[test]
    fn write_written_on_changed_content() {
        let dir = TempDir::new().unwrap();
        let p = dir.path().join("test.md");
        write_fragment(&p, "v1").unwrap();
        assert_eq!(write_fragment(&p, "v2").unwrap(), WriteOutcome::Written);
    }

    #[test]
    fn sanitize_replaces_spaces() {
        // trailing punctuation is trimmed from the slug
        let s1 = sanitize("Hello World!");
        assert!(!s1.ends_with('-'), "trailing dash must be trimmed: {s1}");
        assert!(s1.starts_with("hello"), "slug starts with hello: {s1}");
        let s2 = sanitize("foo bar");
        assert!(!s2.ends_with('-'), "trailing dash must be trimmed: {s2}");
    }
}
