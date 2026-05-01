//! Rule-file conflict detector.
//!
//! Heuristic: look for contradictory directive pairs like
//! "never X" vs "prefer X" or "forbidden: X" vs "required: X" across
//! `rules/*.md`. Tokens compared after stripping filler words.

use crate::conflict::{Category, Conflict, Severity};
use crate::tree::{collect_markdown, read_lossy, rel};
use regex::Regex;
use std::path::Path;

fn extract_directives(content: &str) -> Vec<(String, String)> {
    // Returns (polarity, token) pairs. polarity ∈ {"pos","neg"}.
    let neg = Regex::new(r"(?im)^\s*(?:never|forbidden|prohibited|do not|don't|no):?\s+(.{3,80})$")
        .expect("static regex");
    let pos = Regex::new(r"(?im)^\s*(?:always|required|prefer|must|do):?\s+(.{3,80})$")
        .expect("static regex");
    let mut out = Vec::new();
    for c in neg.captures_iter(content) {
        out.push(("neg".to_string(), normalize(&c[1])));
    }
    for c in pos.captures_iter(content) {
        out.push(("pos".to_string(), normalize(&c[1])));
    }
    out
}

fn normalize(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .take(6)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn scan(root: &Path) -> Vec<Conflict> {
    let files = collect_markdown(root, "rules");
    let mut indexed: Vec<(String, Vec<(String, String)>)> = Vec::new();
    for f in &files {
        let c = read_lossy(f);
        indexed.push((rel(root, f), extract_directives(&c)));
    }
    find_pairs(&indexed)
}

fn find_pairs(indexed: &[(String, Vec<(String, String)>)]) -> Vec<Conflict> {
    let mut out = Vec::new();
    for i in 0..indexed.len() {
        for j in (i + 1)..indexed.len() {
            for (pi, ti) in &indexed[i].1 {
                for (pj, tj) in &indexed[j].1 {
                    if pi != pj && !ti.is_empty() && ti == tj {
                        out.push(mk_conflict(&indexed[i].0, &indexed[j].0, ti));
                    }
                }
            }
        }
    }
    out
}

fn mk_conflict(a: &str, b: &str, token: &str) -> Conflict {
    Conflict::new(
        Category::Rules,
        Severity::High,
        vec![a.to_string(), b.to_string()],
        format!("contradictory directive on '{}'", token),
        format!(
            "review both files; keep directive in the more-specific rule, drop or narrow in the other"
        ),
        false,
    )
}
