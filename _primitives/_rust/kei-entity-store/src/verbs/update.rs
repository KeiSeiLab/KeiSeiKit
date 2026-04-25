//! `update` verb — partial update by id. Only declared schema keys
//! that appear in the input JSON are written. Type mismatch →
//! `InvalidType` (no silent coercion). UPDATE + FTS reindex run in a
//! single transaction so a mid-flight failure leaves neither the row
//! nor the FTS entry in a torn state.

use crate::error::VerbError;
use crate::schema::{EntitySchema, FieldDef, FieldKind};
use crate::verbs::pk::{self, PkValue};
use crate::verbs::update_invariant as inv;
use crate::verbs::validate;
use chrono::Utc;
use rusqlite::{types::Value as SqlValue, Connection};
use serde_json::{json, Value};

pub fn run(
    conn: &Connection,
    schema: &EntitySchema,
    input: Value,
) -> Result<Value, VerbError> {
    guard_enabled(schema)?;
    let obj = input
        .as_object()
        .ok_or_else(|| VerbError::InvalidInput("update: expected JSON object".into()))?;
    let id = pk::extract(schema, &input, "update")?;
    let now = Utc::now().timestamp();
    let (set_cols, values) = build_set(schema, obj, now)?;
    if set_cols.is_empty() {
        return Err(VerbError::InvalidInput("update: no writable fields supplied".into()));
    }
    update_tx(conn, schema, &id, &set_cols, values, obj)?;
    Ok(json!({ "ok": true, "id": id.as_json() }))
}

fn guard_enabled(schema: &EntitySchema) -> Result<(), VerbError> {
    if !schema.verb_enabled("update") {
        return Err(VerbError::VerbDisabled {
            verb: "update".into(),
            schema: schema.name.into(),
        });
    }
    Ok(())
}

fn update_tx(
    conn: &Connection,
    schema: &EntitySchema,
    id: &PkValue,
    set_cols: &[&'static str],
    values: Vec<SqlValue>,
    obj: &serde_json::Map<String, Value>,
) -> Result<(), VerbError> {
    let tx = conn.unchecked_transaction()?;
    // Debug-build snapshot for the non-input-FTS-stable invariant
    // asserted in `reindex_fts`. Release: empty map, no SELECT.
    let pre_update = inv::pre_update_snapshot(&tx, schema, id);
    exec_update_tx(&tx, schema, id, set_cols, values)?;
    if let Some(cols) = schema.fts_columns {
        reindex_fts(&tx, schema, cols, id, obj, &pre_update)?;
    }
    tx.commit()?;
    Ok(())
}

fn exec_update_tx(
    tx: &rusqlite::Transaction<'_>,
    schema: &EntitySchema,
    id: &PkValue,
    set_cols: &[&'static str],
    values: Vec<SqlValue>,
) -> Result<(), VerbError> {
    let placeholders: Vec<String> = (1..=set_cols.len())
        .map(|i| format!("{} = ?{i}", set_cols[i - 1])).collect();
    let id_idx = set_cols.len() + 1;
    let sql = format!(
        "UPDATE {} SET {} WHERE {}=?{}",
        schema.table, placeholders.join(", "), schema.pk().name, id_idx,
    );
    let mut all: Vec<SqlValue> = values;
    all.push(id.as_sql());
    let params: Vec<&dyn rusqlite::ToSql> =
        all.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
    let rows = tx.execute(&sql, params.as_slice())?;
    if rows == 0 {
        return Err(VerbError::not_found_text(schema.name, id.as_string()));
    }
    Ok(())
}

fn build_set(
    schema: &EntitySchema,
    input: &serde_json::Map<String, Value>,
    now: i64,
) -> Result<(Vec<&'static str>, Vec<SqlValue>), VerbError> {
    let mut cols: Vec<&'static str> = Vec::new();
    let mut values: Vec<SqlValue> = Vec::new();
    for f in schema.writable_fields() {
        if f.kind == FieldKind::TimestampUpdated {
            cols.push(f.name);
            values.push(SqlValue::Integer(now));
            continue;
        }
        if let Some(sql_val) = value_from_input(f, input)? {
            cols.push(f.name);
            values.push(sql_val);
        }
    }
    Ok((cols, values))
}

fn value_from_input(
    f: &FieldDef,
    input: &serde_json::Map<String, Value>,
) -> Result<Option<SqlValue>, VerbError> {
    let Some(raw) = input.get(f.name) else {
        return Ok(None);
    };
    if f.is_pk() {
        return Ok(None);
    }
    Ok(Some(validate::coerce(f, raw)?))
}

/// Rebuild the FTS5 row after the primary UPDATE. INVARIANT: FTS
/// columns NOT in `input` keep their pre-UPDATE value through the
/// UPDATE (holds while UPDATE only touches `input` columns). Proof
/// and debug-build assertion in `verbs/update_invariant.rs`.
fn reindex_fts(
    tx: &rusqlite::Transaction<'_>,
    schema: &EntitySchema,
    cols: &[&str],
    id: &PkValue,
    input: &serde_json::Map<String, Value>,
    pre_update: &serde_json::Map<String, Value>,
) -> Result<(), VerbError> {
    let table = schema.table;
    let existing = read_existing_fts(tx, schema, cols, id)?;
    #[cfg(debug_assertions)]
    inv::debug_assert_non_input_fts_stable(cols, input, pre_update, &existing);
    let _ = pre_update; // unused in release builds
    tx.execute(
        &format!("DELETE FROM fts_{table} WHERE {table}_id=?1"),
        rusqlite::params![id.as_sql()],
    )?;
    let placeholders: Vec<String> = (2..=(cols.len() + 1)).map(|i| format!("?{i}")).collect();
    let sql = format!(
        "INSERT INTO fts_{table} ({table}_id, {}) VALUES (?1, {})",
        cols.join(", "),
        placeholders.join(", "),
    );
    let values = fts_row_values(id, cols, input, &existing);
    let params: Vec<&dyn rusqlite::ToSql> =
        values.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
    tx.execute(&sql, params.as_slice())?;
    Ok(())
}

fn fts_row_values(
    id: &PkValue,
    cols: &[&str],
    input: &serde_json::Map<String, Value>,
    existing: &serde_json::Map<String, Value>,
) -> Vec<SqlValue> {
    let mut values: Vec<SqlValue> = vec![id.as_sql()];
    for c in cols {
        let val = input
            .get(*c)
            .and_then(|v| v.as_str())
            .or_else(|| existing.get(*c).and_then(|v| v.as_str()))
            .unwrap_or("")
            .to_string();
        values.push(SqlValue::Text(val));
    }
    values
}

pub(super) fn read_existing_fts(
    tx: &rusqlite::Transaction<'_>,
    schema: &EntitySchema,
    cols: &[&str],
    id: &PkValue,
) -> Result<serde_json::Map<String, Value>, VerbError> {
    let col_list = cols.join(",");
    let sql = format!(
        "SELECT {col_list} FROM {} WHERE {}=?1",
        schema.table,
        schema.pk().name
    );
    let mut stmt = tx.prepare(&sql)?;
    let mut rows = stmt.query(rusqlite::params![id.as_sql()])?;
    let mut out = serde_json::Map::new();
    if let Some(r) = rows.next()? {
        for (i, c) in cols.iter().enumerate() {
            let v: String = r.get(i).unwrap_or_default();
            out.insert((*c).to_string(), Value::from(v));
        }
    }
    Ok(out)
}
