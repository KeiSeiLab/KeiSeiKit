//! SQLite-backed cache store.
//!
//! Constructor Pattern: one cube = cache table DDL + put/get/stats/purge/clear.
//! Every fn <30 LOC. Schema is append-only migration list; expiry is
//! timestamp-based (`expires_ts = created_ts + ttl_sec`).
//!
//! Layout: one row per unique (atom_id, canonical_input) → cache key.
//! Payload stored as raw JSON text to keep the primitive format-neutral.

use anyhow::{anyhow, Context, Result};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// Ordered migrations. Index = schema version. Never reorder; append only.
pub const MIGRATIONS: &[&str] = &[
    // v1 — initial schema (2026-04-23)
    "CREATE TABLE IF NOT EXISTS cache (
        key TEXT PRIMARY KEY,
        atom_id TEXT NOT NULL,
        payload TEXT NOT NULL,
        created_ts INTEGER NOT NULL,
        expires_ts INTEGER NOT NULL,
        bytes INTEGER NOT NULL
    );
    CREATE INDEX IF NOT EXISTS idx_cache_expires ON cache(expires_ts);
    CREATE INDEX IF NOT EXISTS idx_cache_atom ON cache(atom_id);
    CREATE TABLE IF NOT EXISTS counters (
        name TEXT PRIMARY KEY,
        value INTEGER NOT NULL
    );",
];

/// Open or create the cache DB and run migrations.
pub fn open(path: &Path) -> Result<Connection> {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let conn = Connection::open(path).with_context(|| format!("open {}", path.display()))?;
    migrate(&conn)?;
    Ok(conn)
}

/// Apply pending migrations atomically (DDL + user_version bump per txn).
fn migrate(conn: &Connection) -> Result<()> {
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

fn apply_one(conn: &Connection, sql: &str, target: i64) -> Result<()> {
    conn.execute_batch("BEGIN IMMEDIATE")?;
    let step = (|| -> rusqlite::Result<()> {
        conn.execute_batch(sql)?;
        conn.pragma_update(None, "user_version", target)?;
        Ok(())
    })();
    match step {
        Ok(()) => conn.execute_batch("COMMIT").map_err(Into::into),
        Err(e) => {
            let _ = conn.execute_batch("ROLLBACK");
            Err(anyhow!("migration v{target}: {e}"))
        }
    }
}

/// Current unix timestamp in seconds.
pub fn now_ts() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Insert (upsert) a cache entry. `ttl_sec` must be > 0.
pub fn put(conn: &Connection, key: &str, atom_id: &str, payload: &str, ttl_sec: i64) -> Result<()> {
    if ttl_sec <= 0 {
        return Err(anyhow!("ttl must be positive, got {ttl_sec}"));
    }
    let now = now_ts();
    let expires = now + ttl_sec;
    let bytes = payload.len() as i64;
    conn.execute(
        "INSERT INTO cache (key, atom_id, payload, created_ts, expires_ts, bytes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(key) DO UPDATE SET
            payload = excluded.payload,
            created_ts = excluded.created_ts,
            expires_ts = excluded.expires_ts,
            bytes = excluded.bytes",
        params![key, atom_id, payload, now, expires, bytes],
    )?;
    Ok(())
}

/// Look up a key; returns `None` on miss or expired entry.
/// Expired entries are evicted lazily on lookup.
pub fn get(conn: &Connection, key: &str) -> Result<Option<String>> {
    let now = now_ts();
    let row: Option<(String, i64)> = conn
        .query_row(
            "SELECT payload, expires_ts FROM cache WHERE key = ?1",
            params![key],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()?;
    match row {
        Some((payload, expires)) if expires > now => Ok(Some(payload)),
        Some(_) => {
            let _ = conn.execute("DELETE FROM cache WHERE key = ?1", params![key]);
            Ok(None)
        }
        None => Ok(None),
    }
}

/// Increment a named counter (hits / misses) by 1.
pub fn bump(conn: &Connection, name: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO counters (name, value) VALUES (?1, 1)
         ON CONFLICT(name) DO UPDATE SET value = value + 1",
        params![name],
    )?;
    Ok(())
}

/// Read aggregate stats: (hits, misses, live_entries, total_bytes).
pub fn stats(conn: &Connection) -> Result<Stats> {
    let hits = counter(conn, "hits")?;
    let misses = counter(conn, "misses")?;
    let now = now_ts();
    let (entries, bytes): (i64, i64) = conn
        .query_row(
            "SELECT COUNT(*), COALESCE(SUM(bytes), 0)
             FROM cache WHERE expires_ts > ?1",
            params![now],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap_or((0, 0));
    Ok(Stats { hits, misses, entries, bytes })
}

fn counter(conn: &Connection, name: &str) -> Result<i64> {
    let v: Option<i64> = conn
        .query_row(
            "SELECT value FROM counters WHERE name = ?1",
            params![name],
            |r| r.get(0),
        )
        .optional()?;
    Ok(v.unwrap_or(0))
}

/// Aggregate cache stats snapshot.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Stats {
    pub hits: i64,
    pub misses: i64,
    pub entries: i64,
    pub bytes: i64,
}

/// Evict expired rows; returns number deleted.
pub fn purge(conn: &Connection) -> Result<usize> {
    let now = now_ts();
    let n = conn.execute("DELETE FROM cache WHERE expires_ts <= ?1", params![now])?;
    Ok(n)
}

/// Wipe everything (cache + counters). Returns rows removed from `cache`.
pub fn clear(conn: &Connection) -> Result<usize> {
    let n = conn.execute("DELETE FROM cache", [])?;
    conn.execute("DELETE FROM counters", [])?;
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn fresh() -> (tempfile::TempDir, Connection) {
        let d = tempdir().unwrap();
        let p = d.path().join("c.sqlite");
        let c = open(&p).unwrap();
        (d, c)
    }

    #[test]
    fn put_get_roundtrip() {
        let (_d, c) = fresh();
        put(&c, "k1", "atom:x", "{\"r\":1}", 60).unwrap();
        assert_eq!(get(&c, "k1").unwrap().as_deref(), Some("{\"r\":1}"));
    }

    #[test]
    fn miss_returns_none() {
        let (_d, c) = fresh();
        assert!(get(&c, "missing").unwrap().is_none());
    }

    #[test]
    fn purge_removes_expired() {
        let (_d, c) = fresh();
        // ttl=1 means expires in 1s; manually backdate via direct update.
        put(&c, "k1", "atom:x", "v", 60).unwrap();
        c.execute(
            "UPDATE cache SET expires_ts = 1 WHERE key = 'k1'",
            [],
        )
        .unwrap();
        assert_eq!(purge(&c).unwrap(), 1);
        assert!(get(&c, "k1").unwrap().is_none());
    }

    #[test]
    fn stats_count_live_only() {
        let (_d, c) = fresh();
        put(&c, "a", "atom:x", "xx", 60).unwrap();
        put(&c, "b", "atom:x", "yyyy", 60).unwrap();
        bump(&c, "hits").unwrap();
        bump(&c, "misses").unwrap();
        bump(&c, "misses").unwrap();
        let s = stats(&c).unwrap();
        assert_eq!(s.entries, 2);
        assert_eq!(s.bytes, 6);
        assert_eq!(s.hits, 1);
        assert_eq!(s.misses, 2);
    }
}
