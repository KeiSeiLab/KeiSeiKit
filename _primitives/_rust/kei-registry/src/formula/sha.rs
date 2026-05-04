//! Canonical short-sha8 of a `BlockFormula` (excluding `block_id`).
//!
//! Constructor Pattern: hashing is one cube. The canonical serialisation
//! uses `serde_json` over the four formula facets in the canonical order
//! `(type, invariants, effects, deps)`. `effects` and `deps` are
//! `BTreeSet`, so their iteration order is sorted; `invariants` order is
//! the author's chosen order — predicates are positional by design.

use sha2::{Digest, Sha256};

use super::types::BlockFormula;

/// Compute the deterministic 8-hex-char digest of the formula's contents.
/// Excludes `block_id` so two equivalent formulas on different blocks hash
/// the same. Matches the `formula_sha` column the v2 migration introduced
/// on `blocks`.
pub fn formula_sha(formula: &BlockFormula) -> String {
    let mut h = Sha256::new();
    h.update(serde_json::to_string(&formula.r#type).unwrap_or_default().as_bytes());
    h.update(b"|");
    for inv in &formula.invariants {
        h.update(serde_json::to_string(inv).unwrap_or_default().as_bytes());
        h.update(b",");
    }
    h.update(b"|");
    for eff in &formula.effects {
        h.update(serde_json::to_string(eff).unwrap_or_default().as_bytes());
        h.update(b",");
    }
    h.update(b"|");
    for dep in &formula.deps {
        h.update(dep.as_bytes());
        h.update(b",");
    }
    h.update(b"|");
    h.update(serde_json::to_string(&formula.source).unwrap_or_default().as_bytes());
    let digest = h.finalize();
    format!(
        "{:02x}{:02x}{:02x}{:02x}",
        digest[0], digest[1], digest[2], digest[3]
    )
}
