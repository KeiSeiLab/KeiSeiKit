//! `archive` verb — soft-delete. If the configured `archived_field`
//! column has kind `TextArchiveEnum`, writes the column's
//! `archived` sentinel string; otherwise flips an INTEGER column to 1.
//! A sibling `<archived_field>_at` column is stamped with the current
//! Unix timestamp when present.
//!
//! Required schema configuration: `archived_field: Some("<col>")`.
//! Without it the verb errors with `InvalidInput` — the engine does NOT
//! fall back to legacy `archived` heuristics (those remain in
//! `delete.rs` soft-path only).
//!
//! FTS semantics: archiving means "hidden from active listing". The verb
//! therefore DELETEs the row from `fts_<table>` inside the same
//! transaction as the UPDATE, so `search` will no longer return the
//! archived row. If a future caller flips the column back to active
//! (unarchive), it MUST reinsert the row into the FTS index — the
//! current contract does not auto-reindex on unarchive. "Keep
//! searchable while archived" is a future feature (`search
//! --include-archived`), NOT today.
//!
//! Input: `{ id: <int|string> }`.
//! Output: `{ id, archived_at }` — `archived_at` is the stamped
//! timestamp when a `<field>_at` column exists, else `null`.

use crate::error::VerbError;
use crate::schema::{EntitySchema, FieldKind};
use crate::verbs::pk::{self, PkValue};
use rusqlite::{types::Value as SqlValue, Connection};
use serde_json::{json, Value};

pub fn run(
    conn: &Connection,
    schema: &EntitySchema,
    input: Value,
) -> Result<Value, VerbError> {
    guard_enabled(schema)?;
    let field_name = schema.archived_field.ok_or_else(|| {
        VerbError::InvalidInput(format!(
            "archive: schema {} has no archived_field configured",
            schema.name
        ))
    })?;
    let id = pk::extract(schema, &input, "archive")?;
    let ts_col = format!("{field_name}_at");
    let has_ts = schema.fields.iter().any(|f| f.name == ts_col);
    let now: i64 = chrono::Utc::now().timestamp();

    let rows = archive_tx(conn, schema, field_name, &ts_col, has_ts, &id, now)?;
    if rows == 0 {
        return Err(VerbError::not_found_text(schema.name, id.as_string()));
    }
    let stamped = if has_ts { json!(now) } else { Value::Null };
    Ok(json!({ "id": id.as_json(), "archived_at": stamped }))
}

fn guard_enabled(schema: &EntitySchema) -> Result<(), VerbError> {
    if !schema.verb_enabled("archive") {
        return Err(VerbError::VerbDisabled {
            verb: "archive".into(),
            schema: schema.name.into(),
        });
    }
    Ok(())
}

/// UPDATE the archived column (+ stamp) and DELETE the row from FTS (if
/// configured) in one transaction. Either both persist or neither does.
fn archive_tx(
    conn: &Connection,
    schema: &EntitySchema,
    field_name: &str,
    ts_col: &str,
    has_ts: bool,
    id: &PkValue,
    now: i64,
) -> Result<usize, VerbError> {
    let tx = conn.unchecked_transaction()?;
    let rows = execute_archive(&tx, schema, field_name, ts_col, has_ts, id, now)?;
    if rows > 0 && schema.fts_columns.is_some() {
        tx.execute(
            &format!(
                "DELETE FROM fts_{t} WHERE {t}_id=?1",
                t = schema.table
            ),
            rusqlite::params![id.as_sql()],
        )?;
    }
    tx.commit()?;
    Ok(rows)
}

fn execute_archive(
    tx: &rusqlite::Transaction<'_>,
    schema: &EntitySchema,
    field_name: &str,
    ts_col: &str,
    has_ts: bool,
    id: &PkValue,
    now: i64,
) -> Result<usize, VerbError> {
    let marker = archive_marker(schema, field_name);
    let pk_name = schema.pk().name;
    let rows = if has_ts {
        tx.execute(
            &format!(
                "UPDATE {t} SET {field_name} = ?1, {ts_col} = ?2 WHERE {pk_name} = ?3",
                t = schema.table
            ),
            rusqlite::params![marker, now, id.as_sql()],
        )?
    } else {
        tx.execute(
            &format!(
                "UPDATE {t} SET {field_name} = ?1 WHERE {pk_name} = ?2",
                t = schema.table
            ),
            rusqlite::params![marker, id.as_sql()],
        )?
    };
    Ok(rows)
}

/// Pick the SQL value written to the archived column. `TextArchiveEnum`
/// columns receive the `archived` sentinel; any other kind receives
/// the integer flag `1` (legacy behaviour).
fn archive_marker(schema: &EntitySchema, field_name: &str) -> SqlValue {
    let Some(field) = schema.field(field_name) else {
        return SqlValue::Integer(1);
    };
    match field.kind {
        FieldKind::TextArchiveEnum => {
            let (_active, archived) =
                field.archive_enum.unwrap_or(("active", "archived"));
            SqlValue::Text(archived.to_string())
        }
        _ => SqlValue::Integer(1),
    }
}
