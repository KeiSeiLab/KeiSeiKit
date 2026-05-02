// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! PKCE (RFC 7636) helpers and URL percent-encoder shared by the Google
//! auth provider.

use base64::Engine as _;
use sha2::{Digest, Sha256};

/// Compute PKCE `code_challenge` = `BASE64URL-no-pad(SHA256(verifier))`.
///
/// The `code_verifier` is a high-entropy random string (ASCII unreserved
/// characters, 43–128 chars). See RFC 7636 §4.1.
pub fn pkce_challenge(verifier: &str) -> String {
    let hash = Sha256::digest(verifier.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash)
}

/// Percent-encode a string per RFC 3986 §2.1 (only unreserved chars pass).
pub(crate) fn url_encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for b in input.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_encode_basics() {
        assert_eq!(url_encode("a b"), "a%20b");
        assert_eq!(url_encode("openid email profile"), "openid%20email%20profile");
        assert_eq!(url_encode("https://x/cb"), "https%3A%2F%2Fx%2Fcb");
        assert_eq!(url_encode("safe-_.~"), "safe-_.~");
    }

    #[test]
    fn pkce_challenge_is_base64url_sha256() {
        // RFC 7636 §B.1 test vector.
        let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
        let challenge = pkce_challenge(verifier);
        assert_eq!(challenge, "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM");
    }
}
