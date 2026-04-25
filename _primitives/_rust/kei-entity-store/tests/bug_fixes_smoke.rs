//! Regression tests for post-convergence audit findings (C1/C2/FTS5
//! injection/M3/TEXT-cap/M2). Each test names the finding it pins.

use kei_entity_store::error::VerbError;
use kei_entity_store::schema::{EdgeKeyKind, EntitySchema, FieldDef};
use kei_entity_store::verbs::{archive, create, delete, search, update};
use kei_entity_store::verbs::validate::MAX_TEXT_BYTES;
use kei_entity_store::Store;
use rusqlite::Connection;
use serde_json::json;

static FIELDS: &[FieldDef] = &[
    FieldDef::pk("id"),
    FieldDef::text_nn("title"),
    FieldDef::text("description"),
    FieldDef::text_default("status", "pending"),
    FieldDef::integer("parent_id"),
    FieldDef::created_at(),
    FieldDef::updated_at(),
];

static SCHEMA: EntitySchema = EntitySchema {
    name: "item",
    table: "items",
    fields: FIELDS,
    enabled_verbs: &["create", "get", "list", "search", "update", "delete"],
    fts_columns: Some(&["title", "description"]),
    edge_table: None,
    edge_key_kind: kei_entity_store::EdgeKeyKind::IntegerPair,
    archived_field: None,
    custom_migrations: &[],
};

fn mk() -> Store { Store::open_memory(&[&SCHEMA]).unwrap() }

// ---------- C1 — silent type coercion ----------

fn expect_invalid_type(err: VerbError, expected_field: &str) {
    match err {
        VerbError::InvalidType { ref field, .. } if field == expected_field => {}
        other => panic!("expected InvalidType on `{expected_field}`, got {other:?}"),
    }
}

#[test]
fn c1_create_rejects_integer_for_text_field() {
    let s = mk();
    let err = create::run(s.conn(), &SCHEMA, json!({ "title": 42 })).unwrap_err();
    assert_eq!(err.exit_code(), 2);
    expect_invalid_type(err, "title");
}

#[test]
fn c1_create_rejects_string_for_integer_field() {
    let s = mk();
    let err = create::run(
        s.conn(),
        &SCHEMA,
        json!({ "title": "ok", "parent_id": "not-a-number" }),
    )
    .unwrap_err();
    expect_invalid_type(err, "parent_id");
}

#[test]
fn c1_update_rejects_integer_for_text_field() {
    let s = mk();
    let id = create::run(s.conn(), &SCHEMA, json!({ "title": "orig" }))
        .unwrap()["id"]
        .as_i64()
        .unwrap();
    let err = update::run(s.conn(), &SCHEMA, json!({ "id": id, "status": 7 })).unwrap_err();
    expect_invalid_type(err, "status");
}

// ---------- C2 — FTS transaction ----------

