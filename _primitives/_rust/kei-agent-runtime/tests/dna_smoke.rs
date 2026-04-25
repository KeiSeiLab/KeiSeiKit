//! Layer G — DNA identity smoke tests.
//!
//! Asserts render ↔ parse round-trip, hashing is sensitive to the
//! documented inputs, and the total string length stays short enough to
//! embed in ledger rows.

use kei_agent_runtime::capability::TaskSpec;
use kei_agent_runtime::dna::{Dna, DnaError};
use kei_agent_runtime::role::ResolvedRole;

fn fixture_task(body: &str, wl: &[&str], dl: &[&str]) -> TaskSpec {
    let mut t = TaskSpec::default();
    t.task.role = "edit-local".into();
    t.task.agent_id = "edit-local-forge-abc".into();
    t.body.text = body.into();
    t.scope.files_whitelist = wl.iter().map(|s| (*s).to_string()).collect();
    t.scope.files_denylist = dl.iter().map(|s| (*s).to_string()).collect();
    t
}

fn edit_local_resolved() -> ResolvedRole {
    ResolvedRole {
        required: vec![
            "policy::no-git-ops".into(),
            "scope::files-whitelist".into(),
            "scope::files-denylist".into(),
            "quality::constructor-pattern".into(),
            "quality::cargo-check-green".into(),
            "quality::tests-green".into(),
            "safety::no-dep-bump".into(),
            "output::report-format".into(),
        ],
        warnings: Vec::new(),
    }
}

#[test]
fn render_parse_roundtrip_equals_original() {
    let task = fixture_task("Port kei-forge templating.", &["_primitives/_rust/kei-forge/**"], &[]);
    let resolved = edit_local_resolved();
    let dna = Dna::compose(&task, &resolved);
    let rendered = dna.render();
    let parsed = Dna::parse(&rendered).expect("parse");
    assert_eq!(parsed, dna);
    assert_eq!(parsed.render(), rendered);
}

#[test]
fn different_scopes_yield_different_scope_hashes() {
    let a = fixture_task("same body", &["a/**"], &[]);
    let b = fixture_task("same body", &["b/**"], &[]);
    let r = edit_local_resolved();
    let da = Dna::compose(&a, &r);
    let db = Dna::compose(&b, &r);
    assert_eq!(da.body_hash, db.body_hash, "body hash should match");
    assert_ne!(da.scope_hash, db.scope_hash, "scope hash must differ");
}

#[test]
fn same_body_yields_same_body_hash() {
    let a = fixture_task("exact body", &[], &[]);
    let b = fixture_task("exact body", &[], &[]);
    let r = edit_local_resolved();
    assert_eq!(Dna::compose(&a, &r).body_hash, Dna::compose(&b, &r).body_hash);
}

#[test]
fn rendered_dna_length_within_budget() {
    let task = fixture_task("body", &["a"], &["b"]);
    let r = edit_local_resolved();
    let s = Dna::compose(&task, &r).render();
    // role(10) + sep(2) + caps bitmap (8 caps * 3 - 1 = 23) + sep(2) +
    // scope(8) + sep(2) + body(8) + `-` + nonce(8) = 64. Budget ≤88 per
    // H4/M4/S3 widening spec; hard ceiling stays comfortably short.
    assert!(
        s.len() <= 88,
        "DNA string should stay short; got {} chars: {}",
        s.len(),
        s
    );
}

#[test]
fn parse_rejects_malformed_shape() {
    assert_eq!(Dna::parse("too::few::segments").unwrap_err(), DnaError::Shape);
    assert_eq!(
        Dna::parse("role::caps::scope::no_nonce").unwrap_err(),
        DnaError::Shape,
        "missing `-nonce` separator must fail"
    );
}

#[test]
fn widened_dna_uses_8_hex_for_all_entropy_segments() {
    let task = fixture_task("entropy budget check", &["x"], &["y"]);
    let r = edit_local_resolved();
    let dna = Dna::compose(&task, &r);
    assert_eq!(
        dna.scope_hash.len(),
        8,
        "scope_hash must be 8 hex (32-bit), got {}: {}",
        dna.scope_hash.len(),
        dna.scope_hash
    );
    assert_eq!(
        dna.body_hash.len(),
        8,
        "body_hash must be 8 hex (32-bit), got {}: {}",
        dna.body_hash.len(),
        dna.body_hash
    );
    assert_eq!(
        dna.nonce.len(),
        8,
        "nonce must be 8 hex (32-bit), got {}: {}",
        dna.nonce.len(),
        dna.nonce
    );
    let round = Dna::parse(&dna.render()).expect("parse widened");
    assert_eq!(round, dna);
}

#[test]
fn parse_accepts_legacy_4_hex_segments_for_rolling_upgrade() {
    // Hand-built legacy DNA: pre-widening format with 4-hex segments.
    // Parser MUST accept it (with stderr warning) so rolling upgrade works.
    let legacy = "edit-local::NG-FW::ABCD::1234-9f0a";
    let parsed = Dna::parse(legacy).expect("legacy 4-hex DNA must parse");
    assert_eq!(parsed.role, "edit-local");
    assert_eq!(parsed.scope_hash, "ABCD");
    assert_eq!(parsed.body_hash, "1234");
    assert_eq!(parsed.nonce, "9f0a");
    assert_eq!(parsed.render(), legacy, "render preserves legacy widths");
}

#[test]
fn nonce_is_unique_across_10000_generated_dnas() {
    // 32-bit nonce → birthday collision at ~65k; at 10k the expected number
    // of duplicates is ~0.6. A single collision fails the test loudly rather
    // than silently on the regression path.
    let task = fixture_task("same body", &["same"], &["same"]);
    let r = edit_local_resolved();
    let mut seen: std::collections::HashSet<String> =
        std::collections::HashSet::with_capacity(10_000);
    for _ in 0..10_000 {
        let n = Dna::compose(&task, &r).nonce;
        assert!(
            seen.insert(n.clone()),
            "nonce collision at 32-bit entropy: {n}"
        );
    }
}
