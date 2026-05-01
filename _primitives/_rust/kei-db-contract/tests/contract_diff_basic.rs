//! Integration tests for kei-db-contract.
//! Each test isolates its own input files in a tmpdir to avoid coupling.

use kei_db_contract::diff::{diff_project, FieldStatus, TableStatus};
use kei_db_contract::sql_parse::parse_migrations_dir;
use kei_db_contract::ts_parse::parse_ts_glob;
use std::fs;
use std::path::Path;

fn write_project(root: &Path, sql: &str, ts: &str) {
    let migrations = root.join("migrations");
    let src = root.join("src");
    fs::create_dir_all(&migrations).expect("mkdir migrations");
    fs::create_dir_all(&src).expect("mkdir src");
    fs::write(migrations.join("0001.sql"), sql).expect("write sql");
    fs::write(src.join("types.ts"), ts).expect("write ts");
}

fn run_diff(root: &Path) -> kei_db_contract::diff::DriftReport {
    let tables = parse_migrations_dir(&root.join("migrations")).expect("parse sql");
    let ts_types = parse_ts_glob(&[root.join("src").as_path()]).expect("parse ts");
    diff_project(&tables, &ts_types)
}

#[test]
fn no_drift_when_shapes_match() {
    let tmp = tempfile::tempdir().unwrap();
    write_project(
        tmp.path(),
        "CREATE TABLE users (id INTEGER NOT NULL, email TEXT NOT NULL);",
        "export type User = { id: number; email: string; };",
    );
    let report = run_diff(tmp.path());
    assert_eq!(report.drift_count, 0, "expected no drift, got {report:?}");
    let users = &report.tables[0];
    assert_eq!(users.status, TableStatus::Ok);
    assert!(users.fields.iter().all(|f| f.status == FieldStatus::Ok));
}

#[test]
fn orphan_sql_when_ts_missing_field() {
    let tmp = tempfile::tempdir().unwrap();
    write_project(
        tmp.path(),
        "CREATE TABLE users (id INTEGER NOT NULL, email TEXT NOT NULL);",
        "export type User = { id: number; };",
    );
    let report = run_diff(tmp.path());
    assert!(report.drift_count >= 1);
    let users = report.tables.iter().find(|t| t.name == "users").unwrap();
    assert_eq!(users.status, TableStatus::Drift);
    let email = users.fields.iter().find(|f| f.name == "email").unwrap();
    assert_eq!(email.status, FieldStatus::OrphanSql);
}

#[test]
fn orphan_ts_when_sql_missing_field() {
    let tmp = tempfile::tempdir().unwrap();
    write_project(
        tmp.path(),
        "CREATE TABLE users (id INTEGER NOT NULL);",
        "export type User = { id: number; phone: string; };",
    );
    let report = run_diff(tmp.path());
    assert!(report.drift_count >= 1);
    let users = report.tables.iter().find(|t| t.name == "users").unwrap();
    let phone = users.fields.iter().find(|f| f.name == "phone").unwrap();
    assert_eq!(phone.status, FieldStatus::OrphanTs);
}

#[test]
fn type_mismatch_when_age_integer_vs_string() {
    let tmp = tempfile::tempdir().unwrap();
    write_project(
        tmp.path(),
        "CREATE TABLE users (age INTEGER NOT NULL);",
        "export type User = { age: string; };",
    );
    let report = run_diff(tmp.path());
    assert!(report.drift_count >= 1);
    let users = report.tables.iter().find(|t| t.name == "users").unwrap();
    let age = users.fields.iter().find(|f| f.name == "age").unwrap();
    assert_eq!(age.status, FieldStatus::TypeMismatch);
}

#[test]
fn fixture_project_matches_expected_drift() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("sample-project");
    let tables =
        parse_migrations_dir(&fixture.join("migrations")).expect("fixture parse sql");
    let ts_types = parse_ts_glob(&[fixture.join("src").as_path()]).expect("fixture parse ts");
    let report = diff_project(&tables, &ts_types);
    let users = report.tables.iter().find(|t| t.name == "users").unwrap();
    let age = users.fields.iter().find(|f| f.name == "age").unwrap();
    assert_eq!(age.status, FieldStatus::TypeMismatch);
    let phone = users.fields.iter().find(|f| f.name == "phone").unwrap();
    assert_eq!(phone.status, FieldStatus::OrphanTs);
    let magic = report.tables.iter().find(|t| t.name == "magic_tokens").unwrap();
    assert_eq!(magic.status, TableStatus::Ok);
}
