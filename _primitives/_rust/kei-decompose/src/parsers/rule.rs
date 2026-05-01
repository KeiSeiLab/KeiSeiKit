//! Rule file parser — splits `~/.claude/rules/*.md` into per-H2-section fragments.
//!
//! Each fragment maps to one `RuleFragment` record. The body of the fragment
//! is registered in `kei-registry` as a `BlockType::Rule` block. H3 headings
//! stay inside their H2 parent; code blocks are preserved verbatim (no split
//! inside fences).
//!
//! Constructor Pattern: this cube owns the parsing/splitting logic only.
//! Registry writes live in the `decompose_rules` CLI handler (main.rs).

use std::path::Path;
use anyhow::{Context, Result};

/// One H2-bounded section of a rule file.
#[derive(Debug, Clone, PartialEq)]
pub struct RuleFragment {
    /// Stem of the source file, e.g. `"karpathy-behavioral"`.
    pub rule_slug: String,
    /// Kebab-cased section heading, e.g. `"think-before-coding"`.
    pub section_slug: String,
    /// Verbatim heading text (without leading `## `).
    pub heading: String,
    /// Section body (heading line not included).
    pub body: String,
    /// 1-based line number of the H2 heading (or 1 for `_root`).
    pub line_start: usize,
    /// 1-based line number of the last body line (inclusive).
    pub line_end: usize,
}

/// Parse `path` into per-section `RuleFragment`s.
///
/// Splitting rules:
/// - Split on `## ` H2 headings (not H1).
/// - Skip the H1 block and leading block-quote front-matter.
/// - H3+ headings remain inside their H2 parent body.
/// - Lines inside code fences (````` ``` `````) are never treated as headings.
/// - Empty sections (heading with no body content) are skipped.
/// - Files with no `## ` headings → single `_root` fragment.
pub fn parse_rule_file(path: &Path) -> Result<Vec<RuleFragment>> {
    let slug = file_stem(path);
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("read rule file: {}", path.display()))?;
    Ok(split_sections(&slug, &text))
}

// ── internals ────────────────────────────────────────────────────────────────

fn file_stem(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string()
}

/// Convert a heading string to a kebab-case slug.
fn to_section_slug(heading: &str) -> String {
    let lower = heading.to_lowercase();
    let mut slug = String::new();
    let mut prev_dash = true; // suppress leading dash
    for ch in lower.chars() {
        if ch.is_alphanumeric() {
            slug.push(ch);
            prev_dash = false;
        } else if !prev_dash {
            slug.push('-');
            prev_dash = true;
        }
    }
    // trim trailing dash
    slug.trim_end_matches('-').to_string()
}

/// Push a completed section into `out` if body is non-empty.
fn flush_fragment(
    heading: Option<(String, usize)>,
    body_lines: &[&str],
    rule_slug: &str,
    out: &mut Vec<RuleFragment>,
) {
    let Some((heading, line_start)) = heading else { return };
    let body_text = body_lines.join("\n");
    if body_text.trim().is_empty() {
        return;
    }
    let line_end = line_start + body_lines.len();
    let raw_slug = to_section_slug(&heading);
    let section_slug = if raw_slug.is_empty() { "_root".to_string() } else { raw_slug };
    out.push(RuleFragment {
        rule_slug: rule_slug.to_string(),
        section_slug,
        heading,
        body: body_text,
        line_start,
        line_end,
    });
}

/// Return a fallback `_root` fragment when no H2 headings were found.
fn root_fragment(rule_slug: &str, text: &str, line_count: usize) -> Option<RuleFragment> {
    if text.trim().is_empty() {
        return None;
    }
    Some(RuleFragment {
        rule_slug: rule_slug.to_string(),
        section_slug: "_root".to_string(),
        heading: String::new(),
        body: text.to_string(),
        line_start: 1,
        line_end: line_count,
    })
}

/// State machine for one line during section scanning.
enum LineEffect<'a> {
    StartSection(String, usize), // new H2 heading + 1-based line number
    AppendBody(&'a str),         // append line to current body
    Skip,                        // preamble / H1 outside section
}

/// Classify one line given current fence state and section state.
fn classify_line<'a>(
    line: &'a str,
    lineno: usize,
    in_fence: &mut bool,
    has_section: bool,
) -> LineEffect<'a> {
    let trimmed = line.trim();
    if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
        *in_fence = !*in_fence;
        return LineEffect::AppendBody(line);
    }
    if *in_fence {
        return LineEffect::AppendBody(line);
    }
    if let Some(h) = parse_h2(line) {
        return LineEffect::StartSection(h, lineno);
    }
    if is_h1(line) && !has_section {
        return LineEffect::Skip;
    }
    LineEffect::AppendBody(line)
}

