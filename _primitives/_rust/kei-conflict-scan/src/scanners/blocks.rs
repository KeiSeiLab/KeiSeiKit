//! Block-duplication detector (>70% text overlap).
//!
//! Uses shingled-word Jaccard similarity — cheap and deterministic,
//! no ML / embeddings. Flags pairs above threshold.

use crate::conflict::{Category, Conflict, Severity};
use crate::tree::{collect_markdown, read_lossy, rel};
use std::collections::HashSet;
use std::path::Path;

const THRESHOLD: f64 = 0.70;
const SHINGLE: usize = 5;

fn shingles(text: &str) -> HashSet<String> {
    let words: Vec<String> = text
        .split_whitespace()
        .map(|w| {
            w.to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect()
        })
        .filter(|w: &String| !w.is_empty())
        .collect();
    if words.len() < SHINGLE {
        return HashSet::new();
    }
    let mut out = HashSet::new();
    for window in words.windows(SHINGLE) {
        out.insert(window.join(" "));
    }
    out
}

fn jaccard(a: &HashSet<String>, b: &HashSet<String>) -> f64 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    let inter = a.intersection(b).count() as f64;
    let union = a.union(b).count() as f64;
    inter / union
}

pub fn scan(root: &Path) -> Vec<Conflict> {
    let files = collect_markdown(root, "_blocks");
    let indexed: Vec<(String, HashSet<String>)> = files
        .iter()
        .map(|f| (rel(root, f), shingles(&read_lossy(f))))
        .collect();
    find_duplicates(&indexed)
}

fn find_duplicates(indexed: &[(String, HashSet<String>)]) -> Vec<Conflict> {
    let mut out = Vec::new();
    for i in 0..indexed.len() {
        for j in (i + 1)..indexed.len() {
            let s = jaccard(&indexed[i].1, &indexed[j].1);
            if s >= THRESHOLD {
                out.push(dup_conflict(&indexed[i].0, &indexed[j].0, s));
            }
        }
    }
    out
}

fn dup_conflict(a: &str, b: &str, score: f64) -> Conflict {
    let pct = (score * 100.0).round() as u32;
    Conflict::new(
        Category::Blocks,
        Severity::Medium,
        vec![a.to_string(), b.to_string()],
        format!("shingle-Jaccard {}% overlap", pct),
        "keep the better-cited block; mark the other as deprecated with a pointer".to_string(),
        true,
    )
}
