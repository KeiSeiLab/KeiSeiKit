// SPDX-License-Identifier: Apache-2.0
//! Buddy-specific SQLite DDL.
//!
//! Constructor Pattern: schema only, no business logic.
//!
//! Table `buddy_state`:
//!   - `chat_id INTEGER PRIMARY KEY` — Telegram chat ID
//!   - `state TEXT NOT NULL`          — serialized `OnboardState` JSON
//!   - `persona TEXT`                 — serialized persona JSON, nullable
//!   - `created_at INTEGER NOT NULL`  — unix epoch seconds
//!   - `updated_at INTEGER NOT NULL`  — unix epoch seconds

use rusqlite::{Connection, Result};

/// DDL for the buddy_state table. Idempotent (`IF NOT EXISTS`).
pub const DDL: &str = "
    CREATE TABLE IF NOT EXISTS buddy_state (
        chat_id    INTEGER PRIMARY KEY,
        state      TEXT    NOT NULL,
        persona    TEXT,
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL
    );
";

/// Apply the buddy schema. Idempotent — safe to call on every connection open.
pub fn apply_schema_buddy(conn: &Connection) -> Result<()> {
    conn.execute_batch(DDL)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_applies_idempotently() {
        let conn = Connection::open_in_memory().unwrap();
        apply_schema_buddy(&conn).unwrap();
        apply_schema_buddy(&conn).unwrap(); // second call must not error
    }

    #[test]
    fn buddy_state_table_exists_after_apply() {
        let conn = Connection::open_in_memory().unwrap();
        apply_schema_buddy(&conn).unwrap();
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='buddy_state'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }
}
