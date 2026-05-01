//! Inline unit tests for `auth.rs`.
//!
//! Coverage:
//!   - `tokens_match` is case-insensitive (MISS-6 fix).
//!   - `validate_hex` accepts both cases, rejects bad length / non-hex.
//!   - `generate_token` round-trips through validate + match.

use super::*;

#[test]
fn tokens_match_lowercase_to_lowercase() {
    let token = "deadbeef".repeat(8); // 64 chars
    assert!(tokens_match(&token, &token));
}

#[test]
fn tokens_match_uppercase_input_against_lowercase_expected() {
    // The daemon stores tokens lowercase; user pastes uppercase via UI.
    let lower = "deadbeef".repeat(8);
    let upper = lower.to_ascii_uppercase();
    assert!(
        tokens_match(&lower, &upper),
        "uppercase paste must validate against lowercase stored token"
    );
}

#[test]
fn tokens_match_mixed_case_both_sides() {
    let a = "DeadBeef".repeat(8);
    let b = "dEADbEEF".repeat(8);
    assert!(tokens_match(&a, &b));
}

#[test]
fn tokens_match_rejects_different_content() {
    let a = "deadbeef".repeat(8);
    let mut b = a.clone();
    b.replace_range(0..1, "0"); // flip one nibble
    assert!(!tokens_match(&a, &b));
}

#[test]
fn tokens_match_rejects_length_mismatch() {
    assert!(!tokens_match("abcdef", "abcde"));
    assert!(!tokens_match("abcde", "abcdef"));
}

#[test]
fn validate_hex_accepts_both_cases() {
    let lower = "0123456789abcdef".repeat(4);
    let upper = lower.to_ascii_uppercase();
    let mixed = "0123456789AbCdEf".repeat(4);
    assert!(validate_hex(&lower).is_ok());
    assert!(validate_hex(&upper).is_ok());
    assert!(validate_hex(&mixed).is_ok());
}

#[test]
fn validate_hex_rejects_wrong_length() {
    let too_short = "ab".to_string();
    assert!(matches!(
        validate_hex(&too_short),
        Err(AuthError::BadLength(2))
    ));
}

#[test]
fn validate_hex_rejects_non_hex_byte() {
    let mut bad = "a".repeat(TOKEN_HEX_LEN);
    bad.replace_range(5..6, "z");
    assert!(matches!(validate_hex(&bad), Err(AuthError::NotHex(5))));
}

#[test]
fn generate_token_round_trips_through_validate_and_match() {
    let t = generate_token();
    assert_eq!(t.len(), TOKEN_HEX_LEN);
    assert!(validate_hex(&t).is_ok());
    assert!(tokens_match(&t, &t));
}
