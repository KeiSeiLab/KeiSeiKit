//! Archive-verb smoke tests.
//!
//! Covers kei-chat-store migration: schemas opt-in to soft-delete via
//! `archived_field: Some("archived")`. The verb flips the column + an
//! optional `<field>_at` sibling timestamp.

use kei_entity_store::schema::{EdgeKeyKind, EntitySchema, FieldDef};
use kei_entity_store::verbs::{archive, create, get};
use kei_entity_store::Store;
use serde_json::json;

static ARCHIVABLE_FIELDS: &[FieldDef] = &[
    FieldDef::pk("id"),
    FieldDef::text_nn("title"),
    FieldDef::integer("archived"),
    FieldDef::integer("archived_at"),
    FieldDef::created_at(),
];

static ARCHIVABLE: EntitySchema = EntitySchema {
    name: "msg",
    table: "msgs",
    fields: ARCHIVABLE_FIELDS,
    enabled_verbs: &["create", "get", "archive"],
    fts_columns: None,
    edge_table: None,
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: Some("archived"),
    custom_migrations: &[],
};

static NO_ARCHIVE_FIELDS: &[FieldDef] = &[
    FieldDef::pk("id"),
    FieldDef::text_nn("title"),
    FieldDef::created_at(),
];

static WITHOUT_FIELD: EntitySchema = EntitySchema {
    name: "msg",
    table: "msgs_plain",
    fields: NO_ARCHIVE_FIELDS,
    enabled_verbs: &["create", "archive"],
    fts_columns: None,
    edge_table: None,
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: None,
    custom_migrations: &[],
};

#[test]
fn archive_sets_flag_and_stamps_timestamp() {
    let s = Store::open_memory(&[&ARCHIVABLE]).unwrap();
    let v = create::run(s.conn(), &ARCHIVABLE, json!({ "title": "hi" })).unwrap();
    let id = v["id"].as_i64().unwrap();

    let before: i64 = chrono::Utc::now().timestamp();
    let out = archive::run(s.conn(), &ARCHIVABLE, json!({ "id": id })).unwrap();
    assert_eq!(out["id"].as_i64().unwrap(), id);
    let stamped = out["archived_at"].as_i64().unwrap();
    assert!(stamped >= before);

    let row = get::run(s.conn(), &ARCHIVABLE, json!({ "id": id })).unwrap();
    assert_eq!(row["archived"].as_i64().unwrap(), 1);
    assert_eq!(row["archived_at"].as_i64().unwrap(), stamped);
}

#[test]
fn archive_preserves_id_and_other_fields() {
    let s = Store::open_memory(&[&ARCHIVABLE]).unwrap();
    let v = create::run(s.conn(), &ARCHIVABLE, json!({ "title": "keep" })).unwrap();
    let id = v["id"].as_i64().unwrap();
    archive::run(s.conn(), &ARCHIVABLE, json!({ "id": id })).unwrap();
    let row = get::run(s.conn(), &ARCHIVABLE, json!({ "id": id })).unwrap();
    assert_eq!(row["title"], "keep");
    assert_eq!(row["id"].as_i64().unwrap(), id);
}

#[test]
fn archive_errors_when_archived_field_missing() {
    let s = Store::open_memory(&[&WITHOUT_FIELD]).unwrap();
    let v = create::run(s.conn(), &WITHOUT_FIELD, json!({ "title": "x" })).unwrap();
    let id = v["id"].as_i64().unwrap();
    let err = archive::run(s.conn(), &WITHOUT_FIELD, json!({ "id": id })).unwrap_err();
    assert_eq!(err.exit_code(), 2);
}

#[test]
fn archive_not_found_errors() {
    let s = Store::open_memory(&[&ARCHIVABLE]).unwrap();
    let err = archive::run(s.conn(), &ARCHIVABLE, json!({ "id": 9999 })).unwrap_err();
    assert_eq!(err.exit_code(), 2);
}

#[test]
fn archive_rejects_missing_id() {
    let s = Store::open_memory(&[&ARCHIVABLE]).unwrap();
    let err = archive::run(s.conn(), &ARCHIVABLE, json!({})).unwrap_err();
    assert_eq!(err.exit_code(), 2);
}
