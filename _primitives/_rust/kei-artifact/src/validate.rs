//! Minimal JSON Schema validator — strict subset of draft 2020-12.
//!
//! Keyword support (chosen for the 5 built-in schemas):
//!   - `type` (object, array, string, integer, number, boolean, null)
//!   - `required` (array of property names)
//!   - `properties` (object → sub-schema)
//!   - `additionalProperties` (bool; default true, we set false on ours)
//!   - `enum` (array of allowed scalar values)
//!   - `items` (sub-schema for array elements)
//!   - `minLength` (integer) / `minItems` (integer) / `minimum` (number)
//!
//! Intentionally NOT supported: $ref, oneOf/anyOf/allOf, patternProperties,
//! format validation, conditional schemas. The 5 built-in schemas are written
//! to avoid needing those — keeps the validator under 200 LOC and removes the
//! 40+ transitive-dep `jsonschema` crate.
//!
//! RULE 0.4 note: draft 2020-12 is the current JSON Schema standard
//! [VERIFIED: https://json-schema.org/draft/2020-12 — spec page].
//! This implementation is a strict subset — any schema author sticking to
//! the keywords above gets draft-2020-12-compatible semantics.

use serde_json::Value;

/// Top-level entry. Returns `Ok(())` on pass, `Err(msg)` with a path-style
/// location on first failure.
pub fn validate_content(schema: &Value, content: &Value) -> Result<(), String> {
    check(schema, content, "$")
}

/// Keywords the minimal validator knows about. Used by `warn_unsupported_keywords`
/// to flag — but not reject — schemas that lean on unsupported features (so an
/// operator writing human-readable docs in a schema still sees them stored,
/// while being warned they do not actually enforce anything).
const KNOWN_KEYWORDS: &[&str] = &[
    "$schema",
    "$id",
    "title",
    "description",
    "type",
    "required",
    "properties",
    "additionalProperties",
    "enum",
    "items",
    "minLength",
    "minItems",
    "minimum",
];

/// Emit a stderr warning for each schema keyword this validator does not
/// enforce. Non-fatal: the schema is still accepted and stored verbatim —
/// operators can keep `pattern` / `format` / `oneOf` etc. as human-readable
/// hints without expecting runtime validation of them.
///
/// Walks the schema recursively so a nested `items` / `properties` sub-schema
/// with an unsupported keyword is caught too.
pub fn warn_unsupported_keywords(schema: &Value) {
    fn walk(v: &Value, path: &str) {
        if let Value::Object(map) = v {
            for (k, sub) in map {
                if !KNOWN_KEYWORDS.contains(&k.as_str()) {
                    eprintln!(
                        "[kei-artifact] schema warning: unsupported keyword '{k}' at {path} — \
stored but not enforced by the minimal validator (see validate.rs KNOWN_KEYWORDS)"
                    );
                }
                walk(sub, &format!("{path}.{k}"));
            }
        } else if let Value::Array(arr) = v {
            for (i, el) in arr.iter().enumerate() {
                walk(el, &format!("{path}[{i}]"));
            }
        }
    }
    walk(schema, "$");
}

fn check(schema: &Value, value: &Value, path: &str) -> Result<(), String> {
    if let Some(t) = schema.get("type") {
        check_type(t, value, path)?;
    }
    if let Some(e) = schema.get("enum") {
        check_enum(e, value, path)?;
    }
    match value {
        Value::Object(_) => check_object(schema, value, path)?,
        Value::Array(_) => check_array(schema, value, path)?,
        Value::String(s) => check_min_length(schema, s, path)?,
        Value::Number(n) => check_minimum(schema, n, path)?,
        _ => {}
    }
    Ok(())
}

fn check_type(schema_type: &Value, value: &Value, path: &str) -> Result<(), String> {
    let want = schema_type
        .as_str()
        .ok_or_else(|| format!("{path}: schema 'type' must be string"))?;
    let ok = match (want, value) {
        ("object", Value::Object(_)) => true,
        ("array", Value::Array(_)) => true,
        ("string", Value::String(_)) => true,
        ("boolean", Value::Bool(_)) => true,
        ("null", Value::Null) => true,
        ("integer", Value::Number(n)) => n.is_i64() || n.is_u64(),
        ("number", Value::Number(_)) => true,
        _ => false,
    };
    if !ok {
        return Err(format!(
            "{path}: expected type '{want}', got {}",
            type_of(value)
        ));
    }
    Ok(())
}

