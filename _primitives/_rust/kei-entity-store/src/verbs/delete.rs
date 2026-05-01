//! `delete` verb — hard DELETE by id, OR soft (if schema has an
//! `archived` integer field, flips it to 1).
//!
//! The hard-delete path wraps the `fts_<table>` DELETE and the base-table
//! DELETE in a single `unchecked_transaction`, so a mid-flight FTS failure
//! rolls back the row delete too (mirrors create/update C2 fix).

use crate::error::VerbError;
use crate::schema::EntitySchema;
use crate::verbs::pk::{self, PkValue};
use rusqlite::Connection;
use serde_json::{json, Value};

pub fn run(
    conn: &Connection,
    schema: &EntitySchema,
    input: Value,
) -> Result<Value, VerbError> {
    if !schema.verb_enabled("delete") {
        return Err(VerbError::VerbDisabled {
            verb: "delete".into(),
            schema: schema.name.into(),
        });
    }
    let id = pk::extract(schema, &input, "delete")?;
    let soft = input.get("soft").and_then(|v| v.as_bool()).unwrap_or(false);

    let rows = if soft && has_archived_field(schema) {
        soft_delete(conn, schema, &id)?
    } else {
        hard_delete_tx(conn, schema, &id)?
    };
    if rows == 0 {
        return Err(VerbError::not_found_text(schema.name, id.as_string()));
    }
    Ok(json!({ "ok": true, "id": id.as_json() }))
}

fn soft_delete(
    conn: &Connection,
    schema: &EntitySchema,
    id: &PkValue,
) -> Result<usize, VerbError> {
    let rows = conn.execute(
        &format!(
            "UPDATE {} SET archived = 1 WHERE {}=?1",
            schema.table,
            schema.pk().name
        ),
        rusqlite::params![id.as_sql()],
    )?;
    Ok(rows)
}

/// FTS DELETE + base-table DELETE in one transaction. If either fails,
/// neither persists.
fn hard_delete_tx(
    conn: &Connection,
    schema: &EntitySchema,
    id: &PkValue,
) -> Result<usize, VerbError> {
    let tx = conn.unchecked_transaction()?;
    if schema.fts_columns.is_some() {
        tx.execute(
            &format!(
                "DELETE FROM fts_{t} WHERE {t}_id=?1",
                t = schema.table
            ),
            rusqlite::params![id.as_sql()],
        )?;
    }
    let rows = tx.execute(
        &format!(
            "DELETE FROM {} WHERE {}=?1",
            schema.table,
            schema.pk().name
        ),
        rusqlite::params![id.as_sql()],
    )?;
    tx.commit()?;
    Ok(rows)
}

fn has_archived_field(schema: &EntitySchema) -> bool {
    schema.fields.iter().any(|f| f.name == "archived")
}
