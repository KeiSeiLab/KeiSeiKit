//! SQLite-backed [`Store`]. Schema migrates on every `open` / `open_in_memory`.

use std::path::Path;

use rusqlite::{params, Connection, OpenFlags, OptionalExtension};

use crate::aggregate::ModelAggregate;
use crate::error::Error;
use crate::event::TokenEvent;
use crate::schema;

/// Token-event SQLite store. Holds an owned [`Connection`]; clone the
/// database file to share across processes.
pub struct Store {
    conn: Connection,
}

impl Store {
    /// Open or create a SQLite database at `path`, applying pending
    /// migrations. Parent directory must already exist — the store
    /// does not auto-create it (callers know intent better than we do).
    pub fn open(path: &Path) -> Result<Self, Error> {
        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )?;
        schema::migrate(&conn)?;
        Ok(Self { conn })
    }

    /// In-memory variant — same migrations applied. For tests + ephemeral
    /// invocations (e.g. CLI dry-runs).
    pub fn open_in_memory() -> Result<Self, Error> {
        let conn = Connection::open_in_memory()?;
        schema::migrate(&conn)?;
        Ok(Self { conn })
    }

    /// Insert one [`TokenEvent`]. Returns the freshly-allocated row id so
    /// callers can correlate events with downstream artefacts.
    pub fn record_event(&self, ev: &TokenEvent) -> Result<i64, Error> {
        self.conn.execute(
            "INSERT INTO token_events
                (ts, agent_id, conversation_id, model, role,
                 input_tokens, output_tokens, micro_cents,
                 category, source_kind, latency_ms)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                ev.ts,
                ev.agent_id,
                ev.conversation_id,
                ev.model,
                ev.role,
                ev.input_tokens,
                ev.output_tokens,
                ev.micro_cents as i64,
                ev.category,
                ev.source_kind,
                ev.latency_ms,
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Most recent `limit` events, newest first.
    pub fn list_recent(&self, limit: u32) -> Result<Vec<TokenEvent>, Error> {
        let mut stmt = self.conn.prepare(
            "SELECT ts, agent_id, conversation_id, model, role,
                    input_tokens, output_tokens, micro_cents,
                    category, source_kind, latency_ms
             FROM token_events
             ORDER BY ts DESC, id DESC
             LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit as i64], row_to_event)?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    /// Aggregate by model since a unix-epoch lower bound (`ts >= since`).
    /// Sorted alphabetically by model so report output is deterministic.
    pub fn aggregate_by_model(&self, since_unix: i64) -> Result<Vec<ModelAggregate>, Error> {
        let mut stmt = self.conn.prepare(
            "SELECT model,
                    COUNT(*) AS events,
                    COALESCE(SUM(input_tokens), 0) AS input_tokens,
                    COALESCE(SUM(output_tokens), 0) AS output_tokens,
                    COALESCE(SUM(micro_cents), 0) AS micro_cents
             FROM token_events
             WHERE ts >= ?1
             GROUP BY model
             ORDER BY model ASC",
        )?;
        let rows = stmt.query_map(params![since_unix], |r| {
            Ok(ModelAggregate {
                model: r.get(0)?,
                events: r.get::<_, i64>(1)? as u32,
                input_tokens: r.get::<_, i64>(2)? as u64,
                output_tokens: r.get::<_, i64>(3)? as u64,
                micro_cents: r.get::<_, i64>(4)? as u64,
            })
        })?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    /// Total event count. Used by the CLI `count` subcommand.
    pub fn count(&self) -> Result<i64, Error> {
        let n: Option<i64> = self
            .conn
            .query_row("SELECT COUNT(*) FROM token_events", [], |r| r.get(0))
            .optional()?;
        Ok(n.unwrap_or(0))
    }
}

fn row_to_event(r: &rusqlite::Row<'_>) -> rusqlite::Result<TokenEvent> {
    Ok(TokenEvent {
        ts: r.get(0)?,
        agent_id: r.get(1)?,
        conversation_id: r.get(2)?,
        model: r.get(3)?,
        role: r.get(4)?,
        input_tokens: r.get::<_, i64>(5)? as u32,
        output_tokens: r.get::<_, i64>(6)? as u32,
        micro_cents: r.get::<_, i64>(7)? as u64,
        category: r.get(8)?,
        source_kind: r.get(9)?,
        latency_ms: r.get::<_, Option<i64>>(10)?.map(|v| v as u32),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::TokenEvent;

    fn ev(ts: i64, model: &str, in_tok: u32, out_tok: u32, micro: u64) -> TokenEvent {
        TokenEvent::chat_turn(ts, "agent-x", model, "assistant", in_tok, out_tok, micro)
    }

    #[test]
    fn record_round_trips() {
        let s = Store::open_in_memory().unwrap();
        let id = s.record_event(&ev(100, "claude-haiku-4-5", 10, 5, 1_500)).unwrap();
        assert!(id >= 1);
        let rows = s.list_recent(10).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].input_tokens, 10);
        assert_eq!(rows[0].output_tokens, 5);
        assert_eq!(rows[0].micro_cents, 1_500);
    }

    #[test]
    fn count_empty_and_populated() {
        let s = Store::open_in_memory().unwrap();
        assert_eq!(s.count().unwrap(), 0);
        s.record_event(&ev(1, "m", 1, 1, 1)).unwrap();
        s.record_event(&ev(2, "m", 1, 1, 1)).unwrap();
        assert_eq!(s.count().unwrap(), 2);
    }
}
