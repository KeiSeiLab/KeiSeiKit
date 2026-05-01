//! Multi-schema smoke tests — verify that `Store::open` accepts a
//! slice of `&EntitySchema`, runs every schema's migrations inside a
//! single transaction, and that verbs dispatched per-schema work
//! independently against the same underlying connection.
//!
//! Added 2026-04-23 with the multi-schema breaking change. Parity
//! target: unblock kei-chat-store from its single-schema constraint
//! (two entity types — integer-PK messages + text-PK sessions).

use kei_entity_store::schema::{EdgeKeyKind, EntitySchema, FieldDef};
use kei_entity_store::verbs::{create, get};
use kei_entity_store::Store;
use serde_json::json;

static MSG_FIELDS: &[FieldDef] = &[
    FieldDef::pk("id"),
    FieldDef::text_nn("session_id"),
    FieldDef::text_nn("content"),
    FieldDef::created_at(),
];

static MSGS: EntitySchema = EntitySchema {
    name: "msg",
    table: "msgs",
    fields: MSG_FIELDS,
    enabled_verbs: &["create", "get"],
    fts_columns: None,
    edge_table: None,
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: None,
    custom_migrations: &[],
};

static SESS_FIELDS: &[FieldDef] = &[
    FieldDef::text_pk("id"),
    FieldDef::text_nn("project"),
    FieldDef::text_archive_enum("status", "active", "archived"),
    FieldDef::integer("status_at"),
    FieldDef::created_at(),
];

static SESSIONS: EntitySchema = EntitySchema {
    name: "sess",
    table: "sessions",
    fields: SESS_FIELDS,
    enabled_verbs: &["create", "get", "archive"],
    fts_columns: None,
    edge_table: None,
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: Some("status"),
    custom_migrations: &[],
};

// ---------- 1. Both schemas' tables are created on open. ----------

#[test]
fn two_schema_store_creates_both_tables() {
    let s = Store::open_memory(&[&MSGS, &SESSIONS]).unwrap();
    // Both tables must exist; sqlite_master lookup is the cheapest
    // structural check available without leaning on a verb.
    let found: Vec<String> = s
        .conn()
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name IN ('msgs','sessions') ORDER BY name")
        .unwrap()
        .query_map([], |r| r.get::<_, String>(0))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(found, vec!["msgs".to_string(), "sessions".to_string()]);
}

// ---------- 2. Verbs dispatch per-schema on the same connection. ----------

#[test]
fn verbs_dispatch_per_schema_across_both_tables() {
    let s = Store::open_memory(&[&MSGS, &SESSIONS]).unwrap();

    // INSERT via integer-PK schema.
    let m = create::run(
        s.conn(),
        &MSGS,
        json!({ "session_id": "abc", "content": "hi" }),
    )
    .unwrap();
    let mid = m["id"].as_i64().unwrap();
    assert!(mid >= 1);

    // INSERT via text-PK schema — same connection, different schema.
    let uuid = "550e8400-e29b-41d4-a716-446655440000";
    create::run(
        s.conn(),
        &SESSIONS,
        json!({ "id": uuid, "project": "demo" }),
    )
    .unwrap();

    // GET via each schema returns only its table's row.
    let m_row = get::run(s.conn(), &MSGS, json!({ "id": mid })).unwrap();
    assert_eq!(m_row["content"], "hi");

    let s_row = get::run(s.conn(), &SESSIONS, json!({ "id": uuid })).unwrap();
    assert_eq!(s_row["project"], "demo");
    assert_eq!(s_row["status"], "active");
}

// ---------- 3. Migrations are transactional across schemas. ----------

/// Schema that deliberately breaks on migration (custom_migrations
/// references a column that doesn't exist). Used to prove that a
/// failing schema[1] rolls back schema[0]'s table too — confirming
/// atomic cross-schema migration.
static BROKEN: EntitySchema = EntitySchema {
    name: "broken",
    table: "broken_tbl",
    fields: MSG_FIELDS,
    enabled_verbs: &["create"],
    fts_columns: None,
    edge_table: None,
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: None,
    // Intentionally invalid SQL — references `nonexistent_tbl`.
    custom_migrations: &["CREATE INDEX bad_idx ON nonexistent_tbl(id);"],
};

#[test]
fn failing_migration_rolls_back_prior_schemas() {
    // Attempt to open with [good, broken]. The broken schema's
    // custom_migrations will error; the whole open must fail and
    // schema[0]'s `msgs` table must NOT exist afterwards (rollback).
    let path = tempfile::tempdir().unwrap();
    let db = path.path().join("atomic.db");
    let err = Store::open(&db, &[&MSGS, &BROKEN]);
    assert!(err.is_err(), "expected migration failure on BROKEN schema");

    // Re-open raw connection and verify neither table leaked through.
    let raw = rusqlite::Connection::open(&db).unwrap();
    let count: i64 = raw
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('msgs','broken_tbl')",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(
        count, 0,
        "failed multi-schema migration must roll back schema[0]'s tables too"
    );
}