/// Core split logic — operates on the raw text, not file I/O.
fn split_sections(rule_slug: &str, text: &str) -> Vec<RuleFragment> {
    let lines: Vec<&str> = text.lines().collect();
    let mut fragments: Vec<RuleFragment> = Vec::new();
    let mut in_fence = false;
    let mut current_heading: Option<(String, usize)> = None;
    let mut body_lines: Vec<&str> = Vec::new();

    for (idx, &line) in lines.iter().enumerate() {
        match classify_line(line, idx + 1, &mut in_fence, current_heading.is_some()) {
            LineEffect::StartSection(h, n) => {
                flush_fragment(current_heading.take(), &body_lines, rule_slug, &mut fragments);
                body_lines.clear();
                current_heading = Some((h, n));
            }
            LineEffect::AppendBody(l) => body_lines.push(l),
            LineEffect::Skip => {}
        }
    }
    flush_fragment(current_heading, &body_lines, rule_slug, &mut fragments);

    if fragments.is_empty() {
        if let Some(frag) = root_fragment(rule_slug, text, lines.len()) {
            fragments.push(frag);
        }
    }
    fragments
}

/// Returns the heading text if this line is an H2 (`## …`), else None.
fn parse_h2(line: &str) -> Option<String> {
    let rest = line.strip_prefix("## ")?;
    // Reject lines that start with `### ` or more hashes (they would
    // have been caught by strip_prefix("## ") only if literally "## "
    // — but e.g. "### foo" does NOT start with "## ").
    // Disambiguate "## " from "### ": ensure the next char is NOT '#'.
    if rest.starts_with('#') {
        return None;
    }
    let heading = rest.trim().to_string();
    if heading.is_empty() { None } else { Some(heading) }
}

fn is_h1(line: &str) -> bool {
    line.starts_with("# ") && !line.starts_with("## ")
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(text: &str) -> Vec<RuleFragment> {
        split_sections("test-rule", text)
    }

    #[test]
    fn happy_path_three_sections() {
        let md = "# Title\n\n## Section One\n\nBody one.\n\n## Section Two\n\nBody two.\n\n## Third\n\nBody three.\n";
        let frags = parse(md);
        assert_eq!(frags.len(), 3);
        assert_eq!(frags[0].section_slug, "section-one");
        assert_eq!(frags[1].section_slug, "section-two");
        assert_eq!(frags[2].section_slug, "third");
        assert!(frags[0].body.contains("Body one."));
    }

    #[test]
    fn no_h2_headings_returns_root_fragment() {
        let md = "# Title\n\nSome content without any H2 headings.\n";
        let frags = parse(md);
        assert_eq!(frags.len(), 1);
        assert_eq!(frags[0].section_slug, "_root");
        assert!(frags[0].body.contains("Some content"));
    }

    #[test]
    fn headings_inside_code_block_do_not_split() {
        let md = "## Real Section\n\nSome text.\n\n```\n## NOT a heading\n```\n\nMore text.\n";
        let frags = parse(md);
        assert_eq!(frags.len(), 1, "code-block heading must not split");
        assert!(frags[0].body.contains("## NOT a heading"));
    }

    #[test]
    fn empty_section_is_skipped() {
        let md = "## Has Body\n\nActual body.\n\n## Empty Section\n\n## Has Body Too\n\nAnother body.\n";
        let frags = parse(md);
        assert_eq!(frags.len(), 2, "empty section must be skipped");
        assert_eq!(frags[0].section_slug, "has-body");
        assert_eq!(frags[1].section_slug, "has-body-too");
    }

    #[test]
    fn h3_stays_inside_h2_parent() {
        let md = "## Parent\n\n### Child\n\nChild body.\n";
        let frags = parse(md);
        assert_eq!(frags.len(), 1);
        assert!(frags[0].body.contains("### Child"));
    }

    #[test]
    fn front_matter_quotes_ignored_as_headings() {
        let md = "> Some rule description.\n\n## The Rule\n\nRule body.\n";
        let frags = parse(md);
        assert_eq!(frags.len(), 1);
        assert_eq!(frags[0].section_slug, "the-rule");
        assert!(frags[0].body.contains("Rule body."));
    }

    #[test]
    fn slug_generation_collapses_special_chars() {
        assert_eq!(to_section_slug("1. Think Before Coding"), "1-think-before-coding");
        assert_eq!(to_section_slug("No Patching / No Overlays"), "no-patching-no-overlays");
        assert_eq!(to_section_slug("  leading + trailing  "), "leading-trailing");
    }
}
