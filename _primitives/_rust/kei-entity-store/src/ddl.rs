//! DDL-string generators split out of `engine.rs` to keep that file
//! under the Constructor-Pattern 200-LOC cap. One function per emitted
//! `CREATE` statement; the engine's `run_migrations` orchestrates the
//! calls and stamps `user_version`.
//!
//! Edge-table DDL lives in `ddl_edge.rs` and is re-exported below;
//! `DdlError` lives in `ddl_error.rs`. Split preserves the 200-LOC cap
//! per Constructor Pattern.

pub use crate::ddl_edge::{edge_table_for, try_edge_table_for};
pub use crate::ddl_error::DdlError;
use crate::schema::{EntitySchema, FieldDef, FieldKind};

pub fn primary_table(schema: &EntitySchema) -> String {
    let cols: Vec<String> = schema.fields.iter().map(column).collect();
    format!(
        "CREATE TABLE IF NOT EXISTS {} (\n  {}\n);",
        schema.table,
        cols.join(",\n  ")
    )
}

fn column(f: &FieldDef) -> String {
    match f.kind {
        FieldKind::IntegerPk => format!("{} INTEGER PRIMARY KEY", f.name),
        FieldKind::TextPk => format!("{} TEXT PRIMARY KEY", f.name),
        FieldKind::IntegerNotNull => format!("{} INTEGER NOT NULL", f.name),
        FieldKind::Integer => format!("{} INTEGER DEFAULT 0", f.name),
        FieldKind::TextNotNull => format!("{} TEXT NOT NULL", f.name),
        FieldKind::Text => format!("{} TEXT DEFAULT ''", f.name),
        FieldKind::TextDefault => text_default_column(f),
        FieldKind::TextArchiveEnum => archive_enum_column(f),
        FieldKind::Real => format!("{} REAL NOT NULL DEFAULT 0.0", f.name),
        FieldKind::RealDefault => real_default_column(f),
        FieldKind::TimestampCreated => format!("{} INTEGER NOT NULL", f.name),
        FieldKind::TimestampUpdated => format!("{} INTEGER NOT NULL", f.name),
    }
}

fn text_default_column(f: &FieldDef) -> String {
    let d = f.default.unwrap_or("");
    // SQL-escape embedded single quotes (per SQL standard: `'` → `''`)
    // so `text_default("status", "don't know")` does not inject.
    let escaped = d.replace('\'', "''");
    format!("{} TEXT NOT NULL DEFAULT '{}'", f.name, escaped)
}

fn archive_enum_column(f: &FieldDef) -> String {
    let (active, _archived) = f.archive_enum.unwrap_or(("active", "archived"));
    let escaped = active.replace('\'', "''");
    format!("{} TEXT NOT NULL DEFAULT '{}'", f.name, escaped)
}

fn real_default_column(f: &FieldDef) -> String {
    let d = f.real_default.unwrap_or(0.0);
    format!("{} REAL NOT NULL DEFAULT {}", f.name, format_real(d))
}

/// Deterministic SQL literal for an f64 — always has a decimal point,
/// no exponent for finite values. Non-finite values fall back to 0.0.
fn format_real(v: f64) -> String {
    if !v.is_finite() {
        return "0.0".to_string();
    }
    if v.fract() == 0.0 {
        format!("{:.1}", v)
    } else {
        format!("{}", v)
    }
}

pub fn indexes(schema: &EntitySchema) -> String {
    let mut out = String::new();
    for f in schema.fields.iter().filter(|f| f.indexed) {
        out.push_str(&format!(
            "CREATE INDEX IF NOT EXISTS idx_{t}_{c} ON {t}({c});\n",
            t = schema.table,
            c = f.name
        ));
    }
    out
}

pub fn fts_table(table: &str, cols: &[&str]) -> String {
    let col_list = cols.join(", ");
    format!(
        "CREATE VIRTUAL TABLE IF NOT EXISTS fts_{table} \
         USING fts5({table}_id UNINDEXED, {col_list}, tokenize='porter unicode61');"
    )
}

