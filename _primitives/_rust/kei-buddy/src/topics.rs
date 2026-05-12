// SPDX-License-Identifier: Apache-2.0
//! `Topics` — async adapter storing topics + digests in kei-sage.
//! Constructor Pattern: one responsibility — bridge kei-buddy to the
//! kei-sage knowledge vault. All rusqlite calls via `spawn_blocking`.

use std::path::Path;
use std::sync::{Arc, Mutex};

use kei_sage::{
    edges::{add_edge, list_outgoing},
    search::fts_search,
    store::Store,
    Unit,
};
use rusqlite::params;

use crate::error::BuddyError;
/// Wraps kei-sage `Store` with buddy-domain topic/digest API.
pub struct Topics {
    store: Arc<Mutex<Store>>,
}

impl Topics {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, BuddyError> {
        let store = Store::open(path.as_ref())
            .map_err(|e| BuddyError::Memory(format!("{e}")))?;
        Ok(Self { store: Arc::new(Mutex::new(store)) })
    }

    pub fn from_memory() -> Result<Self, BuddyError> {
        let store = Store::open_memory()
            .map_err(|e| BuddyError::Memory(format!("{e}")))?;
        Ok(Self { store: Arc::new(Mutex::new(store)) })
    }

    /// Add a topic; idempotent by `source_path`. Returns unit id.
    pub async fn add_topic(
        &self, chat_id: i64, slug: &str, title: &str, content: &str,
    ) -> Result<i64, BuddyError> {
        let src = format!("kei-buddy/chat-{chat_id}/topic/{slug}");
        let unit = make_unit("buddy_topic", title, content, "", &src);
        let store = Arc::clone(&self.store);
        spawn(move || find_or_add(&store.lock().expect("poisoned"), &unit)).await
    }

    /// Add a digest note linked to a topic. Returns digest unit id.
    pub async fn add_digest(
        &self, chat_id: i64, topic_slug: &str, timestamp: i64, content: &str,
    ) -> Result<i64, BuddyError> {
        let topic_path = format!("kei-buddy/chat-{chat_id}/topic/{topic_slug}");
        let dst = format!("kei-buddy/chat-{chat_id}/digest/{timestamp}");
        let unit = make_unit("buddy_digest", &format!("digest-{timestamp}"), content, "E3", &dst);
        let store = Arc::clone(&self.store);
        spawn(move || {
            let locked = store.lock().expect("poisoned");
            let id = find_or_add(&locked, &unit)?;
            add_edge(&locked, &topic_path, &dst, "digest_for", 1.0)
                .map_err(|e| BuddyError::Memory(format!("{e}")))?;
            Ok(id)
        })
        .await
    }

    /// Full-text search across all kei-buddy units.
    pub async fn search(&self, q: &str, limit: i64) -> Result<Vec<Unit>, BuddyError> {
        let store = Arc::clone(&self.store);
        let q = q.to_string();
        spawn(move || {
            fts_search(&store.lock().expect("poisoned"), &q, limit)
                .map_err(|e| BuddyError::Memory(format!("{e}")))
        })
        .await
    }

    /// List digest units linked from a topic via "digest_for" edges.
    pub async fn digests_for(&self, chat_id: i64, slug: &str) -> Result<Vec<Unit>, BuddyError> {
        let topic_path = format!("kei-buddy/chat-{chat_id}/topic/{slug}");
        let store = Arc::clone(&self.store);
        spawn(move || {
            let locked = store.lock().expect("poisoned");
            let edges = list_outgoing(&locked, &topic_path)
                .map_err(|e| BuddyError::Memory(format!("{e}")))?;
            let mut out = Vec::new();
            for e in edges.into_iter().filter(|e| e.edge_type == "digest_for") {
                if let Some(u) = unit_by_path(&locked, &e.dst_path)
                    .map_err(|e| BuddyError::Memory(format!("{e}")))?
                {
                    out.push(u);
                }
            }
            Ok(out)
        })
        .await
    }

