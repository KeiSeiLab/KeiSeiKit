//! `get` verb — SELECT one row by id, returning a JSON object with
//! every declared field.

use crate::error::VerbError;
use crate::schema::{EntitySchema, FieldDef, FieldKind};
use crate::verbs::pk;
use rusqlite::Connection;
use serde_json::{Map, Value};

pub fn run(
    conn: &Connection,
    schema: &EntitySchema,
    input: Value,
) -> Result<Value, VerbError> {
    if !schema.verb_enabled("get") {
        return Err(VerbError::VerbDisabled {
            verb: "get".into(),
            schema: schema.name.into(),
        });
    }
    let id = pk::extract(schema, &input, "get")?;
    let cols: Vec<&str> = schema.fields.iter().map(|f| f.name).collect();
    let sql = format!(
        "SELECT {} FROM {} WHERE {}=?1",
        cols.join(","),
        schema.table,
        schema.pk().name
    );
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query(rusqlite::params![id.as_sql()])?;
    match rows.next()? {
        Some(r) => Ok(row_to_json(schema, r)?),
        None => Err(VerbError::not_found_text(schema.name, id.as_string())),
    }
}

pub(crate) fn row_to_json(
    schema: &EntitySchema,
    row: &rusqlite::Row,
) -> Result<Value, VerbError> {
    let mut obj = Map::new();
    for (idx, f) in schema.fields.iter().enumerate() {
        obj.insert(f.name.to_string(), field_to_json(f, row, idx)?);
    }
    Ok(Value::Object(obj))
}

fn field_to_json(f: &FieldDef, row: &rusqlite::Row, idx: usize) -> Result<Value, VerbError> {
    Ok(match f.kind {
        FieldKind::IntegerPk
        | FieldKind::IntegerNotNull
        | FieldKind::Integer
        | FieldKind::TimestampCreated
        | FieldKind::TimestampUpdated => {
            let n: i64 = row.get(idx)?;
            Value::from(n)
        }
        FieldKind::TextPk
        | FieldKind::TextNotNull
        | FieldKind::Text
        | FieldKind::TextDefault
        | FieldKind::TextArchiveEnum => {
            let s: String = row.get(idx)?;
            Value::from(s)
        }
        FieldKind::Real | FieldKind::RealDefault => {
            let n: f64 = row.get(idx)?;
            Value::from(n)
        }
    })
}

