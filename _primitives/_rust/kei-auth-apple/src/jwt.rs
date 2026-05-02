// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Apple id_token verification — ES256 signature check against Apple JWKS.
//!
//! Production path: [`verify_id_token`] — verifies signature, validates
//! standard claims (`iss`, `aud`, `exp`, `iat`).
//!
//! Test-only path: [`decode_id_token_unverified`] — available only under
//! `#[cfg(test)]`; never present in production builds.

use crate::claims::IdTokenClaims;
use crate::error::{Error, Result};
use jsonwebtoken::{
    decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, Validation,
};
use std::time::{SystemTime, UNIX_EPOCH};

/// Verify an Apple id_token against the provided JWKS JSON, checking:
/// - ES256 signature against the matching `kid` in `jwks_json`.
/// - `iss == "https://appleid.apple.com"`.
/// - `aud` contains `client_id`.
/// - `exp > now` (not expired).
/// - `iat <= now` (not in the future).
///
/// `jwks_json` is the raw JSON body of Apple's public JWKS endpoint
/// (`https://appleid.apple.com/auth/keys`). The caller is responsible for
/// fetching and caching it.
pub fn verify_id_token(
    token: &str,
    jwks_json: &str,
    client_id: &str,
) -> Result<IdTokenClaims> {
    let header = decode_header(token)
        .map_err(|e| Error::JwtVerify(format!("header: {e}")))?;
    let kid = header
        .kid
        .ok_or_else(|| Error::JwtVerify("missing kid in JWT header".into()))?;
    let jwks: JwkSet = serde_json::from_str(jwks_json)
        .map_err(|e| Error::JwtVerify(format!("jwks json: {e}")))?;
    let jwk = jwks
        .find(&kid)
        .ok_or_else(|| Error::JwtVerify(format!("kid {kid} not found in JWKS")))?;
    let decoding_key = DecodingKey::from_jwk(jwk)
        .map_err(|e| Error::JwtVerify(format!("decoding key: {e}")))?;
    let mut validation = Validation::new(Algorithm::ES256);
    validation.validate_exp = true;
    validation.validate_aud = false; // we validate aud manually below
    let data = decode::<IdTokenClaims>(token, &decoding_key, &validation)
        .map_err(|e| Error::JwtVerify(format!("verify: {e}")))?;
    validate_claims(&data.claims, client_id)?;
    Ok(data.claims)
}

fn validate_claims(c: &IdTokenClaims, client_id: &str) -> Result<()> {
    const APPLE_ISS: &str = "https://appleid.apple.com";
    if c.iss != APPLE_ISS {
        return Err(Error::JwtVerify(format!(
            "iss mismatch: expected {APPLE_ISS}, got {}", c.iss
        )));
    }
    if !c.aud.contains(client_id) {
        return Err(Error::JwtVerify(format!(
            "aud does not contain client_id {client_id}"
        )));
    }
    let now = now_unix_secs();
    if c.exp <= now {
        return Err(Error::JwtVerify("token expired".into()));
    }
    if c.iat > now + 300 {
        // 5-minute clock-skew tolerance
        return Err(Error::JwtVerify("iat is in the future".into()));
    }
    if c.sub.is_empty() {
        return Err(Error::MissingClaim("sub".into()));
    }
    Ok(())
}

fn now_unix_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Decode the claims segment of a JWT WITHOUT verifying the signature.
///
/// ONLY available under `#[cfg(test)]`. Production code MUST use
/// [`verify_id_token`] which validates the ES256 signature.
#[cfg(test)]
pub fn decode_id_token_unverified(jwt: &str) -> Result<IdTokenClaims> {
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use base64::Engine as _;
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
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use base64::Engine as _;

    fn b64(input: &str) -> String {
        URL_SAFE_NO_PAD.encode(input.as_bytes())
    }

    fn make_jwt(header_b64: &str, payload_b64: &str, sig_b64: &str) -> String {
        format!("{header_b64}.{payload_b64}.{sig_b64}")
    }

    #[test]
    fn decode_unverified_valid() {
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
    fn decode_unverified_reject_two_segments() {
        let header = b64("{\"alg\":\"ES256\"}");
        let payload = b64("{\"sub\":\"x\"}");
        let jwt = format!("{header}.{payload}");
        let err = decode_id_token_unverified(&jwt).unwrap_err();
        assert!(matches!(err, Error::JwtDecode(_)));
    }

    #[test]
    fn decode_unverified_reject_invalid_base64() {
        let jwt = "abc.!!!not-base64!!!.zzz";
        let err = decode_id_token_unverified(jwt).unwrap_err();
        assert!(matches!(err, Error::JwtDecode(_)));
    }
}
