// SPDX-License-Identifier: Apache-2.0
//! Buddy-specific persistence layer.
//!
//! Constructor Pattern: async trait + thin SQLite impl.
//!
//! `BuddyStore` is the async trait contract.
//! `SqliteBuddyStore` wraps a shared `kei_memory_sqlite::SqliteStore`
//! and implements it via `tokio::task::spawn_blocking` (rusqlite is sync).
//! Blocking SQL logic lives in `store_ops` (Constructor-pattern split).

use async_trait::async_trait;
use kei_memory_sqlite::SqliteStore;
use serde_json::Value;
use std::path::Path;
use std::sync::Arc;

use crate::error::BuddyError;
use crate::schema::apply_schema_buddy;
use crate::state::OnboardState;
use crate::store_ops::{db_load_persona, db_load_state, db_save_persona, db_save_state, now_epoch};

// ─── trait ───────────────────────────────────────────────────────────────────

/// Async persistence contract for per-chat buddy state.
#[async_trait]
pub trait BuddyStore: Send + Sync {
    /// Load the onboarding state for `chat_id`. Returns `None` if no row.
    async fn load_state(&self, chat_id: i64) -> Result<Option<OnboardState>, BuddyError>;

    /// Persist the onboarding state for `chat_id`.
    async fn save_state(&self, chat_id: i64, state: &OnboardState) -> Result<(), BuddyError>;

    /// Load the persona blob for `chat_id`. Returns `None` if not set.
    async fn load_persona(&self, chat_id: i64) -> Result<Option<Value>, BuddyError>;

    /// Persist the persona blob for `chat_id`.
    async fn save_persona(&self, chat_id: i64, persona: &Value) -> Result<(), BuddyError>;
}

// ─── impl ────────────────────────────────────────────────────────────────────

/// SQLite-backed `BuddyStore`. Cheap to clone (inner is `Arc`).
#[derive(Clone)]
pub struct SqliteBuddyStore {
    inner: Arc<SqliteStore>,
}

impl SqliteBuddyStore {
    /// Wrap an existing `SqliteStore`. Applies the buddy schema.
    pub fn new(store: Arc<SqliteStore>) -> Result<Self, BuddyError> {
        {
            let conn = store.lock();
            apply_schema_buddy(&conn)?;
        }
        Ok(Self { inner: store })
    }

    /// Open or create a file-backed SQLite DB and apply the buddy schema.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, BuddyError> {
        let store = Arc::new(SqliteStore::from_path(path)?);
        Self::new(store)
    }

    /// Open an in-memory SQLite DB. Useful for tests.
    pub fn from_memory() -> Result<Self, BuddyError> {
        let store = Arc::new(SqliteStore::from_memory()?);
        Self::new(store)
    }
}

// ─── BuddyStore impl ─────────────────────────────────────────────────────────

#[async_trait]
impl BuddyStore for SqliteBuddyStore {
    async fn load_state(&self, chat_id: i64) -> Result<Option<OnboardState>, BuddyError> {
        let store = Arc::clone(&self.inner);
        tokio::task::spawn_blocking(move || db_load_state(&store.lock(), chat_id))
            .await
            .map_err(|e| BuddyError::Memory(e.to_string()))?
    }

    async fn save_state(&self, chat_id: i64, state: &OnboardState) -> Result<(), BuddyError> {
        let json =
            serde_json::to_string(state).map_err(|e| BuddyError::Memory(e.to_string()))?;
        let store = Arc::clone(&self.inner);
        let now = now_epoch();
        tokio::task::spawn_blocking(move || db_save_state(&store.lock(), chat_id, &json, now))
            .await
            .map_err(|e| BuddyError::Memory(e.to_string()))?
    }

    async fn load_persona(&self, chat_id: i64) -> Result<Option<Value>, BuddyError> {
        let store = Arc::clone(&self.inner);
        tokio::task::spawn_blocking(move || db_load_persona(&store.lock(), chat_id))
            .await
            .map_err(|e| BuddyError::Memory(e.to_string()))?
    }

    async fn save_persona(&self, chat_id: i64, persona: &Value) -> Result<(), BuddyError> {
        let json =
            serde_json::to_string(persona).map_err(|e| BuddyError::Memory(e.to_string()))?;
        let store = Arc::clone(&self.inner);
        let now = now_epoch();
        tokio::task::spawn_blocking(move || db_save_persona(&store.lock(), chat_id, &json, now))
            .await
            .map_err(|e| BuddyError::Memory(e.to_string()))?
    }
}

// ─── tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn rt() -> tokio::runtime::Runtime {
        tokio::runtime::Runtime::new().unwrap()
    }

    #[test]
    fn roundtrip_state_save_and_load() {
        rt().block_on(async {
            let store = SqliteBuddyStore::from_memory().unwrap();
            store.save_state(42, &OnboardState::AskName).await.unwrap();
            let loaded = store.load_state(42).await.unwrap();
            assert_eq!(loaded, Some(OnboardState::AskName));
        });
    }

    #[test]
    fn load_state_returns_none_for_unknown_chat() {
        rt().block_on(async {
            let store = SqliteBuddyStore::from_memory().unwrap();
            let loaded = store.load_state(999).await.unwrap();
            assert_eq!(loaded, None);
        });
    }

    #[test]
    fn save_state_updates_existing_row() {
        rt().block_on(async {
            let store = SqliteBuddyStore::from_memory().unwrap();
            store.save_state(1, &OnboardState::AskName).await.unwrap();
            store.save_state(1, &OnboardState::Ready).await.unwrap();
            let loaded = store.load_state(1).await.unwrap();
            assert_eq!(loaded, Some(OnboardState::Ready));
        });
    }

    #[test]
    fn roundtrip_persona_independent_of_state() {
        rt().block_on(async {
            let store = SqliteBuddyStore::from_memory().unwrap();
            let persona = json!({ "name": "Alice", "tone": "formal" });
            store.save_state(7, &OnboardState::AskTone).await.unwrap();
            store.save_persona(7, &persona).await.unwrap();
            let state = store.load_state(7).await.unwrap();
            let loaded = store.load_persona(7).await.unwrap();
            assert_eq!(state, Some(OnboardState::AskTone));
            assert_eq!(loaded, Some(persona));
        });
    }
}
