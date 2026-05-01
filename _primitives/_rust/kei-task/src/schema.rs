//! kei-task EntitySchema — declarative spec consumed by
//! `kei_entity_store::Store` and its verb templates.
//!
//! Columns match the legacy `CREATE TABLE tasks` DDL byte-for-byte so
//! on-disk databases created before the convergence layer continue to
//! work.
//!
//! Task-specific secondary tables (`milestones`, `task_deps`,
//! `task_milestones`) ride the engine's `custom_migrations` slot — they
//! are not generic CRUD and keep their existing column names so
//! `deps.rs` / `milestones.rs` / `graph.rs` don't need to change.

use kei_entity_store::schema::{EdgeKeyKind, EntitySchema, FieldDef};

static FIELDS: &[FieldDef] = &[
    FieldDef::pk("id"),
    FieldDef::text_nn("title"),
    FieldDef::text("description"),
    FieldDef::text_default("status", "pending"),
    FieldDef::text_default("priority", "medium"),
    FieldDef::text("task_type"),
    FieldDef::integer("parent_id"),
    FieldDef::text("assigned_to"),
    FieldDef::integer("due_date"),
    FieldDef::integer("completed_at"),
    FieldDef::created_at(),
    FieldDef::updated_at(),
];

const DDL_SECONDARY: &str = r#"
    CREATE INDEX IF NOT EXISTS idx_task_status ON tasks(status);
    CREATE INDEX IF NOT EXISTS idx_task_priority ON tasks(priority);
    CREATE INDEX IF NOT EXISTS idx_task_parent ON tasks(parent_id);

    CREATE TABLE IF NOT EXISTS milestones (
        id          INTEGER PRIMARY KEY,
        name        TEXT NOT NULL,
        description TEXT DEFAULT '',
        target_date INTEGER DEFAULT 0,
        status      TEXT DEFAULT 'open',
        created_at  INTEGER NOT NULL
    );

    CREATE TABLE IF NOT EXISTS task_deps (
        task_id    INTEGER NOT NULL,
        depends_on INTEGER NOT NULL,
        dep_type   TEXT DEFAULT 'blocks',
        PRIMARY KEY(task_id, depends_on)
    );
    CREATE INDEX IF NOT EXISTS idx_dep_depends ON task_deps(depends_on);

    CREATE TABLE IF NOT EXISTS task_milestones (
        task_id      INTEGER NOT NULL,
        milestone_id INTEGER NOT NULL,
        PRIMARY KEY(task_id, milestone_id)
    );
"#;

pub static TASK_SCHEMA: EntitySchema = EntitySchema {
    name: "task",
    table: "tasks",
    fields: FIELDS,
    enabled_verbs: &["create", "get", "list", "search", "update", "delete"],
    fts_columns: Some(&["title", "description"]),
    edge_table: None, // task_deps has bespoke column names — managed by deps.rs
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: None,
    custom_migrations: &[DDL_SECONDARY],
};
