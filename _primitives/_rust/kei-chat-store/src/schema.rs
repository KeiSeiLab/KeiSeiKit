//! kei-chat-store EntitySchemas — declarative specs consumed by
//! `kei_entity_store::Store` and its verb templates.
//!
//! Shape (multi-schema convergence, 2026-04-23):
//!
//! - `MESSAGES_SCHEMA`: primary entity `chat_messages` (INTEGER PK;
//!   engine-owned create/get/list/search + FTS reindex).
//! - `SESSIONS_SCHEMA`: second entity `chat_sessions` (TEXT UUID PK +
//!   `TextArchiveEnum` status column, engine-owned create/get/archive).
//!   Previously rode `custom_migrations`; now a first-class schema
//!   since `Store::open` accepts a slice of schemas.
//! - `ALL_SCHEMAS`: the `&[&EntitySchema]` slice the `Store` wrapper
//!   hands to the engine on open.
//!
//! The session aggregates (`message_count`, `total_tokens`, `total_cost`)
//! are still updated via bespoke SQL in `sessions.rs` because the
//! engine has no `increment-on-related-insert` verb. That bespoke path
//! shrank from "whole row lifecycle" to "UPDATE counters only".

use kei_entity_store::schema::{EdgeKeyKind, EntitySchema, FieldDef};

// ---- chat_messages ---------------------------------------------------

static MESSAGE_FIELDS: &[FieldDef] = &[
    FieldDef::pk("id"),
    FieldDef::text_nn("session_id"),
    FieldDef::text_nn("role"),
    FieldDef::text_nn("content"),
    FieldDef::integer("tokens_in"),
    FieldDef::integer("tokens_out"),
    FieldDef::real_default("cost", 0.0),
    FieldDef::created_at(),
];

/// Keep the idx_cm_session index around — generic schema has no
/// `indexed` flag for one-off single-column indexes on non-PK fields.
const MESSAGES_INDEX_DDL: &str =
    "CREATE INDEX IF NOT EXISTS idx_cm_session ON chat_messages(session_id);";

pub static MESSAGES_SCHEMA: EntitySchema = EntitySchema {
    name: "chat_message",
    table: "chat_messages",
    fields: MESSAGE_FIELDS,
    enabled_verbs: &["create", "get", "list", "search"],
    fts_columns: Some(&["content"]),
    edge_table: None,
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: None,
    custom_migrations: &[MESSAGES_INDEX_DDL],
};

// ---- chat_sessions ---------------------------------------------------

static SESSION_FIELDS: &[FieldDef] = &[
    FieldDef::text_pk("id"),
    FieldDef::text_nn("project"),
    FieldDef::text_default("title", ""),
    FieldDef::text_default("model", ""),
    FieldDef::text_archive_enum("status", "active", "archived"),
    FieldDef::integer("status_at"),
    FieldDef::integer("message_count"),
    FieldDef::integer("total_tokens"),
    FieldDef::real_default("total_cost", 0.0),
    FieldDef::created_at(),
    FieldDef::updated_at(),
];

/// Legacy indexes on chat_sessions (project, status). `indexed` flag on
/// FieldDef only covers single-column indexes with a deterministic
/// `idx_<table>_<col>` name — matches what we need here.
const SESSIONS_INDEX_DDL: &str = "\
    CREATE INDEX IF NOT EXISTS idx_cs_project ON chat_sessions(project);\n\
    CREATE INDEX IF NOT EXISTS idx_cs_status  ON chat_sessions(status);";

pub static SESSIONS_SCHEMA: EntitySchema = EntitySchema {
    name: "chat_session",
    table: "chat_sessions",
    fields: SESSION_FIELDS,
    enabled_verbs: &["create", "get", "archive", "update"],
    fts_columns: None,
    edge_table: None,
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: Some("status"),
    custom_migrations: &[SESSIONS_INDEX_DDL],
};

// ---- aggregate slice for Store::open -------------------------------

pub static ALL_SCHEMAS: &[&EntitySchema] = &[&MESSAGES_SCHEMA, &SESSIONS_SCHEMA];
