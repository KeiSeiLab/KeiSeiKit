//! Orphan-reference detector.
//!
//! Finds `[[wikilink]]` references whose targets do not exist anywhere
//! under the root. Case-insensitive basename match.
//!
//! The earlier `handoffs: - **name**` heuristic was removed (2026-05-12)
//! after a sync-repo scan showed it matched 0 real handoff sections and
//! every match was a prose bold-bullet (e.g. `- **english-jargon** —`).
//! Real handoff syntax in agent-graph repos uses YAML frontmatter, not
//! prose markdown.

use crate::conflict::{Category, Conflict, Severity};
use crate::tree::{read_lossy, rel, should_skip_path};
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;

fn all_basenames(root: &Path) -> HashSet<String> {
    let mut out = HashSet::new();
    for e in WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !should_skip_path(e.path()))
        .filter_map(|e| e.ok())
    {
        if e.file_type().is_file() {
            if let Some(stem) = e.path().file_stem().and_then(|s| s.to_str()) {
                out.insert(stem.to_lowercase());
            }
        }
    }
    out
}

fn extract_wikilinks(content: &str) -> Vec<String> {
    let rx = Regex::new(r"\[\[([^\]\|#]+?)(?:#[^\]]*)?(?:\|[^\]]*)?\]\]").expect("static regex");
    rx.captures_iter(content)
        .map(|c| c[1].trim().to_lowercase())
        .collect()
}

/// Normalize a wikilink target to a basename comparable against
/// `all_basenames` (file_stem-based index).
///
/// Returns `None` when the target escapes the scan root via `../` —
/// such refs point outside the scan tree (e.g. `~/.claude/rules/*` from
/// inside a sync-repo MEMORY.md) and cannot be validated by this scanner.
///
/// For path-prefixed targets (`chatlogs/X/Y`, `concepts/Z`) only the
/// last segment is compared, matching how `all_basenames` builds its
/// index. The `.md` suffix is stripped — `file_stem` does the same.
fn normalize_target(raw: &str) -> Option<String> {
    if raw.starts_with("../") || raw.contains("/../") {
        return None;
    }
    let bn = raw.rsplit('/').next().unwrap_or(raw);
    let bn = bn.strip_suffix(".md").unwrap_or(bn);
    Some(bn.to_string())
}

pub fn scan(root: &Path) -> Vec<Conflict> {
    let index = all_basenames(root);
    let mut out = Vec::new();
    for e in WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !should_skip_path(e.path()))
        .filter_map(|e| e.ok())
    {
        if !e.file_type().is_file() {
            continue;
        }
        if e.path().extension().is_none_or(|x| x != "md") {
            continue;
        }
        let content = read_lossy(e.path());
        let file_rel = rel(root, e.path());
        for raw in extract_wikilinks(&content) {
            let Some(normalized) = normalize_target(&raw) else {
                continue;
            };
            if !index.contains(&normalized) {
                out.push(orphan(&file_rel, &raw, "wikilink"));
            }
        }
    }
    out
}

fn orphan(file: &str, target: &str, kind: &str) -> Conflict {
    Conflict::new(
        Category::Orphans,
        Severity::Low,
        vec![file.to_string()],
        format!("{} target '{}' not found under root", kind, target),
        "either create the target file or remove the stale reference".to_string(),
        true,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_repo_ref_skipped() {
        assert_eq!(normalize_target("../../../rules/recurrence-escalate"), None);
        assert_eq!(normalize_target("../foo"), None);
        assert_eq!(normalize_target("docs/../escape"), None);
    }

    #[test]
    fn path_prefixed_target_basenamed() {
        assert_eq!(
            normalize_target("chatlogs/ml-keilab/2026-05-08-something"),
            Some("2026-05-08-something".to_string())
        );
        assert_eq!(
            normalize_target("concepts/keibeta"),
            Some("keibeta".to_string())
        );
    }

    #[test]
    fn plain_basename_passes_through() {
        assert_eq!(
            normalize_target("ai-creative-engine"),
            Some("ai-creative-engine".to_string())
        );
    }

    #[test]
    fn md_suffix_stripped() {
        assert_eq!(
            normalize_target("docs/intro.md"),
            Some("intro".to_string())
        );
    }
}
