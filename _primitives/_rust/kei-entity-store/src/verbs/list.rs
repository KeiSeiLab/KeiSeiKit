//! `list` verb — paginated SELECT, ordered by pk DESC.
//!
//! Input: `{ "limit": <int = 50>, "offset": <int = 0> }`. Both optional.

use crate::error::VerbError;
use crate::schema::EntitySchema;
use crate::verbs::get::row_to_json;
use rusqlite::Connection;
use serde_json::{json, Value};

const DEFAULT_LIMIT: i64 = 50;
const MAX_LIMIT: i64 = 10_000;

pub fn run(
    conn: &Connection,
    schema: &EntitySchema,
    input: Value,
) -> Result<Value, VerbError> {
    if !schema.verb_enabled("list") {
        return Err(VerbError::VerbDisabled {
            verb: "list".into(),
            schema: schema.name.into(),
        });
    }
    let limit = clamp(input.get("limit").and_then(|v| v.as_i64()), DEFAULT_LIMIT);
    let offset = input.get("offset").and_then(|v| v.as_i64()).unwrap_or(0).max(0);

    let cols: Vec<&str> = schema.fields.iter().map(|f| f.name).collect();
    let sql = format!(
        "SELECT {} FROM {} ORDER BY {} DESC LIMIT ?1 OFFSET ?2",
        cols.join(","),
        schema.table,
        schema.pk().name
    );
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query(rusqlite::params![limit, offset])?;
    let mut results: Vec<Value> = Vec::new();
    while let Some(r) = rows.next()? {
        results.push(row_to_json(schema, r)?);
    }
    Ok(json!({ "results": results }))
}

fn clamp(raw: Option<i64>, default: i64) -> i64 {
    match raw {
        Some(n) if n > 0 && n <= MAX_LIMIT => n,
        _ => default,
    }
}
