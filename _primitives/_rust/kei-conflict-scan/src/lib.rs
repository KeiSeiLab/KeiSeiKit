//! kei-conflict-scan — library surface.
//!
//! Detects inconsistencies inside a `~/.claude`-style root:
//!   - rule conflicts (contradictory directives in `rules/*.md`)
//!   - hook overlap (two hooks on same matcher)
//!   - block duplication (>70% text overlap in `_blocks/*.md`)
//!   - orphan refs (wikilinks / handoffs to non-existent files)
//!   - Constructor-Pattern violations (file >200 LOC / fn >30 LOC)
//!
//! Produces a JSON array consumable by `kei-refactor-engine`.

pub mod conflict;
pub mod scanners;
pub mod tree;

pub use conflict::{Category, Conflict, Severity};
