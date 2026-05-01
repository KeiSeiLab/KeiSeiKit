//! Hook-overlap detector.
//!
//! Heuristic: two hook scripts in `hooks/` whose first line-match of
//! `tool_name|matcher|event|PreToolUse|PostToolUse|UserPromptSubmit`
//! targets the same value. Flags the pair as possibly-redundant.

use crate::conflict::{Category, Conflict, Severity};
use crate::tree::{collect_with_ext, read_lossy, rel};
use regex::Regex;
use std::path::Path;

fn extract_matcher(content: &str) -> Vec<String> {
    let rx = Regex::new(
        r#"(?i)(?:tool[_ ]?name|matcher|event)\s*[:=]\s*["']?([A-Za-z0-9_|/-]+)["']?"#,
    )
    .expect("static regex");
    let mut out = Vec::new();
    for c in rx.captures_iter(content) {
        out.push(c[1].to_lowercase());
    }
    out.sort();
    out.dedup();
    out
}

pub fn scan(root: &Path) -> Vec<Conflict> {
    let mut files = collect_with_ext(root, "hooks", "sh");
    files.extend(collect_with_ext(root, "hooks", "py"));
    files.extend(collect_with_ext(root, "hooks", "rs"));

    let indexed: Vec<(String, Vec<String>)> = files
        .iter()
        .map(|f| (rel(root, f), extract_matcher(&read_lossy(f))))
        .collect();

    pairs(&indexed)
}

fn pairs(indexed: &[(String, Vec<String>)]) -> Vec<Conflict> {
    let mut out = Vec::new();
    for i in 0..indexed.len() {
        for j in (i + 1)..indexed.len() {
            let shared: Vec<&String> =
                indexed[i].1.iter().filter(|m| indexed[j].1.contains(m)).collect();
            if !shared.is_empty() {
                out.push(overlap_conflict(&indexed[i].0, &indexed[j].0, &shared));
            }
        }
    }
    out
}

fn overlap_conflict(a: &str, b: &str, shared: &[&String]) -> Conflict {
    let shared_str = shared
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .join(",");
    Conflict::new(
        Category::Hooks,
        Severity::Medium,
        vec![a.to_string(), b.to_string()],
        format!("hooks share matcher(s): {}", shared_str),
        "consider merging into a single hook with union of patterns; keep separate if responsibilities are genuinely distinct".to_string(),
        false,
    )
}
