//! Shared input-type validator for create / update.
//!
//! Strict typed validation: integer fields require JSON numbers that
//! fit i64; text fields require JSON strings; real fields require JSON
//! numbers convertible to f64. Wrong-type input returns
//! `VerbError::InvalidType` instead of silent coercion to `0` / `""`.
//!
//! TEXT size cap: any text value longer than `MAX_TEXT_BYTES` is
//! rejected to prevent OOM from hostile input. Per-field override is
//! planned (TODO B5: add `max_bytes: Option<usize>` to `FieldDef`).

use crate::error::VerbError;
use crate::schema::{FieldDef, FieldKind};
use rusqlite::types::Value as SqlValue;
use serde_json::Value;

/// Default TEXT size cap — 64 KiB. Enforced for every TextNotNull /
/// Text / TextDefault field unless overridden per-field (TODO).
pub const MAX_TEXT_BYTES: usize = 64 * 1024;

/// Convert an input JSON value to a typed `SqlValue` for `f`.
///
/// Errors if the JSON kind does not match the field kind, or if a
/// text value exceeds `MAX_TEXT_BYTES`.
pub fn coerce(f: &FieldDef, raw: &Value) -> Result<SqlValue, VerbError> {
    match f.kind {
        FieldKind::IntegerPk => Err(VerbError::InvalidInput(format!(
            "field `{}` is PK and cannot be set directly",
            f.name
        ))),
        FieldKind::TextPk => coerce_text(f, raw),
        FieldKind::IntegerNotNull
        | FieldKind::Integer
        | FieldKind::TimestampCreated
        | FieldKind::TimestampUpdated => coerce_int(f, raw),
        FieldKind::TextNotNull
        | FieldKind::Text
        | FieldKind::TextDefault
        | FieldKind::TextArchiveEnum => coerce_text(f, raw),
        FieldKind::Real | FieldKind::RealDefault => coerce_real(f, raw),
    }
}

fn coerce_int(f: &FieldDef, raw: &Value) -> Result<SqlValue, VerbError> {
    match raw {
        Value::Null => Ok(SqlValue::Integer(0)),
        Value::Number(n) => n.as_i64().map(SqlValue::Integer).ok_or_else(|| {
            type_err(f, "integer (i64)", &format!("number {} out of range", n))
        }),
        other => Err(type_err(f, "integer", kind_name(other))),
    }
}

fn coerce_text(f: &FieldDef, raw: &Value) -> Result<SqlValue, VerbError> {
    let s = match raw {
        Value::Null => String::new(),
        Value::String(s) => s.clone(),
        other => return Err(type_err(f, "string", kind_name(other))),
    };
    if s.len() > MAX_TEXT_BYTES {
        return Err(type_err(
            f,
            &format!("string ≤ {} bytes", MAX_TEXT_BYTES),
            &format!("{} bytes", s.len()),
        ));
    }
    Ok(SqlValue::Text(s))
}

fn coerce_real(f: &FieldDef, raw: &Value) -> Result<SqlValue, VerbError> {
    match raw {
        Value::Null => Ok(SqlValue::Real(f.real_default.unwrap_or(0.0))),
        Value::Number(n) => n
            .as_f64()
            .map(SqlValue::Real)
            .ok_or_else(|| type_err(f, "real (f64)", &format!("number {} out of range", n))),
        other => Err(type_err(f, "real", kind_name(other))),
    }
}

fn type_err(f: &FieldDef, expected: &str, got: &str) -> VerbError {
    VerbError::InvalidType {
        field: f.name.to_string(),
        expected: expected.to_string(),
        got: got.to_string(),
    }
}

fn kind_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

/// Reject text values that exceed the configured cap. Used by create
/// for fields that flow through the old "default on missing" path
/// (where coerce is not invoked for missing keys).
pub fn check_text_len(f: &FieldDef, s: &str) -> Result<(), VerbError> {
    if s.len() > MAX_TEXT_BYTES {
        return Err(type_err(
            f,
            &format!("string ≤ {} bytes", MAX_TEXT_BYTES),
            &format!("{} bytes", s.len()),
        ));
    }
    Ok(())
}
