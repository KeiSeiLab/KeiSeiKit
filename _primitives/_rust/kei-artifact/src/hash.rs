//! sha256-based artifact id.
//!
//! Id = sha256(schema_name || 0x00 || content_bytes). Including the schema
//! name prevents trivial collisions across different content types with the
//! same payload bytes. Hex-encoded 64-char string.

use sha2::{Digest, Sha256};

/// Deterministic artifact id from schema name + content bytes.
pub fn artifact_id(schema_name: &str, content: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(schema_name.as_bytes());
    h.update([0u8]);
    h.update(content);
    let out = h.finalize();
    hex_encode(&out)
}

fn hex_encode(bytes: &[u8]) -> String {
    const TABLE: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push(TABLE[(*b >> 4) as usize] as char);
        s.push(TABLE[(*b & 0x0f) as usize] as char);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_is_deterministic_for_same_input() {
        let a = artifact_id("spec", b"hello");
        let b = artifact_id("spec", b"hello");
        assert_eq!(a, b);
        assert_eq!(a.len(), 64);
    }

    #[test]
    fn id_changes_with_schema_name() {
        let a = artifact_id("spec", b"hello");
        let b = artifact_id("plan", b"hello");
        assert_ne!(a, b);
    }

    #[test]
    fn id_changes_with_content() {
        let a = artifact_id("spec", b"hello");
        let b = artifact_id("spec", b"world");
        assert_ne!(a, b);
    }
}
