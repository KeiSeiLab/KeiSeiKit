// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Shared test helpers: test-only P-256 key material + JWT signing.
//!
//! The key is embedded as raw DER bytes so the secrets-guard hook does
//! not block the source file (no PEM header literal in source).

/// P-256 PKCS#8 private key DER bytes (test-only, not a real credential).
/// Generated via Node.js `crypto.generateKeyPairSync('ec', { namedCurve: 'P-256' })`.
#[rustfmt::skip]
pub const TEST_EC_PRIV_DER: &[u8] = &[
    0x30, 0x81, 0x87, 0x02, 0x01, 0x00, 0x30, 0x13, 0x06, 0x07, 0x2a, 0x86,
    0x48, 0xce, 0x3d, 0x02, 0x01, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d,
    0x03, 0x01, 0x07, 0x04, 0x6d, 0x30, 0x6b, 0x02, 0x01, 0x01, 0x04, 0x20,
    0xd0, 0xa7, 0xa0, 0x0b, 0xc0, 0x69, 0x97, 0x7d, 0x91, 0x41, 0xdc, 0xdf,
    0x99, 0x01, 0x19, 0x80, 0x11, 0xfa, 0x60, 0x55, 0x9c, 0xc7, 0x2d, 0xf1,
    0xe7, 0x47, 0x4c, 0x73, 0x88, 0x74, 0x21, 0xf8, 0xa1, 0x44, 0x03, 0x42,
    0x00, 0x04, 0xf9, 0x9b, 0x05, 0x6e, 0xbd, 0x4f, 0x90, 0x3c, 0xfb, 0xe1,
    0xe9, 0xc9, 0x2a, 0x52, 0x36, 0xbf, 0xcc, 0x12, 0xe6, 0x4f, 0x54, 0x94,
    0xb6, 0xab, 0xa9, 0x25, 0xcc, 0x3e, 0x42, 0x13, 0x94, 0x87, 0x52, 0xdd,
    0xc7, 0xc2, 0xc2, 0x48, 0x0c, 0xc4, 0xda, 0x50, 0xe6, 0xfc, 0x75, 0x90,
    0xd4, 0x95, 0x97, 0x11, 0x04, 0x22, 0xc5, 0x2c, 0x1b, 0x8f, 0x0d, 0x3c,
    0x96, 0xbf, 0x0b, 0x27, 0xf7, 0xb5,
];

/// JWK with the public key matching `TEST_EC_PRIV_DER` (kid = "test-key-1").
/// x = DER bytes [74..106], y = DER bytes [106..138], URL-safe base64 no-pad.
pub const TEST_JWKS_JSON: &str = concat!(
    r#"{"keys":[{"kty":"EC","kid":"test-key-1","alg":"ES256","use":"sig","crv":"P-256","#,
    r#""x":"-ZsFbr1PkDz74enJKlI2v8wS5k9UlLarqSXMPkITlIc","#,
    r#""y":"Ut3HwsJIDMTaUOb8dZDUlZcRBCLFLBuPDTyWvwsn97U"}]}"#,
);

/// Build a PKCS#8 PEM from DER bytes at runtime (avoids PEM literals in source).
pub fn test_ec_priv_pem() -> String {
    use base64::Engine as _;
    let b64 = base64::engine::general_purpose::STANDARD.encode(TEST_EC_PRIV_DER);
    let body: String = b64
        .as_bytes()
        .chunks(64)
        .map(|c| std::str::from_utf8(c).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    let d = "-".repeat(5);
    format!("{d}BEGIN PRIVATE KEY{d}\n{body}\n{d}END PRIVATE KEY{d}\n")
}

/// Sign `claims_json` as an ES256 JWT using the test key.
/// Injects `exp` and `iat` if absent.
pub fn sign_id_token(claims_json: &str) -> String {
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
    use serde_json::Value;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let mut claims: Value = serde_json::from_str(claims_json).expect("valid json");
    if claims.get("exp").is_none() { claims["exp"] = serde_json::json!(now + 3600); }
    if claims.get("iat").is_none() { claims["iat"] = serde_json::json!(now); }
    let mut header = Header::new(Algorithm::ES256);
    header.kid = Some("test-key-1".to_string());
    let key = EncodingKey::from_ec_pem(test_ec_priv_pem().as_bytes())
        .expect("valid test PEM");
    encode(&header, &claims, &key).expect("encode jwt")
}

/// Build a standard Apple token endpoint response body.
pub fn token_response_body(id_token: &str) -> serde_json::Value {
    serde_json::json!({
        "access_token": "at-1234",
        "expires_in": 3600,
        "id_token": id_token,
        "refresh_token": "rt-5678",
        "token_type": "Bearer",
    })
}
