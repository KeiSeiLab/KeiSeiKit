//! `Store` — Connection wrapper. Single responsibility: open/migrate sqlite.

use crate::schema::migrate;
use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::Path;

pub struct Store {
    conn: Connection,
}

impl Store {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let conn = Connection::open(path).context("open sqlite")?;
        conn.pragma_update(None, "journal_mode", "WAL").ok();
        migrate(&conn)?;
        Ok(Self { conn })
    }

    pub fn open_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        migrate(&conn)?;
        Ok(Self { conn })
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }
}
