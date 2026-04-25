//! `$step-id.path.to.field` resolver.
//!
//! A reference is a string of the form `$<step-id>[.<segment>]*` where a
//! segment is either a dotted key (`foo`) or a numeric index (`0`). The
//! leading segment after the step id is matched against the step's
//! returned `{"atom":..., "result":...}` envelope the same way a caller
//! would type it — including an explicit `result.` prefix.
//!
//! A string that is `$step.path.to.field` verbatim is substituted by the
//! referenced value (which may itself be a JSON object, array, number).
//! Strings that merely contain a `$` somewhere else are left alone.

use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum ResolveError {
    #[error("unknown step in reference `{0}`")]
    UnknownStep(String),
    #[error("path `{0}` not found in result of step `{1}`")]
    MissingPath(String, String),
    #[error("path segment `{0}` expects an object/array but got `{1}`")]
    WrongKind(String, String),
}

/// Walk `input` recursively, replacing every `$step.path` string with the
/// resolved JSON value from `previous`.
pub fn resolve_input(
    input: &Value,
    previous: &HashMap<String, Value>,
) -> Result<Value, ResolveError> {
    match input {
        Value::String(s) => resolve_string(s, previous),
        Value::Array(items) => resolve_array(items, previous),
        Value::Object(map) => resolve_object(map, previous),
        other => Ok(other.clone()),
    }
}

fn resolve_string(s: &str, previous: &HashMap<String, Value>) -> Result<Value, ResolveError> {
    if let Some(stripped) = s.strip_prefix('$') {
        return lookup_reference(stripped, previous);
    }
    Ok(Value::String(s.to_string()))
}

fn resolve_array(
    items: &[Value],
    previous: &HashMap<String, Value>,
) -> Result<Value, ResolveError> {
    let mut out: Vec<Value> = Vec::with_capacity(items.len());
    for v in items {
        out.push(resolve_input(v, previous)?);
    }
    Ok(Value::Array(out))
}

fn resolve_object(
    map: &Map<String, Value>,
    previous: &HashMap<String, Value>,
) -> Result<Value, ResolveError> {
    let mut out = Map::with_capacity(map.len());
    for (k, v) in map {
        out.insert(k.clone(), resolve_input(v, previous)?);
    }
    Ok(Value::Object(out))
}

fn lookup_reference(
    stripped: &str,
    previous: &HashMap<String, Value>,
) -> Result<Value, ResolveError> {
    let (step_id, remainder) = split_head(stripped);
    let envelope = previous
        .get(step_id)
        .ok_or_else(|| ResolveError::UnknownStep(step_id.to_string()))?;
    walk_path(envelope, remainder, step_id)
}

fn split_head(s: &str) -> (&str, &str) {
    match s.find('.') {
        Some(i) => (&s[..i], &s[i + 1..]),
        None => (s, ""),
    }
}

fn walk_path(root: &Value, path: &str, step_id: &str) -> Result<Value, ResolveError> {
    if path.is_empty() {
        return Ok(root.clone());
    }
    let mut current = root;
    for seg in path.split('.') {
        current = descend(current, seg, path, step_id)?;
    }
    Ok(current.clone())
}

fn descend<'a>(
    current: &'a Value,
    seg: &str,
    path: &str,
    step_id: &str,
) -> Result<&'a Value, ResolveError> {
    match current {
        Value::Object(m) => m
            .get(seg)
            .ok_or_else(|| ResolveError::MissingPath(path.into(), step_id.into())),
        Value::Array(a) => {
            let idx: usize = seg
                .parse()
                .map_err(|_| ResolveError::MissingPath(path.into(), step_id.into()))?;
            a.get(idx)
                .ok_or_else(|| ResolveError::MissingPath(path.into(), step_id.into()))
        }
        other => Err(ResolveError::WrongKind(seg.into(), kind_of(other).into())),
    }
}

fn kind_of(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}