    /// List all topic units for a chat via raw SELECT.
    pub async fn list_topics(&self, chat_id: i64) -> Result<Vec<Unit>, BuddyError> {
        let prefix = format!("kei-buddy/chat-{chat_id}/topic/%");
        let store = Arc::clone(&self.store);
        spawn(move || {
            topics_by_prefix(&store.lock().expect("poisoned"), &prefix)
                .map_err(|e| BuddyError::Memory(format!("{e}")))
        })
        .await
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────
fn make_unit(unit_type: &str, title: &str, content: &str, grade: &str, path: &str) -> Unit {
    Unit {
        id: 0, unit_type: unit_type.to_string(), title: title.to_string(),
        content: content.to_string(), evidence_grade: grade.to_string(),
        source_path: path.to_string(), vault_path: path.to_string(),
        category: "kei-buddy".to_string(), created_at: 0, updated_at: 0,
    }
}

fn find_or_add(store: &Store, unit: &Unit) -> Result<i64, BuddyError> {
    let existing: Option<i64> = store.conn()
        .query_row("SELECT id FROM knowledge_units WHERE source_path=?1 LIMIT 1",
            params![unit.source_path], |r| r.get(0))
        .ok();
    if let Some(id) = existing { return Ok(id); }
    store.add_unit(unit).map_err(|e| BuddyError::Memory(format!("{e}")))
}

fn unit_by_path(store: &Store, src: &str) -> anyhow::Result<Option<Unit>> {
    let mut stmt = store.conn().prepare(
        "SELECT id,unit_type,title,content,evidence_grade,source_path,vault_path,
                category,created_at,updated_at FROM knowledge_units WHERE source_path=?1 LIMIT 1")?;
    let mut rows = stmt.query(params![src])?;
    if let Some(r) = rows.next()? { return Ok(Some(row_to_unit(r)?)); }
    Ok(None)
}

fn topics_by_prefix(store: &Store, prefix: &str) -> anyhow::Result<Vec<Unit>> {
    let mut stmt = store.conn().prepare(
        "SELECT id,unit_type,title,content,evidence_grade,source_path,vault_path,
                category,created_at,updated_at FROM knowledge_units
         WHERE category='kei-buddy' AND unit_type='buddy_topic' AND source_path LIKE ?1")?;
    let rows = stmt.query_map(params![prefix], row_to_unit)?;
    let mut out = Vec::new();
    for r in rows { out.push(r?); }
    Ok(out)
}

fn row_to_unit(r: &rusqlite::Row) -> rusqlite::Result<Unit> {
    Ok(Unit { id: r.get(0)?, unit_type: r.get(1)?, title: r.get(2)?,
        content: r.get(3)?, evidence_grade: r.get(4)?, source_path: r.get(5)?,
        vault_path: r.get(6)?, category: r.get(7)?, created_at: r.get(8)?, updated_at: r.get(9)? })
}

/// Thin wrapper: run closure in `spawn_blocking`, flatten join error.
async fn spawn<F, T>(f: F) -> Result<T, BuddyError>
where
    F: FnOnce() -> Result<T, BuddyError> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(f)
        .await
        .map_err(|e| BuddyError::Memory(format!("spawn_blocking join: {e}")))?
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn add_topic_then_search_finds_it() {
        let t = Topics::from_memory().unwrap();
        t.add_topic(42, "ml", "ML Concepts", "talk about ml").await.unwrap();
        let res = t.search("ml", 10).await.unwrap();
        assert!(!res.is_empty());
    }

    #[tokio::test]
    async fn add_topic_is_idempotent() {
        let t = Topics::from_memory().unwrap();
        t.add_topic(42, "ml", "ML Concepts", "first").await.unwrap();
        t.add_topic(42, "ml", "ML Concepts", "second").await.unwrap();
        assert_eq!(t.list_topics(42).await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn add_digest_creates_edge_and_dest() {
        let t = Topics::from_memory().unwrap();
        t.add_topic(42, "ml", "ML Concepts", "about ml").await.unwrap();
        t.add_digest(42, "ml", 1_000_000, "digest content").await.unwrap();
        assert_eq!(t.digests_for(42, "ml").await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn list_topics_scopes_per_chat() {
        let t = Topics::from_memory().unwrap();
        t.add_topic(1, "rust", "Rust", "rust stuff").await.unwrap();
        t.add_topic(2, "go", "Go", "go stuff").await.unwrap();
        assert_eq!(t.list_topics(1).await.unwrap().len(), 1);
    }
}
