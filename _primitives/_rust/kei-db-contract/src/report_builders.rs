//! Small builders that translate raw SQL/TS pieces into report rows.

use crate::diff::{FieldReport, FieldStatus, TableReport, TableStatus};
use crate::sql_parse::SqlTable;
use crate::ts_parse::{TsField, TsType};

/// Whole-table report for an SQL table with no matching TS type.
pub fn orphan_table_report(t: &SqlTable) -> TableReport {
    let fields = t
        .columns
        .iter()
        .map(|c| FieldReport {
            name: c.name.clone(),
            sql_type: Some(c.sql_type.clone()),
            ts_type: None,
            status: FieldStatus::OrphanSql,
        })
        .collect();
    TableReport {
        name: t.name.clone(),
        status: TableStatus::OrphanSql,
        fields,
    }
}

/// Whole-table report for a TS type with no matching SQL table.
pub fn orphan_type_report(ty: &TsType) -> TableReport {
    let fields = ty
        .fields
        .iter()
        .map(|f| FieldReport {
            name: f.name.clone(),
            sql_type: None,
            ts_type: Some(f.ts_type.clone()),
            status: FieldStatus::OrphanTs,
        })
        .collect();
    TableReport {
        name: ty.name.clone(),
        status: TableStatus::OrphanTs,
        fields,
    }
}

/// Vacuous report for the (None, None) pair (only triggered by an empty workspace).
pub fn empty_report() -> TableReport {
    TableReport {
        name: String::new(),
        status: TableStatus::Ok,
        fields: Vec::new(),
    }
}

/// Append every TS field that no SQL column claimed as orphan-TS rows.
pub fn append_orphan_ts_fields(
    fields: &mut Vec<FieldReport>,
    consumed: &[bool],
    ts_fields: &[TsField],
) {
    for (i, f) in ts_fields.iter().enumerate() {
        if !consumed[i] {
            fields.push(FieldReport {
                name: f.name.clone(),
                sql_type: None,
                ts_type: Some(f.ts_type.clone()),
                status: FieldStatus::OrphanTs,
            });
        }
    }
}
