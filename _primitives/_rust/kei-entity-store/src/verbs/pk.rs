//! Shared primary-key extraction helper — bridges IntegerPk / TextPk
//! schemas so each verb can accept `{"id": <int>}` or `{"id": "<str>"}`
//! without duplicating the dispatch logic.

use crate::error::VerbError;
use crate::schema::{EntitySchema, FieldKind};
use rusqlite::types::Value as SqlValue;
use serde_json::Value;

/// A primary-key value bound for SQLite. Text PKs carry the raw string;
/// integer PKs carry an i64.
#[derive(Debug, Clone)]
pub enum PkValue {
    Integer(i64),
    Text(String),
}

impl PkValue {
    pub fn as_sql(&self) -> SqlValue {
        match self {
            PkValue::Integer(n) => SqlValue::Integer(*n),
            PkValue::Text(s) => SqlValue::Text(s.clone()),
        }
    }

    pub fn as_json(&self) -> Value {
        match self {
            PkValue::Integer(n) => Value::from(*n),
            PkValue::Text(s) => Value::from(s.clone()),
        }
    }

    /// String form — used to render `NotFound` errors uniformly.
    pub fn as_string(&self) -> String {
        match self {
            PkValue::Integer(n) => n.to_string(),
            PkValue::Text(s) => s.clone(),
        }
    }
}

/// Extract the primary-key value from a verb input JSON object. `verb`
/// appears in the error message; caller passes its own name.
pub fn extract(
    schema: &EntitySchema,
    input: &Value,
    verb: &str,
) -> Result<PkValue, VerbError> {
    let raw = input.get("id").ok_or_else(|| {
        VerbError::InvalidInput(format!("{verb}: missing `id`"))
    })?;
    match schema.pk().kind {
        FieldKind::IntegerPk => raw
            .as_i64()
            .map(PkValue::Integer)
            .ok_or_else(|| VerbError::InvalidInput(format!("{verb}: `id` must be integer"))),
        FieldKind::TextPk => raw
            .as_str()
            .map(|s| PkValue::Text(s.to_string()))
            .ok_or_else(|| VerbError::InvalidInput(format!("{verb}: `id` must be string"))),
        other => Err(VerbError::InvalidInput(format!(
            "{verb}: schema `{}` PK kind {:?} is not a primary key",
            schema.name, other
        ))),
    }
}

/// The PK column name.
pub fn pk_name(schema: &EntitySchema) -> &'static str {
    schema.pk().name
}
