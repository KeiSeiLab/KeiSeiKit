//! Facet-by-facet diff between two blocks.
//!
//! Constructor Pattern: pure data, no I/O. The four compared facets are
//! `block_type`, `caps`, `scope_sha`, `body_sha` — i.e. the four
//! identity-bearing inputs to the DNA wire format. `name` is intentionally
//! NOT diffed (it is a derived label) and `path` is reflected via
//! `scope_sha` (which is `SHA256(path)`).

use serde::{Deserialize, Serialize};

use crate::block::Block;

/// Diff result. `differs` lists facet names whose values disagree;
/// `identical` lists facets that match. Together they cover all four
/// compared facets exactly once. Strings are owned so the result
/// round-trips through serde.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockDiff {
    pub differs: Vec<String>,
    pub identical: Vec<String>,
}

/// Compute the diff. Order of facets is canonical so output is stable
/// across calls and across processes.
pub fn diff_blocks(a: &Block, b: &Block) -> BlockDiff {
    let mut differs = Vec::new();
    let mut identical = Vec::new();
    push_facet(
        "block_type",
        a.block_type.as_str() == b.block_type.as_str(),
        &mut differs,
        &mut identical,
    );
    push_facet("caps", a.caps == b.caps, &mut differs, &mut identical);
    push_facet(
        "scope_sha",
        a.scope_sha == b.scope_sha,
        &mut differs,
        &mut identical,
    );
    push_facet(
        "body_sha",
        a.body_sha == b.body_sha,
        &mut differs,
        &mut identical,
    );
    BlockDiff { differs, identical }
}

fn push_facet(
    name: &'static str,
    same: bool,
    differs: &mut Vec<String>,
    identical: &mut Vec<String>,
) {
    if same {
        identical.push(name.to_string());
    } else {
        differs.push(name.to_string());
    }
}
