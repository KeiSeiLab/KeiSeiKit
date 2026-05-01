//! Render a `DriftReport` as JSON or human-readable text.

use crate::diff::{DriftReport, FieldStatus, TableStatus};

/// Pretty-printed JSON via serde.
pub fn render_json(report: &DriftReport) -> String {
    serde_json::to_string_pretty(report).unwrap_or_else(|_| "{}".to_string())
}

/// Human-readable summary; one block per table, severity-tagged.
pub fn render_text(report: &DriftReport) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "kei-db-contract: drift_count={} tables={}\n",
        report.drift_count,
        report.tables.len()
    ));
    for table in &report.tables {
        out.push_str(&format_table(table));
    }
    out
}

fn format_table(table: &crate::diff::TableReport) -> String {
    let tag = table_status_tag(&table.status);
    let mut block = format!("\n[{}] {}\n", tag, table.name);
    for field in &table.fields {
        let ftag = field_status_tag(&field.status);
        let sql = field.sql_type.as_deref().unwrap_or("-");
        let ts = field.ts_type.as_deref().unwrap_or("-");
        block.push_str(&format!(
            "  [{ftag}] {} :: sql={} ts={}\n",
            field.name, sql, ts
        ));
    }
    block
}

fn table_status_tag(status: &TableStatus) -> &'static str {
    match status {
        TableStatus::Ok => "OK",
        TableStatus::Drift => "DRIFT",
        TableStatus::OrphanSql => "ORPHAN-SQL",
        TableStatus::OrphanTs => "ORPHAN-TS",
    }
}

fn field_status_tag(status: &FieldStatus) -> &'static str {
    match status {
        FieldStatus::Ok => "OK",
        FieldStatus::OrphanSql => "ORPHAN-SQL",
        FieldStatus::OrphanTs => "ORPHAN-TS",
        FieldStatus::TypeMismatch => "TYPE-MISMATCH",
        FieldStatus::NullMismatch => "NULL-MISMATCH",
    }
}
