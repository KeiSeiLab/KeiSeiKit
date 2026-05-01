//! TextPair edge-key regression tests for link + rank verbs.
//!
//! Covers kei-sage migration target: `(src_path, dst_path)` TEXT
//! composite edge keys. Also keeps one IntegerPair regression case to
//! prove we did not disturb the default behaviour.

use kei_entity_store::schema::{EdgeKeyKind, EntitySchema, FieldDef, FieldKind};
use kei_entity_store::verbs::{link, rank};
use kei_entity_store::Store;
use serde_json::json;

static NODE_FIELDS: &[FieldDef] = &[
    FieldDef::pk("id"),
    FieldDef::text_nn("path"),
    FieldDef::created_at(),
];

static TEXT_SCHEMA: EntitySchema = EntitySchema {
    name: "doc",
    table: "docs",
    fields: NODE_FIELDS,
    enabled_verbs: &["link", "rank"],
    fts_columns: None,
    edge_table: Some("doc_edges"),
    edge_key_kind: EdgeKeyKind::TextPair,
    archived_field: None,
    custom_migrations: &[],
};

static INTEGER_SCHEMA: EntitySchema = EntitySchema {
    name: "doc",
    table: "docs_int",
    fields: NODE_FIELDS,
    enabled_verbs: &["link", "rank"],
    fts_columns: None,
    edge_table: Some("doc_edges_int"),
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: None,
    custom_migrations: &[],
};

#[test]
fn text_pair_link_and_lookup() {
    let s = Store::open_memory(&[&TEXT_SCHEMA]).unwrap();
    link::run(
        s.conn(),
        &TEXT_SCHEMA,
        json!({ "from": "a.md", "to": "b.md" }),
    )
    .unwrap();
    let count: i64 = s
        .conn()
        .query_row(
            "SELECT COUNT(*) FROM doc_edges WHERE src_path='a.md' AND dst_path='b.md'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 1);
}

#[test]
fn text_pair_link_idempotent() {
    let s = Store::open_memory(&[&TEXT_SCHEMA]).unwrap();
    for _ in 0..3 {
        link::run(
            s.conn(),
            &TEXT_SCHEMA,
            json!({ "from": "a.md", "to": "b.md", "edge_type": "links" }),
        )
        .unwrap();
    }
    let count: i64 = s
        .conn()
        .query_row("SELECT COUNT(*) FROM doc_edges", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 1);
}

#[test]
fn text_pair_rank_returns_string_ids() {
    let s = Store::open_memory(&[&TEXT_SCHEMA]).unwrap();
    let pairs = [("a.md", "b.md"), ("a.md", "c.md"), ("b.md", "c.md")];
    for (from, to) in pairs {
        link::run(s.conn(), &TEXT_SCHEMA, json!({ "from": from, "to": to })).unwrap();
    }
    let v = rank::run(s.conn(), &TEXT_SCHEMA, json!({})).unwrap();
    let results = v["results"].as_array().unwrap();
    assert_eq!(results.len(), 3);
    // c.md has 2 inbound edges → highest rank.
    assert_eq!(results[0]["id"], "c.md");
    assert!(results[0]["score"].as_f64().unwrap() > 0.0);
}

#[test]
fn text_pair_rejects_integer_input() {
    let s = Store::open_memory(&[&TEXT_SCHEMA]).unwrap();
    let err = link::run(s.conn(), &TEXT_SCHEMA, json!({ "from": 1, "to": 2 }))
        .unwrap_err();
    assert_eq!(err.exit_code(), 2);
}

// ---- Extended TextPairWithMetadata: custom col names + extra columns ----

static META_EXTRAS_SCHEMA: EntitySchema = EntitySchema {
    name: "xdoc",
    table: "xdocs",
    fields: NODE_FIELDS,
    enabled_verbs: &["link", "rank"],
    fts_columns: None,
    edge_table: Some("xdoc_edges"),
    edge_key_kind: EdgeKeyKind::TextPairWithMetadata {
        from_col: "from_uri",
        to_col: "to_uri",
        has_id: true,
        has_weight: true,
        has_created_at: true,
        extra_columns: &[
            ("evidence", FieldKind::Text),
            ("metadata", FieldKind::Text),
        ],
    },
    archived_field: None,
    custom_migrations: &[],
};

#[test]
fn text_pair_with_extras_roundtrip() {
    let s = Store::open_memory(&[&META_EXTRAS_SCHEMA]).unwrap();
    link::run(
        s.conn(),
        &META_EXTRAS_SCHEMA,
        json!({
            "from": "code://a.rs",
            "to": "note://n1",
            "edge_type": "refs",
            "weight": 2.5,
            "evidence": "E2",
            "metadata": "{\"tag\":\"important\"}",
        }),
    )
    .unwrap();
    let (w, ev, md): (f64, String, String) = s
        .conn()
        .query_row(
            "SELECT weight, evidence, metadata FROM xdoc_edges \
             WHERE from_uri='code://a.rs' AND to_uri='note://n1'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .unwrap();
    assert_eq!(w, 2.5);
    assert_eq!(ev, "E2");
    assert_eq!(md, "{\"tag\":\"important\"}");
}

#[test]
fn text_pair_with_custom_col_names_rank_uses_from_to_cols() {
    let s = Store::open_memory(&[&META_EXTRAS_SCHEMA]).unwrap();
    for (f, t) in [("a://x", "b://y"), ("a://x", "c://z"), ("b://y", "c://z")] {
        link::run(
            s.conn(),
            &META_EXTRAS_SCHEMA,
            json!({ "from": f, "to": t, "edge_type": "refs" }),
        )
        .unwrap();
    }
    let v = rank::run(s.conn(), &META_EXTRAS_SCHEMA, json!({})).unwrap();
    let results = v["results"].as_array().unwrap();
    assert_eq!(results.len(), 3);
    // c://z has 2 inbound edges → highest rank.
    assert_eq!(results[0]["id"], "c://z");
}

#[test]
fn integer_pair_still_works_after_refactor() {
    // Regression guard — kei-task uses IntegerPair implicitly.
    let s = Store::open_memory(&[&INTEGER_SCHEMA]).unwrap();
    link::run(s.conn(), &INTEGER_SCHEMA, json!({ "from": 1, "to": 2 })).unwrap();
    link::run(s.conn(), &INTEGER_SCHEMA, json!({ "from": 1, "to": 3 })).unwrap();
    link::run(s.conn(), &INTEGER_SCHEMA, json!({ "from": 2, "to": 3 })).unwrap();
    let v = rank::run(s.conn(), &INTEGER_SCHEMA, json!({})).unwrap();
    let results = v["results"].as_array().unwrap();
    assert_eq!(results.len(), 3);
    assert_eq!(results[0]["id"], 3);
}
