//! Shared input validators — pure functions, no I/O, no state.
//!
//! Centralised so every handler touching `:user_id` goes through the SAME
//! whitelist. Rejects anything we would not want substituted into a path,
//! a SQL LIKE clause, or a TOML filename.

use crate::error::AppError;

/// Upper bound on `user_id` length. Keeps path construction trivial and
/// headers / query strings bounded.
pub const MAX_USER_ID_LEN: usize = 64;

/// Strict whitelist validator for the `:user_id` path parameter.
///
/// Allowed characters: ASCII letters, digits, underscore, hyphen. Nothing
/// else. No dots (prevents `..` traversal), no slashes, no control
/// characters, no unicode (avoids normalization surprises on the
/// filesystem layer). Length 1..=64.
pub fn user_id(s: &str) -> Result<(), AppError> {
    if s.is_empty() {
        return Err(AppError::BadRequest("invalid user_id".into()));
    }
    if s.len() > MAX_USER_ID_LEN {
        return Err(AppError::BadRequest("invalid user_id".into()));
    }
    if !s
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        return Err(AppError::BadRequest("invalid user_id".into()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_alnum_and_hyphen_and_underscore() {
        assert!(user_id("u0").is_ok());
        assert!(user_id("alice_01").is_ok());
        assert!(user_id("a-b-c-123").is_ok());
        assert!(user_id("ABC_def-789").is_ok());
        assert!(user_id(&"a".repeat(MAX_USER_ID_LEN)).is_ok());
    }

    #[test]
    fn rejects_empty() {
        let err = user_id("").unwrap_err();
        assert!(matches!(err, AppError::BadRequest(ref m) if m == "invalid user_id"));
    }

    #[test]
    fn rejects_too_long() {
        let s = "a".repeat(MAX_USER_ID_LEN + 1);
        assert!(matches!(user_id(&s), Err(AppError::BadRequest(_))));
    }

    #[test]
    fn rejects_path_traversal_and_special_chars() {
        assert!(user_id("..").is_err());
        assert!(user_id("../etc").is_err());
        assert!(user_id("a/b").is_err());
        assert!(user_id("a\\b").is_err());
        assert!(user_id("a.b").is_err());
        assert!(user_id("a b").is_err());
        assert!(user_id("a\nb").is_err());
        assert!(user_id("a\tb").is_err());
        assert!(user_id("a\0b").is_err());
    }

    #[test]
    fn rejects_unicode() {
        assert!(user_id("кириллица").is_err());
        assert!(user_id("a\u{00e9}").is_err());
        assert!(user_id("\u{1f600}").is_err());
    }
}
