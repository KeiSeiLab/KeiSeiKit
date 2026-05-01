//! `schema-lint` — correctness pass over every `atoms/*.md` under `<root>`.
//!
//! Checks (from SUBSTRATE-SCHEMA §Validation):
//!   1. Frontmatter has required fields (atom, kind, version, input, output,
//!      side_effects, idempotent, stability).
//!   2. Schema paths resolve to existing JSON files inside the atom's dir
//!      (safe_join — rejects `..` and absolute paths).
//!   3. JSON Schemas declare draft-07 via `$schema`.
//!   4. `kind` ∈ {command, query, stream, transform}.
//!   5. `side_effects` entries are `{op, domain}` objects.
//!   6. `related` wikilinks point to another atom OR `rules/...` (dangling rule
//!      refs allowed).

use crate::discover::extract_frontmatter;
use kei_atom_discovery::safe_join;
use serde_yaml_ng::Value as YamlValue;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const REQUIRED_FIELDS: &[&str] = &[
    "atom",
    "kind",
    "version",
    "input",
    "output",
    "side_effects",
    "idempotent",
    "stability",
];
const ALLOWED_KINDS: &[&str] = &["command", "query", "stream", "transform"];

#[derive(Debug, Default)]
pub struct LintReport {
    pub passed: Vec<String>,
    pub failed: Vec<(String, Vec<String>)>,
}

/// Run the full lint over `<root>/*/atoms/*.md`.
pub fn schema_lint(root: &Path) -> LintReport {
    let mut report = LintReport::default();
    let all_atoms = collect_atom_ids(root);
    for md in find_atom_files(root) {
        let label = md.display().to_string();
        match lint_one(&md, &all_atoms) {
            Ok(()) => report.passed.push(label),
            Err(errs) => report.failed.push((label, errs)),
        }
    }
    report
}

