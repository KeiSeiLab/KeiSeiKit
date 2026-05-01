//! Session persistence layer (port of Hermes `gateway/session.py:640-721`).
//!
//! SQLite-backed `(session_key → SessionData)` index with an in-memory LRU
//! cache for the hot set. Uses `sqlx` so the API is async-friendly.

use std::num::NonZeroUsize;
use std::sync::Arc;

use anyhow::Result;
use chrono::{DateTime, Utc};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use tokio::sync::Mutex;

/// Persistent session record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub session_key: String,
    /// Opaque agent / transcript ID. The runner uses it to look up an
    /// AIAgent from elsewhere.
    pub session_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Number of turns processed so far (heartbeat).
    pub turn_count: u64,
}

/// Async session store with embedded LRU cache.
#[derive(Clone)]
pub struct SessionStore {
    pool: SqlitePool,
    cache: Arc<Mutex<LruCache<String, Arc<SessionData>>>>,
}

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS sessions (
    session_key   TEXT PRIMARY KEY NOT NULL,
    session_id    TEXT NOT NULL,
    created_at    INTEGER NOT NULL,
    updated_at    INTEGER NOT NULL,
    turn_count    INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS sessions_updated_idx ON sessions(updated_at);
"#;

impl SessionStore {
    /// Open or create a SQLite-backed session store.
    pub async fn open(db_path: &str, cache_capacity: usize) -> Result<Self> {
        let opts = SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(true);
        let pool = SqlitePoolOptions::new()
            .max_connections(8)
            .connect_with(opts)
            .await?;
        sqlx::query(SCHEMA).execute(&pool).await?;

        let cap = NonZeroUsize::new(cache_capacity.max(1)).unwrap();
        Ok(Self {
            pool,
            cache: Arc::new(Mutex::new(LruCache::new(cap))),
        })
    }

    /// Look up an existing session or insert a fresh row keyed on `session_key`.
    pub async fn get_or_create(
        &self,
        session_key: &str,
        new_session_id: impl Fn() -> String,
    ) -> Result<Arc<SessionData>> {
        if let Some(hit) = self.cache_get(session_key).await {
            return Ok(hit);
        }

        if let Some(row) = self.fetch_row(session_key).await? {
            let arc = Arc::new(row);
            self.cache_put(session_key, arc.clone()).await;
            return Ok(arc);
        }

        let now = Utc::now();
        let data = SessionData {
            session_key: session_key.to_string(),
            session_id: new_session_id(),
            created_at: now,
            updated_at: now,
            turn_count: 0,
        };
        self.insert_row(&data).await?;
        let arc = Arc::new(data);
        self.cache_put(session_key, arc.clone()).await;
        Ok(arc)
    }

    /// Increment turn_count + bump updated_at. Cheap read-modify-write.
    pub async fn record_turn(&self, session_key: &str) -> Result<()> {
        let now = Utc::now().timestamp();
        sqlx::query("UPDATE sessions SET turn_count = turn_count + 1, updated_at = ?1 WHERE session_key = ?2")
            .bind(now)
            .bind(session_key)
            .execute(&self.pool)
            .await?;
        // Invalidate the cache; next get_or_create reloads.
        self.cache.lock().await.pop(session_key);
        Ok(())
    }

    /// Drop sessions whose `updated_at` is older than `cutoff`. Returns count.
    pub async fn evict_idle(&self, cutoff: DateTime<Utc>) -> Result<u64> {
        let res = sqlx::query("DELETE FROM sessions WHERE updated_at < ?1")
            .bind(cutoff.timestamp())
            .execute(&self.pool)
            .await?;
        self.cache.lock().await.clear();
        Ok(res.rows_affected())
    }

    async fn fetch_row(&self, session_key: &str) -> Result<Option<SessionData>> {
        let row: Option<(String, String, i64, i64, i64)> = sqlx::query_as(
            "SELECT session_key, session_id, created_at, updated_at, turn_count
             FROM sessions WHERE session_key = ?1",
        )
        .bind(session_key)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|(k, id, c, u, t)| SessionData {
            session_key: k,
            session_id: id,
            created_at: DateTime::<Utc>::from_timestamp(c, 0).unwrap_or_else(Utc::now),
            updated_at: DateTime::<Utc>::from_timestamp(u, 0).unwrap_or_else(Utc::now),
            turn_count: t.max(0) as u64,
        }))
    }

    async fn insert_row(&self, data: &SessionData) -> Result<()> {
        sqlx::query(
            "INSERT INTO sessions (session_key, session_id, created_at, updated_at, turn_count)
             VALUES (?1, ?2, ?3, ?4, ?5)",
        )
        .bind(&data.session_key)
        .bind(&data.session_id)
        .bind(data.created_at.timestamp())
        .bind(data.updated_at.timestamp())
        .bind(data.turn_count as i64)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn cache_get(&self, session_key: &str) -> Option<Arc<SessionData>> {
        self.cache.lock().await.get(session_key).cloned()
    }

    async fn cache_put(&self, session_key: &str, value: Arc<SessionData>) {
        self.cache.lock().await.put(session_key.to_string(), value);
    }
}
