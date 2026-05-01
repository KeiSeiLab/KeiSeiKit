//! Key-path helpers for the S3 cloud backend.
//!
//! v0.22 Track B: the actual logic moved one level up into
//! `crate::async_backend` (shared by every future cloud backend — GCS,
//! Azure Blob, Bunny, etc.). This module now re-exports the helpers for
//! backward source-compat and keeps the unit tests green.

#[allow(unused_imports)]
pub use crate::async_backend::{is_manifest_key, short_hash, validate_rel};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_rel_rejects_absolute() {
        let err = validate_rel("/etc/passwd").unwrap_err();
        let msg = format!("{err:#}");
        assert!(msg.contains("absolute"), "unexpected err: {msg}");
    }

    #[test]
    fn validate_rel_rejects_parent() {
        let err = validate_rel("a/../b").unwrap_err();
        let msg = format!("{err:#}");
        assert!(msg.contains("parent-dir"), "unexpected err: {msg}");
    }

    #[test]
    fn validate_rel_accepts_normal() {
        validate_rel("traces/session.jsonl").unwrap();
        validate_rel("a/b/c.txt").unwrap();
    }

    #[test]
    fn short_hash_deterministic() {
        assert_eq!(short_hash("abc"), short_hash("abc"));
        assert_ne!(short_hash("abc"), short_hash("abd"));
    }
}
