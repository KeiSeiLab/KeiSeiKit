//! `link` verb — INSERT edge into `<edge_table>` (idempotent via
//! INSERT OR IGNORE). Caller is responsible for higher-level semantic
//! checks (cycle detection, self-loop) — those live in the sibling
//! crate (e.g. kei-task::deps).
//!
//! Dispatches on `schema.edge_key_kind`:
//! - `IntegerPair`             — input `{from: i64, to: i64, edge_type?}`
//! - `TextPair`                — input `{from: str, to: str, edge_type?}`
//! - `TextPairWithMetadata {…}` — same text keys plus optional
//!   `weight: f64` input; `edge_id` / `created_at` are engine-managed
//!   and NEVER taken from the caller.

use crate::error::VerbError;
use crate::schema::{EdgeKeyKind, EntitySchema, FieldKind};
use rusqlite::{types::Value as SqlValue, Connection};
use serde_json::{json, Value};

pub fn run(
    conn: &Connection,
    schema: &EntitySchema,
    input: Value,
) -> Result<Value, VerbError> {
    if !schema.verb_enabled("link") {
        return Err(VerbError::VerbDisabled {
            verb: "link".into(),
            schema: schema.name.into(),
        });
    }
    let edge = schema.edge_table.ok_or_else(|| {
        VerbError::InvalidInput(format!(
            "link: schema {} has no edge_table configured",
            schema.name
        ))
    })?;
    let edge_type = input
        .get("edge_type")
        .and_then(|v| v.as_str())
        .unwrap_or("links")
        .to_string();

    match schema.edge_key_kind {
        EdgeKeyKind::IntegerPair => insert_integer(conn, edge, &input, &edge_type),
        EdgeKeyKind::TextPair => insert_text(conn, edge, &input, &edge_type),
        EdgeKeyKind::TextPairWithMetadata {
            from_col,
            to_col,
            has_id: _,
            has_weight,
            has_created_at,
            extra_columns,
        } => insert_text_meta(
            conn,
            edge,
            &input,
            &edge_type,
            from_col,
            to_col,
            has_weight,
            has_created_at,
            extra_columns,
        ),
    }
}

fn insert_integer(
    conn: &Connection,
    edge: &str,
    input: &Value,
    edge_type: &str,
) -> Result<Value, VerbError> {
    let from = input
        .get("from")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| VerbError::InvalidInput("link: missing `from` integer".into()))?;
    let to = input
        .get("to")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| VerbError::InvalidInput("link: missing `to` integer".into()))?;
    conn.execute(
        &format!(
            "INSERT OR IGNORE INTO {edge} (from_id, to_id, edge_type) VALUES (?1, ?2, ?3)"
        ),
        rusqlite::params![from, to, edge_type],
    )?;
    Ok(json!({ "ok": true }))
}

fn insert_text(
    conn: &Connection,
    edge: &str,
    input: &Value,
    edge_type: &str,
) -> Result<Value, VerbError> {
    let (from, to) = extract_text_pair(input)?;
    conn.execute(
        &format!(
            "INSERT OR IGNORE INTO {edge} (src_path, dst_path, edge_type) VALUES (?1, ?2, ?3)"
        ),
        rusqlite::params![from, to, edge_type],
    )?;
    Ok(json!({ "ok": true }))
}

#[allow(clippy::too_many_arguments)]
fn insert_text_meta(
    conn: &Connection,
    edge: &str,
    input: &Value,
    edge_type: &str,
    from_col: &str,
    to_col: &str,
    has_weight: bool,
    has_created_at: bool,
    extras: &[(&str, FieldKind)],
) -> Result<Value, VerbError> {
    let (from, to) = extract_text_pair(input)?;
    let mut cols: Vec<String> = vec![from_col.into(), to_col.into(), "edge_type".into()];
    let mut values: Vec<SqlValue> = vec![
        SqlValue::Text(from),
        SqlValue::Text(to),
        SqlValue::Text(edge_type.to_string()),
    ];
    if has_weight {
        let weight = input.get("weight").and_then(|v| v.as_f64()).unwrap_or(1.0);
        cols.push("weight".into());
        values.push(SqlValue::Real(weight));
    }
    push_extras(&mut cols, &mut values, input, extras);
    if has_created_at {
        cols.push("created_at".into());
        values.push(SqlValue::Integer(chrono::Utc::now().timestamp()));
    }
    exec_insert(conn, edge, &cols, &values)
}

fn push_extras(
    cols: &mut Vec<String>,
    values: &mut Vec<SqlValue>,
    input: &Value,
    extras: &[(&str, FieldKind)],
) {
    for (name, kind) in extras {
        if let Some(v) = input.get(*name) {
            cols.push((*name).into());
            values.push(json_to_sql(v, *kind));
        }
    }
}

fn json_to_sql(v: &Value, kind: FieldKind) -> SqlValue {
    match kind {
        FieldKind::Text | FieldKind::TextNotNull => {
            SqlValue::Text(v.as_str().unwrap_or("").to_string())
        }
        FieldKind::Integer | FieldKind::IntegerNotNull => {
            SqlValue::Integer(v.as_i64().unwrap_or(0))
        }
        FieldKind::Real => SqlValue::Real(v.as_f64().unwrap_or(0.0)),
        _ => SqlValue::Null,
    }
}

fn exec_insert(
    conn: &Connection,
    edge: &str,
    cols: &[String],
    values: &[SqlValue],
) -> Result<Value, VerbError> {
    let placeholders: Vec<String> = (1..=cols.len()).map(|i| format!("?{i}")).collect();
    let sql = format!(
        "INSERT OR IGNORE INTO {edge} ({}) VALUES ({})",
        cols.join(","),
        placeholders.join(",")
    );
    let params: Vec<&dyn rusqlite::ToSql> =
        values.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
    conn.execute(&sql, params.as_slice())?;
    Ok(json!({ "ok": true }))
}

fn extract_text_pair(input: &Value) -> Result<(String, String), VerbError> {
    let from = input
        .get("from")
        .and_then(|v| v.as_str())
        .ok_or_else(|| VerbError::InvalidInput("link: missing `from` string".into()))?;
    let to = input
        .get("to")
        .and_then(|v| v.as_str())
        .ok_or_else(|| VerbError::InvalidInput("link: missing `to` string".into()))?;
    Ok((from.to_string(), to.to_string()))
}
