//! Schema migration tests (v1 → v4).
//!
//! Two scenarios:
//! 1. Simulate an "old" v1 database on disk, then re-open via `open_db` and
//!    confirm it auto-migrates: `user_version=4`, new columns on `blocks`,
//!    new tables `block_predicates` + `block_deps` exist.
//! 2. Re-opening an already-v4 database is a no-op (migration is idempotent).

use kei_registry::store::{MIGRATIONS, SCHEMA_VERSION};
use kei_registry::open_db;
use rusqlite::Connection;
use tempfile::tempdir;

fn list_columns(conn: &Connection, table: &str) -> Vec<String> {
    let sql = format!("PRAGMA table_info({})", table);
    let mut stmt = conn.prepare(&sql).unwrap();
    stmt.query_map([], |r| r.get::<_, String>(1))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
}

fn list_tables(conn: &Connection) -> Vec<String> {
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")
        .unwrap();
    stmt.query_map([], |r| r.get::<_, String>(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
}

#[test]
fn migrate_v1_to_v4_on_existing_db() {
    let tmp = tempdir().unwrap();
    let db_path = tmp.path().join("registry.sqlite");

    // Stage 1 — write only the v1 migration directly, then mark as v1.
    {
        let seed = Connection::open(&db_path).unwrap();
        seed.execute_batch(MIGRATIONS[0]).unwrap();
        seed.pragma_update(None, "user_version", 1i64).unwrap();
    }

    // Stage 2 — `open_db` must auto-migrate to current SCHEMA_VERSION.
    let conn = open_db(&db_path).unwrap();
    let v: i64 = conn
        .pragma_query_value(None, "user_version", |r| r.get(0))
        .unwrap();
    assert_eq!(v, SCHEMA_VERSION as i64, "user_version must be {}", SCHEMA_VERSION);

    // Stage 3 — new columns on `blocks` are present.
    let cols = list_columns(&conn, "blocks");
    for needed in ["type_sig_json", "effects_json", "formula_source", "formula_sha"] {
        assert!(cols.contains(&needed.to_string()), "blocks missing column {}", needed);
    }

    // Stage 4 — new tables exist.
    let tables = list_tables(&conn);
    assert!(tables.contains(&"block_predicates".to_string()));
    assert!(tables.contains(&"block_deps".to_string()));
}

#[test]
fn migrate_idempotent_on_v4_db() {
    let tmp = tempdir().unwrap();
    let db_path = tmp.path().join("registry.sqlite");

    // First open creates schema at SCHEMA_VERSION.
    drop(open_db(&db_path).unwrap());
    // Second open must observe v4 and apply zero migrations.
    let conn = open_db(&db_path).unwrap();
    let v: i64 = conn
        .pragma_query_value(None, "user_version", |r| r.get(0))
        .unwrap();
    assert_eq!(v, SCHEMA_VERSION as i64);
}
