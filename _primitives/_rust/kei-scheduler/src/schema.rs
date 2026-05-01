//! kei-scheduler EntitySchema — declarative spec consumed by
//! `kei_entity_store::Store`.
//!
//! Schema matches the sibling pattern (kei-task, kei-chat-store): one
//! primary table with standard CRUD fields plus scheduler-specific
//! trigger + run-tracking columns. The `name` UNIQUE constraint rides
//! the engine's `custom_migrations` slot because `FieldDef` doesn't
//! expose a UNIQUE flag — a unique index on the column provides the
//! same semantics.

use kei_entity_store::schema::{EdgeKeyKind, EntitySchema, FieldDef};

static FIELDS: &[FieldDef] = &[
    FieldDef::pk("id"),
    FieldDef::text_nn("name"),
    FieldDef::text_nn("trigger_kind"),
    FieldDef::text_nn("trigger_spec"),
    FieldDef::text_nn("command"),
    FieldDef::text_default("status", "pending"),
    FieldDef::integer("last_run_at"),
    FieldDef::integer("next_run_at"),
    FieldDef::integer("last_exit_code"),
    FieldDef::created_at(),
    FieldDef::updated_at(),
];

const DDL_SECONDARY: &str = r#"
    CREATE UNIQUE INDEX IF NOT EXISTS idx_scheduler_tasks_name
        ON scheduler_tasks(name);
    CREATE INDEX IF NOT EXISTS idx_scheduler_tasks_status
        ON scheduler_tasks(status);
    CREATE INDEX IF NOT EXISTS idx_scheduler_tasks_next_run
        ON scheduler_tasks(next_run_at);
"#;

pub static SCHEDULER_SCHEMA: EntitySchema = EntitySchema {
    name: "scheduler_task",
    table: "scheduler_tasks",
    fields: FIELDS,
    // Generic verbs are disabled — scheduler-specific ops live in
    // `schedule.rs` / `query.rs` / `run.rs`. The engine still owns DDL.
    enabled_verbs: &[],
    fts_columns: None,
    edge_table: None,
    edge_key_kind: EdgeKeyKind::IntegerPair,
    archived_field: None,
    custom_migrations: &[DDL_SECONDARY],
};

/// Full schema list passed to the engine when opening a Store. Declared
/// here so the shim in `store.rs` stays a one-liner.
pub static ALL_SCHEMAS: &[&EntitySchema] = &[&SCHEDULER_SCHEMA];
