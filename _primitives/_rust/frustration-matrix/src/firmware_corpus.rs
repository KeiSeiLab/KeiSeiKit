//! Corpus loader — walks a directory, concatenates training text.
//!
//! Dispatch by extension:
//!   * `.md`         — full body minus `### Assistant` blocks
//!   * `.txt`        — raw body
//!   * `.jsonl`      — user turns only (via existing `jsonl::parse_user_lines`)
//!
//! Files separated by `\n` in the resulting buffer so n-grams don't bleed
//! across file boundaries (single-char gap is enough — the alphabet builder
//! collects `\n` like any other char, and its unigram drops below min_count
//! only if the corpus has no newlines at all).
//!
//! Constructor Pattern: this cube only wires `fs` + existing parsers.

use crate::jsonl::parse_user_lines;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Kind of training file, matched on extension.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum FileKind {
    Markdown,
    Text,
    Jsonl,
}

/// Load and concatenate all training text under `root`. Returns one big
/// buffer suitable for `Firmware::train_from_text`.
pub fn load_corpus_text(root: &Path) -> Result<String> {
    let files = collect_files(root);
    let mut buf = String::new();
    for (path, kind) in files {
        let chunk = read_one(&path, kind)
            .with_context(|| format!("read {}", path.display()))?;
        if !chunk.is_empty() {
            if !buf.is_empty() {
                buf.push('\n');
            }
            buf.push_str(&chunk);
        }
    }
    Ok(buf)
}

/// Walk `root`, return every (path, kind) pair in deterministic order
/// (WalkDir's default is alphabetical within each directory — good enough
/// for reproducibility of the resulting firmware).
fn collect_files(root: &Path) -> Vec<(PathBuf, FileKind)> {
    WalkDir::new(root)
        .follow_links(false)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|r| r.ok())
        .filter(|e| e.path().is_file())
        .filter_map(|e| classify(e.path()).map(|k| (e.into_path(), k)))
        .collect()
}

fn classify(p: &Path) -> Option<FileKind> {
    let ext = p.extension().and_then(|e| e.to_str())?;
    match ext.to_ascii_lowercase().as_str() {
        "md" => Some(FileKind::Markdown),
        "txt" => Some(FileKind::Text),
        "jsonl" => Some(FileKind::Jsonl),
        _ => None,
    }
}

fn read_one(path: &Path, kind: FileKind) -> Result<String> {
    match kind {
        FileKind::Text => fs::read_to_string(path).map_err(Into::into),
        FileKind::Markdown => {
            let body = fs::read_to_string(path)?;
            Ok(strip_assistant_blocks(&body))
        }
        FileKind::Jsonl => {
            let lines = parse_user_lines(path)?;
            let mut out = String::new();
            for l in lines {
                out.push_str(&l.text);
                out.push('\n');
            }
            Ok(out)
        }
    }
}

/// Drop every line starting with `### Assistant` (or `## Assistant` /
/// `**Assistant:**`) until the next user/header boundary. Matches the
/// markdown.rs classifier on block start markers — we only need the
/// *start* of an assistant block here, since the next same-level header
/// ends it.
pub fn strip_assistant_blocks(body: &str) -> String {
    let mut in_assistant = false;
    let mut out = String::with_capacity(body.len());
    for line in body.lines() {
        let t = line.trim_start();
        if is_assistant_start(t) {
            in_assistant = true;
            continue;
        }
        if in_assistant && is_user_or_header_boundary(t) {
            in_assistant = false;
            // Fall through — this line IS a user boundary, we keep it.
        }
        if !in_assistant {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

fn is_assistant_start(t: &str) -> bool {
    t.starts_with("### Assistant")
        || t.starts_with("## Assistant")
        || t.starts_with("# Assistant")
        || t.starts_with("**Assistant:**")
        || t.starts_with("**Assistant**:")
}

fn is_user_or_header_boundary(t: &str) -> bool {
    t.starts_with("### User")
        || t.starts_with("## User")
        || t.starts_with("# User")
        || t.starts_with("**User:**")
        || t.starts_with("**User**:")
        || t.starts_with("## ")
        || t.starts_with("# ")
}
