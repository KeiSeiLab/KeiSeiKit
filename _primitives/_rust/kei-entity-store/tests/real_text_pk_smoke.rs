//! Smoke tests for the four M1 / M4 / M5 engine improvements:
//!
//! 1. `FieldKind::TextPk` — TEXT primary key schemas with caller-
//!    supplied UUID-style ids.
//! 2. `FieldKind::Real` / `RealDefault` — REAL columns round-tripped as
//!    f64 through create + get.
//! 3. `FieldKind::TextArchiveEnum` — archive verb writes the archived
//!    sentinel string on schemas that encode status as a TEXT enum.
//! 4. `EdgeKeyKind::TextPairWithMetadata` — text-keyed edges with
//!    optional weight / id / created_at columns, used by rank.

use kei_entity_store::schema::{EdgeKeyKind, EntitySchema, FieldDef};
use kei_entity_store::verbs::{archive, create, delete, get, link, rank, update};
use kei_entity_store::Store;
use serde_json::json;

// ---------- 1. TextPk ----------

static SESSION_FIELDS: &[FieldDef] = &[
    FieldDef::text_pk("id"),
    FieldDef::text_nn("title"),
    FieldDef::created_at(),
];

static SESSION_SCHEMA: EntitySchema = EntitySchema {
    name: "session",
    table: "sessions",
    fields: SESSION_FIELDS,
    enabled_verbs: &["create", "get", "list", "update", "delete"],
    fts_columns: None,
    edge_table: None,
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: None,
    custom_migrations: &[],
};

#[test]
fn text_pk_create_with_string_id_and_get_by_string_id() {
    let s = Store::open_memory(&[&SESSION_SCHEMA]).unwrap();
    let uuid = "550e8400-e29b-41d4-a716-446655440000";
    let out = create::run(
        s.conn(),
        &SESSION_SCHEMA,
        json!({ "id": uuid, "title": "first session" }),
    )
    .unwrap();
    assert_eq!(out["id"], uuid);
    assert!(out["created_at"].as_i64().unwrap() > 0);

    let got = get::run(s.conn(), &SESSION_SCHEMA, json!({ "id": uuid })).unwrap();
    assert_eq!(got["id"], uuid);
    assert_eq!(got["title"], "first session");
}

#[test]
fn text_pk_update_and_delete_by_string_id() {
    let s = Store::open_memory(&[&SESSION_SCHEMA]).unwrap();
    let uuid = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
    create::run(
        s.conn(),
        &SESSION_SCHEMA,
        json!({ "id": uuid, "title": "orig" }),
    )
    .unwrap();
    update::run(
        s.conn(),
        &SESSION_SCHEMA,
        json!({ "id": uuid, "title": "updated" }),
    )
    .unwrap();
    let got = get::run(s.conn(), &SESSION_SCHEMA, json!({ "id": uuid })).unwrap();
    assert_eq!(got["title"], "updated");

    delete::run(s.conn(), &SESSION_SCHEMA, json!({ "id": uuid })).unwrap();
    let err = get::run(s.conn(), &SESSION_SCHEMA, json!({ "id": uuid })).unwrap_err();
    assert_eq!(err.exit_code(), 2);
}

#[test]
fn text_pk_create_rejects_missing_id() {
    let s = Store::open_memory(&[&SESSION_SCHEMA]).unwrap();
    let err = create::run(s.conn(), &SESSION_SCHEMA, json!({ "title": "x" })).unwrap_err();
    assert_eq!(err.exit_code(), 2);
}

// ---------- 2. Real + RealDefault ----------

static COST_FIELDS: &[FieldDef] = &[
    FieldDef::pk("id"),
    FieldDef::text_nn("label"),
    FieldDef::real("cost"),
    FieldDef::real_default("multiplier", 1.5),
    FieldDef::created_at(),
];

static COST_SCHEMA: EntitySchema = EntitySchema {
    name: "cost_entry",
    table: "cost_entries",
    fields: COST_FIELDS,
    enabled_verbs: &["create", "get"],
    fts_columns: None,
    edge_table: None,
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: None,
    custom_migrations: &[],
};

#[test]
fn real_column_round_trips_f64_unchanged() {
    let s = Store::open_memory(&[&COST_SCHEMA]).unwrap();
    let v = create::run(
        s.conn(),
        &COST_SCHEMA,
        json!({ "label": "gpt-4", "cost": 0.03125 }),
    )
    .unwrap();
    let id = v["id"].as_i64().unwrap();
    let row = get::run(s.conn(), &COST_SCHEMA, json!({ "id": id })).unwrap();
    assert_eq!(row["cost"].as_f64().unwrap(), 0.03125);
}

