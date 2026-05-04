//! Schema migration tests (v1 → v5).
//!
//! Three scenarios:
//! 1. Simulate an "old" v1 database on disk, then re-open via `open_db` and
//!    confirm it auto-migrates to current SCHEMA_VERSION: new columns on
//!    `blocks`, new tables `block_predicates` + `block_deps` exist.
//! 2. Re-opening an already-current database is a no-op (idempotent).
//! 3. Migrating from v4 to v5 adds `cleanup_findings` table + 2 indices.

use kei_registry::open_db;
use kei_registry::store::{MIGRATIONS, SCHEMA_VERSION};
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

fn list_indices(conn: &Connection, table: &str) -> Vec<String> {
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='index' AND tbl_name = ?1")
        .unwrap();
    stmt.query_map([table], |r| r.get::<_, String>(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
}

#[test]
fn migrate_v1_to_current_on_existing_db() {
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
    assert_eq!(
        v, SCHEMA_VERSION as i64,
        "user_version must be {}",
        SCHEMA_VERSION
    );

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
fn migrate_idempotent_on_current_db() {
    let tmp = tempdir().unwrap();
    let db_path = tmp.path().join("registry.sqlite");

    // First open creates schema at SCHEMA_VERSION.
    drop(open_db(&db_path).unwrap());
    // Second open must observe current and apply zero migrations.
    let conn = open_db(&db_path).unwrap();
    let v: i64 = conn
        .pragma_query_value(None, "user_version", |r| r.get(0))
        .unwrap();
    assert_eq!(v, SCHEMA_VERSION as i64);
}

#[test]
fn migrate_v4_to_v5_adds_cleanup_findings() {
    let tmp = tempdir().unwrap();
    let db_path = tmp.path().join("registry.sqlite");

    // Stage 1 — bring DB up to v4 via the first 4 migrations only.
    {
        let seed = Connection::open(&db_path).unwrap();
        for sql in &MIGRATIONS[..4] {
            seed.execute_batch(sql).unwrap();
        }
        seed.pragma_update(None, "user_version", 4i64).unwrap();
    }

    // Pre-condition: at v4 there is NO cleanup_findings table.
    {
        let probe = Connection::open(&db_path).unwrap();
        let tables = list_tables(&probe);
        assert!(
            !tables.contains(&"cleanup_findings".to_string()),
            "v4 must not yet have cleanup_findings"
        );
    }

    // Stage 2 — open via `open_db` triggers v5 migration.
    let conn = open_db(&db_path).unwrap();
    let v: i64 = conn
        .pragma_query_value(None, "user_version", |r| r.get(0))
        .unwrap();
    assert_eq!(v, 5i64, "user_version must be 5 after v5 migration");

    // Stage 3 — cleanup_findings table exists.
    let tables = list_tables(&conn);
    assert!(
        tables.contains(&"cleanup_findings".to_string()),
        "cleanup_findings table missing after v5 migration"
    );

    // Stage 4 — required columns present.
    let cols = list_columns(&conn, "cleanup_findings");
    for needed in [
        "id",
        "workspace_sha",
        "timestamp_unix",
        "severity",
        "kind",
        "finding_json",
    ] {
        assert!(
            cols.contains(&needed.to_string()),
            "cleanup_findings missing column {}",
            needed
        );
    }

    // Stage 5 — both indices exist.
    let idx = list_indices(&conn, "cleanup_findings");
    assert!(
        idx.contains(&"idx_cleanup_workspace_sha".to_string()),
        "missing idx_cleanup_workspace_sha; have {:?}",
        idx
    );
    assert!(
        idx.contains(&"idx_cleanup_severity".to_string()),
        "missing idx_cleanup_severity; have {:?}",
        idx
    );
}
