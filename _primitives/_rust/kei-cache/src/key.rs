//! Cache key derivation.
//!
//! Constructor Pattern: one cube = canonical JSON serialisation + SHA-256.
//! Key = SHA-256(atom_id || '\0' || canonical_json(input)).
//!
//! Canonical JSON: object keys sorted lexicographically at every depth, no
//! insignificant whitespace. Ensures semantically-identical inputs hash to
//! the same bytes regardless of source formatting.

use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

/// Produce canonical JSON bytes: stable key order, minimal whitespace.
pub fn canonical_json(v: &Value) -> String {
    let canon = canonicalise(v.clone());
    serde_json::to_string(&canon).expect("canonical_json: serialise never fails for owned Value")
}

/// Recursively canonicalise: sort object keys at every nesting level.
fn canonicalise(v: Value) -> Value {
    match v {
        Value::Object(m) => {
            let mut keys: Vec<String> = m.keys().cloned().collect();
            keys.sort();
            let mut out = Map::with_capacity(keys.len());
            let mut src = m;
            for k in keys {
                if let Some(val) = src.remove(&k) {
                    out.insert(k, canonicalise(val));
                }
            }
            Value::Object(out)
        }
        Value::Array(a) => Value::Array(a.into_iter().map(canonicalise).collect()),
        other => other,
    }
}

/// Compute cache key as 64-hex SHA-256 digest of (atom_id \0 canonical_json).
pub fn cache_key(atom_id: &str, input: &Value) -> String {
    let mut hasher = Sha256::new();
    hasher.update(atom_id.as_bytes());
    hasher.update([0u8]);
    hasher.update(canonical_json(input).as_bytes());
    let digest = hasher.finalize();
    hex_lower(&digest)
}

/// Hex-encode lowercase without pulling a separate crate.
fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push(HEX[(b >> 4) as usize] as char);
        s.push(HEX[(b & 0x0f) as usize] as char);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn canonical_sorts_keys() {
        let a = json!({"z": 1, "a": 2, "m": {"y": 1, "b": 2}});
        let b = json!({"a": 2, "m": {"b": 2, "y": 1}, "z": 1});
        assert_eq!(canonical_json(&a), canonical_json(&b));
    }

    #[test]
    fn key_stable_across_formatting() {
        let a = json!({"x": 1, "y": [1, 2]});
        let b: Value = serde_json::from_str("  {\"y\":[1,2],\"x\":1}  ").unwrap();
        assert_eq!(cache_key("atom:foo", &a), cache_key("atom:foo", &b));
    }

    #[test]
    fn key_differs_by_input() {
        let a = json!({"x": 1});
        let b = json!({"x": 2});
        assert_ne!(cache_key("atom:foo", &a), cache_key("atom:foo", &b));
    }

    #[test]
    fn key_differs_by_atom_id() {
        let v = json!({"x": 1});
        assert_ne!(cache_key("atom:foo", &v), cache_key("atom:bar", &v));
    }

    #[test]
    fn key_is_64_hex() {
        let k = cache_key("atom:x", &json!({}));
        assert_eq!(k.len(), 64);
        assert!(k.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
    }
}
