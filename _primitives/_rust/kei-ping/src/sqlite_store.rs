// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! SQLite-backed PingStore. WAL + busy_timeout for concurrent windows.
//! 1 row per agent_id; UPDATE on every heartbeat (idempotent).

use crate::model::{now_epoch, Heartbeat, PingFilter};
use crate::store::{BackendKind, PingStore};
use anyhow::Result;
use rusqlite::{params, Connection};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct SqlitePingStore {
    conn: Mutex<Connection>,
}

impl SqlitePingStore {
    pub fn open(path: PathBuf) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&path)?;
        // WAL + busy_timeout — survive 6+ concurrent windows.
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "busy_timeout", 5000)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS ping (
                agent_id        TEXT PRIMARY KEY,
                session_id      TEXT,
                phase           TEXT NOT NULL,
                dna             TEXT,
                branch          TEXT,
                cwd             TEXT,
                last_seen_epoch INTEGER NOT NULL,
                note            TEXT
            )",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_ping_last_seen ON ping(last_seen_epoch)",
            [],
        )?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}

#[async_trait::async_trait]
impl PingStore for SqlitePingStore {
    fn kind(&self) -> BackendKind {
        BackendKind::Sqlite
    }

    async fn send(&self, h: &Heartbeat) -> Result<()> {
        let conn = self.conn.lock().expect("ping mutex");
        conn.execute(
            "INSERT INTO ping
             (agent_id, session_id, phase, dna, branch, cwd, last_seen_epoch, note)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(agent_id) DO UPDATE SET
                session_id=excluded.session_id,
                phase=excluded.phase,
                dna=excluded.dna,
                branch=excluded.branch,
                cwd=excluded.cwd,
                last_seen_epoch=excluded.last_seen_epoch,
                note=excluded.note",
            params![
                h.agent_id,
                h.session_id,
                h.phase,
                h.dna,
                h.branch,
                h.cwd,
                h.last_seen_epoch as i64,
                h.note,
            ],
        )?;
        Ok(())
    }

    async fn list(&self, f: &PingFilter) -> Result<Vec<Heartbeat>> {
        let conn = self.conn.lock().expect("ping mutex");
        let now = now_epoch();
        let cutoff = (now as i64).saturating_sub(f.max_age_s.unwrap_or(90) as i64);
        let mut stmt = conn.prepare(
            "SELECT agent_id, session_id, phase, dna, branch, cwd, last_seen_epoch, note
             FROM ping
             WHERE last_seen_epoch >= ?1
             ORDER BY last_seen_epoch DESC",
        )?;
        let rows = stmt.query_map(params![cutoff], |r| {
            Ok(Heartbeat {
                agent_id: r.get(0)?,
                session_id: r.get(1)?,
                phase: r.get(2)?,
                dna: r.get(3)?,
                branch: r.get(4)?,
                cwd: r.get(5)?,
                last_seen_epoch: r.get::<_, i64>(6)? as u64,
                note: r.get(7)?,
            })
        })?;
        let mut out = Vec::new();
        for r in rows {
            let h = r?;
            if f.alive(&h, now) {
                out.push(h);
            }
        }
        Ok(out)
    }

    async fn clear(&self, agent_id: &str) -> Result<()> {
        let conn = self.conn.lock().expect("ping mutex");
        conn.execute("DELETE FROM ping WHERE agent_id = ?1", params![agent_id])?;
        Ok(())
    }
}
