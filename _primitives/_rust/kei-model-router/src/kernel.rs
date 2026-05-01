//! DNA similarity kernel for unseen task-classes.
//!
//! When a new task arrives whose `task_class_dna` is not in the ledger,
//! we transfer learning from similar past task-classes via this kernel.
//!
//! K(d, d') = α_role · 1[role=role'] +
//!            α_caps · |caps ∩ caps'| / |caps ∪ caps'| +
//!            α_scope · 1[scope=scope'] +
//!            α_body · jaccard(body8, body8')   ← coarse n-gram on hex
//!
//! Calibrated weights are reset by the `calibrate` CLI subcommand from
//! observed outcomes; defaults below are seed values.
//!
//! Constructor Pattern: pure-fn cube. No SQL, no I/O. Caller composes
//! with `posterior::from_ledger` to weight transferred posteriors.

use crate::dna_class;

#[derive(Debug, Clone, Copy)]
pub struct KernelWeights {
    pub alpha_role: f64,
    pub alpha_caps: f64,
    pub alpha_scope: f64,
    pub alpha_body: f64,
}

impl Default for KernelWeights {
    fn default() -> Self {
        Self {
            alpha_role: 0.40,
            alpha_caps: 0.25,
            alpha_scope: 0.25,
            alpha_body: 0.10,
        }
    }
}

/// Similarity score in [0, 1]. Higher = more similar.
pub fn similarity(a: &str, b: &str, w: KernelWeights) -> f64 {
    let mut s = 0.0;

    // role match
    if let (Some(ra), Some(rb)) = (dna_class::role(a), dna_class::role(b)) {
        if ra == rb {
            s += w.alpha_role;
        }
    }

    // caps Jaccard on '-' tokens
    if let (Some(ca), Some(cb)) = (dna_class::caps(a), dna_class::caps(b)) {
        s += w.alpha_caps * jaccard_caps(ca, cb);
    }

    // scope_sha exact match
    if let (Some(sa), Some(sb)) = (dna_class::scope_sha(a), dna_class::scope_sha(b)) {
        if sa == sb {
            s += w.alpha_scope;
        }
    }

    // body sha hex similarity (only meaningful for task_class_dna inputs)
    if let (Some(ba), Some(bb)) = (dna_class::body_sha(a), dna_class::body_sha(b)) {
        s += w.alpha_body * jaccard_hex(ba, bb);
    }

    s.clamp(0.0, 1.0)
}

fn jaccard_caps(a: &str, b: &str) -> f64 {
    let sa: std::collections::BTreeSet<&str> = a.split('-').filter(|t| !t.is_empty()).collect();
    let sb: std::collections::BTreeSet<&str> = b.split('-').filter(|t| !t.is_empty()).collect();
    if sa.is_empty() && sb.is_empty() {
        return 1.0;
    }
    let inter = sa.intersection(&sb).count() as f64;
    let union = sa.union(&sb).count() as f64;
    if union == 0.0 {
        0.0
    } else {
        inter / union
    }
}

/// Coarse character-bigram Jaccard for two short hex strings.
fn jaccard_hex(a: &str, b: &str) -> f64 {
    if a.len() < 2 || b.len() < 2 {
        return if a == b { 1.0 } else { 0.0 };
    }
    let bigrams = |s: &str| -> std::collections::BTreeSet<[u8; 2]> {
        s.as_bytes().windows(2).map(|w| [w[0], w[1]]).collect()
    };
    let sa = bigrams(a);
    let sb = bigrams(b);
    let inter = sa.intersection(&sb).count() as f64;
    let union = sa.union(&sb).count() as f64;
    if union == 0.0 {
        0.0
    } else {
        inter / union
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_dna_full_match() {
        let dna = "code-implementer-rust::?::e3929e37::041b7526";
        let s = similarity(dna, dna, KernelWeights::default());
        // role(0.40) + caps_jaccard=1(0.25) + scope(0.25) + body_jaccard=1(0.10) = 1.00
        assert!((s - 1.0).abs() < 1e-9);
    }

    #[test]
    fn different_role_only_keeps_other_signals() {
        let a = "role-a::cap1-cap2::s1::b1";
        let b = "role-b::cap1-cap2::s1::b1";
        let s = similarity(a, b, KernelWeights::default());
        // role miss(0) + caps(0.25) + scope(0.25) + body bigram match
        // body "b1" len=2 → bigram set={[b,1]}, identical → 1.0
        // total = 0 + 0.25 + 0.25 + 0.10 = 0.60
        assert!((s - 0.60).abs() < 1e-9, "got {}", s);
    }

    #[test]
    fn caps_partial_overlap() {
        let a = "r::a-b-c::s::body1234";
        let b = "r::a-b-d::s::body5678";
        let s = similarity(a, b, KernelWeights::default());
        // role(0.40) + caps Jaccard 2/4 = 0.5 → 0.125 + scope(0.25) + body bigram
        // body bigrams: {bo,od,dy,y1,12,23,34} vs {bo,od,dy,y5,56,67,78}: shared={bo,od,dy}=3, union=11, jaccard=3/11≈0.273
        // body weight: 0.10 * 0.273 ≈ 0.0273
        // total ≈ 0.40 + 0.125 + 0.25 + 0.027 = 0.802
        assert!(s > 0.78 && s < 0.82, "got {}", s);
    }

    #[test]
    fn unrelated_dna_low_score() {
        let a = "role-x::FOO-BAR::deadbeef::aaaa1111";
        let b = "role-y::QUX-ZAP::cafebabe::ffff8888";
        let s = similarity(a, b, KernelWeights::default());
        // role miss(0) + caps Jaccard 0(0) + scope miss(0) + body bigrams
        // mostly disjoint hex bigrams
        assert!(s < 0.10, "got {}", s);
    }

    #[test]
    fn empty_caps_treated_as_full_match() {
        // "?" placeholder caps in some real DNAs
        let a = "r::?::s::b1";
        let b = "r::?::s::b1";
        let s = similarity(a, b, KernelWeights::default());
        assert!((s - 1.0).abs() < 1e-9);
    }
}
