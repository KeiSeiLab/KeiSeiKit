// SPDX-License-Identifier: Apache-2.0
//! `ChatLog` — maps Telegram chat_id to kei-chat-store sessions.
//!
//! Constructor Pattern: one responsibility — bridge between Telegram chat_id
//! (i64) and kei-chat-store session (String UUID). All rusqlite calls are
//! dispatched via `tokio::task::spawn_blocking`.
//!
//! `rusqlite::Connection` is not `Sync`, so `Store` is not `Sync`. We wrap
//! `Store` in `Mutex<Store>` to obtain `Send + Sync` on `Arc<Mutex<Store>>`.
//! Each blocking task locks the mutex for the duration of the DB call.

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use chrono::Utc;
use kei_chat_store::{
    search::search as cs_search,
    sessions::{save_message, start_session, ChatMessage},
    Store,
};
use rusqlite::OptionalExtension;

use crate::error::BuddyError;

/// Thin wrapper over `kei-chat-store` keyed by Telegram chat_id.
pub struct ChatLog {
    store: Arc<Mutex<Store>>,
    /// chat_id → session_id; populated lazily.
    sessions: Mutex<HashMap<i64, String>>,
}

impl ChatLog {
    /// Open (or create) a file-backed store at `path`.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, BuddyError> {
        let store = Store::open(path.as_ref()).map_err(|e| BuddyError::Memory(e.to_string()))?;
        Ok(Self {
            store: Arc::new(Mutex::new(store)),
            sessions: Mutex::new(HashMap::new()),
        })
    }

    /// Open an in-memory store (for tests).
    pub fn from_memory() -> Result<Self, BuddyError> {
        let store = Store::open_memory().map_err(|e| BuddyError::Memory(e.to_string()))?;
        Ok(Self {
            store: Arc::new(Mutex::new(store)),
            sessions: Mutex::new(HashMap::new()),
        })
    }

    /// Return the session_id for `chat_id`, creating one if absent.
    pub async fn ensure_session(&self, chat_id: i64) -> Result<String, BuddyError> {
        // Fast path: session already cached.
        {
            let guard = self.sessions.lock().expect("sessions mutex poisoned");
            if let Some(id) = guard.get(&chat_id) {
                return Ok(id.clone());
            }
        }
        // Slow path: query or create in blocking thread.
        let store = Arc::clone(&self.store);
        let title = format!("tg-{chat_id}");
        let session_id = tokio::task::spawn_blocking(move || {
            let locked = store.lock().expect("store mutex poisoned");
            find_or_create_session(&locked, &title)
        })
        .await
        .map_err(|e| BuddyError::Memory(format!("spawn_blocking join: {e}")))?
        .map_err(|e| BuddyError::Memory(e.to_string()))?;

        let mut guard = self.sessions.lock().expect("sessions mutex poisoned");
        guard.insert(chat_id, session_id.clone());
        Ok(session_id)
    }

    /// Persist a user-side message.
    pub async fn log_user(&self, chat_id: i64, content: &str) -> Result<(), BuddyError> {
        self.log_role(chat_id, "user", content).await
    }

    /// Persist a bot-side response.
    pub async fn log_bot(&self, chat_id: i64, content: &str) -> Result<(), BuddyError> {
        self.log_role(chat_id, "assistant", content).await
    }

    /// Full-text search; optionally filter by chat_id.
    pub async fn search(
        &self,
        query: &str,
        chat_id: Option<i64>,
        limit: i64,
    ) -> Result<Vec<ChatMessage>, BuddyError> {
        let filter_session = match chat_id {
            Some(cid) => Some(self.ensure_session(cid).await?),
            None => None,
        };
        let store = Arc::clone(&self.store);
        let q = query.to_string();
        let msgs = tokio::task::spawn_blocking(move || {
            let locked = store.lock().expect("store mutex poisoned");
            cs_search(&locked, &q, limit).map_err(|e| BuddyError::Memory(e.to_string()))
        })
        .await
        .map_err(|e| BuddyError::Memory(format!("spawn_blocking join: {e}")))?
        .map_err(|e: BuddyError| e)?;

        match filter_session {
            None => Ok(msgs),
            Some(sid) => Ok(msgs.into_iter().filter(|m| m.session_id == sid).collect()),
        }
    }

    // ── Private helpers ─────────────────────────────────────────────────────

    async fn log_role(
        &self,
        chat_id: i64,
        role: &str,
        content: &str,
    ) -> Result<(), BuddyError> {
        let session_id = self.ensure_session(chat_id).await?;
        let msg = ChatMessage {
            id: 0,
            session_id,
            role: role.to_string(),
            content: content.to_string(),
            tokens_in: 0,
            tokens_out: 0,
            cost: 0.0,
            created_at: Utc::now().timestamp(),
        };
        let store = Arc::clone(&self.store);
        tokio::task::spawn_blocking(move || {
            let locked = store.lock().expect("store mutex poisoned");
            save_message(&locked, &msg).map_err(|e| BuddyError::Memory(e.to_string()))
        })
        .await
        .map_err(|e| BuddyError::Memory(format!("spawn_blocking join: {e}")))?
        .map(|_| ())
    }
}

/// Query the DB for an existing session; create if absent.
fn find_or_create_session(store: &Store, title: &str) -> anyhow::Result<String> {
    let existing: Option<String> = store
        .conn()
        .query_row(
            "SELECT id FROM chat_sessions WHERE title = ?1 LIMIT 1",
            rusqlite::params![title],
            |row| row.get(0),
        )
        .optional()?;

    if let Some(id) = existing {
        return Ok(id);
    }
    start_session(store, "kei-buddy", title, "telegram")
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn log_user_creates_session_and_message() {
        let log = ChatLog::from_memory().unwrap();
        log.log_user(42, "hi there").await.unwrap();
        let results = log.search("hi there", Some(42), 10).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].content, "hi there");
        assert_eq!(results[0].role, "user");
    }

    #[tokio::test]
    async fn log_bot_uses_same_session_as_log_user() {
        let log = ChatLog::from_memory().unwrap();
        log.log_user(42, "hello").await.unwrap();
        log.log_bot(42, "world").await.unwrap();
        let results = log.search("world", Some(42), 10).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].role, "assistant");
        let user_results = log.search("hello", Some(42), 10).await.unwrap();
        assert_eq!(results[0].session_id, user_results[0].session_id);
    }

    #[tokio::test]
    async fn different_chats_get_different_sessions() {
        let log = ChatLog::from_memory().unwrap();
        log.log_user(1, "alpha").await.unwrap();
        log.log_user(2, "beta").await.unwrap();
        let sid1 = log.ensure_session(1).await.unwrap();
        let sid2 = log.ensure_session(2).await.unwrap();
        assert_ne!(sid1, sid2);
    }
}
