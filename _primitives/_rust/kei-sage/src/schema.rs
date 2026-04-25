//! SQLite schema — declarative via `kei_entity_store::EntitySchema`.
//!
//! Primary entity = `knowledge_units` ("unit"). Secondary tables (tags,
//! unit_tags, edges, fts_knowledge) ship as `custom_migrations` because
//! they pre-date the generic engine and carry sage-specific columns
//! (edge `id`/`weight`/`created_at`, FTS `unit_id`-named column, unique
//! partial index on `vault_path`).
//!
//! Why `edge_table: None` + `fts_columns: None`:
//!   - Engine's default `TextPair` edge layout lacks `id`/`weight`/
//!     `created_at` that sage's `list_outgoing` returns.
//!   - Engine's FTS auto-table name is `fts_<table>` with column
//!     `<table>_id` — sage uses `fts_knowledge` with column `unit_id`.
//!
//! The primary-table DDL produced by the engine matches the legacy
//! `knowledge_units` layout byte-for-byte (every column maps to an
//! engine `FieldKind`), so opening an existing sage DB stays idempotent.

use kei_entity_store::{EdgeKeyKind, EntitySchema, FieldDef};
use rusqlite::{Connection, Result};

/// Engine-owned primary-table fields for `knowledge_units`.
static UNIT_FIELDS: &[FieldDef] = &[
    FieldDef::pk("id"),
    FieldDef::text_nn("unit_type"),
    FieldDef::text_nn("title"),
    FieldDef::text("content"),
    FieldDef::text("evidence_grade"),
    FieldDef::text("source_path"),
    FieldDef::text("vault_path"),
    FieldDef::text("category"),
    FieldDef::created_at(),
    FieldDef::updated_at(),
];

/// Extra indexes on `knowledge_units` beyond the engine's per-field
/// auto-indexes. The unique partial index on `vault_path` is what makes
/// `INSERT OR REPLACE` idempotent by vault path in `Store::add_unit`.
const DDL_EXTRA_INDEXES: &str = r#"
    CREATE INDEX IF NOT EXISTS idx_ku_type ON knowledge_units(unit_type);
    CREATE UNIQUE INDEX IF NOT EXISTS idx_ku_vault
        ON knowledge_units(vault_path) WHERE vault_path != '';
    CREATE INDEX IF NOT EXISTS idx_ku_grade ON knowledge_units(evidence_grade);
"#;

/// Tags tables (currently unused by the CLI but preserved for parity
/// with the LBM port — external tooling may read them).
const DDL_TAGS: &str = r#"
    CREATE TABLE IF NOT EXISTS tags (
        id   INTEGER PRIMARY KEY,
        name TEXT UNIQUE NOT NULL
    );
    CREATE TABLE IF NOT EXISTS unit_tags (
        unit_id INTEGER NOT NULL REFERENCES knowledge_units(id) ON DELETE CASCADE,
        tag_id  INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
        PRIMARY KEY (unit_id, tag_id)
    );
"#;

/// Typed wikilink edges between `vault_path`s — src_path/dst_path text
/// keys plus sage-specific `id`/`weight`/`created_at`.
const DDL_EDGES: &str = r#"
    CREATE TABLE IF NOT EXISTS edges (
        id        INTEGER PRIMARY KEY,
        src_path  TEXT NOT NULL,
        dst_path  TEXT NOT NULL,
        edge_type TEXT NOT NULL,
        weight    REAL DEFAULT 1.0,
        created_at INTEGER NOT NULL,
        UNIQUE(src_path, dst_path, edge_type)
    );
    CREATE INDEX IF NOT EXISTS idx_sage_edges_src ON edges(src_path);
    CREATE INDEX IF NOT EXISTS idx_sage_edges_dst ON edges(dst_path);
"#;

/// FTS5 virtual table — legacy column name `unit_id` kept so existing
/// search/CRUD SQL in `search.rs` and `store.rs` compiles unchanged.
const DDL_FTS: &str = r#"
    CREATE VIRTUAL TABLE IF NOT EXISTS fts_knowledge
    USING fts5(unit_id UNINDEXED, title, content, tokenize='porter unicode61');
"#;

/// Declarative SSoT for sage's SQLite layout. `edge_key_kind` is
/// `TextPair` because sage's graph nodes are vault paths (strings), but
/// `edge_table: None` keeps the custom `edges` schema with extra
/// columns — engine-side `link`/`rank` verbs are not used today.
pub static SAGE_SCHEMA: EntitySchema = EntitySchema {
    name: "unit",
    table: "knowledge_units",
    fields: UNIT_FIELDS,
    enabled_verbs: &["create", "get", "search", "link", "rank"],
    fts_columns: None,
    edge_table: None,
    edge_key_kind: EdgeKeyKind::TextPair,
    archived_field: None,
    custom_migrations: &[DDL_EXTRA_INDEXES, DDL_TAGS, DDL_EDGES, DDL_FTS],
};

/// Apply schema + FTS5 virtual table. Idempotent.
///
/// Delegates to `kei_entity_store::engine::run_migrations` against
/// `SAGE_SCHEMA`. Preserved as a named entry point so downstream
/// callers and tests can still spell out the migration explicitly.
pub fn create_schema(conn: &Connection) -> Result<()> {
    kei_entity_store::engine::run_migrations(conn, &[&SAGE_SCHEMA])
        .map_err(|e| match e {
            kei_entity_store::VerbError::Sqlite(sq) => sq,
            other => rusqlite::Error::ToSqlConversionFailure(Box::new(
                std::io::Error::new(std::io::ErrorKind::Other, other.to_string()),
            )),
        })?;
    Ok(())
}
