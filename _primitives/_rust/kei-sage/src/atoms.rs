//! Substrate-atom discovery — thin façade over `kei-atom-discovery`.
//!
//! Historical `AtomRecord` is preserved as a type alias for `AtomMeta` so
//! that downstream sage modules (`atom_index`, `atom_cli`) keep compiling.

use crate::atom_parse::{is_atom_target, parse_wikilink};
use anyhow::Result;
use kei_atom_discovery as shared;
use std::path::Path;

pub use kei_atom_discovery::AtomKind;

/// Legacy alias: sage used to call this `AtomRecord`. New code should use
/// `AtomMeta` directly (identical shape, authored in `kei-atom-discovery`).
pub type AtomRecord = shared::AtomMeta;

/// Walk `<root>/*/atoms/*.md` and return parsed atom metadata.
/// Tolerant: invalid frontmatter → stderr warning + skipped record.
pub fn discover_atoms(root: &Path) -> Result<Vec<AtomRecord>> {
    if !root.is_dir() {
        return Ok(Vec::new());
    }
    Ok(shared::discover_atoms(root))
}

/// Extract `(source_atom_id, target)` edges from `related:` wikilinks.
/// Non-atom targets (rules, notes) are filtered out — scope: atoms only.
pub fn resolve_wikilinks(records: &[AtomRecord]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for rec in records {
        for w in &rec.related {
            if let Some(target) = parse_wikilink(w) {
                if is_atom_target(&target) {
                    out.push((rec.full_id.clone(), target));
                }
            }
        }
    }
    out
}