#[test]
fn c2_update_fts_failure_rolls_back_row_update() {
    // Fresh DB, then manually drop the FTS table so the next update's
    // DELETE-INTO-FTS fails mid-flight. The row UPDATE that ran first
    // in the SAME transaction must roll back.
    let s = mk();
    let id = create::run(s.conn(), &SCHEMA, json!({ "title": "before" }))
        .unwrap()["id"]
        .as_i64()
        .unwrap();

    // Sabotage: drop the fts virtual table.
    s.conn().execute_batch("DROP TABLE fts_items;").unwrap();

    let result = update::run(
        s.conn(),
        &SCHEMA,
        json!({ "id": id, "title": "after" }),
    );
    assert!(result.is_err(), "update should fail when FTS is missing");

    // Row must still read as `before` — the UPDATE was rolled back.
    let title: String = s
        .conn()
        .query_row(
            "SELECT title FROM items WHERE id=?1",
            rusqlite::params![id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(title, "before", "update must have rolled back on FTS failure");
}

// ---------- FTS5 injection ----------

fn count_hits(s: &Store, q: &str) -> usize {
    let v = search::run(s.conn(), &SCHEMA, json!({ "query": q })).unwrap();
    v["results"].as_array().unwrap().len()
}

#[test]
fn fts5_injection_neutralized_by_phrase_quoting() {
    // Column-prefix / NEAR / wildcard all become literal tokens when
    // wrapped in the FTS5 double-quoted phrase. None should match the
    // seeded rows — no doc contains the literal text `title:secret`.
    let s = mk();
    create::run(s.conn(), &SCHEMA, json!({
        "title": "ordinary record", "description": "nothing special"
    })).unwrap();
    create::run(s.conn(), &SCHEMA, json!({
        "title": "secret handshake", "description": "hidden"
    })).unwrap();

    assert_eq!(count_hits(&s, "title:secret"), 0, "column-prefix leaked");
    assert_eq!(count_hits(&s, "NEAR(secret hidden, 5)"), 0, "NEAR leaked");
    assert_eq!(count_hits(&s, "secr*"), 0, "wildcard leaked");
}

#[test]
fn fts5_phrase_quoting_preserves_legitimate_queries() {
    // Inverse failure mode of the sanitizer: over-broad escape would
    // also destroy real tokens, so the injection test alone (hits==0)
    // would pass even for a broken `fts5_quote` that returns "". This
    // pins: a real token MUST still match the seeded row.
    let s = mk();
    create::run(s.conn(), &SCHEMA, json!({
        "title": "ordinary record", "description": "nothing special"
    })).unwrap();
    create::run(s.conn(), &SCHEMA, json!({
        "title": "secret handshake", "description": "hidden"
    })).unwrap();

    assert_eq!(count_hits(&s, "secret"), 1, "plain token must match");
    assert_eq!(count_hits(&s, "handshake"), 1, "second plain token must match");
    assert_eq!(count_hits(&s, "nothing"), 1, "description-side token must match");
}

#[test]
fn search_rejects_query_with_no_searchable_tokens() {
    // Punctuation-only query passes the trim().is_empty() check but
    // produces zero FTS5 tokens. Without the guard this would surface
    // as an opaque rusqlite syntax error (exit code 1). The typed
    // `InvalidInput` response keeps the exit-code-2 contract.
    let s = mk();
    create::run(s.conn(), &SCHEMA, json!({ "title": "anything" })).unwrap();

    let err = search::run(s.conn(), &SCHEMA, json!({ "query": "!@#$" })).unwrap_err();
    assert_eq!(err.exit_code(), 2, "must map to validation exit code");
    match err {
        VerbError::InvalidInput(ref msg) => assert!(
            msg.contains("no searchable tokens"),
            "message should identify the tokenization failure, got: {msg}"
        ),
        other => panic!("expected InvalidInput, got {other:?}"),
    }

    // Also cover whitespace + punctuation combo and long punctuation.
    let err = search::run(s.conn(), &SCHEMA, json!({ "query": "   ...   " })).unwrap_err();
    assert_eq!(err.exit_code(), 2);
    let err = search::run(s.conn(), &SCHEMA, json!({ "query": "-+=*/" })).unwrap_err();
    assert_eq!(err.exit_code(), 2);
}

// ---------- DdlError — unsupported extra_column FieldKind ----------

#[test]
fn ddl_try_edge_table_for_rejects_unsupported_kind() {
    // Reachable from public API: `EdgeKeyKind::TextPairWithMetadata
    // { extra_columns: [("x", FieldKind::TextDefault)] }`. Must return
    // a typed DdlError, not panic. Integration-level proof that
    // Store::open's migration path maps this to InvalidInput (exit 2).
    use kei_entity_store::ddl::try_edge_table_for;
    use kei_entity_store::ddl_error::DdlError;
    use kei_entity_store::schema::FieldKind;

    static BAD_EXTRAS: &[(&str, FieldKind)] = &[("bogus", FieldKind::TextDefault)];
    let kind = EdgeKeyKind::TextPairWithMetadata {
        from_col: "from_uri",
        to_col: "to_uri",
        has_id: true,
        has_weight: true,
        has_created_at: true,
        extra_columns: BAD_EXTRAS,
    };

    let err = try_edge_table_for("edges_bad", kind).unwrap_err();
    match err {
        DdlError::UnsupportedExtraColumn { ref column_name, ref kind_debug } => {
            assert_eq!(column_name, "bogus");
            assert!(
                kind_debug.contains("TextDefault"),
                "kind_debug should name the offending FieldKind, got {kind_debug}"
            );
        }
    }
}

#[test]
fn store_open_maps_ddl_error_to_verb_error() {
    // End-to-end: Store::open_memory on a schema with a bad
    // extra_columns kind must surface the error through the
    // `anyhow::Error` chain rather than panicking the thread.
    use kei_entity_store::schema::FieldKind;

    static BAD_EXTRAS: &[(&str, FieldKind)] = &[("bogus", FieldKind::TextDefault)];
    static BAD_FIELDS: &[FieldDef] = &[FieldDef::pk("id")];
    static BAD_SCHEMA: EntitySchema = EntitySchema {
        name: "bad",
        table: "bad_nodes",
        fields: BAD_FIELDS,
        enabled_verbs: &[],
        fts_columns: None,
        edge_table: Some("bad_edges"),
        edge_key_kind: EdgeKeyKind::TextPairWithMetadata {
            from_col: "from_uri",
            to_col: "to_uri",
            has_id: true,
            has_weight: true,
            has_created_at: true,
            extra_columns: BAD_EXTRAS,
        },
        archived_field: None,
        custom_migrations: &[],
    };

    let res = Store::open_memory(&[&BAD_SCHEMA]);
    let err = match res {
        Ok(_) => panic!("Store::open_memory must reject bad schema, not panic / succeed"),
        Err(e) => e,
    };
    let msg = format!("{err:#}");
    assert!(
        msg.contains("bogus") && msg.contains("TextDefault"),
        "error chain should mention column + kind, got: {msg}"
    );
}

// ---------- TEXT size cap ----------

#[test]
fn text_cap_create_rejects_oversize() {
    let s = mk();
    let oversize: String = "a".repeat(MAX_TEXT_BYTES + 1);
    let err = create::run(s.conn(), &SCHEMA, json!({ "title": oversize })).unwrap_err();
    expect_invalid_type(err, "title");
}

#[test]
fn text_cap_update_rejects_oversize() {
    let s = mk();
    let id = create::run(s.conn(), &SCHEMA, json!({ "title": "ok" }))
        .unwrap()["id"]
        .as_i64()
        .unwrap();
    let oversize: String = "a".repeat(MAX_TEXT_BYTES + 1);
    let err = update::run(
        s.conn(),
        &SCHEMA,
        json!({ "id": id, "description": oversize }),
    )
    .unwrap_err();
    expect_invalid_type(err, "description");
}

// ---------- M2 — migration version ----------

#[test]
fn m2_user_version_stamped_on_fresh_db() {
    let s = mk();
    let v: u32 = s
        .conn()
        .pragma_query_value(None, "user_version", |r| r.get(0))
        .unwrap();
    assert_eq!(v, kei_entity_store::engine::CURRENT_USER_VERSION);
}

#[test]
fn m2_user_version_applied_once_idempotent() {
    // Open twice — second open must leave user_version unchanged (not
    // bumped past CURRENT).
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("store.db");
    {
        let _s = Store::open(&path, &[&SCHEMA]).unwrap();
    }
    {
        let _s = Store::open(&path, &[&SCHEMA]).unwrap();
        let conn = Connection::open(&path).unwrap();
        let v: u32 = conn
            .pragma_query_value(None, "user_version", |r| r.get(0))
            .unwrap();
        assert_eq!(v, kei_entity_store::engine::CURRENT_USER_VERSION);
    }
}

// ---------- delete/archive transaction semantics ----------
//
// Two audit findings pinned here:
//   (1) delete.rs hard-path wraps FTS DELETE + table DELETE in one tx
//       (was two separate execs — partial failure orphaned FTS rows).
//   (2) archive.rs removes the row from FTS inside the same tx as the
//       UPDATE (was UPDATE only — archived rows stayed searchable).

static ARCHIVABLE_FIELDS: &[FieldDef] = &[
    FieldDef::pk("id"),
    FieldDef::text_nn("title"),
    FieldDef::text("description"),
    FieldDef::integer("status"),
    FieldDef::integer("status_at"),
    FieldDef::created_at(),
];

/// FTS + archived_field both configured — required for
/// `archive_removes_from_fts` and `archive_rollback_on_sabotage`.
static ARCHIVABLE_FTS: EntitySchema = EntitySchema {
    name: "item",
    table: "items",
    fields: ARCHIVABLE_FIELDS,
    enabled_verbs: &["create", "get", "search", "archive"],
    fts_columns: Some(&["title", "description"]),
    edge_table: None,
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: Some("status"),
    custom_migrations: &[],
};

static NO_FTS_FIELDS: &[FieldDef] = &[
    FieldDef::pk("id"),
    FieldDef::text_nn("title"),
    FieldDef::created_at(),
];

/// No FTS — required for `delete_succeeds_when_no_fts_configured`.
static NO_FTS_SCHEMA: EntitySchema = EntitySchema {
    name: "item",
    table: "plain_items",
    fields: NO_FTS_FIELDS,
    enabled_verbs: &["create", "get", "delete"],
    fts_columns: None,
    edge_table: None,
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: None,
    custom_migrations: &[],
};

#[test]
fn delete_rollback_on_fts_sabotage() {
    // Hard-delete path must roll back when the FTS DELETE fails — the
    // row itself must survive. Pins: delete.rs tx wrapping (finding 1).
    let s = mk();
    let id = create::run(s.conn(), &SCHEMA, json!({ "title": "keepme" }))
        .unwrap()["id"]
        .as_i64()
        .unwrap();

    s.conn().execute_batch("DROP TABLE fts_items;").unwrap();

    let result = delete::run(s.conn(), &SCHEMA, json!({ "id": id }));
    assert!(result.is_err(), "delete must fail when FTS is missing");

    let count: i64 = s
        .conn()
        .query_row(
            "SELECT COUNT(*) FROM items WHERE id=?1",
            rusqlite::params![id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 1, "row must survive FTS-failed delete (rollback)");
}

#[test]
fn archive_removes_from_fts() {
    // Archive must remove the row from FTS so `search` no longer returns
    // it. Pins: archive.rs fts-delete-in-tx (finding 2).
    let s = Store::open_memory(&[&ARCHIVABLE_FTS]).unwrap();
    let id = create::run(
        s.conn(),
        &ARCHIVABLE_FTS,
        json!({ "title": "uniqword42", "description": "findme" }),
    )
    .unwrap()["id"]
    .as_i64()
    .unwrap();

    // Before archive: search finds the row.
    let before =
        search::run(s.conn(), &ARCHIVABLE_FTS, json!({ "query": "uniqword42" })).unwrap();
    assert_eq!(
        before["results"].as_array().unwrap().len(),
        1,
        "row must be searchable before archive"
    );

    archive::run(s.conn(), &ARCHIVABLE_FTS, json!({ "id": id })).unwrap();

    // After archive: search returns zero hits.
    let after =
        search::run(s.conn(), &ARCHIVABLE_FTS, json!({ "query": "uniqword42" })).unwrap();
    assert_eq!(
        after["results"].as_array().unwrap().len(),
        0,
        "archived row must not be searchable"
    );
}

#[test]
fn archive_rollback_on_sabotage() {
    // Archive wraps UPDATE + FTS DELETE in one tx. If the FTS delete
    // fails, the UPDATE must roll back too. Pins: archive.rs tx
    // wrapping (finding 2).
    let s = Store::open_memory(&[&ARCHIVABLE_FTS]).unwrap();
    let id = create::run(
        s.conn(),
        &ARCHIVABLE_FTS,
        json!({ "title": "stable", "description": "x" }),
    )
    .unwrap()["id"]
    .as_i64()
    .unwrap();

    s.conn().execute_batch("DROP TABLE fts_items;").unwrap();

    let result = archive::run(s.conn(), &ARCHIVABLE_FTS, json!({ "id": id }));
    assert!(result.is_err(), "archive must fail when FTS is missing");

    // Status column must still be 0 (unarchived) — the UPDATE rolled back.
    let status: i64 = s
        .conn()
        .query_row(
            "SELECT status FROM items WHERE id=?1",
            rusqlite::params![id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(
        status, 0,
        "archived_field must be unchanged after FTS-failed archive"
    );
}

#[test]
fn delete_succeeds_when_no_fts_configured() {
    // Delete on a schema with `fts_columns: None` must work — the tx
    // wrapping must not introduce a spurious FTS DELETE.
    let s = Store::open_memory(&[&NO_FTS_SCHEMA]).unwrap();
    let id = create::run(s.conn(), &NO_FTS_SCHEMA, json!({ "title": "bye" }))
        .unwrap()["id"]
        .as_i64()
        .unwrap();

    delete::run(s.conn(), &NO_FTS_SCHEMA, json!({ "id": id })).unwrap();

    let count: i64 = s
        .conn()
        .query_row(
            "SELECT COUNT(*) FROM plain_items WHERE id=?1",
            rusqlite::params![id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 0, "row must be gone after hard delete on no-FTS schema");
}
