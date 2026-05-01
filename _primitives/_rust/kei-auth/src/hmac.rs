//! HMAC-SHA256 signer for token bodies.

use ::hmac::{Hmac, Mac};
use anyhow::{anyhow, Result};
use base64::Engine;
use sha2::Sha256;

type H = Hmac<Sha256>;

/// Sign `body` with `key`. Returns URL-safe base64 MAC.
pub fn sign(key: &[u8], body: &[u8]) -> String {
    let mut mac = <H as Mac>::new_from_slice(key).expect("HMAC accepts any key size");
    mac.update(body);
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes())
}

/// Verify `body` against MAC. Returns Err if mismatch.
pub fn verify(key: &[u8], body: &[u8], mac_b64: &str) -> Result<()> {
    let mut mac = <H as Mac>::new_from_slice(key).expect("HMAC accepts any key size");
    mac.update(body);
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(mac_b64)
        .map_err(|e| anyhow!("bad b64 mac: {e}"))?;
    mac.verify_slice(&bytes).map_err(|_| anyhow!("hmac mismatch"))
}
