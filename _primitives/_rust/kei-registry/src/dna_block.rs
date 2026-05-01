//! DNA composition for kit blocks.
//!
//! Wire-format `<block_type>::<caps>::<scope_sha8>::<body_sha8>-<nonce8>`
//! delegates to `kei_shared::compose_dna` so the format string SSoT stays
//! in one place. `compose_for_block` is the only public surface — all
//! other crates that want a block DNA call this helper.
//!
//! Determinism: scope_sha and body_sha are pure SHA-256 over canonical
//! inputs. The nonce is the only entropy source; callers that want
//! idempotency pass the existing row's nonce.

use kei_shared::dna::compose_dna;
use sha2::{Digest, Sha256};

use crate::block::BlockType;

/// 8-hex (32-bit) prefix of SHA-256(`input`). Lowercase, deterministic.
pub fn short_sha8(input: &[u8]) -> String {
    let digest = Sha256::digest(input);
    format!(
        "{:02x}{:02x}{:02x}{:02x}",
        digest[0], digest[1], digest[2], digest[3]
    )
}

/// Compose a block DNA. `block_type` becomes the wire `<role>` segment.
/// `path` and `body` are hashed independently so a move (path change) and
/// a rewrite (body change) produce distinct supersede chains.
///
/// Spec-shape (5 args): nonce is generated from system entropy. For a
/// deterministic variant use [`compose_for_block_with_nonce`].
pub fn compose_for_block(
    block_type: BlockType,
    name: &str,
    path: &str,
    body: &[u8],
    caps: &str,
) -> String {
    compose_for_block_with_nonce(block_type, name, path, body, caps, &fresh_nonce())
}

/// Deterministic variant — caller supplies the nonce. Used by `register`
/// for idempotent re-registration (existing row's nonce is preserved).
pub fn compose_for_block_with_nonce(
    block_type: BlockType,
    _name: &str,
    path: &str,
    body: &[u8],
    caps: &str,
    nonce: &str,
) -> String {
    let scope_sha = short_sha8(path.as_bytes());
    let body_sha = short_sha8(body);
    let caps_segment = if caps.is_empty() { "_" } else { caps };
    compose_dna(
        block_type.as_str(),
        caps_segment,
        &scope_sha,
        &body_sha,
        nonce,
    )
}

/// Generate a fresh 8-hex nonce from system entropy. Stable per-block: the
/// caller (registry::register) reuses an existing row's nonce when the
/// (path, body_sha) tuple matches, so the DNA stays idempotent.
pub fn fresh_nonce() -> String {
    let now_ns = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64 ^ d.as_secs())
        .unwrap_or(0);
    let pid = std::process::id() as u64;
    let mixed = now_ns
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(pid.wrapping_mul(0xBF58_476D_1CE4_E5B9));
    let truncated = (mixed ^ (mixed >> 32)) as u32;
    format!("{truncated:08x}")
}
