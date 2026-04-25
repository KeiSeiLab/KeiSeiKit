//! Per-kind value-for-insert helpers split out of `create.rs` to keep
//! that file under the Constructor-Pattern 200-LOC cap. Each function
//! handles one FieldKind's default / coerce logic.

use crate::error::VerbError;
use crate::schema::{FieldDef, FieldKind};
use crate::verbs::validate;
use rusqlite::types::Value as SqlValue;
use serde_json::Value;

pub fn field_value(
    f: &FieldDef,
    input: &serde_json::Map<String, Value>,
    now: i64,
) -> Result<SqlValue, VerbError> {
    match f.kind {
        FieldKind::TimestampCreated | FieldKind::TimestampUpdated => Ok(timestamp(input, f, now)),
        FieldKind::TextDefault => text_default(f, input),
        FieldKind::TextArchiveEnum => archive_enum(f, input),
        FieldKind::RealDefault => real_default(f, input),
        FieldKind::IntegerPk | FieldKind::TextPk => Ok(SqlValue::Null),
        _ => match input.get(f.name) {
            Some(raw) => validate::coerce(f, raw),
            None => Ok(default_for_kind(f)),
        },
    }
}

fn timestamp(
    input: &serde_json::Map<String, Value>,
    f: &FieldDef,
    now: i64,
) -> SqlValue {
    match input.get(f.name).and_then(|v| v.as_i64()) {
        Some(ts) if ts > 0 => SqlValue::Integer(ts),
        _ => SqlValue::Integer(now),
    }
}

fn text_default(
    f: &FieldDef,
    input: &serde_json::Map<String, Value>,
) -> Result<SqlValue, VerbError> {
    match input.get(f.name) {
        Some(raw) => coerce_with_text_fallback(f, raw),
        None => text_literal_default(f),
    }
}

fn coerce_with_text_fallback(
    f: &FieldDef,
    raw: &Value,
) -> Result<SqlValue, VerbError> {
    let coerced = validate::coerce(f, raw)?;
    if let SqlValue::Text(ref s) = coerced {
        if s.is_empty() {
            return text_literal_default(f);
        }
    }
    Ok(coerced)
}

fn text_literal_default(f: &FieldDef) -> Result<SqlValue, VerbError> {
    let d = f.default.unwrap_or("");
    validate::check_text_len(f, d)?;
    Ok(SqlValue::Text(d.to_string()))
}

fn archive_enum(
    f: &FieldDef,
    input: &serde_json::Map<String, Value>,
) -> Result<SqlValue, VerbError> {
    let (active, _archived) = f.archive_enum.unwrap_or(("active", "archived"));
    match input.get(f.name) {
        Some(raw) => {
            let coerced = validate::coerce(f, raw)?;
            if let SqlValue::Text(ref s) = coerced {
                if s.is_empty() {
                    return Ok(SqlValue::Text(active.to_string()));
                }
            }
            Ok(coerced)
        }
        None => Ok(SqlValue::Text(active.to_string())),
    }
}

fn real_default(
    f: &FieldDef,
    input: &serde_json::Map<String, Value>,
) -> Result<SqlValue, VerbError> {
    match input.get(f.name) {
        Some(raw) => validate::coerce(f, raw),
        None => Ok(SqlValue::Real(f.real_default.unwrap_or(0.0))),
    }
}

fn default_for_kind(f: &FieldDef) -> SqlValue {
    match f.kind {
        FieldKind::IntegerNotNull | FieldKind::Integer => SqlValue::Integer(0),
        FieldKind::TextNotNull | FieldKind::Text | FieldKind::TextDefault => {
            SqlValue::Text(String::new())
        }
        FieldKind::Real | FieldKind::RealDefault => {
            SqlValue::Real(f.real_default.unwrap_or(0.0))
        }
        _ => SqlValue::Null,
    }
}