#[test]
fn real_default_applies_when_missing() {
    let s = Store::open_memory(&[&COST_SCHEMA]).unwrap();
    let v = create::run(
        s.conn(),
        &COST_SCHEMA,
        json!({ "label": "claude", "cost": 0.01 }),
    )
    .unwrap();
    let id = v["id"].as_i64().unwrap();
    let row = get::run(s.conn(), &COST_SCHEMA, json!({ "id": id })).unwrap();
    assert_eq!(row["multiplier"].as_f64().unwrap(), 1.5);
}

// ---------- 3. TextArchiveEnum ----------

static CHAT_FIELDS: &[FieldDef] = &[
    FieldDef::text_pk("id"),
    FieldDef::text_nn("project"),
    FieldDef::text_archive_enum("status", "active", "archived"),
    FieldDef::integer("status_at"),
    FieldDef::created_at(),
];

static CHAT_SCHEMA: EntitySchema = EntitySchema {
    name: "chat_session",
    table: "chat_sessions",
    fields: CHAT_FIELDS,
    enabled_verbs: &["create", "get", "archive"],
    fts_columns: None,
    edge_table: None,
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: Some("status"),
    custom_migrations: &[],
};

#[test]
fn archive_textenum_writes_archived_sentinel_string() {
    let s = Store::open_memory(&[&CHAT_SCHEMA]).unwrap();
    let uuid = "aaaa-bbbb-cccc";
    create::run(
        s.conn(),
        &CHAT_SCHEMA,
        json!({ "id": uuid, "project": "test" }),
    )
    .unwrap();

    // Before archive: status defaults to "active" sentinel.
    let before = get::run(s.conn(), &CHAT_SCHEMA, json!({ "id": uuid })).unwrap();
    assert_eq!(before["status"], "active");

    let out = archive::run(s.conn(), &CHAT_SCHEMA, json!({ "id": uuid })).unwrap();
    assert_eq!(out["id"], uuid);
    let stamped = out["archived_at"].as_i64().unwrap();
    assert!(stamped > 0);

    let after = get::run(s.conn(), &CHAT_SCHEMA, json!({ "id": uuid })).unwrap();
    assert_eq!(after["status"], "archived");
    assert_eq!(after["status_at"].as_i64().unwrap(), stamped);
}

// ---------- 4. TextPairWithMetadata ----------

static NODE_FIELDS: &[FieldDef] = &[
    FieldDef::pk("id"),
    FieldDef::text_nn("path"),
    FieldDef::created_at(),
];

static META_EDGE_SCHEMA: EntitySchema = EntitySchema {
    name: "doc",
    table: "docs_meta",
    fields: NODE_FIELDS,
    enabled_verbs: &["link", "rank"],
    fts_columns: None,
    edge_table: Some("doc_edges_meta"),
    edge_key_kind: EdgeKeyKind::TextPairWithMetadata {
        from_col: "src_path",
        to_col: "dst_path",
        has_id: true,
        has_weight: true,
        has_created_at: true,
        extra_columns: &[],
    },
    archived_field: None,
    custom_migrations: &[],
};

#[test]
fn text_pair_metadata_link_stores_weight_and_timestamp() {
    let s = Store::open_memory(&[&META_EDGE_SCHEMA]).unwrap();
    link::run(
        s.conn(),
        &META_EDGE_SCHEMA,
        json!({ "from": "a.md", "to": "b.md", "weight": 3.5 }),
    )
    .unwrap();
    let (weight, created_at): (f64, i64) = s
        .conn()
        .query_row(
            "SELECT weight, created_at FROM doc_edges_meta \
             WHERE src_path='a.md' AND dst_path='b.md'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(weight, 3.5);
    assert!(created_at > 0);
}

#[test]
fn text_pair_metadata_rank_respects_weight() {
    // Graph: a → b (weight 10), a → c (weight 1). b and c both sink.
    // Weighted PageRank should push `b` above `c`; unweighted would
    // split flow 50/50 (→ tie or identical rank).
    let s = Store::open_memory(&[&META_EDGE_SCHEMA]).unwrap();
    link::run(
        s.conn(),
        &META_EDGE_SCHEMA,
        json!({ "from": "a.md", "to": "b.md", "weight": 10.0 }),
    )
    .unwrap();
    link::run(
        s.conn(),
        &META_EDGE_SCHEMA,
        json!({ "from": "a.md", "to": "c.md", "weight": 1.0 }),
    )
    .unwrap();

    let v = rank::run(s.conn(), &META_EDGE_SCHEMA, json!({})).unwrap();
    let results = v["results"].as_array().unwrap();
    let score_of = |id: &str| -> f64 {
        results
            .iter()
            .find(|r| r["id"].as_str().unwrap() == id)
            .unwrap()["score"]
            .as_f64()
            .unwrap()
    };
    let b = score_of("b.md");
    let c = score_of("c.md");
    assert!(
        b > c * 1.5,
        "expected weighted rank b ({b}) to exceed c ({c}) by > 1.5x"
    );
}
