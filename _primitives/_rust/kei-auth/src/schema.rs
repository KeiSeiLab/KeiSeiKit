use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::Path;

pub fn open(path: &Path) -> Result<Connection> {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let conn = Connection::open(path).context("open sqlite")?;
    create_schema(&conn)?;
    Ok(conn)
}

pub fn open_memory() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    create_schema(&conn)?;
    Ok(conn)
}

pub fn create_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(r#"
    CREATE TABLE IF NOT EXISTS auth_tokens (
        id         INTEGER PRIMARY KEY,
        token_hash TEXT NOT NULL UNIQUE,
        user_id    TEXT NOT NULL,
        project    TEXT NOT NULL,
        scope      TEXT NOT NULL CHECK(scope IN ('read','write','admin')),
        expires_at INTEGER NOT NULL,
        created_at INTEGER NOT NULL,
        revoked_at INTEGER DEFAULT 0
    );
    CREATE INDEX IF NOT EXISTS idx_tok_user ON auth_tokens(user_id);
    CREATE INDEX IF NOT EXISTS idx_tok_project ON auth_tokens(project);
    "#)?;
    Ok(())
}
