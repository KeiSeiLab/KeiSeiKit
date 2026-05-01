//! `compose_for_block` and `compose_for_block_with_nonce` produce parseable
//! DNA via `kei_shared::parse_dna` for every block_type. Validates the
//! wire-format SSoT contract.

use kei_registry::dna_block::compose_for_block_with_nonce;
use kei_registry::{compose_for_block, BlockType};
use kei_shared::dna::parse_dna;

const NONCE: &str = "deadbeef";

fn assert_round_trip(bt: BlockType) {
    // 5-arg surface: nonce generated internally, parses cleanly.
    let dna = compose_for_block(bt, "myname", "/abs/path/to/thing", b"body bytes", "md");
    let parsed = parse_dna(&dna).expect("DNA must parse with kei_shared::parse_dna");
    assert_eq!(parsed.role, bt.as_str(), "role segment must equal block_type for {bt}");
    assert_eq!(parsed.scope_sha.len(), 8, "scope_sha must be 8 hex chars");
    assert_eq!(parsed.body_sha.len(), 8, "body_sha must be 8 hex chars");
    assert_eq!(parsed.nonce.len(), 8, "nonce must be 8 hex chars");
}

#[test]
fn primitive_block_dna_round_trips() {
    assert_round_trip(BlockType::Primitive);
}

#[test]
fn skill_block_dna_round_trips() {
    assert_round_trip(BlockType::Skill);
}

#[test]
fn rule_block_dna_round_trips() {
    assert_round_trip(BlockType::Rule);
}

#[test]
fn hook_block_dna_round_trips() {
    assert_round_trip(BlockType::Hook);
}

#[test]
fn atom_block_dna_round_trips() {
    assert_round_trip(BlockType::Atom);
}

#[test]
fn with_nonce_variant_is_deterministic() {
    let a = compose_for_block_with_nonce(BlockType::Atom, "x", "/p", b"b", "md", NONCE);
    let b = compose_for_block_with_nonce(BlockType::Atom, "x", "/p", b"b", "md", NONCE);
    assert_eq!(a, b, "same inputs → same DNA when nonce is fixed");
    let parsed = parse_dna(&a).unwrap();
    assert_eq!(parsed.nonce, NONCE);
}

#[test]
fn empty_caps_renders_underscore_segment() {
    // kei_shared rejects empty `caps`. dna_block must substitute "_" so the
    // composed DNA still parses.
    let dna = compose_for_block_with_nonce(BlockType::Atom, "x", "/p", b"b", "", NONCE);
    let parsed = parse_dna(&dna).expect("empty caps must compose to parseable DNA");
    assert_eq!(parsed.caps, "_");
}

#[test]
fn body_change_changes_body_sha() {
    let dna_a = compose_for_block_with_nonce(BlockType::Atom, "x", "/p", b"body-a", "md", NONCE);
    let dna_b = compose_for_block_with_nonce(BlockType::Atom, "x", "/p", b"body-b", "md", NONCE);
    let parsed_a = parse_dna(&dna_a).unwrap();
    let parsed_b = parse_dna(&dna_b).unwrap();
    assert_ne!(parsed_a.body_sha, parsed_b.body_sha);
    assert_eq!(parsed_a.scope_sha, parsed_b.scope_sha, "path unchanged → scope_sha stable");
}

#[test]
fn path_change_changes_scope_sha() {
    let dna_a = compose_for_block_with_nonce(BlockType::Atom, "x", "/p1", b"body", "md", NONCE);
    let dna_b = compose_for_block_with_nonce(BlockType::Atom, "x", "/p2", b"body", "md", NONCE);
    let parsed_a = parse_dna(&dna_a).unwrap();
    let parsed_b = parse_dna(&dna_b).unwrap();
    assert_ne!(parsed_a.scope_sha, parsed_b.scope_sha);
}
