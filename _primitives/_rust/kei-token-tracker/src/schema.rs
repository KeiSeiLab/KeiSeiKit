//! SQLite schema runner. One table + three indexes, applied at `open`.

use rusqlite::Connection;

use crate::error::Error;

pub const SCHEMA_VERSION: u32 = 1;

const DDL_V1: &str = "
CREATE TABLE IF NOT EXISTS token_events (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    ts              INTEGER NOT NULL,
    agent_id        TEXT NOT NULL,
    conversation_id TEXT,
    model           TEXT NOT NULL,
    role            TEXT NOT NULL,
    input_tokens    INTEGER NOT NULL,
    output_tokens   INTEGER NOT NULL,
    micro_cents     INTEGER NOT NULL,
    category        TEXT,
    source_kind     TEXT,
    latency_ms      INTEGER
);

CREATE INDEX IF NOT EXISTS idx_token_events_ts
    ON token_events(ts);
CREATE INDEX IF NOT EXISTS idx_token_events_model_ts
    ON token_events(model, ts);
CREATE INDEX IF NOT EXISTS idx_token_events_agent_ts
    ON token_events(agent_id, ts);
";

const MIGRATIONS: &[&str] = &[DDL_V1];

/// Apply pending migrations. Idempotent: re-running on an up-to-date
/// database is a no-op. Each migration runs in its own transaction so a
/// partial failure rolls back rather than wedging the schema.
pub fn migrate(conn: &Connection) -> Result<(), Error> {
    let current: i64 = conn
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap_or(0);
    for (i, sql) in MIGRATIONS.iter().enumerate() {
        let target = (i + 1) as i64;
        if current < target {
            apply_one(conn, sql, target)?;
        }
    }
    Ok(())
}

fn apply_one(conn: &Connection, sql: &str, target: i64) -> Result<(), Error> {
    conn.execute_batch("BEGIN IMMEDIATE")?;
    let step = (|| -> Result<(), rusqlite::Error> {
        conn.execute_batch(sql)?;
        conn.pragma_update(None, "user_version", target)?;
        Ok(())
    })();
    match step {
        Ok(()) => {
            conn.execute_batch("COMMIT")?;
            Ok(())
        }
        Err(e) => {
            let _ = conn.execute_batch("ROLLBACK");
            Err(Error::Migration(format!("v{target}: {e}")))
        }
    }
}
