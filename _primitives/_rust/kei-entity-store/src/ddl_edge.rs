//! Edge-table DDL generators. Split out of `ddl.rs` to keep each file
//! inside the Constructor Pattern 200-LOC cap. `ddl.rs` retains the
//! entity-table, index, and FTS DDL; this module owns edge-table DDL
//! in all three variants (`IntegerPair`, `TextPair`,
//! `TextPairWithMetadata`).

use crate::ddl_error::DdlError;
use crate::schema::{EdgeKeyKind, FieldKind};

/// Dispatcher — picks edge-table DDL for a given `EdgeKeyKind`. Added
/// for kei-sage migration; `IntegerPair` branch preserves legacy body.
///
/// Backward-compat shim — prefer `try_edge_table_for` from new code.
/// This variant panics on unsupported `extra_columns` FieldKinds; the
/// engine's migration path uses the fallible variant to surface typed
/// errors without panicking.
pub fn edge_table_for(edge: &str, kind: EdgeKeyKind) -> String {
    try_edge_table_for(edge, kind).expect("edge_table_for: unsupported extra_column FieldKind")
}

/// Fallible dispatcher — same as `edge_table_for` but returns
/// `DdlError::UnsupportedExtraColumn` instead of panicking when an
/// `extra_columns` entry carries a FieldKind outside the supported
/// subset. This is the path `Store::open` takes.
pub fn try_edge_table_for(edge: &str, kind: EdgeKeyKind) -> Result<String, DdlError> {
    match kind {
        EdgeKeyKind::IntegerPair => Ok(edge_integer(edge)),
        EdgeKeyKind::TextPair => Ok(edge_text(edge)),
        EdgeKeyKind::TextPairWithMetadata {
            from_col,
            to_col,
            has_id,
            has_weight,
            has_created_at,
            extra_columns,
        } => edge_text_meta(
            edge,
            from_col,
            to_col,
            has_id,
            has_weight,
            has_created_at,
            extra_columns,
        ),
    }
}

fn edge_integer(edge: &str) -> String {
    format!(
        "CREATE TABLE IF NOT EXISTS {edge} (\n  \
            from_id INTEGER NOT NULL,\n  \
            to_id INTEGER NOT NULL,\n  \
            edge_type TEXT NOT NULL DEFAULT 'links',\n  \
            PRIMARY KEY(from_id, to_id, edge_type)\n\
         );\n\
         CREATE INDEX IF NOT EXISTS idx_{edge}_to ON {edge}(to_id);"
    )
}

/// Text-keyed edge DDL: `(src_path TEXT, dst_path TEXT, edge_type TEXT)`.
fn edge_text(edge: &str) -> String {
    format!(
        "CREATE TABLE IF NOT EXISTS {edge} (\n  \
            src_path TEXT NOT NULL,\n  \
            dst_path TEXT NOT NULL,\n  \
            edge_type TEXT NOT NULL DEFAULT 'links',\n  \
            PRIMARY KEY(src_path, dst_path, edge_type)\n\
         );\n\
         CREATE INDEX IF NOT EXISTS idx_{edge}_dst ON {edge}(dst_path);"
    )
}

/// Text-keyed edge DDL with optional metadata columns + caller-chosen
/// key column names + arbitrary extra columns. Fallible — returns
/// `DdlError::UnsupportedExtraColumn` if any `extras` entry uses a
/// disallowed `FieldKind`.
fn edge_text_meta(
    edge: &str,
    from_col: &str,
    to_col: &str,
    has_id: bool,
    has_weight: bool,
    has_created_at: bool,
    extras: &[(&str, FieldKind)],
) -> Result<String, DdlError> {
    let mut cols: Vec<String> = Vec::new();
    if has_id {
        cols.push("edge_id INTEGER PRIMARY KEY AUTOINCREMENT".to_string());
    }
    cols.push(format!("{from_col} TEXT NOT NULL"));
    cols.push(format!("{to_col} TEXT NOT NULL"));
    cols.push("edge_type TEXT NOT NULL DEFAULT 'links'".to_string());
    if has_weight {
        cols.push("weight REAL NOT NULL DEFAULT 1.0".to_string());
    }
    for (name, kind) in extras {
        cols.push(try_extra_column(name, *kind)?);
    }
    if has_created_at {
        cols.push("created_at INTEGER NOT NULL".to_string());
    }
    // Without an autoincrement PK we still want `INSERT OR IGNORE`
    // idempotent over the triple; with one we emit a UNIQUE instead.
    if has_id {
        cols.push(format!("UNIQUE({from_col}, {to_col}, edge_type)"));
    } else {
        cols.push(format!("PRIMARY KEY({from_col}, {to_col}, edge_type)"));
    }
    let body = cols.join(",\n  ");
    Ok(format!(
        "CREATE TABLE IF NOT EXISTS {edge} (\n  {body}\n);\n\
         CREATE INDEX IF NOT EXISTS idx_{edge}_dst ON {edge}({to_col});"
    ))
}

/// DDL for one extra edge column. Limited subset of `FieldKind` — edge
/// extras can't be PKs, archive enums, or auto-stamped timestamps.
/// Fallible — returns `DdlError::UnsupportedExtraColumn` outside the
/// supported set instead of panicking.
fn try_extra_column(name: &str, kind: FieldKind) -> Result<String, DdlError> {
    match kind {
        FieldKind::Text => Ok(format!("{name} TEXT DEFAULT ''")),
        FieldKind::TextNotNull => Ok(format!("{name} TEXT NOT NULL")),
        FieldKind::Integer => Ok(format!("{name} INTEGER DEFAULT 0")),
        FieldKind::IntegerNotNull => Ok(format!("{name} INTEGER NOT NULL")),
        FieldKind::Real => Ok(format!("{name} REAL NOT NULL DEFAULT 0.0")),
        other => Err(DdlError::UnsupportedExtraColumn {
            kind_debug: format!("{other:?}"),
            column_name: name.to_string(),
        }),
    }
}
