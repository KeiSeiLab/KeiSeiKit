//! Knowledge-unit CRUD + FTS indexer.
//!
//! `Store::open` / `Store::open_memory` delegate to
//! `kei_entity_store::Store` which runs `SAGE_SCHEMA` migrations.
//! The sage-specific `add_unit` / `update_unit` / `delete_unit`
//! helpers stay here because they use `INSERT OR REPLACE` idempotency
//! by `vault_path` and maintain sage's custom FTS table (`fts_knowledge`
//! with column `unit_id`) — engine's generic `create` verb assumes a
//! different FTS shape (`fts_<table>` with column `<table>_id`).

use crate::schema::SAGE_SCHEMA;
use crate::types::Unit;
use anyhow::{Context, Result};
use chrono::Utc;
use kei_entity_store::Store as EngineStore;
use rusqlite::{params, Connection};
use std::path::Path;

pub struct Store {
    engine: EngineStore,
}

impl Store {
    pub fn open(path: &Path) -> Result<Self> {
        let engine = EngineStore::open(path, &[&SAGE_SCHEMA]).context("engine store open")?;
        Ok(Self { engine })
    }

    pub fn open_memory() -> Result<Self> {
        let engine = EngineStore::open_memory(&[&SAGE_SCHEMA]).context("engine store open_memory")?;
        Ok(Self { engine })
    }

    pub fn conn(&self) -> &Connection {
        self.engine.conn()
    }

    /// Insert a new knowledge unit. Indexes title+content into FTS5. Idempotent by vault_path.
    pub fn add_unit(&self, unit: &Unit) -> Result<i64> {
        let now = Utc::now().timestamp();
        let created = if unit.created_at == 0 { now } else { unit.created_at };
        self.conn().execute(
            "INSERT OR REPLACE INTO knowledge_units
             (unit_type, title, content, evidence_grade, source_path,
              vault_path, category, created_at, updated_at)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
            params![unit.unit_type, unit.title, unit.content, unit.evidence_grade,
                unit.source_path, unit.vault_path, unit.category, created, now],
        )?;
        let id = self.conn().last_insert_rowid();
        self.reindex_fts(id, &unit.title, &unit.content)?;
        Ok(id)
    }

    pub fn get_unit(&self, id: i64) -> Result<Option<Unit>> {
        let mut stmt = self.conn().prepare(
            "SELECT id, unit_type, title, content, evidence_grade, source_path,
                    vault_path, category, created_at, updated_at
             FROM knowledge_units WHERE id=?1",
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(r) = rows.next()? {
            return Ok(Some(row_to_unit(r)?));
        }
        Ok(None)
    }

    pub fn update_unit(&self, unit: &Unit) -> Result<()> {
        let now = Utc::now().timestamp();
        self.conn().execute(
            "UPDATE knowledge_units SET title=?1, content=?2, evidence_grade=?3,
             category=?4, updated_at=?5 WHERE id=?6",
            params![unit.title, unit.content, unit.evidence_grade,
                unit.category, now, unit.id],
        )?;
        self.reindex_fts(unit.id, &unit.title, &unit.content)?;
        Ok(())
    }

    pub fn delete_unit(&self, id: i64) -> Result<()> {
        self.conn().execute("DELETE FROM fts_knowledge WHERE unit_id=?1", params![id])?;
        self.conn().execute("DELETE FROM knowledge_units WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn count_units(&self) -> Result<i64> {
        Ok(self.conn().query_row(
            "SELECT COUNT(*) FROM knowledge_units", [], |r| r.get(0))?)
    }

    fn reindex_fts(&self, id: i64, title: &str, content: &str) -> Result<()> {
        self.conn().execute("DELETE FROM fts_knowledge WHERE unit_id=?1", params![id])?;
        self.conn().execute(
            "INSERT INTO fts_knowledge (unit_id, title, content) VALUES (?1,?2,?3)",
            params![id, title, content],
        )?;
        Ok(())
    }
}

fn row_to_unit(r: &rusqlite::Row) -> rusqlite::Result<Unit> {
    Ok(Unit {
        id: r.get(0)?,
        unit_type: r.get(1)?,
        title: r.get(2)?,
        content: r.get(3)?,
        evidence_grade: r.get(4)?,
        source_path: r.get(5)?,
        vault_path: r.get(6)?,
        category: r.get(7)?,
        created_at: r.get(8)?,
        updated_at: r.get(9)?,
    })
}
