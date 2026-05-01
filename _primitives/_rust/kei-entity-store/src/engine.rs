//! Store — thin wrapper over `rusqlite::Connection` that runs the
//! schemas' migration DDL on open.
//!
//! The engine does NOT take ownership of verb dispatch. Sibling crates
//! call verb modules directly (e.g. `verbs::create::run(&mut conn,
//! &SCHEMA, input)`). This keeps the engine a passive provider of
//! connection + schema-aware DDL.
//!
//! As of the multi-schema breaking change (2026-04-23), `Store::open`
//! accepts a SLICE of `&EntitySchema`. Every schema's DDL runs inside
//! a SINGLE transaction — if schema[i] migration fails, schema[0..i]
//! rolls back too. Verbs remain per-schema-dispatched by the caller.

use crate::ddl;
use crate::error::VerbError;
use crate::schema::EntitySchema;
use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::Path;

/// Schema-level version stamped into SQLite's `user_version` pragma on
/// first open. Future migrations bump this constant and gate their DDL
/// on the pragma's current value — idempotent `CREATE TABLE IF NOT
/// EXISTS` is not enough once column shapes diverge.
pub const CURRENT_USER_VERSION: u32 = 1;

pub struct Store {
    conn: Connection,
}

impl Store {
    /// Open (creates parent dirs, enables WAL, runs migrations for all
    /// schemas in a single transaction).
    ///
    /// WAL mode is a best-effort optimisation — some filesystems (NFS,
    /// read-only mounts, certain FUSE backends) refuse the pragma. On
    /// failure we emit a single-line stderr notice and fall back to the
    /// default rollback journal instead of swallowing the error; the
    /// store still opens correctly and the exit-code contract is
    /// preserved (WAL unavailability is not fatal by design).
    pub fn open(path: &Path, schemas: &[&EntitySchema]) -> Result<Self> {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let conn = Connection::open(path).context("open sqlite")?;
        if let Err(e) = conn.pragma_update(None, "journal_mode", "WAL") {
            eprintln!(
                "kei-entity-store: WAL mode unavailable at {} ({}); \
                 falling back to rollback journal",
                path.display(),
                e
            );
        }
        run_migrations(&conn, schemas)?;
        Ok(Self { conn })
    }

    /// In-memory store — unit-test constructor. Same migrations, no FS.
    pub fn open_memory(schemas: &[&EntitySchema]) -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        run_migrations(&conn, schemas)?;
        Ok(Self { conn })
    }

    pub fn conn(&self) -> &Connection { &self.conn }
    pub fn conn_mut(&mut self) -> &mut Connection { &mut self.conn }

    /// Escape hatch: consume the Store and return the raw Connection.
    /// Callers that still need direct SQL (kei-task milestones,
    /// cycle-detection) can use this during migration.
    pub fn into_conn(self) -> Connection { self.conn }
}

/// Run all schemas' migrations atomically. For each schema: primary
/// table, indexes, FTS virtual table, edge table, custom DDL. Finally
/// stamp `user_version`. The transaction rolls back entirely if any
/// schema fails — callers never see a half-migrated DB.
pub fn run_migrations(
    conn: &Connection,
    schemas: &[&EntitySchema],
) -> Result<(), VerbError> {
    let tx = conn.unchecked_transaction()?;
    for schema in schemas {
        apply_schema(&tx, schema)?;
    }
    apply_user_version(&tx)?;
    tx.commit()?;
    Ok(())
}

/// Apply one schema's DDL set inside an already-open transaction.
fn apply_schema(
    tx: &rusqlite::Transaction<'_>,
    schema: &EntitySchema,
) -> Result<(), VerbError> {
    tx.execute_batch(&ddl::primary_table(schema))?;
    tx.execute_batch(&ddl::indexes(schema))?;
    if let Some(cols) = schema.fts_columns {
        tx.execute_batch(&ddl::fts_table(schema.table, cols))?;
    }
    if let Some(edge) = schema.edge_table {
        // Fallible path: unsupported `extra_columns` FieldKinds surface
        // as `VerbError::InvalidInput` (exit 2), never a panic.
        tx.execute_batch(&ddl::try_edge_table_for(edge, schema.edge_key_kind)?)?;
    }
    for stmt in schema.custom_migrations {
        tx.execute_batch(stmt)?;
    }
    Ok(())
}

/// Set `PRAGMA user_version` exactly once per DB lifetime (fresh DBs
/// default to 0). If already stamped at `CURRENT_USER_VERSION` this is
/// a no-op; if stamped at an older version a future bump will gate
/// version-indexed DDL here.
fn apply_user_version(tx: &rusqlite::Transaction<'_>) -> Result<(), VerbError> {
    let current: u32 = tx
        .pragma_query_value(None, "user_version", |r| r.get(0))
        .unwrap_or(0);
    if current < CURRENT_USER_VERSION {
        tx.pragma_update(None, "user_version", CURRENT_USER_VERSION)?;
    }
    Ok(())
}
