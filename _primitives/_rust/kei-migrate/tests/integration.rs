//! Integration tests for kei-migrate against a SQLite file (safe, no deps).
//!
//! SQLite is chosen as the test backend because it has no server dependency
//! and the sqlx-Any path through it exercises the same code path as Postgres
//! / MySQL for everything except dialect-specific DDL (which we abstract in
//! `db::Backend::create_tracker_sql`).

use kei_migrate::{cmd_create, db, discover, do_down, do_status, do_up, tracker};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

struct Env {
    _tmp: TempDir,
    url: String,
    dir: PathBuf,
}

fn setup() -> Env {
    let tmp = TempDir::new().unwrap();
    let db_path = tmp.path().join("test.db");
    let url = format!("sqlite://{}?mode=rwc", db_path.display());
    let dir = tmp.path().join("migrations");
    fs::create_dir_all(&dir).unwrap();
    Env { _tmp: tmp, url, dir }
}

fn write_migration(dir: &std::path::Path, version: i64, name: &str, up: &str, down: Option<&str>) {
    fs::write(dir.join(format!("{}_{}.sql", version, name)), up).unwrap();
    if let Some(d) = down {
        fs::write(dir.join(format!("{}_{}.down.sql", version, name)), d).unwrap();
    }
}

#[test]
fn detects_backend_from_url_scheme() {
    assert_eq!(
        db::detect_backend("postgres://u:p@h/d").unwrap(),
        db::Backend::Postgres
    );
    assert_eq!(
        db::detect_backend("sqlite:///tmp/x.db").unwrap(),
        db::Backend::Sqlite
    );
    assert_eq!(
        db::detect_backend("mysql://u:p@h/d").unwrap(),
        db::Backend::Mysql
    );
    assert!(db::detect_backend("mongodb://h").is_err());
}

#[test]
fn scan_empty_dir_is_empty() {
    let env = setup();
    let migs = discover::scan(&env.dir).unwrap();
    assert!(migs.is_empty());
}

#[test]
fn scan_sorts_by_version_and_skips_down_files() {
    let env = setup();
    write_migration(&env.dir, 2, "second", "SELECT 1;", Some("SELECT 2;"));
    write_migration(&env.dir, 1, "first", "SELECT 3;", None);
    let migs = discover::scan(&env.dir).unwrap();
    assert_eq!(migs.len(), 2);
    assert_eq!(migs[0].version, 1);
    assert_eq!(migs[1].version, 2);
    assert!(migs[0].down_path.is_none());
    assert!(migs[1].down_path.is_some());
}

#[test]
fn scan_rejects_duplicate_versions() {
    let env = setup();
    write_migration(&env.dir, 1, "a", "SELECT 1;", None);
    // same version, different name
    fs::write(env.dir.join("1_b.sql"), "SELECT 2;").unwrap();
    let err = discover::scan(&env.dir).unwrap_err();
    assert!(err.to_string().contains("duplicate migration version"));
}

#[tokio::test]
async fn up_applies_pending_and_tracks_them() {
    let env = setup();
    write_migration(
        &env.dir,
        1,
        "create_t",
        "CREATE TABLE t (id INTEGER PRIMARY KEY, v TEXT);",
        Some("DROP TABLE t;"),
    );
    write_migration(
        &env.dir,
        2,
        "insert_row",
        "INSERT INTO t (id, v) VALUES (1, 'a');",
        Some("DELETE FROM t WHERE id = 1;"),
    );
    let n = do_up(&env.url, &env.dir).await.unwrap();
    assert_eq!(n, 2);
    // re-running is a no-op
    let n2 = do_up(&env.url, &env.dir).await.unwrap();
    assert_eq!(n2, 0);
    // status: 2 applied, 0 pending
    let (a, p) = do_status(&env.url, &env.dir).await.unwrap();
    assert_eq!(a, 2);
    assert_eq!(p, 0);
}

#[tokio::test]
async fn down_reverts_last_n() {
    let env = setup();
    write_migration(
        &env.dir,
        1,
        "create_t",
        "CREATE TABLE t (id INTEGER PRIMARY KEY);",
        Some("DROP TABLE t;"),
    );
    write_migration(
        &env.dir,
        2,
        "add_col",
        "ALTER TABLE t ADD COLUMN v TEXT;",
        Some("-- down unsupported on sqlite but we just drop the table for this test\nDROP TABLE t; CREATE TABLE t (id INTEGER PRIMARY KEY);"),
    );
    do_up(&env.url, &env.dir).await.unwrap();
    let reverted = do_down(&env.url, &env.dir, 1).await.unwrap();
    assert_eq!(reverted, 1);
    let (a, p) = do_status(&env.url, &env.dir).await.unwrap();
    assert_eq!(a, 1);
    assert_eq!(p, 1);
}

#[tokio::test]
async fn down_refuses_irreversible_marker() {
    let env = setup();
    write_migration(
        &env.dir,
        1,
        "make_t",
        "CREATE TABLE t (id INTEGER PRIMARY KEY);",
        Some("-- IRREVERSIBLE\n-- don't auto-revert"),
    );
    do_up(&env.url, &env.dir).await.unwrap();
    let err = do_down(&env.url, &env.dir, 1).await.unwrap_err();
    assert!(err.to_string().contains("IRREVERSIBLE"));
}

#[tokio::test]
async fn up_detects_checksum_drift() {
    let env = setup();
    let up_path = env.dir.join("1_seed.sql");
    fs::write(&up_path, "CREATE TABLE t (id INTEGER PRIMARY KEY);").unwrap();
    do_up(&env.url, &env.dir).await.unwrap();
    // someone edits the already-applied file
    fs::write(&up_path, "CREATE TABLE t (id INTEGER PRIMARY KEY, extra TEXT);").unwrap();
    let err = do_up(&env.url, &env.dir).await.unwrap_err();
    assert!(err.to_string().contains("checksum drift"));
    // tracker module is exercised end-to-end here
    let _ = tracker::applied_versions; // keep tracker import used
}

#[test]
fn create_scaffolds_up_and_down_files() {
    let env = setup();
    let (up, down) = cmd_create::run(&env.dir, "add_users_index").unwrap();
    assert!(up.exists());
    assert!(down.exists());
    let up_txt = fs::read_to_string(&up).unwrap();
    let down_txt = fs::read_to_string(&down).unwrap();
    assert!(up_txt.contains("up migration"));
    assert!(down_txt.contains("down migration"));
}
