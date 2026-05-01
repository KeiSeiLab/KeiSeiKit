//! Diff — compare two DNAs facet-by-facet.
//!
//! Pure parser + comparator. No I/O, no ledger lookup. Callers that want
//! the composed-body text diff can run `replay` on each DNA first and diff
//! the resulting `composed_prompt` themselves.

use anyhow::{anyhow, Result};
use kei_agent_runtime::dna::Dna;

/// Diff report between two DNA strings.
#[derive(Debug, Clone)]
pub struct DnaDiff {
    pub left: Dna,
    pub right: Dna,
    pub role_changed: bool,
    pub caps_changed: bool,
    pub scope_changed: bool,
    pub body_changed: bool,
    pub nonce_changed: bool,
}

impl DnaDiff {
    /// `true` when every facet is identical (same composition, same nonce).
    pub fn is_identical(&self) -> bool {
        !(self.role_changed
            || self.caps_changed
            || self.scope_changed
            || self.body_changed
            || self.nonce_changed)
    }

    /// `true` when the two DNAs would re-compose to the same prompt body
    /// (nonce difference allowed — nonces are per-invocation salt).
    pub fn is_same_composition(&self) -> bool {
        !(self.role_changed || self.caps_changed || self.scope_changed || self.body_changed)
    }

    /// Human-readable multi-line report.
    pub fn render(&self) -> String {
        let mut out = Vec::with_capacity(7);
        out.push(format!("left : {}", self.left.render()));
        out.push(format!("right: {}", self.right.render()));
        out.push(format!("role       : {}", flag(self.role_changed)));
        out.push(format!("caps       : {}", flag(self.caps_changed)));
        out.push(format!("scope      : {}", flag(self.scope_changed)));
        out.push(format!("body       : {}", flag(self.body_changed)));
        out.push(format!("nonce      : {}", flag(self.nonce_changed)));
        out.join("\n")
    }
}

fn flag(changed: bool) -> &'static str {
    if changed {
        "CHANGED"
    } else {
        "same"
    }
}

/// Parse both DNAs and emit the facet-level diff.
pub fn diff(left: &str, right: &str) -> Result<DnaDiff> {
    let l = Dna::parse(left).map_err(|e| anyhow!("invalid left DNA: {e}"))?;
    let r = Dna::parse(right).map_err(|e| anyhow!("invalid right DNA: {e}"))?;
    Ok(DnaDiff {
        role_changed: l.role != r.role,
        caps_changed: l.caps_bitmap != r.caps_bitmap,
        scope_changed: l.scope_hash != r.scope_hash,
        body_changed: l.body_hash != r.body_hash,
        nonce_changed: l.nonce != r.nonce,
        left: l,
        right: r,
    })
}
