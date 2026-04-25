//! kei-content-store EntitySchemas — declarative specs consumed by
//! `kei_entity_store::Store` and its verb templates.
//!
//! Shape (multi-schema convergence, 2026-04-23):
//!
//! - `CONTENT_SCHEMA`: primary entity `content_units` (assets; INTEGER
//!   PK; engine-owned create/get/list/search/update/delete + FTS).
//! - `CAMPAIGNS_SCHEMA`: plain-CRUD INTEGER-PK table promoted to engine
//!   on this pass (create/get only — no idempotency or dedup).
//! - `ALL_SCHEMAS`: the `&[&EntitySchema]` slice `Store::open` hands
//!   to the engine.
//!
//! Secondary tables that stay in `custom_migrations` (on CONTENT_SCHEMA)
//! and keep bespoke SQL in their sibling modules:
//!
//! - `prompts` — hash-dedup via `INSERT OR IGNORE` + re-query by
//!   `UNIQUE(prompt_hash, model)`; engine `create` is plain INSERT,
//!   would break `prompt_dedup_by_hash` test. Sibling: `prompts.rs`.
//! - `campaign_assets` — composite `(campaign_id, asset_id)` PK, no
//!   single-column PK; engine schemas require one PK field. Also uses
//!   `INSERT OR IGNORE` for idempotent attach. Sibling: `campaigns.rs`.

use kei_entity_store::schema::{EdgeKeyKind, EntitySchema, FieldDef};

// ---- content_units (primary, assets) ---------------------------------

static FIELDS: &[FieldDef] = &[
    FieldDef::pk("id"),
    FieldDef::text_default("unit_type", "asset"),
    FieldDef::text_nn("title"),
    FieldDef::text("content"),
    FieldDef::text("media_type"),
    FieldDef::text("file_path"),
    FieldDef::text("file_hash"),
    FieldDef::text("provider"),
    FieldDef::integer("cost_cents"),
    FieldDef::integer("parent_id"),
    FieldDef::created_at(),
    FieldDef::updated_at(),
];

/// Secondary DDL co-located with `content_units` — indexes on the
/// primary table plus the two bespoke-CRUD tables (prompts,
/// campaign_assets). Kept byte-for-byte compatible with the legacy
/// pre-multi-schema DB layout.
const DDL_SECONDARY: &str = r#"
    CREATE INDEX IF NOT EXISTS idx_cu_type ON content_units(unit_type);
    CREATE INDEX IF NOT EXISTS idx_cu_hash ON content_units(file_hash) WHERE file_hash != '';

    CREATE TABLE IF NOT EXISTS prompts (
        id          INTEGER PRIMARY KEY,
        prompt_text TEXT NOT NULL,
        prompt_hash TEXT NOT NULL,
        prompt_type TEXT DEFAULT '',
        model       TEXT DEFAULT '',
        version     INTEGER DEFAULT 1,
        parent_id   INTEGER DEFAULT 0,
        created_at  INTEGER NOT NULL,
        UNIQUE(prompt_hash, model)
    );

    CREATE TABLE IF NOT EXISTS campaign_assets (
        campaign_id INTEGER NOT NULL,
        asset_id    INTEGER NOT NULL,
        PRIMARY KEY(campaign_id, asset_id)
    );
"#;

pub static CONTENT_SCHEMA: EntitySchema = EntitySchema {
    name: "asset",
    table: "content_units",
    fields: FIELDS,
    enabled_verbs: &["create", "get", "list", "search", "update", "delete"],
    fts_columns: Some(&["title", "content"]),
    edge_table: None,
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: None,
    custom_migrations: &[DDL_SECONDARY],
};

// ---- campaigns (promoted 2026-04-23) --------------------------------

static CAMPAIGN_FIELDS: &[FieldDef] = &[
    FieldDef::pk("id"),
    FieldDef::text_nn("name"),
    FieldDef::text_default("description", ""),
    FieldDef::text_default("status", "draft"),
    FieldDef::created_at(),
];

pub static CAMPAIGNS_SCHEMA: EntitySchema = EntitySchema {
    name: "campaign",
    table: "campaigns",
    fields: CAMPAIGN_FIELDS,
    enabled_verbs: &["create", "get"],
    fts_columns: None,
    edge_table: None,
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: None,
    custom_migrations: &[],
};

// ---- aggregate slice for Store::open --------------------------------

pub static ALL_SCHEMAS: &[&EntitySchema] = &[&CONTENT_SCHEMA, &CAMPAIGNS_SCHEMA];
