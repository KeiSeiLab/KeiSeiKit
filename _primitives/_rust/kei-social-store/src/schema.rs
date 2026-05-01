//! kei-social-store EntitySchemas — Layer A convergence.
//!
//! Shape (multi-schema audit, 2026-04-23):
//!
//! - `SOCIAL_SCHEMA`: primary entity `person` (table `people`; INTEGER
//!   PK; engine-owned create/get/search/list + FTS).
//! - `ALL_SCHEMAS`: the `&[&EntitySchema]` slice for `Store::open`.
//!
//! Secondary tables stay in `custom_migrations` and keep bespoke SQL
//! paths — **none were promotable** on this pass:
//!
//! - `organizations` — uses `INSERT OR IGNORE` + re-query by
//!   `UNIQUE(name)` for idempotent name-keyed upsert (`orgs_idempotent`
//!   test relies on `add_org` returning the SAME id for repeat names).
//!   The engine `create` verb is plain INSERT, not OR-IGNORE, and
//!   would break that semantic. Sibling: `people.rs::add_org`.
//! - `interactions` — append-only log with `FOREIGN KEY ... ON DELETE
//!   CASCADE` on `person_id`, filter query `WHERE person_id=?`, and
//!   aggregate `GROUP BY person_id, target_id, channel` used by
//!   `graph.rs::relationship_graph`. None of these are covered by
//!   the engine's generic verbs. Sibling: `interactions.rs` + `graph.rs`.
//!
//! FTS columns cover name, handle, email, bio — search verb routes
//! through `fts_people`. The legacy `fts_social` virtual table is
//! replaced; FTS is rebuilt on first open against the new name.

use kei_entity_store::schema::{EdgeKeyKind, EntitySchema, FieldDef};

static FIELDS: &[FieldDef] = &[
    FieldDef::pk("id"),
    FieldDef::text_nn("name"),
    FieldDef::text("email"),
    FieldDef::text("handle"),
    FieldDef::text("role"),
    FieldDef::text("organization"),
    FieldDef::text_default("source", "manual"),
    FieldDef::text("bio"),
    FieldDef::created_at(),
    FieldDef::updated_at(),
];

const DDL_SECONDARY: &str = r#"
    CREATE UNIQUE INDEX IF NOT EXISTS idx_people_email
        ON people(email) WHERE email != '';
    CREATE UNIQUE INDEX IF NOT EXISTS idx_people_handle_source
        ON people(handle, source) WHERE handle != '';

    CREATE TABLE IF NOT EXISTS organizations (
        id          INTEGER PRIMARY KEY,
        name        TEXT NOT NULL UNIQUE,
        org_type    TEXT DEFAULT 'company',
        description TEXT DEFAULT '',
        created_at  INTEGER NOT NULL
    );

    CREATE TABLE IF NOT EXISTS interactions (
        id               INTEGER PRIMARY KEY,
        person_id        INTEGER NOT NULL REFERENCES people(id) ON DELETE CASCADE,
        target_id        INTEGER NOT NULL DEFAULT 0,
        interaction_type TEXT NOT NULL,
        channel          TEXT NOT NULL DEFAULT 'manual',
        content          TEXT DEFAULT '',
        timestamp        INTEGER NOT NULL,
        created_at       INTEGER NOT NULL
    );
    CREATE INDEX IF NOT EXISTS idx_int_person ON interactions(person_id);
"#;

pub static SOCIAL_SCHEMA: EntitySchema = EntitySchema {
    name: "person",
    table: "people",
    fields: FIELDS,
    enabled_verbs: &["create", "get", "search", "list"],
    fts_columns: Some(&["name", "handle", "email", "bio"]),
    edge_table: None, // interactions has bespoke columns — managed by interactions.rs
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: None,
    custom_migrations: &[DDL_SECONDARY],
};

/// Aggregate slice for `Store::open`. Currently a single-element slice;
/// lives here for parity with sister multi-schema stores
/// (kei-chat-store, kei-content-store) and future promotions.
pub static ALL_SCHEMAS: &[&EntitySchema] = &[&SOCIAL_SCHEMA];
