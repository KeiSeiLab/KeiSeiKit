//! Per-verb integration smoke tests on a fixture schema.

use kei_entity_store::schema::{EdgeKeyKind, EntitySchema, FieldDef};
use kei_entity_store::verbs::{create, delete, get, link, list, rank, search, update};
use kei_entity_store::Store;
use serde_json::json;

static FIELDS: &[FieldDef] = &[
    FieldDef::pk("id"),
    FieldDef::text_nn("title"),
    FieldDef::text("description"),
    FieldDef::text_default("status", "pending"),
    FieldDef::text_default("priority", "medium"),
    FieldDef::integer("parent_id"),
    FieldDef::created_at(),
    FieldDef::updated_at(),
];

static SCHEMA: EntitySchema = EntitySchema {
    name: "note",
    table: "notes",
    fields: FIELDS,
    enabled_verbs: &["create", "get", "list", "search", "update", "delete", "link", "rank"],
    fts_columns: Some(&["title", "description"]),
    edge_table: Some("note_edges"),
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: None,
    custom_migrations: &[],
};

fn mk() -> Store { Store::open_memory(&[&SCHEMA]).unwrap() }

fn create_one(s: &Store, title: &str) -> i64 {
    let v = create::run(s.conn(), &SCHEMA, json!({ "title": title })).unwrap();
    v["id"].as_i64().unwrap()
}

#[test]
fn create_then_get() {
    let s = mk();
    let id = create_one(&s, "alpha");
    let out = get::run(s.conn(), &SCHEMA, json!({ "id": id })).unwrap();
    assert_eq!(out["title"], "alpha");
    assert_eq!(out["status"], "pending");
    assert_eq!(out["priority"], "medium");
    assert!(out["created_at"].as_i64().unwrap() > 0);
}

#[test]
fn create_returns_id_and_created_at() {
    let s = mk();
    let v = create::run(s.conn(), &SCHEMA, json!({ "title": "x" })).unwrap();
    assert!(v["id"].as_i64().unwrap() >= 1);
    assert!(v["created_at"].as_i64().unwrap() > 0);
}

#[test]
fn list_paginated() {
    let s = mk();
    for i in 0..5 {
        create_one(&s, &format!("n{i}"));
    }
    let v = list::run(s.conn(), &SCHEMA, json!({ "limit": 3 })).unwrap();
    assert_eq!(v["results"].as_array().unwrap().len(), 3);
}

#[test]
fn search_fts() {
    let s = mk();
    create::run(
        s.conn(),
        &SCHEMA,
        json!({ "title": "refactor router", "description": "split monolith" }),
    )
    .unwrap();
    create_one(&s, "unrelated");
    let v = search::run(s.conn(), &SCHEMA, json!({ "query": "refactor" })).unwrap();
    assert_eq!(v["results"].as_array().unwrap().len(), 1);
}

#[test]
fn update_partial() {
    let s = mk();
    let id = create_one(&s, "orig");
    update::run(
        s.conn(),
        &SCHEMA,
        json!({ "id": id, "status": "in_progress" }),
    )
    .unwrap();
    let out = get::run(s.conn(), &SCHEMA, json!({ "id": id })).unwrap();
    assert_eq!(out["status"], "in_progress");
    assert_eq!(out["title"], "orig");
}

#[test]
fn update_not_found_errors() {
    let s = mk();
    let err = update::run(
        s.conn(),
        &SCHEMA,
        json!({ "id": 9999, "status": "x" }),
    )
    .unwrap_err();
    assert_eq!(err.exit_code(), 2);
}

#[test]
fn delete_removes_row() {
    let s = mk();
    let id = create_one(&s, "gone");
    delete::run(s.conn(), &SCHEMA, json!({ "id": id })).unwrap();
    let err = get::run(s.conn(), &SCHEMA, json!({ "id": id })).unwrap_err();
    assert_eq!(err.exit_code(), 2);
}

#[test]
fn link_and_rank() {
    let s = mk();
    let a = create_one(&s, "a");
    let b = create_one(&s, "b");
    let c = create_one(&s, "c");
    link::run(s.conn(), &SCHEMA, json!({ "from": a, "to": b })).unwrap();
    link::run(s.conn(), &SCHEMA, json!({ "from": a, "to": c })).unwrap();
    link::run(s.conn(), &SCHEMA, json!({ "from": b, "to": c })).unwrap();
    let v = rank::run(s.conn(), &SCHEMA, json!({})).unwrap();
    let results = v["results"].as_array().unwrap();
    assert_eq!(results.len(), 3);
    // c receives 2 inbound edges → highest rank.
    assert_eq!(results[0]["id"], c);
}

#[test]
fn create_missing_title_becomes_empty_string() {
    let s = mk();
    // engine does NOT validate NOT NULL content — the sibling atom does
    // (e.g. kei-task::atoms::create rejects empty title). Engine only
    // enforces SQL-level NOT NULL; defaults to empty string.
    let v = create::run(s.conn(), &SCHEMA, json!({})).unwrap();
    let id = v["id"].as_i64().unwrap();
    let out = get::run(s.conn(), &SCHEMA, json!({ "id": id })).unwrap();
    assert_eq!(out["title"], "");
}

#[test]
fn disabled_verb_errors() {
    static DISABLED: EntitySchema = EntitySchema {
        name: "ro",
        table: "ro_items",
        fields: FIELDS,
        enabled_verbs: &["get", "list"],
        fts_columns: None,
        edge_table: None,
        edge_key_kind: EdgeKeyKind::IntegerPair,
        archived_field: None,
        custom_migrations: &[],
    };
    let s = Store::open_memory(&[&DISABLED]).unwrap();
    let err = create::run(s.conn(), &DISABLED, json!({ "title": "x" })).unwrap_err();
    assert_eq!(err.exit_code(), 2);
}
