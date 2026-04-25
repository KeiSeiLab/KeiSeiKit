//! Structural JSON diff.
//!
//! Algorithm:
//! * Both objects → recurse per-key across the union (add/remove/recurse).
//! * Both arrays → index-based (recurse on overlap; add-tail or remove-tail
//!   for length delta). NOT LCS — simpler, idempotent enough for drift
//!   detection, and cheap (O(n)).
//! * Otherwise, if values differ → `replace`.
//! * Equal values → no-op.
//!
//! Rationale for skipping LCS: the consumer (kei-replay drift check) cares
//! about "does anything differ" and "at which logical coordinate", not
//! minimum-edit-distance. Index-based gives stable paths; LCS would produce
//! a smaller patch on shuffled arrays but with ambiguous paths.

use crate::op::{Op, Patch};
use crate::path::PathBuf;
use serde_json::Value;

/// Compute an RFC 6902 subset patch that transforms `old` into `new`.
/// Invariant: `apply(old, diff(old, new)) == new`.
pub fn diff(old: &Value, new: &Value) -> Patch {
    let mut patch = Patch::new();
    let mut path = PathBuf::new();
    diff_recurse(old, new, &mut path, &mut patch);
    patch
}

fn diff_recurse(old: &Value, new: &Value, path: &mut PathBuf, patch: &mut Patch) {
    if old == new {
        return;
    }
    match (old, new) {
        (Value::Object(a), Value::Object(b)) => diff_objects(a, b, path, patch),
        (Value::Array(a), Value::Array(b)) => diff_arrays(a, b, path, patch),
        _ => patch.push(Op::Replace {
            path: path.as_string(),
            value: new.clone(),
        }),
    }
}

fn diff_objects(
    a: &serde_json::Map<String, Value>,
    b: &serde_json::Map<String, Value>,
    path: &mut PathBuf,
    patch: &mut Patch,
) {
    // Removals: keys in `a` but not `b`. Emit in stable key order for determinism.
    for key in a.keys() {
        if !b.contains_key(key) {
            path.push_key(key);
            patch.push(Op::Remove { path: path.as_string() });
            path.pop();
        }
    }
    // Additions + recursion: iterate `b` in its key order.
    for (key, b_val) in b {
        path.push_key(key);
        match a.get(key) {
            None => patch.push(Op::Add {
                path: path.as_string(),
                value: b_val.clone(),
            }),
            Some(a_val) => diff_recurse(a_val, b_val, path, patch),
        }
        path.pop();
    }
}

fn diff_arrays(a: &[Value], b: &[Value], path: &mut PathBuf, patch: &mut Patch) {
    let common = a.len().min(b.len());
    // Recurse on overlapping prefix.
    for i in 0..common {
        path.push_index(i);
        diff_recurse(&a[i], &b[i], path, patch);
        path.pop();
    }
    if a.len() > b.len() {
        emit_array_truncate(a.len(), b.len(), path, patch);
    } else if b.len() > a.len() {
        emit_array_append(b, a.len(), path, patch);
    }
}

// Remove trailing indices highest-first so surviving indices don't shift.
fn emit_array_truncate(old_len: usize, new_len: usize, path: &mut PathBuf, patch: &mut Patch) {
    for i in (new_len..old_len).rev() {
        path.push_index(i);
        patch.push(Op::Remove { path: path.as_string() });
        path.pop();
    }
}

// Append new tail. Emit in ascending order so each add references the
// just-created length as the next insertion point.
fn emit_array_append(b: &[Value], old_len: usize, path: &mut PathBuf, patch: &mut Patch) {
    for i in old_len..b.len() {
        path.push_index(i);
        patch.push(Op::Add {
            path: path.as_string(),
            value: b[i].clone(),
        });
        path.pop();
    }
}
