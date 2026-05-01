// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Unverified JWT claim decoder.
//!
//! KNOWN LIMITATION (v0.1):
//!   This module performs ZERO signature verification. It only splits the
//!   JWT into three segments and base64-url-decodes the middle (claims)
//!   segment. Production code that trusts these claims for an
//!   authentication decision MUST verify the signature against Apple's
//!   JWKS first. Full verification will live in a future sister crate
//!   `kei-auth-apple-jwt`.

use crate::error::{Error, Result};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use serde::{Deserialize, Serialize};

/// Subset of standard OIDC + Apple-specific claims we read.
///
/// Apple's id_token always carries `sub` (the stable Apple user id) and
/// `iss` (`https://appleid.apple.com`). `email` is present on first
/// authorization but may be absent on subsequent ones; it may also be a
/// private-relay address (`@privaterelay.appleid.com`).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdTokenClaims {
    pub sub: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub exp: i64,
    #[serde(default)]
    pub iss: String,
}

/// Decode the claims segment of a JWT WITHOUT verifying the signature.
///
/// Splits on `.`, expects exactly three segments (`header.payload.sig`),
/// base64-url-decodes the middle segment, then `serde_json`-parses it.
pub fn decode_id_token_unverified(jwt: &str) -> Result<IdTokenClaims> {
    let mut parts = jwt.split('.');
    let _header = parts
        .next()
        .ok_or_else(|| Error::JwtDecode("missing header segment".into()))?;
    let payload = parts
        .next()
        .ok_or_else(|| Error::JwtDecode("missing payload segment".into()))?;
    let _sig = parts
        .next()
        .ok_or_else(|| Error::JwtDecode("missing signature segment".into()))?;
    if parts.next().is_some() {
        return Err(Error::JwtDecode("more than 3 segments".into()));
    }
    let bytes = URL_SAFE_NO_PAD
        .decode(payload.as_bytes())
        .map_err(|e| Error::JwtDecode(format!("base64: {e}")))?;
    let claims: IdTokenClaims = serde_json::from_slice(&bytes)
        .map_err(|e| Error::JwtDecode(format!("json: {e}")))?;
    if claims.sub.is_empty() {
        return Err(Error::MissingClaim("sub".into()));
    }
    Ok(claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a JWT-shaped string with arbitrary header / payload / sig
    /// segments. Each segment is base64-url-encoded (no padding) where
    /// applicable; non-encoded raw inputs are passed through (used for
    /// negative tests).
    fn make_jwt(header_b64: &str, payload_b64: &str, sig_b64: &str) -> String {
        format!("{header_b64}.{payload_b64}.{sig_b64}")
    }

    fn b64(input: &str) -> String {
        URL_SAFE_NO_PAD.encode(input.as_bytes())
    }

    #[test]
    fn decode_valid() {
        let header = b64("{\"alg\":\"ES256\"}");
        let payload = b64(
            "{\"sub\":\"001234.aabbcc\",\"email\":\"x@y.example\",\"exp\":9999999999,\"iss\":\"https://appleid.apple.com\"}",
        );
        let sig = b64("fake-sig");
        let jwt = make_jwt(&header, &payload, &sig);
        let claims = decode_id_token_unverified(&jwt).unwrap();
        assert_eq!(claims.sub, "001234.aabbcc");
        assert_eq!(claims.email.as_deref(), Some("x@y.example"));
        assert_eq!(claims.exp, 9_999_999_999);
        assert_eq!(claims.iss, "https://appleid.apple.com");
    }

    #[test]
    fn reject_two_segments() {
        let header = b64("{\"alg\":\"ES256\"}");
        let payload = b64("{\"sub\":\"x\"}");
        let jwt = format!("{header}.{payload}");
        let err = decode_id_token_unverified(&jwt).unwrap_err();
        assert!(matches!(err, Error::JwtDecode(_)));
    }

    #[test]
    fn reject_invalid_base64() {
        // Middle segment contains characters illegal in base64-url.
        let jwt = "abc.!!!not-base64!!!.zzz";
        let err = decode_id_token_unverified(jwt).unwrap_err();
        assert!(matches!(err, Error::JwtDecode(_)));
    }
}
