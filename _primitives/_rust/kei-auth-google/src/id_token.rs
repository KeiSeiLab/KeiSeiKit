// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! ID-token claim extraction for Google OIDC.
//!
//! **Scope (deliberate, narrow).** This module decodes the *claims*
//! payload of a JWT — the middle base64url segment — and surfaces the
//! `sub` field. It does **not** verify the JWT signature against
//! Google's JWKS. Signature verification is a follow-up (load JWKS
//! over HTTPS, cache by `kid`, run RS256/ES256). Until then, the
//! `id_token.sub` is treated as a defence-in-depth cross-check
//! against the userinfo `sub` (the token came from a TLS-validated
//! token endpoint, but a malicious userinfo response could still
//! ship a different `sub` if the access token leaked).
//!
//! See RFC 7519 §3 (JWT compact serialization) and OIDC Core §2
//! (id_token claims).
//!
//! [VERIFIED: https://datatracker.ietf.org/doc/html/rfc7519]

use crate::error::{Error, Result};
use base64::Engine as _;
use serde::Deserialize;

/// Minimal projection of the OIDC id_token claims payload.
#[derive(Debug, Clone, Deserialize)]
pub struct IdTokenClaims {
    /// Stable Google account identifier; matches userinfo `sub`.
    pub sub: String,
}

/// Parse the **claims** segment of a JWT and decode `sub`.
///
/// Returns [`Error::IdTokenMalformed`] if the token is not three
/// segments, base64url-decode fails, or the JSON lacks `sub`.
///
/// **Does not** verify the JWT signature — see module-level docs.
pub fn extract_sub(id_token: &str) -> Result<String> {
    let claims_b64 = jwt_claims_segment(id_token)?;
    let claims_json = decode_b64url(claims_b64)?;
    let claims: IdTokenClaims = serde_json::from_slice(&claims_json)
        .map_err(|e| Error::IdTokenMalformed(format!("claims json: {e}")))?;
    Ok(claims.sub)
}

/// Pull the middle (claims) segment of a JWT compact serialization.
fn jwt_claims_segment(id_token: &str) -> Result<&str> {
    let mut parts = id_token.split('.');
    let _header = parts.next()
        .ok_or_else(|| Error::IdTokenMalformed("missing header".into()))?;
    let claims = parts.next()
        .ok_or_else(|| Error::IdTokenMalformed("missing claims".into()))?;
    let _sig = parts.next()
        .ok_or_else(|| Error::IdTokenMalformed("missing signature".into()))?;
    if parts.next().is_some() {
        return Err(Error::IdTokenMalformed("too many segments".into()));
    }
    Ok(claims)
}

/// base64url-no-pad decode (RFC 7515 §2). Tolerant of optional padding.
fn decode_b64url(input: &str) -> Result<Vec<u8>> {
    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(input)
        .or_else(|_| base64::engine::general_purpose::URL_SAFE.decode(input))
        .map_err(|e| Error::IdTokenMalformed(format!("b64: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_jwt(claims_json: &str) -> String {
        let header = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(br#"{"alg":"RS256","typ":"JWT"}"#);
        let claims = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(claims_json.as_bytes());
        format!("{header}.{claims}.fake-sig")
    }

    #[test]
    fn extract_sub_happy_path() {
        let jwt = make_jwt(r#"{"sub":"1234567890","email":"a@b.c"}"#);
        assert_eq!(extract_sub(&jwt).unwrap(), "1234567890");
    }

    #[test]
    fn extract_sub_rejects_two_segment_token() {
        let err = extract_sub("only.two").unwrap_err();
        assert!(format!("{err}").contains("id_token"));
    }

    #[test]
    fn extract_sub_rejects_garbage_claims() {
        let jwt = "header.@@@@.sig";
        assert!(extract_sub(jwt).is_err());
    }

    #[test]
    fn extract_sub_rejects_missing_sub_field() {
        let jwt = make_jwt(r#"{"email":"x@y.z"}"#);
        assert!(extract_sub(&jwt).is_err());
    }
}
