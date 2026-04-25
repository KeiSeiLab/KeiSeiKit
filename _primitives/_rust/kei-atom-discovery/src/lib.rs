//! kei-atom-discovery — shared substrate-atom discovery primitives.
//!
//! Single authoritative implementation of:
//!   - `AtomMeta` / `AtomKind` / `SideEffect` — locked frontmatter schema
//!   - `parse_frontmatter` — YAML split with 64 KiB cap (billion-laughs guard)
//!   - `discover_atoms` — walks `<root>/*/atoms/*.md`, symlink-safe
//!   - `parse_wikilink` — strict `[[target]]` matcher
//!   - `safe_join` — path-traversal-safe base+rel join
//!
//! Both `kei-sage` and `kei-runtime` consume this crate — no parallel
//! frontmatter structs, no parallel YAML parsers.

pub mod error;
pub mod frontmatter;
pub mod path_safety;
pub mod walk;
pub mod wikilink;

pub use error::Error;
pub use frontmatter::{
    parse_frontmatter, AtomKind, AtomMeta, Frontmatter, Lineage, SideEffect, TaxonomyFacets,
    MAX_FRONTMATTER_BYTES,
};
pub use path_safety::safe_join;
pub use walk::{discover_atoms, split_atom_id};
pub use wikilink::{classify_wikilink, is_atom_target, parse_wikilink, WikilinkTarget};
