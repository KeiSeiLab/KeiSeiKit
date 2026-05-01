//! `create` verb — INSERT one row using fields declared on the schema.
//! Per-kind value defaulting lives in `create_defaults`.
//!
//! TextPk schemas require the caller to supply `id`; IntegerPk schemas
//! get an auto-assigned rowid. Output `{id, created_at}`.

use crate::error::VerbError;
use crate::schema::{EntitySchema, FieldKind};
use crate::verbs::create_defaults::field_value;
use chrono::Utc;
use rusqlite::{types::Value as SqlValue, Connection};
use serde_json::{json, Value};

pub fn run(
    conn: &Connection,
    schema: &EntitySchema,
    input: Value,
) -> Result<Value, VerbError> {
    guard_enabled(schema)?;
    let obj = as_object(&input, "create")?;
    let now = Utc::now().timestamp();
    let (cols, values) = build_insert(schema, obj, now)?;
    let id = insert_tx(conn, schema, &cols, &values, obj)?;
    let created_at = read_created_at(conn, schema, &id).unwrap_or(now);
    Ok(json!({ "id": id_to_json(&id), "created_at": created_at }))
}

fn guard_enabled(schema: &EntitySchema) -> Result<(), VerbError> {
    if !schema.verb_enabled("create") {
        return Err(VerbError::VerbDisabled {
            verb: "create".into(),
            schema: schema.name.into(),
        });
    }
    Ok(())
}

/// Stored PK of the inserted row. `Integer` for auto-rowid schemas,
/// `Text` for caller-supplied TEXT PKs.
pub(super) enum InsertedPk {
    Integer(i64),
    Text(String),
}

fn id_to_json(pk: &InsertedPk) -> Value {
    match pk {
        InsertedPk::Integer(n) => Value::from(*n),
        InsertedPk::Text(s) => Value::from(s.clone()),
    }
}

fn pk_sql(pk: &InsertedPk) -> SqlValue {
    match pk {
        InsertedPk::Integer(n) => SqlValue::Integer(*n),
        InsertedPk::Text(s) => SqlValue::Text(s.clone()),
    }
}

/// INSERT + FTS reindex wrapped in one `unchecked_transaction` so a
/// mid-flight FTS failure rolls back the row insert too.
fn insert_tx(
    conn: &Connection,
    schema: &EntitySchema,
    cols: &[&'static str],
    values: &[SqlValue],
    obj: &serde_json::Map<String, Value>,
) -> Result<InsertedPk, VerbError> {
    let tx = conn.unchecked_transaction()?;
    let id = exec_insert_tx(&tx, schema, cols, values, obj)?;
    if let Some(fts_cols) = schema.fts_columns {
        reindex_fts(&tx, schema, fts_cols, &id, obj)?;
    }
    tx.commit()?;
    Ok(id)
}

fn exec_insert_tx(
    tx: &rusqlite::Transaction<'_>,
    schema: &EntitySchema,
    cols: &[&'static str],
    values: &[SqlValue],
    obj: &serde_json::Map<String, Value>,
) -> Result<InsertedPk, VerbError> {
    if schema.pk().kind == FieldKind::TextPk {
        return exec_text_pk_insert(tx, schema, cols, values, obj);
    }
    exec_raw_insert(tx, schema.table, cols, values)?;
    Ok(InsertedPk::Integer(tx.last_insert_rowid()))
}

fn exec_text_pk_insert(
    tx: &rusqlite::Transaction<'_>,
    schema: &EntitySchema,
    cols: &[&'static str],
    values: &[SqlValue],
    obj: &serde_json::Map<String, Value>,
) -> Result<InsertedPk, VerbError> {
    let pk_name = schema.pk().name;
    let id_str = obj
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            VerbError::InvalidInput("create: `id` required for TextPk schemas".into())
        })?
        .to_string();
    let mut all_cols: Vec<&'static str> = vec![pk_name];
    all_cols.extend_from_slice(cols);
    let mut all_vals: Vec<SqlValue> = vec![SqlValue::Text(id_str.clone())];
    all_vals.extend_from_slice(values);
    exec_raw_insert(tx, schema.table, &all_cols, &all_vals)?;
    Ok(InsertedPk::Text(id_str))
}

fn exec_raw_insert(
    tx: &rusqlite::Transaction<'_>,
    table: &str,
    cols: &[&'static str],
    values: &[SqlValue],
) -> Result<(), VerbError> {
    let placeholders: Vec<String> = (1..=cols.len()).map(|i| format!("?{i}")).collect();
    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table,
        cols.join(","),
        placeholders.join(","),
    );
    let params: Vec<&dyn rusqlite::ToSql> =
        values.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
    tx.execute(&sql, params.as_slice())?;
    Ok(())
}

fn as_object<'a>(
    v: &'a Value,
    verb: &str,
) -> Result<&'a serde_json::Map<String, Value>, VerbError> {
    v.as_object()
        .ok_or_else(|| VerbError::InvalidInput(format!("{verb}: expected JSON object")))
}

fn build_insert(
    schema: &EntitySchema,
    input: &serde_json::Map<String, Value>,
    now: i64,
) -> Result<(Vec<&'static str>, Vec<SqlValue>), VerbError> {
    let mut cols: Vec<&'static str> = Vec::new();
    let mut values: Vec<SqlValue> = Vec::new();
    for f in schema.writable_fields() {
        cols.push(f.name);
        values.push(field_value(f, input, now)?);
    }
    Ok((cols, values))
}

fn reindex_fts(
    tx: &rusqlite::Transaction<'_>,
    schema: &EntitySchema,
    cols: &[&str],
    id: &InsertedPk,
    input: &serde_json::Map<String, Value>,
) -> Result<(), VerbError> {
    let table = schema.table;
    let pk_param = pk_sql(id);
    tx.execute(
        &format!("DELETE FROM fts_{table} WHERE {table}_id=?1"),
        rusqlite::params![pk_param],
    )?;
    let placeholders: Vec<String> = (2..=(cols.len() + 1)).map(|i| format!("?{i}")).collect();
    let sql = format!(
        "INSERT INTO fts_{table} ({table}_id, {}) VALUES (?1, {})",
        cols.join(", "),
        placeholders.join(", "),
    );
    let mut values: Vec<SqlValue> = vec![pk_param];
    for c in cols {
        let v = input.get(*c).and_then(|v| v.as_str()).unwrap_or("").to_string();
        values.push(SqlValue::Text(v));
    }
    let params: Vec<&dyn rusqlite::ToSql> =
        values.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
    tx.execute(&sql, params.as_slice())?;
    Ok(())
}

fn read_created_at(conn: &Connection, schema: &EntitySchema, id: &InsertedPk) -> Option<i64> {
    if !schema.fields.iter().any(|f| f.kind == FieldKind::TimestampCreated) {
        return None;
    }
    let sql = format!("SELECT created_at FROM {} WHERE {}=?1", schema.table, schema.pk().name);
    conn.query_row(&sql, rusqlite::params![pk_sql(id)], |r| r.get::<_, i64>(0)).ok()
}
