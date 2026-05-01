//! Diff cube: compare SqlTable vs TsType, produce per-field statuses.

use crate::matching::{pair_tables_with_types, Pair};
use crate::report_builders::{
    append_orphan_ts_fields, empty_report, orphan_table_report, orphan_type_report,
};
use crate::sql_parse::{SqlColumn, SqlTable};
use crate::ts_parse::{TsField, TsType};
use crate::types_map::{null_compatible, sql_ts_compatible};
use serde::Serialize;

/// Status of a single field after pairing one SQL column with one TS field.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FieldStatus {
    Ok,
    OrphanSql,
    OrphanTs,
    TypeMismatch,
    NullMismatch,
}

/// Status of a paired (or orphan) table↔type unit.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TableStatus {
    Ok,
    Drift,
    OrphanSql,
    OrphanTs,
}

/// One field-level row in the report.
#[derive(Debug, Clone, Serialize)]
pub struct FieldReport {
    pub name: String,
    pub sql_type: Option<String>,
    pub ts_type: Option<String>,
    pub status: FieldStatus,
}

/// One table-level row in the report.
#[derive(Debug, Clone, Serialize)]
pub struct TableReport {
    pub name: String,
    pub status: TableStatus,
    pub fields: Vec<FieldReport>,
}

/// Top-level report shape.
#[derive(Debug, Clone, Serialize)]
pub struct DriftReport {
    pub drift_count: usize,
    pub tables: Vec<TableReport>,
}

/// Run the full diff over already-parsed inputs.
pub fn diff_project(tables: &[SqlTable], ts_types: &[TsType]) -> DriftReport {
    let pairs = pair_tables_with_types(tables, ts_types);
    let mut report_tables: Vec<TableReport> = Vec::new();
    let mut drift_count = 0usize;
    for pair in pairs {
        let tr = diff_pair(&pair);
        if tr.status != TableStatus::Ok {
            drift_count += count_drift_fields(&tr);
        }
        report_tables.push(tr);
    }
    DriftReport {
        drift_count,
        tables: report_tables,
    }
}

fn count_drift_fields(tr: &TableReport) -> usize {
    tr.fields
        .iter()
        .filter(|f| f.status != FieldStatus::Ok)
        .count()
        .max(1)
}

fn diff_pair(pair: &Pair<'_>) -> TableReport {
    match (pair.table, pair.ts_type) {
        (Some(t), Some(ty)) => diff_table_and_type(t, ty),
        (Some(t), None) => orphan_table_report(t),
        (None, Some(ty)) => orphan_type_report(ty),
        (None, None) => empty_report(),
    }
}

fn diff_table_and_type(table: &SqlTable, ty: &TsType) -> TableReport {
    let mut fields: Vec<FieldReport> = Vec::new();
    let mut consumed = vec![false; ty.fields.len()];
    for col in &table.columns {
        match find_ts_field(&ty.fields, &mut consumed, &col.name) {
            Some(f) => fields.push(compare_one(col, f)),
            None => fields.push(FieldReport {
                name: col.name.clone(),
                sql_type: Some(col.sql_type.clone()),
                ts_type: None,
                status: FieldStatus::OrphanSql,
            }),
        }
    }
    append_orphan_ts_fields(&mut fields, &consumed, &ty.fields);
    let status = if fields.iter().all(|f| f.status == FieldStatus::Ok) {
        TableStatus::Ok
    } else {
        TableStatus::Drift
    };
    TableReport {
        name: table.name.clone(),
        status,
        fields,
    }
}

fn find_ts_field<'a>(
    ts_fields: &'a [TsField],
    consumed: &mut [bool],
    sql_name: &str,
) -> Option<&'a TsField> {
    let target = canonicalize_field(sql_name);
    for (i, f) in ts_fields.iter().enumerate() {
        if consumed[i] {
            continue;
        }
        if canonicalize_field(&f.name) == target {
            consumed[i] = true;
            return Some(f);
        }
    }
    None
}

fn compare_one(col: &SqlColumn, ts_field: &TsField) -> FieldReport {
    let status = if !sql_ts_compatible(&col.sql_type, &ts_field.ts_type) {
        FieldStatus::TypeMismatch
    } else if !null_compatible(col.nullable, ts_field) {
        FieldStatus::NullMismatch
    } else {
        FieldStatus::Ok
    };
    FieldReport {
        name: col.name.clone(),
        sql_type: Some(col.sql_type.clone()),
        ts_type: Some(ts_field.ts_type.clone()),
        status,
    }
}

fn canonicalize_field(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase()
}
