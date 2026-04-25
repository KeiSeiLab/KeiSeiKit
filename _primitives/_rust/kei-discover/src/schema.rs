//! `discover_index` EntitySchema — one row per announced primitive.
//!
//! Fields follow task.toml spec: `slug` (unique indexed), `author`,
//! `source_url`, `description`, `installed` (0/1 stored as INTEGER —
//! SQLite has no native bool), `last_seen_ts`, `created_at`,
//! `updated_at`. A UNIQUE INDEX on `slug` is emitted via
//! `custom_migrations` so duplicate registrations fail at the SQL layer
//! (mapped to `DiscoverError::DuplicateSlug` by the `register` module).
//!
//! FTS columns are `slug` + `description` — callers search by either.

use kei_entity_store::schema::{EdgeKeyKind, EntitySchema, FieldDef};

static FIELDS: &[FieldDef] = &[
    FieldDef::pk("id"),
    FieldDef::text_nn("slug"),
    FieldDef::text_nn("author"),
    FieldDef::text("source_url"),
    FieldDef::text("description"),
    FieldDef::integer("installed"),
    FieldDef::integer("last_seen_ts"),
    FieldDef::created_at(),
    FieldDef::updated_at(),
];

const DDL_UNIQUE_SLUG: &str =
    "CREATE UNIQUE INDEX IF NOT EXISTS ux_discover_slug ON discover_index(slug);";

pub static DISCOVER_SCHEMA: EntitySchema = EntitySchema {
    name: "discover",
    table: "discover_index",
    fields: FIELDS,
    enabled_verbs: &["create", "get", "list", "search", "update"],
    fts_columns: Some(&["slug", "description"]),
    edge_table: None,
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: None,
    custom_migrations: &[DDL_UNIQUE_SLUG],
};
