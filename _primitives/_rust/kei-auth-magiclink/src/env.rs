// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>

//! Environment + key-decoding helpers for `MagicLinkProvider::from_env`.
//!
//! Kept in its own cube so [`provider`](crate::provider) stays under the
//! Constructor-Pattern 200-LOC limit. Pure functions, no trait surface.

use crate::error::{Error, Result};
use base64::engine::general_purpose::STANDARD as B64_STD;
use base64::Engine;

pub const ENV_KEY: &str = "MAGICLINK_HMAC_KEY";
pub const ENV_TTL: &str = "MAGICLINK_TTL_SECS";
pub const DEFAULT_TTL_SECS: i64 = 900; // 15 minutes
pub const MIN_KEY_LEN: usize = 32;

/// Read `MAGICLINK_HMAC_KEY` and `MAGICLINK_TTL_SECS` from the environment.
pub fn read_env() -> Result<(Vec<u8>, i64)> {
    let raw = std::env::var(ENV_KEY)
        .map_err(|_| Error::KeyMissing(format!("{ENV_KEY} unset")))?;
    let key = decode_key(&raw)?;
    let ttl = match std::env::var(ENV_TTL) {
        Ok(v) => v
            .parse::<i64>()
            .map_err(|e| Error::KeyMissing(format!("{ENV_TTL} parse: {e}")))?,
        Err(_) => DEFAULT_TTL_SECS,
    };
    Ok((key, ttl))
}

/// Decode a key string. 64 ASCII hex chars → hex; else standard base64.
pub fn decode_key(raw: &str) -> Result<Vec<u8>> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(Error::KeyMissing("empty value".into()));
    }
    if trimmed.len() == 64 && trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
        return decode_hex(trimmed);
    }
    B64_STD
        .decode(trimmed)
        .map_err(|e| Error::KeyMissing(format!("base64 decode: {e}")))
}

fn decode_hex(s: &str) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(s.len() / 2);
    let bytes = s.as_bytes();
    for i in (0..bytes.len()).step_by(2) {
        let hi = hex_digit(bytes[i])?;
        let lo = hex_digit(bytes[i + 1])?;
        out.push((hi << 4) | lo);
    }
    Ok(out)
}

fn hex_digit(b: u8) -> Result<u8> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        _ => Err(Error::KeyMissing(format!("non-hex digit: 0x{b:02x}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_hex_64chars() {
        let key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let bytes = decode_key(key).expect("ok");
        assert_eq!(bytes.len(), 32);
        assert_eq!(bytes[0], 0x01);
        assert_eq!(bytes[31], 0xef);
    }

    #[test]
    fn decode_base64() {
        // 32 zero bytes → 44-char base64 standard.
        let raw = B64_STD.encode([0u8; 32]);
        let bytes = decode_key(&raw).expect("ok");
        assert_eq!(bytes.len(), 32);
    }

    #[test]
    fn decode_empty_rejected() {
        let err = decode_key("").expect_err("must reject");
        assert!(matches!(err, Error::KeyMissing(_)));
    }
}