fn find_atom_files(root: &Path) -> Vec<PathBuf> {
    WalkDir::new(root)
        .max_depth(3)
        .follow_links(false)
        .into_iter()
        .flatten()
        .filter(|e| {
            e.path().is_file()
                && e.path().extension().is_some_and(|ext| ext == "md")
                && e.path().parent().and_then(|p| p.file_name()).is_some_and(|n| n == "atoms")
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn collect_atom_ids(root: &Path) -> HashSet<String> {
    let mut ids = HashSet::new();
    for md in find_atom_files(root) {
        if let Ok(body) = std::fs::read_to_string(&md) {
            if let Some(fm) = extract_frontmatter(&body) {
                if let Ok(y) = serde_yaml_ng::from_str::<YamlValue>(fm) {
                    if let Some(id) = y.get("atom").and_then(|v| v.as_str()) {
                        ids.insert(id.to_string());
                    }
                }
            }
        }
    }
    ids
}

fn lint_one(md_path: &Path, known_atoms: &HashSet<String>) -> Result<(), Vec<String>> {
    let body = std::fs::read_to_string(md_path).map_err(|e| vec![format!("read: {e}")])?;
    let fm_text = extract_frontmatter(&body).ok_or_else(|| vec!["no frontmatter".to_string()])?;
    let fm: YamlValue =
        serde_yaml_ng::from_str(fm_text).map_err(|e| vec![format!("yaml parse: {e}")])?;
    let mut errs = Vec::new();
    check_required_fields(&fm, &mut errs);
    check_kind(&fm, &mut errs);
    check_side_effects(&fm, &mut errs);
    check_schema_files(md_path, &fm, &mut errs);
    check_related(&fm, known_atoms, &mut errs);
    if errs.is_empty() {
        Ok(())
    } else {
        Err(errs)
    }
}

fn check_required_fields(fm: &YamlValue, errs: &mut Vec<String>) {
    for field in REQUIRED_FIELDS {
        if fm.get(field).is_none() {
            errs.push(format!("missing {field}"));
        }
    }
}

fn check_kind(fm: &YamlValue, errs: &mut Vec<String>) {
    if let Some(k) = fm.get("kind").and_then(|v| v.as_str()) {
        if !ALLOWED_KINDS.contains(&k) {
            errs.push(format!("kind `{k}` not in {ALLOWED_KINDS:?}"));
        }
    }
}

fn check_side_effects(fm: &YamlValue, errs: &mut Vec<String>) {
    let Some(seq) = fm.get("side_effects").and_then(|v| v.as_sequence()) else {
        return;
    };
    for (i, entry) in seq.iter().enumerate() {
        let has_op = entry.get("op").and_then(|v| v.as_str()).is_some();
        let has_domain = entry.get("domain").and_then(|v| v.as_str()).is_some();
        if !has_op || !has_domain {
            errs.push(format!("side_effects[{i}] missing op or domain"));
        }
    }
}

fn check_schema_files(md_path: &Path, fm: &YamlValue, errs: &mut Vec<String>) {
    let Some(md_dir) = md_path.parent() else {
        errs.push("md_path has no parent dir".to_string());
        return;
    };
    for key in &["input", "output"] {
        let Some(rel) = fm.get(key).and_then(|v| v.get("schema")).and_then(|v| v.as_str()) else {
            continue;
        };
        let full = match safe_join(md_dir, rel) {
            Ok(p) => p,
            Err(e) => {
                errs.push(format!("{key} schema path unsafe: {e}"));
                continue;
            }
        };
        if !full.exists() {
            errs.push(format!("{key} schema missing: {}", full.display()));
            continue;
        }
        check_draft07(&full, key, errs);
    }
}

fn check_draft07(schema_path: &Path, key: &str, errs: &mut Vec<String>) {
    let Ok(text) = std::fs::read_to_string(schema_path) else {
        errs.push(format!("{key} schema unreadable"));
        return;
    };
    let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else {
        errs.push(format!("{key} schema not JSON"));
        return;
    };
    let draft = json.get("$schema").and_then(|v| v.as_str()).unwrap_or("");
    if !is_draft07_uri(draft) {
        errs.push(format!("{key} schema missing draft-07 $schema"));
    }
}

/// Exact-match check for the draft-07 meta-schema URI.
///
/// Accepts the canonical URI with or without the trailing `#` fragment.
/// A substring check (`contains("draft-07")`) would falsely match URIs like
/// `http://example.com/draft-07-tutorial.html` — forbidden by §Validation.
fn is_draft07_uri(uri: &str) -> bool {
    uri == "http://json-schema.org/draft-07/schema#"
        || uri == "http://json-schema.org/draft-07/schema"
}

fn check_related(fm: &YamlValue, known: &HashSet<String>, errs: &mut Vec<String>) {
    let Some(seq) = fm.get("related").and_then(|v| v.as_sequence()) else {
        return;
    };
    for entry in seq {
        let Some(link) = entry.as_str() else { continue };
        let Some(inner) = parse_wikilink(link) else {
            errs.push(format!(
                "related entry {link} is not a valid [[atom-id]] wikilink"
            ));
            continue;
        };
        if inner.starts_with("rules/") {
            continue;
        }
        if !known.contains(inner) {
            errs.push(format!("related `{inner}` unresolved"));
        }
    }
}

/// Strict `[[...]]` wikilink parse.
///
/// Returns the inner text only when the string starts with exactly `[[`
/// and ends with exactly `]]`, with no extra brackets on either side
/// and a non-empty body. Rejects malformed forms like `[[[foo]]`,
/// `[[foo]]]`, `[[foo]`, `[foo]]`, and `[[]]`.
fn parse_wikilink(raw: &str) -> Option<&str> {
    let inner = raw.strip_prefix("[[")?.strip_suffix("]]")?;
    if inner.is_empty() || inner.starts_with('[') || inner.ends_with(']') {
        return None;
    }
    Some(inner)
}
