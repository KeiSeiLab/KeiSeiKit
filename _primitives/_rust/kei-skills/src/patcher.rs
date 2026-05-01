//! Fuzzy find-replace on SKILL.md body.
//!
//! Hermes' `fuzzy_match.py` ranks candidate windows by similarity and
//! picks the best match above a threshold. We reuse the `similar` crate
//! (workspace dep) — `TextDiff::ratio` gives a `[0.0, 1.0]` score akin
//! to Python's `difflib.SequenceMatcher.ratio` (Hermes baseline).
//!
//! Atomic write: serialize to a sibling `.tmp` file, fsync, rename.

use crate::format::{serialize, Skill};
use similar::TextDiff;
use std::fs;
use std::io::Write;

/// Minimum similarity for a fuzzy match. 0.85 mirrors agentskills floor.
pub const FUZZY_THRESHOLD: f32 = 0.85;

#[derive(Debug)]
pub enum PatchError {
    NotFound,
    MultipleMatches { count: usize },
    ContentTooLarge,
    Io(std::io::Error),
    Serialize(String),
}

impl std::fmt::Display for PatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatchError::NotFound => write!(f, "no match above similarity threshold"),
            PatchError::MultipleMatches { count } => {
                write!(f, "{count} fuzzy matches; pass replace_all=true to apply all")
            }
            PatchError::ContentTooLarge => write!(f, "body exceeds content limit after patch"),
            PatchError::Io(e) => write!(f, "io: {e}"),
            PatchError::Serialize(s) => write!(f, "serialize: {s}"),
        }
    }
}

impl std::error::Error for PatchError {}

/// Apply a find-replace to the skill's body. In-memory; persist via
/// [`write_atomic`].
pub fn patch_skill(
    skill: &Skill,
    old: &str,
    new: &str,
    replace_all: bool,
) -> Result<Skill, PatchError> {
    if old.is_empty() {
        return Err(PatchError::NotFound);
    }
    let new_body = match try_exact(&skill.body, old, new, replace_all)? {
        Some(b) => b,
        None => fuzzy_replace(&skill.body, old, new, replace_all)?,
    };
    Ok(Skill {
        frontmatter: skill.frontmatter.clone(),
        body: new_body,
        source_path: skill.source_path.clone(),
    })
}

/// Exact-match path. Returns `Ok(Some(_))` on hit, `Ok(None)` on miss
/// (caller falls through to fuzzy), `Err` on ambiguity.
fn try_exact(body: &str, old: &str, new: &str, replace_all: bool) -> Result<Option<String>, PatchError> {
    let count = body.matches(old).count();
    if count == 0 {
        return Ok(None);
    }
    if count > 1 && !replace_all {
        return Err(PatchError::MultipleMatches { count });
    }
    let out = if replace_all { body.replace(old, new) } else { body.replacen(old, new, 1) };
    Ok(Some(out))
}

/// Slide a line-aligned window of `old`'s line-count over `body`; pick
/// the highest-ratio hit (or all hits when `replace_all`).
fn fuzzy_replace(body: &str, old: &str, new: &str, replace_all: bool) -> Result<String, PatchError> {
    let body_lines: Vec<&str> = body.split_inclusive('\n').collect();
    let span = old.split_inclusive('\n').count().max(1);
    if span > body_lines.len() {
        return Err(PatchError::NotFound);
    }
    let hits = collect_hits(&body_lines, span, old);
    if hits.is_empty() {
        return Err(PatchError::NotFound);
    }
    let chosen = pick_hits(&hits, replace_all)?;
    Ok(splice(body, &body_lines, &chosen, new))
}

/// Score every line-aligned window of length `span`; keep those above
/// `FUZZY_THRESHOLD`.
fn collect_hits(body_lines: &[&str], span: usize, old: &str) -> Vec<(usize, usize, f32)> {
    let mut hits: Vec<(usize, usize, f32)> = Vec::new();
    for start in 0..=body_lines.len().saturating_sub(span) {
        let end = start + span;
        let window: String = body_lines[start..end].concat();
        let r = TextDiff::from_chars(window.as_str(), old).ratio();
        if r >= FUZZY_THRESHOLD {
            hits.push((start, end, r));
        }
    }
    hits
}

/// Apply selection rules: when `replace_all`, take every hit; else take
/// the unique top-ratio hit (error if multiple tie).
fn pick_hits(hits: &[(usize, usize, f32)], replace_all: bool) -> Result<Vec<(usize, usize)>, PatchError> {
    if replace_all {
        return Ok(hits.iter().map(|h| (h.0, h.1)).collect());
    }
    let top = hits.iter().map(|h| h.2).fold(0.0_f32, f32::max);
    let tied: Vec<&(usize, usize, f32)> = hits.iter().filter(|h| (h.2 - top).abs() < 1e-6).collect();
    if tied.len() > 1 {
        return Err(PatchError::MultipleMatches { count: tied.len() });
    }
    Ok(vec![(tied[0].0, tied[0].1)])
}

/// Translate (start_line, end_line) tuples into byte offsets and stitch.
fn splice(body: &str, body_lines: &[&str], regions: &[(usize, usize)], new: &str) -> String {
    let mut sorted: Vec<(usize, usize)> = regions.to_vec();
    sorted.sort_by_key(|h| h.0);
    let byte_regions = lines_to_bytes(body_lines, &sorted);
    let mut out = String::with_capacity(body.len() + new.len());
    let mut cursor = 0usize;
    for (s, e) in &byte_regions {
        out.push_str(&body[cursor..*s]);
        out.push_str(new);
        cursor = *e;
    }
    out.push_str(&body[cursor..]);
    out
}

/// Convert line-index regions into byte-index regions over the original body.
fn lines_to_bytes(body_lines: &[&str], regions: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut out = Vec::with_capacity(regions.len());
    let mut byte = 0usize;
    let mut idx = 0usize;
    for (i, line) in body_lines.iter().enumerate() {
        if idx < regions.len() && regions[idx].0 == i {
            let mut end_byte = byte;
            for inner in &body_lines[i..regions[idx].1] {
                end_byte += inner.len();
            }
            out.push((byte, end_byte));
            idx += 1;
        }
        byte += line.len();
    }
    out
}

/// Persist a patched skill atomically: write `<path>.tmp`, fsync, rename.
pub fn write_atomic(skill: &Skill) -> Result<(), PatchError> {
    let serialized = serialize(skill).map_err(|e| PatchError::Serialize(e.to_string()))?;
    if serialized.chars().count() > crate::validator::MAX_SKILL_CONTENT_CHARS {
        return Err(PatchError::ContentTooLarge);
    }
    let tmp = skill.source_path.with_extension("md.tmp");
    {
        let mut f = fs::File::create(&tmp).map_err(PatchError::Io)?;
        f.write_all(serialized.as_bytes()).map_err(PatchError::Io)?;
        f.sync_all().map_err(PatchError::Io)?;
    }
    fs::rename(&tmp, &skill.source_path).map_err(PatchError::Io)
}
