//! Unit coverage for `kei_cortex::auth` — token lifecycle.

use kei_cortex::auth;
use tempfile::tempdir;

#[test]
fn token_generate_creates_64_hex_chars() {
    let tok = auth::generate_token();
    assert_eq!(tok.len(), 64, "hex-encoded 32 bytes = 64 chars");
    assert!(
        tok.chars().all(|c| c.is_ascii_hexdigit()),
        "every char must be hex"
    );
}

#[test]
fn token_load_roundtrips() {
    let tmp = tempdir().unwrap();
    let path = tmp.path().join("cortex.token");
    let original = auth::generate_token();
    auth::save_token(&path, &original).unwrap();
    let loaded = auth::load_token(&path).unwrap();
    assert_eq!(loaded, original);
}

#[test]
#[cfg(unix)]
fn token_file_chmod_600_on_unix() {
    use std::os::unix::fs::PermissionsExt;
    let tmp = tempdir().unwrap();
    let path = tmp.path().join("cortex.token");
    auth::save_token(&path, &auth::generate_token()).unwrap();
    let meta = std::fs::metadata(&path).unwrap();
    let mode = meta.permissions().mode() & 0o777;
    assert_eq!(mode, 0o600, "token file must be 0600, got {mode:o}");
}

#[test]
fn token_validate_rejects_short() {
    assert!(auth::validate_hex("abc").is_err());
}

#[test]
fn token_validate_rejects_non_hex() {
    let mut bad = "a".repeat(63);
    bad.push('Z');
    assert!(auth::validate_hex(&bad).is_err());
}

#[test]
fn tokens_match_true_on_equal() {
    let t = auth::generate_token();
    assert!(auth::tokens_match(&t, &t));
}

#[test]
fn tokens_match_false_on_diff() {
    let a = auth::generate_token();
    let b = auth::generate_token();
    assert!(!auth::tokens_match(&a, &b));
}

#[test]
fn tokens_match_case_insensitive_uppercase_paste() {
    // MISS-6: a user pasting an UPPERCASE token through the UI must
    // validate against the lowercase form the daemon stores.
    let lower = auth::generate_token();
    let upper = lower.to_ascii_uppercase();
    assert!(
        auth::tokens_match(&lower, &upper),
        "UPPERCASE paste must match lowercase stored token"
    );
}
