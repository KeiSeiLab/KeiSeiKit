//! Atom scanner — walks markdown files in `<kit-root>/` recursively and
//! picks out files whose YAML frontmatter declares `type: atom`.
//!
//! Constructor Pattern: this cube knows only the atom frontmatter
//! convention. The minimal regex parser deliberately does NOT pull in
//! kei-atom-discovery — that crate is not yet a workspace dependency
//! here, and a 30-LOC regex pair covers the `type:` and `name:` keys we
//! need.

use anyhow::Result;
use regex::Regex;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use super::{Found, Scanner};
use crate::block::BlockType;

/// Recursive markdown walker that filters on YAML frontmatter `type: atom`.
pub struct AtomScanner;

impl Scanner for AtomScanner {
    fn scan(&self, root: &Path) -> Result<Vec<Found>> {
        if !root.is_dir() {
            return Ok(Vec::new());
        }
        let mut found = Vec::new();
        for entry in WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_map(Result::ok)
        {
            let path = entry.path();
            if !is_md_file(path) {
                continue;
            }
            if let Some(f) = scan_one_atom(path)? {
                found.push(f);
            }
        }
        found.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(found)
    }
}

fn is_md_file(p: &Path) -> bool {
    p.is_file() && p.extension().and_then(|e| e.to_str()) == Some("md")
}

fn scan_one_atom(file: &Path) -> Result<Option<Found>> {
    let body = match std::fs::read(file) {
        Ok(b) => b,
        Err(_) => return Ok(None),
    };
    let txt = match std::str::from_utf8(&body) {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };
    let frontmatter = match extract_frontmatter(txt) {
        Some(f) => f,
        None => return Ok(None),
    };
    if !frontmatter_is_atom(&frontmatter) {
        return Ok(None);
    }
    let name = parse_yaml_value(&frontmatter, "name").unwrap_or_else(|| fallback_name(file));
    Ok(Some(Found {
        block_type: BlockType::Atom,
        name,
        path: canonical_str(file),
        body,
        caps: "md".to_string(),
    }))
}

fn fallback_name(file: &Path) -> String {
    file.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string()
}

/// Pull the contents between `---` fences at the start of the file. Returns
/// `None` if the file does not start with `---\n` or no closing fence.
fn extract_frontmatter(txt: &str) -> Option<String> {
    let stripped = txt.strip_prefix("---\n").or_else(|| txt.strip_prefix("---\r\n"))?;
    let end = find_closing_fence(stripped)?;
    Some(stripped[..end].to_string())
}

fn find_closing_fence(s: &str) -> Option<usize> {
    for (idx, _) in s.match_indices('\n') {
        let after = &s[idx + 1..];
        if after.starts_with("---\n") || after.starts_with("---\r\n") || after == "---" {
            return Some(idx + 1);
        }
    }
    None
}

fn frontmatter_is_atom(frontmatter: &str) -> bool {
    parse_yaml_value(frontmatter, "type").as_deref() == Some("atom")
}

/// Minimal YAML-key extraction: `^<key>:\s*(.*)$` per line. Strips quotes
/// and inline comments. Multiline values not supported (atoms use scalar
/// keys only for `name` / `type`).
fn parse_yaml_value(frontmatter: &str, key: &str) -> Option<String> {
    let pattern = format!(r"(?m)^\s*{}\s*:\s*(.+?)\s*$", regex::escape(key));
    let re = Regex::new(&pattern).ok()?;
    let cap = re.captures(frontmatter)?;
    let raw = cap.get(1)?.as_str().trim();
    let unquoted = raw
        .trim_start_matches(['"', '\''])
        .trim_end_matches(['"', '\'']);
    if unquoted.is_empty() {
        None
    } else {
        Some(unquoted.to_string())
    }
}

fn canonical_str(p: &Path) -> String {
    p.canonicalize()
        .unwrap_or_else(|_| PathBuf::from(p))
        .to_string_lossy()
        .to_string()
}
