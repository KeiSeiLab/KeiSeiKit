//! Markdown H2-heading splitter and description extractor.
//!
//! Provides pure text operations used by skill_extractor:
//! - split_by_h2: parse `## ` sections from markdown text
//! - first_sentences: extract up to 3 sentences for a skill description
//! - strip_markdown: remove markdown syntax for plain-text extraction

use crate::fragment_writer::sanitize;

/// Split markdown text into `(slug, heading, body)` tuples at `## ` boundaries.
pub fn split_by_h2(text: &str) -> Vec<(String, String, String)> {
    let mut out = Vec::new();
    let mut current_heading: Option<String> = None;
    let mut buf = String::new();

    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("## ") {
            flush_section(&mut out, &mut current_heading, &mut buf);
            current_heading = Some(rest.trim().to_string());
        } else {
            buf.push_str(line);
            buf.push('\n');
        }
    }
    flush_section(&mut out, &mut current_heading, &mut buf);
    out
}

fn flush_section(
    out: &mut Vec<(String, String, String)>,
    heading: &mut Option<String>,
    buf: &mut String,
) {
    let body = std::mem::take(buf);
    if let Some(h) = heading.take() {
        let slug = sanitize(&h);
        out.push((slug, h, body));
    }
}

/// Extract first 1-3 sentences up to `max_chars` from body text.
pub fn first_sentences(text: &str, max_chars: usize) -> String {
    let plain = strip_markdown(text);
    let mut out = String::new();
    let mut count = 0usize;
    for part in plain.split_inclusive(&['.', '!', '?'][..]) {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        if out.len() + trimmed.len() + 1 > max_chars || count >= 3 {
            break;
        }
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(trimmed);
        count += 1;
    }
    if out.is_empty() {
        plain.chars().take(max_chars).collect()
    } else {
        out
    }
}

/// Strip markdown syntax (headings, code fences, links) for description use.
pub fn strip_markdown(text: &str) -> String {
    let mut out = String::new();
    for line in text.lines() {
        let l = line.trim();
        if l.starts_with('#') || l.starts_with("```") || l.starts_with("---") {
            continue;
        }
        let clean = l.trim_start_matches(|c: char| c == '*' || c == '-' || c == '>');
        let clean = clean.trim();
        if !clean.is_empty() {
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(clean);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_h2_basic() {
        let md = "preamble\n## Alpha\nbody a\n## Beta\nbody b\n";
        let secs = split_by_h2(md);
        assert_eq!(secs.len(), 2);
        assert_eq!(secs[0].1, "Alpha");
        assert!(secs[0].2.contains("body a"));
    }

    #[test]
    fn first_sentences_truncates() {
        let s = first_sentences("Hello world. Second sentence. Third one. Fourth one.", 200);
        assert!(s.contains("Hello world."));
    }

    #[test]
    fn strip_removes_headings() {
        let stripped = strip_markdown("## Heading\nNormal text.");
        assert!(!stripped.contains("##"));
        assert!(stripped.contains("Normal text."));
    }
}
