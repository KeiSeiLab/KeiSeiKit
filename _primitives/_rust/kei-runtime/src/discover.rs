//! Atom discovery — thin façade over `kei-atom-discovery`.
//!
//! Re-exports `AtomMeta` and `AtomKind` from the shared crate so all runtime
//! modules share exactly one frontmatter-parser implementation.

use kei_atom_discovery as shared;
use std::path::Path;

pub use kei_atom_discovery::{parse_frontmatter, AtomKind, AtomMeta};

/// Walk `<root>/*/atoms/*.md`. Delegates to `kei-atom-discovery::discover_atoms`.
pub fn walk_atoms(root: &Path) -> Vec<AtomMeta> {
    shared::discover_atoms(root)
}

/// Backwards-compatible split — returns the frontmatter YAML body (no body
/// trailing). Returns `None` if the file has no frontmatter fences.
pub fn extract_frontmatter(body: &str) -> Option<&str> {
    shared::parse_frontmatter(body).ok().map(|(fm, _)| fm)
}