fn type_of(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn check_enum(enum_schema: &Value, value: &Value, path: &str) -> Result<(), String> {
    let allowed = enum_schema
        .as_array()
        .ok_or_else(|| format!("{path}: 'enum' must be array"))?;
    if !allowed.iter().any(|a| a == value) {
        return Err(format!("{path}: value {value} not in enum"));
    }
    Ok(())
}

fn check_object(schema: &Value, value: &Value, path: &str) -> Result<(), String> {
    let obj = value.as_object().unwrap();
    if let Some(required) = schema.get("required").and_then(|v| v.as_array()) {
        for r in required {
            if let Some(name) = r.as_str() {
                if !obj.contains_key(name) {
                    return Err(format!("{path}: missing required property '{name}'"));
                }
            }
        }
    }
    let props = schema.get("properties").and_then(|v| v.as_object());
    let additional = schema
        .get("additionalProperties")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    for (k, v) in obj {
        match props.and_then(|p| p.get(k)) {
            Some(sub) => check(sub, v, &format!("{path}.{k}"))?,
            None if !additional => {
                return Err(format!("{path}: unexpected property '{k}'"));
            }
            None => {}
        }
    }
    Ok(())
}

fn check_array(schema: &Value, value: &Value, path: &str) -> Result<(), String> {
    let arr = value.as_array().unwrap();
    if let Some(min) = schema.get("minItems").and_then(|v| v.as_u64()) {
        if (arr.len() as u64) < min {
            return Err(format!("{path}: array has {} items, min {min}", arr.len()));
        }
    }
    if let Some(items) = schema.get("items") {
        for (i, el) in arr.iter().enumerate() {
            check(items, el, &format!("{path}[{i}]"))?;
        }
    }
    Ok(())
}

fn check_min_length(schema: &Value, s: &str, path: &str) -> Result<(), String> {
    if let Some(min) = schema.get("minLength").and_then(|v| v.as_u64()) {
        if (s.chars().count() as u64) < min {
            return Err(format!("{path}: string shorter than minLength {min}"));
        }
    }
    Ok(())
}

fn check_minimum(schema: &Value, n: &serde_json::Number, path: &str) -> Result<(), String> {
    if let Some(min) = schema.get("minimum").and_then(|v| v.as_f64()) {
        if let Some(v) = n.as_f64() {
            if v < min {
                return Err(format!("{path}: number {v} below minimum {min}"));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn type_mismatch_rejected() {
        let schema = json!({"type": "string"});
        let err = validate_content(&schema, &json!(42)).unwrap_err();
        assert!(err.contains("expected type 'string'"), "got: {err}");
    }

    #[test]
    fn missing_required_rejected() {
        let schema = json!({
            "type": "object",
            "required": ["goal"],
            "properties": {"goal": {"type": "string"}}
        });
        let err = validate_content(&schema, &json!({})).unwrap_err();
        assert!(err.contains("goal"));
    }

    #[test]
    fn unknown_additional_rejected() {
        let schema = json!({
            "type": "object",
            "additionalProperties": false,
            "properties": {"a": {"type": "string"}}
        });
        let err = validate_content(&schema, &json!({"a":"x","b":"y"})).unwrap_err();
        assert!(err.contains("unexpected property 'b'"));
    }

    #[test]
    fn enum_and_array_items_enforced() {
        let schema = json!({
            "type": "array",
            "items": {"type": "string", "enum": ["add", "mod", "del"]}
        });
        assert!(validate_content(&schema, &json!(["add", "mod"])).is_ok());
        let err = validate_content(&schema, &json!(["nope"])).unwrap_err();
        assert!(err.contains("enum"));
    }

    #[test]
    fn warn_unsupported_keywords_does_not_panic_or_mutate() {
        // Smoke test — the warn function prints to stderr but returns unit and
        // never mutates the schema. We cannot portably capture stderr without
        // a gag-style helper, so we just assert execution is stable and the
        // schema is still usable by `validate_content` afterwards.
        let schema = json!({
            "type": "object",
            "required": ["k"],
            "properties": {
                "k": {"type": "string", "pattern": "^[a-z]+$", "format": "email"}
            },
            "oneOf": [{"type": "object"}],
            "patternProperties": {"^x_": {"type": "string"}}
        });
        warn_unsupported_keywords(&schema);
        // Validator is still callable and still enforces the supported subset.
        assert!(validate_content(&schema, &json!({"k": "hi"})).is_ok());
        let err = validate_content(&schema, &json!({})).unwrap_err();
        assert!(err.contains("k"));
    }
}
