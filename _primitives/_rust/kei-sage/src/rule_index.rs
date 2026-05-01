//! Rule discovery + indexing for kei-sage.
//!
//! Walks a flat `<rules-root>/*.md` tree (e.g. `~/.claude/rules/`), extracts
//! the first `#` heading from each file as the rule name, and uses the file
//! stem as the rule slug. Rules are persisted as Units with:
//!   - `unit_type = "rule"`
//!   - `vault_path = "rule:<slug>"`
//!   - `title = <heading>`
//!
//! Edges from atoms that `related:` a `[[rules/...]]` wikilink are persisted
//! with `edge_type = "rule_ref"` via `index_rule_edges`.
//!
//! Scope: flat dir only (rules live flat in `~/.claude/rules/`). No recursion.

use crate::atoms::AtomRecord;
use crate::edges::add_edge;
use crate::store::Store;
use crate::types::Unit;
use anyhow::Result;
use kei_atom_discovery::{classify_wikilink, parse_wikilink, WikilinkTarget};
use std::fs;
use std::path::{Path, PathBuf};

/// One discovered rule: slug (file stem), display name (`# heading`), md path.
#[derive(Debug, Clone)]
pub struct RuleRecord {
    pub slug: String,
    pub name: String,
    pub md_path: PathBuf,
}

/// Walk `<root>/*.md` (no recursion) and parse each file's first `#` heading.
/// Files without a heading fall back to the file stem as the display name.
pub fn discover_rules(root: &Path) -> Result<Vec<RuleRecord>> {
    if !root.is_dir() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if !is_rule_md(&path) {
            continue;
        }
        if let Some(rec) = parse_rule_file(&path) {
            out.push(rec);
        }
    }
    out.sort_by(|a, b| a.slug.cmp(&b.slug));
    Ok(out)
}

fn is_rule_md(path: &Path) -> bool {
    path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md")
}

fn parse_rule_file(path: &Path) -> Option<RuleRecord> {
    let slug = path.file_stem().and_then(|s| s.to_str())?.to_string();
    let text = fs::read_to_string(path).ok()?;
    let name = extract_h1(&text).unwrap_or_else(|| slug.clone());
    Some(RuleRecord {
        slug,
        name,
        md_path: path.to_path_buf(),
    })
}

/// Extract the first `# ` heading line, stripping the `#` prefix and trim.
/// Returns `None` if no `# ` line exists in the file.
fn extract_h1(text: &str) -> Option<String> {
    for line in text.lines() {
        let t = line.trim_start();
        if let Some(rest) = t.strip_prefix("# ") {
            return Some(rest.trim().to_string());
        }
    }
    None
}

/// Persist rule units into the store. Returns the number of units indexed.
pub fn index_rules(store: &Store, records: &[RuleRecord]) -> Result<usize> {
    for rec in records {
        store.add_unit(&record_to_unit(rec))?;
    }
    Ok(records.len())
}

fn record_to_unit(rec: &RuleRecord) -> Unit {
    Unit {
        unit_type: "rule".into(),
        title: rec.name.clone(),
        content: String::new(),
        evidence_grade: "rule".into(),
        source_path: rec.md_path.to_string_lossy().into(),
        vault_path: format!("rule:{}", rec.slug),
        category: "rule".into(),
        ..Default::default()
    }
}

/// Walk every atom's `related:` list. For every wikilink that classifies as
/// `Rule`, persist a `rule_ref` edge from the atom to `rule:<slug>`.
/// Returns the number of edges persisted.
pub fn index_rule_edges(store: &Store, records: &[AtomRecord]) -> Result<usize> {
    let mut n = 0;
    for rec in records {
        for w in &rec.related {
            if let Some(slug) = resolve_rule_ref(w) {
                add_edge(
                    store,
                    &rec.full_id,
                    &format!("rule:{}", slug),
                    "rule_ref",
                    1.0,
                )?;
                n += 1;
            }
        }
    }
    Ok(n)
}

fn resolve_rule_ref(raw: &str) -> Option<String> {
    let inner = parse_wikilink(raw)?;
    match classify_wikilink(&inner) {
        WikilinkTarget::Rule(slug) => Some(slug),
        _ => None,
    }
}
