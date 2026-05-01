//! Apply an RFC 6902 patch (add/remove/replace subset) to a JSON document.
//!
//! Root-path `""` replace swaps the entire document. Array `add` with
//! index == len (or `-`) appends; in-range index inserts and shifts.
//! Array `remove` deletes and shifts. Object ops insert/delete/replace keys.

use crate::apply_error::ApplyError;
use crate::op::{Op, Patch};
use crate::path::parse_pointer;
use serde_json::Value;

/// Apply `patch` to `root` and return a new `Value`. `root` is cloned;
/// the original is untouched. Operations are applied in order.
pub fn apply(root: &Value, patch: &Patch) -> Result<Value, ApplyError> {
    let mut doc = root.clone();
    for op in &patch.0 {
        apply_one(&mut doc, op)?;
    }
    Ok(doc)
}

fn apply_one(doc: &mut Value, op: &Op) -> Result<(), ApplyError> {
    match op {
        Op::Add { path, value } => apply_add(doc, path, value.clone()),
        Op::Remove { path } => apply_remove(doc, path).map(|_| ()),
        Op::Replace { path, value } => apply_replace(doc, path, value.clone()),
    }
}

fn apply_add(doc: &mut Value, path: &str, value: Value) -> Result<(), ApplyError> {
    let segs = parse_pointer(path).ok_or_else(|| ApplyError::InvalidPointer(path.into()))?;
    if segs.is_empty() {
        return Err(ApplyError::CannotAddToRoot);
    }
    let (parent_segs, last) = segs.split_at(segs.len() - 1);
    let parent = navigate_mut(doc, parent_segs, path)?;
    insert_into(parent, &last[0], value, path)
}

fn apply_remove(doc: &mut Value, path: &str) -> Result<Value, ApplyError> {
    let segs = parse_pointer(path).ok_or_else(|| ApplyError::InvalidPointer(path.into()))?;
    if segs.is_empty() {
        return Err(ApplyError::CannotRemoveRoot);
    }
    let (parent_segs, last) = segs.split_at(segs.len() - 1);
    let parent = navigate_mut(doc, parent_segs, path)?;
    remove_from(parent, &last[0], path)
}

fn apply_replace(doc: &mut Value, path: &str, value: Value) -> Result<(), ApplyError> {
    let segs = parse_pointer(path).ok_or_else(|| ApplyError::InvalidPointer(path.into()))?;
    if segs.is_empty() {
        *doc = value;
        return Ok(());
    }
    let (parent_segs, last) = segs.split_at(segs.len() - 1);
    let parent = navigate_mut(doc, parent_segs, path)?;
    replace_in(parent, &last[0], value, path)
}

fn navigate_mut<'a>(
    mut cur: &'a mut Value,
    segs: &[String],
    full_path: &str,
) -> Result<&'a mut Value, ApplyError> {
    for seg in segs {
        cur = step_into(cur, seg, full_path)?;
    }
    Ok(cur)
}

fn step_into<'a>(
    cur: &'a mut Value,
    seg: &str,
    full_path: &str,
) -> Result<&'a mut Value, ApplyError> {
    match cur {
        Value::Object(map) => map
            .get_mut(seg)
            .ok_or_else(|| ApplyError::MissingParent(full_path.into())),
        Value::Array(arr) => {
            let idx = parse_index(seg, full_path)?;
            arr.get_mut(idx)
                .ok_or_else(|| ApplyError::MissingParent(full_path.into()))
        }
        _ => Err(ApplyError::TypeMismatch {
            path: full_path.into(),
            expected: "object or array",
        }),
    }
}

fn insert_into(parent: &mut Value, key: &str, value: Value, full: &str) -> Result<(), ApplyError> {
    match parent {
        Value::Object(map) => {
            map.insert(key.to_string(), value);
            Ok(())
        }
        Value::Array(arr) => {
            let idx = parse_array_insert_index(key, arr.len(), full)?;
            arr.insert(idx, value);
            Ok(())
        }
        _ => Err(ApplyError::TypeMismatch { path: full.into(), expected: "object or array" }),
    }
}

fn remove_from(parent: &mut Value, key: &str, full: &str) -> Result<Value, ApplyError> {
    match parent {
        Value::Object(map) => map
            .remove(key)
            .ok_or_else(|| ApplyError::MissingTarget(full.into())),
        Value::Array(arr) => {
            let idx = parse_index(key, full)?;
            if idx >= arr.len() {
                return Err(ApplyError::IndexOutOfBounds {
                    path: full.into(),
                    index: idx,
                    len: arr.len(),
                });
            }
            Ok(arr.remove(idx))
        }
        _ => Err(ApplyError::TypeMismatch { path: full.into(), expected: "object or array" }),
    }
}

fn replace_in(parent: &mut Value, key: &str, value: Value, full: &str) -> Result<(), ApplyError> {
    match parent {
        Value::Object(map) => {
            if !map.contains_key(key) {
                return Err(ApplyError::MissingTarget(full.into()));
            }
            map.insert(key.to_string(), value);
            Ok(())
        }
        Value::Array(arr) => {
            let idx = parse_index(key, full)?;
            if idx >= arr.len() {
                return Err(ApplyError::IndexOutOfBounds {
                    path: full.into(),
                    index: idx,
                    len: arr.len(),
                });
            }
            arr[idx] = value;
            Ok(())
        }
        _ => Err(ApplyError::TypeMismatch { path: full.into(), expected: "object or array" }),
    }
}

fn parse_index(seg: &str, full: &str) -> Result<usize, ApplyError> {
    seg.parse::<usize>()
        .map_err(|_| ApplyError::InvalidPointer(full.into()))
}

fn parse_array_insert_index(seg: &str, len: usize, full: &str) -> Result<usize, ApplyError> {
    if seg == "-" {
        return Ok(len);
    }
    let idx = seg
        .parse::<usize>()
        .map_err(|_| ApplyError::InvalidPointer(full.into()))?;
    if idx > len {
        return Err(ApplyError::IndexOutOfBounds { path: full.into(), index: idx, len });
    }
    Ok(idx)
}
