//! Smoke tests for the shared DNA parser.

use kei_shared::dna::{compose_dna, is_hex8, parse_dna, DnaError};

const CANONICAL: &str =
    "edit-local::NG-FW-FD-CP-CG-TG-ND-RF::5435F821::AC73A6A3-e9bf468d";

#[test]
fn parse_valid_round_trip() {
    let parsed = parse_dna(CANONICAL).expect("canonical DNA must parse");
    assert_eq!(parsed.role, "edit-local");
    assert_eq!(parsed.caps, "NG-FW-FD-CP-CG-TG-ND-RF");
    assert_eq!(parsed.scope_sha, "5435F821");
    assert_eq!(parsed.body_sha, "AC73A6A3");
    assert_eq!(parsed.nonce, "e9bf468d");
}

#[test]
fn parse_empty_rejected() {
    assert_eq!(parse_dna("").unwrap_err(), DnaError::Empty);
}

#[test]
fn parse_missing_segments() {
    let err = parse_dna("only::two::segments").unwrap_err();
    assert!(matches!(err, DnaError::MissingSegments(_)));
}

#[test]
fn parse_missing_nonce_delim() {
    // 4 `::` segments but no '-' in tail.
    let err = parse_dna("r::C::12345678::AC73A6A3e9bf468d").unwrap_err();
    assert_eq!(err, DnaError::MissingNonceDelim);
}

#[test]
fn parse_non_hex_rejected() {
    let err = parse_dna("r::C::12345678::AC73A6A3-ZZZZZZZZ").unwrap_err();
    assert!(matches!(err, DnaError::NonHex { .. }));
}

#[test]
fn parse_short_hex_rejected() {
    // scope_sha has only 4 hex chars — strict parser rejects.
    let err = parse_dna("r::C::1234::AC73A6A3-e9bf468d").unwrap_err();
    assert!(matches!(err, DnaError::HexWidth { .. }));
}

#[test]
fn parse_rejects_empty_role() {
    let err = parse_dna("::C::12345678::AC73A6A3-e9bf468d").unwrap_err();
    assert_eq!(err, DnaError::EmptyRole);
}

#[test]
fn parse_rejects_empty_caps() {
    let err = parse_dna("r::::12345678::AC73A6A3-e9bf468d").unwrap_err();
    assert_eq!(err, DnaError::EmptyCaps);
}

#[test]
fn compose_parse_round_trip() {
    let composed = compose_dna(
        "edit-local",
        "NG-FW-FD-CP-CG-TG-ND-RF",
        "5435F821",
        "AC73A6A3",
        "e9bf468d",
    );
    assert_eq!(composed, CANONICAL);
    let parsed = parse_dna(&composed).expect("round-trip parse");
    assert_eq!(parsed.scope_sha, "5435F821");
    assert_eq!(parsed.body_sha, "AC73A6A3");
    assert_eq!(parsed.nonce, "e9bf468d");
}

#[test]
fn is_hex8_accepts_valid_rejects_invalid() {
    assert!(is_hex8("00000000"));
    assert!(is_hex8("DeAdBeEf"));
    assert!(is_hex8("abcdef01"));
    assert!(!is_hex8("abcdefg1"), "non-hex 'g' must be rejected");
    assert!(!is_hex8(""));
    assert!(!is_hex8("1234567"));
    assert!(!is_hex8("123456789"));
}
