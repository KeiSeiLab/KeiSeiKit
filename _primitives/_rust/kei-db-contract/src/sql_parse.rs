//! SQL parser cube: walks `migrations/*.sql`, extracts `CREATE TABLE` shapes.

use anyhow::{Context, Result};
use serde::Serialize;
use sqlparser::ast::{ColumnOption, DataType, Statement};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::path::Path;
use walkdir::WalkDir;

/// One column extracted from a migration `CREATE TABLE` statement.
#[derive(Debug, Clone, Serialize)]
pub struct SqlColumn {
    pub name: String,
    pub sql_type: String,
    pub nullable: bool,
}

/// One table extracted from a migration. Later tables with the same name override earlier ones.
#[derive(Debug, Clone, Serialize)]
pub struct SqlTable {
    pub name: String,
    pub columns: Vec<SqlColumn>,
    pub source_file: String,
}

/// Walk `dir` recursively for `.sql` files, return parsed tables.
pub fn parse_migrations_dir(dir: &Path) -> Result<Vec<SqlTable>> {
    let mut tables: Vec<SqlTable> = Vec::new();
    if !dir.exists() {
        return Ok(tables);
    }
    for entry in WalkDir::new(dir).sort_by_file_name() {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let ext = path.extension().and_then(|x| x.to_str()).unwrap_or("");
        if !ext.eq_ignore_ascii_case("sql") {
            continue;
        }
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("read {}", path.display()))?;
        let file_label = path.display().to_string();
        for table in parse_sql_text(&text, &file_label) {
            upsert_table(&mut tables, table);
        }
    }
    Ok(tables)
}

fn upsert_table(tables: &mut Vec<SqlTable>, new_table: SqlTable) {
    if let Some(pos) = tables.iter().position(|t| t.name == new_table.name) {
        tables[pos] = new_table;
    } else {
        tables.push(new_table);
    }
}

/// Parse one SQL document into zero or more table definitions.
pub fn parse_sql_text(text: &str, source: &str) -> Vec<SqlTable> {
    let dialect = GenericDialect {};
    let stmts = match Parser::parse_sql(&dialect, text) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };
    let mut out = Vec::new();
    for stmt in stmts {
        if let Statement::CreateTable(create) = stmt {
            let name = create
                .name
                .0
                .last()
                .map(|p| p.value.clone())
                .unwrap_or_default();
            let columns = create.columns.iter().map(column_def_to_sql_column).collect();
            out.push(SqlTable {
                name,
                columns,
                source_file: source.to_string(),
            });
        }
    }
    out
}

fn column_def_to_sql_column(col: &sqlparser::ast::ColumnDef) -> SqlColumn {
    let nullable = !col
        .options
        .iter()
        .any(|opt| matches!(opt.option, ColumnOption::NotNull));
    SqlColumn {
        name: col.name.value.clone(),
        sql_type: data_type_to_string(&col.data_type),
        nullable,
    }
}

fn data_type_to_string(dt: &DataType) -> String {
    let raw = format!("{}", dt);
    raw.split('(').next().unwrap_or(&raw).trim().to_string()
}
