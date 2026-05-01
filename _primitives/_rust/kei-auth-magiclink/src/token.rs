// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

//! Magic-link token codec.
//!
//! Wire format (URL-safe base64, no padding, three `.`-separated parts):
//!
//! ```text
//! <email_b64url>.<expires_unix_ms_b64url>.<hmac_sha256_b64url>
//! ```
//!
//! - `email_b64url`     — UTF-8 bytes of the email, base64url-encoded.
//! - `expires_unix_ms`  — decimal ASCII of an i64 unix-ms timestamp,
//!                        base64url-encoded as bytes (lets us keep the
//!                        same alphabet end-to-end).
//! - `hmac_sha256`      — 32-byte HMAC-SHA256 of the literal ASCII
//!                        `<email_b64url>.<expires_unix_ms_b64url>`,
//!                        base64url-encoded.
//!
//! Verification is stateless: no DB lookup. Revocation, if needed, is
//! the caller's responsibility (e.g. a parallel deny-list keyed on the
//! token's first two parts).

use crate::error::{Error, Result};
use base64::engine::general_purpose::URL_SAFE_NO_PAD as B64;
use base64::Engine;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use subtle::ConstantTimeEq;

type HmacSha256 = Hmac<Sha256>;

/// Build a signed magic-link token.
///
/// `hmac_key` MUST be at least 32 bytes; callers are expected to load it
/// from `MAGICLINK_HMAC_KEY` (see [`crate::provider::MagicLinkProvider::from_env`]).
/// We do not enforce the minimum here — the provider does, before reaching
/// this function — but HMAC itself accepts any non-zero key length.
pub fn build_token(email: &str, expires_unix_ms: i64, hmac_key: &[u8]) -> String {
    let email_b64 = B64.encode(email.as_bytes());
    let expires_b64 = B64.encode(expires_unix_ms.to_string().as_bytes());
    let signed_body = format!("{}.{}", email_b64, expires_b64);
    let tag = sign(hmac_key, signed_body.as_bytes());
    let tag_b64 = B64.encode(tag);
    format!("{}.{}", signed_body, tag_b64)
}

/// Parse, verify HMAC, and check expiry.
///
/// Returns `(email, expires_unix_ms)` on success.
pub fn parse_token(
    token: &str,
    hmac_key: &[u8],
    now_unix_ms: i64,
) -> Result<(String, i64)> {
    let (email_b64, expires_b64, tag_b64) = split_three(token)?;
    let signed_body = format!("{}.{}", email_b64, expires_b64);
    verify_tag(hmac_key, signed_body.as_bytes(), tag_b64)?;
    let email = decode_email(email_b64)?;
    let expires_unix_ms = decode_expiry(expires_b64)?;
    if expires_unix_ms <= now_unix_ms {
        return Err(Error::TokenExpired {
            expires_unix_ms,
            now_unix_ms,
        });
    }
    Ok((email, expires_unix_ms))
}

fn split_three(token: &str) -> Result<(&str, &str, &str)> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(Error::TokenMalformed(format!(
            "expected 3 parts, got {}",
            parts.len()
        )));
    }
    if parts.iter().any(|p| p.is_empty()) {
        return Err(Error::TokenMalformed("empty part".into()));
    }
    Ok((parts[0], parts[1], parts[2]))
}

fn sign(key: &[u8], body: &[u8]) -> Vec<u8> {
    let mut mac = <HmacSha256 as Mac>::new_from_slice(key)
        .expect("HMAC-SHA256 accepts any key length");
    mac.update(body);
    mac.finalize().into_bytes().to_vec()
}

fn verify_tag(key: &[u8], body: &[u8], tag_b64: &str) -> Result<()> {
    let expected = sign(key, body);
    let actual = B64
        .decode(tag_b64)
        .map_err(|e| Error::TokenMalformed(format!("tag b64 decode: {e}")))?;
    if expected.len() != actual.len() {
        return Err(Error::BadSignature);
    }
    if expected.ct_eq(&actual).unwrap_u8() == 1 {
        Ok(())
    } else {
        Err(Error::BadSignature)
    }
}

fn decode_email(email_b64: &str) -> Result<String> {
    let bytes = B64
        .decode(email_b64)
        .map_err(|e| Error::TokenMalformed(format!("email b64 decode: {e}")))?;
    String::from_utf8(bytes)
        .map_err(|e| Error::TokenMalformed(format!("email utf8: {e}")))
}

fn decode_expiry(expires_b64: &str) -> Result<i64> {
    let bytes = B64
        .decode(expires_b64)
        .map_err(|e| Error::TokenMalformed(format!("expiry b64 decode: {e}")))?;
    let s = std::str::from_utf8(&bytes)
        .map_err(|e| Error::TokenMalformed(format!("expiry utf8: {e}")))?;
    s.parse::<i64>()
        .map_err(|e| Error::TokenMalformed(format!("expiry parse: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &[u8] = b"0123456789abcdef0123456789abcdef"; // 32 bytes

    #[test]
    fn base64url_roundtrip_via_token() {
        // Indirectly exercises the URL_SAFE_NO_PAD engine on email + expiry.
        let token = build_token("alice+test@example.com", 9_999_999_999_999, KEY);
        let (email, expires) = parse_token(&token, KEY, 0).expect("parse ok");
        assert_eq!(email, "alice+test@example.com");
        assert_eq!(expires, 9_999_999_999_999);
    }

    #[test]
    fn constant_time_eq_smoke() {
        // Same body + same key MUST verify; different last byte MUST fail.
        let body = b"a.b";
        let tag1 = sign(KEY, body);
        let tag2 = sign(KEY, body);
        assert_eq!(tag1, tag2);
        let mut tampered = tag1.clone();
        tampered[31] ^= 0x01;
        assert!(tag1.ct_eq(&tampered).unwrap_u8() == 0);
    }

    #[test]
    fn build_token_format_three_parts() {
        let token = build_token("u@x", 1, KEY);
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3, "token = `{token}`");
        assert!(parts.iter().all(|p| !p.is_empty()));
    }
}
