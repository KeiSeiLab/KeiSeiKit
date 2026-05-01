//! SQL → TypeScript type compatibility table.
//! Conservative allow-list: anything not listed surfaces as drift.

use crate::ts_parse::TsField;

/// Returns true when the SQL column type is compatible with the TS field type.
pub fn sql_ts_compatible(sql_type: &str, ts_type: &str) -> bool {
    let s = sql_type.to_ascii_uppercase();
    let t = ts_type.trim().to_ascii_lowercase();
    let core = strip_null_union(&t);
    if is_text_family(&s) {
        return core == "string" || core.contains("string");
    }
    if is_int_family(&s) {
        return core == "number" || core == "bigint" || core.contains("number");
    }
    if is_float_family(&s) {
        return core == "number" || core.contains("number");
    }
    if s == "BLOB" {
        return core.contains("buffer") || core.contains("uint8array");
    }
    if matches!(s.as_str(), "BOOLEAN" | "BOOL") {
        return core == "boolean";
    }
    if is_temporal_family(&s) {
        return core.contains("string") || core.contains("date") || core.contains("number");
    }
    false
}

fn is_text_family(s: &str) -> bool {
    matches!(
        s,
        "TEXT" | "VARCHAR" | "CHAR" | "STRING" | "CLOB" | "NVARCHAR" | "JSON" | "JSONB"
    )
}

fn is_int_family(s: &str) -> bool {
    matches!(
        s,
        "INTEGER" | "INT" | "BIGINT" | "NUMERIC" | "DECIMAL" | "SMALLINT"
    )
}

fn is_float_family(s: &str) -> bool {
    matches!(s, "REAL" | "FLOAT" | "DOUBLE" | "DOUBLE PRECISION")
}

fn is_temporal_family(s: &str) -> bool {
    matches!(s, "DATETIME" | "TIMESTAMP" | "DATE" | "TIME")
}

/// Filter out `null` / `undefined` from a TS union to get the core type set.
pub fn strip_null_union(t: &str) -> String {
    t.split('|')
        .map(|s| s.trim())
        .filter(|s| *s != "null" && *s != "undefined")
        .collect::<Vec<&str>>()
        .join(" | ")
}

/// SQL column nullable + TS field shape ⇒ compatible? NOT NULL columns always pass.
pub fn null_compatible(sql_nullable: bool, ts: &TsField) -> bool {
    let permits_null = ts.optional
        || ts.ts_type.contains("null")
        || ts.ts_type.contains("undefined");
    if sql_nullable {
        permits_null
    } else {
        true
    }
}
