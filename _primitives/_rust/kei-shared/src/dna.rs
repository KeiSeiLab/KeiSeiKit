//! DNA wire format: `<role>::<caps>::<sha8-scope>::<sha8-body>-<hex8-nonce>`.
//!
//! SSoT for the substrate identity string. Any format-level change lands
//! here and propagates to consumers (kei-agent-runtime, kei-dna-index)
//! through re-export, not duplication.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Parsed DNA fields. Widths on hash segments are validated by `parse_dna`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParsedDna {
    pub role: String,
    pub caps: String,
    pub scope_sha: String,
    pub body_sha: String,
    pub nonce: String,
}

/// Strict parse errors. Consumers that need looser semantics (e.g. legacy
/// 4-hex rolling-upgrade acceptance in kei-agent-runtime) keep their own
/// parser and error type.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DnaError {
    #[error("DNA empty")]
    Empty,
    #[error("missing segments (expected 4 '::' separators, got {0})")]
    MissingSegments(usize),
    #[error("missing '-' between body and nonce")]
    MissingNonceDelim,
    #[error("empty role segment")]
    EmptyRole,
    #[error("empty caps segment")]
    EmptyCaps,
    #[error("invalid hex8 width for {field} (got {got})")]
    HexWidth { field: &'static str, got: usize },
    #[error("non-hex character in {field}")]
    NonHex { field: &'static str },
}

/// `true` iff `s` is exactly 8 ASCII hex characters.
pub fn is_hex8(s: &str) -> bool {
    s.len() == 8 && s.chars().all(|c| c.is_ascii_hexdigit())
}

/// Strict parse. Requires 4 `::` segments, `<body>-<nonce>` tail, and
/// 8-hex width on `scope_sha`, `body_sha`, `nonce`. Rejects empty role/caps.
pub fn parse_dna(s: &str) -> Result<ParsedDna, DnaError> {
    if s.is_empty() {
        return Err(DnaError::Empty);
    }
    let parts: Vec<&str> = s.split("::").collect();
    if parts.len() != 4 {
        return Err(DnaError::MissingSegments(parts.len()));
    }
    let (body_sha, nonce) = parts[3]
        .split_once('-')
        .ok_or(DnaError::MissingNonceDelim)?;
    check_non_empty(parts[0], parts[1])?;
    check_hex8("scope_sha", parts[2])?;
    check_hex8("body_sha", body_sha)?;
    check_hex8("nonce", nonce)?;
    Ok(ParsedDna {
        role: parts[0].to_string(),
        caps: parts[1].to_string(),
        scope_sha: parts[2].to_string(),
        body_sha: body_sha.to_string(),
        nonce: nonce.to_string(),
    })
}

/// Render the canonical wire format. Deterministic — no I/O, no randomness.
pub fn compose_dna(
    role: &str,
    caps: &str,
    scope_sha: &str,
    body_sha: &str,
    nonce: &str,
) -> String {
    format!("{role}::{caps}::{scope_sha}::{body_sha}-{nonce}")
}

fn check_non_empty(role: &str, caps: &str) -> Result<(), DnaError> {
    if role.is_empty() {
        return Err(DnaError::EmptyRole);
    }
    if caps.is_empty() {
        return Err(DnaError::EmptyCaps);
    }
    Ok(())
}

fn check_hex8(field: &'static str, value: &str) -> Result<(), DnaError> {
    if value.len() != 8 {
        return Err(DnaError::HexWidth {
            field,
            got: value.len(),
        });
    }
    if !value.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(DnaError::NonHex { field });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_matches_manual_format() {
        let s = compose_dna("r", "C", "12345678", "ABCDEF01", "deadbeef");
        assert_eq!(s, "r::C::12345678::ABCDEF01-deadbeef");
    }

    #[test]
    fn is_hex8_basic() {
        assert!(is_hex8("12345678"));
        assert!(is_hex8("AbCdEf01"));
        assert!(!is_hex8("1234567"));
        assert!(!is_hex8("123456789"));
        assert!(!is_hex8("1234567Z"));
    }
}
