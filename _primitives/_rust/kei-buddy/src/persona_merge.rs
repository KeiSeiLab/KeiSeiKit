// SPDX-License-Identifier: Apache-2.0
//! JSON deep-merge helper for persona patches.
//!
//! Constructor Pattern: one responsibility — merge two JSON values.
//! Arrays and primitives in `patch` always overwrite `base`.
//! Object fields are merged recursively (patch wins on conflict).

use serde_json::Value;

/// Deep-merge `patch` into `base`.
///
/// Rules:
/// - Both objects → merge keys recursively (patch wins on primitive/array conflict).
/// - Patch is object but base is not → return patch.
/// - Patch is array or primitive → patch overwrites base entirely.
/// - Base is `Value::Null` and patch is anything → return patch.
pub fn deep_merge(base: Value, patch: Value) -> Value {
    match (base, patch) {
        (Value::Object(mut base_map), Value::Object(patch_map)) => {
            for (key, patch_val) in patch_map {
                let merged = match base_map.remove(&key) {
                    Some(base_val) => deep_merge(base_val, patch_val),
                    None => patch_val,
                };
                base_map.insert(key, merged);
            }
            Value::Object(base_map)
        }
        // Patch wins in all other combinations
        (_, patch) => patch,
    }
}

// ──────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn merge_empty_patch_returns_base() {
        let base = json!({"name": "Alice", "tone": "calm"});
        let patch = json!({});
        let result = deep_merge(base.clone(), patch);
        assert_eq!(result, base);
    }

    #[test]
    fn merge_disjoint_keys() {
        let base = json!({"name": "Alice"});
        let patch = json!({"tone": "stoic"});
        let result = deep_merge(base, patch);
        assert_eq!(result, json!({"name": "Alice", "tone": "stoic"}));
    }

    #[test]
    fn merge_nested_objects() {
        let base = json!({"schedule": {"morning": 8, "timezone": "UTC"}});
        let patch = json!({"schedule": {"evening": 21}});
        let result = deep_merge(base, patch);
        assert_eq!(
            result,
            json!({"schedule": {"morning": 8, "timezone": "UTC", "evening": 21}})
        );
    }

    #[test]
    fn patch_array_overwrites_base_array() {
        let base = json!({"interests": ["rust", "ml"]});
        let patch = json!({"interests": ["music"]});
        let result = deep_merge(base, patch);
        assert_eq!(result, json!({"interests": ["music"]}));
    }

    #[test]
    fn patch_primitive_overwrites_base() {
        let base = json!({"score": 1});
        let patch = json!({"score": 42});
        let result = deep_merge(base, patch);
        assert_eq!(result, json!({"score": 42}));
    }
}
