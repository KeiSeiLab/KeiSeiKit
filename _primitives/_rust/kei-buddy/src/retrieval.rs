// SPDX-License-Identifier: Apache-2.0
//! Retrieval context builder — history from ChatLog + atoms from Topics.
//! Constructor Pattern: one responsibility — gather context for conversational_step.

use std::sync::Arc;

use tracing::warn;

use crate::{chat_log::ChatLog, topics::Topics};

/// Pre-assembled context for `conversational_step`.
pub struct RetrievalContext {
    /// Last N messages formatted as "u: ..." / "b: ..." (latest first).
    pub history: Vec<String>,
    /// Top-K kei-sage Units formatted as "[topic] title: content[..200]".
    pub atoms: Vec<String>,
}

/// Gather conversation history and relevant atoms for the current turn.
///
/// All errors are swallowed and logged at `warn` — the caller must always
/// receive a usable (possibly empty) context.
pub async fn retrieve_context(
    chat_log: &Arc<ChatLog>,
    topics: &Arc<Topics>,
    chat_id: i64,
    query: &str,
    history_n: usize,
    atoms_k: i64,
) -> RetrievalContext {
    let history = fetch_history(chat_log, chat_id, query, history_n).await;
    let atoms = fetch_atoms(topics, query, atoms_k).await;
    RetrievalContext { history, atoms }
}

async fn fetch_history(
    chat_log: &Arc<ChatLog>,
    chat_id: i64,
    query: &str,
    n: usize,
) -> Vec<String> {
    let limit = n.max(1) as i64;
    match chat_log.search(query, Some(chat_id), limit).await {
        Ok(msgs) => msgs
            .iter()
            .map(|m| {
                let prefix = if m.role == "user" { "u" } else { "b" };
                format!("{prefix}: {}", m.content)
            })
            .collect(),
        Err(e) => {
            warn!(chat_id, error = %e, "retrieve_context: history fetch failed");
            vec![]
        }
    }
}

async fn fetch_atoms(topics: &Arc<Topics>, query: &str, k: i64) -> Vec<String> {
    match topics.search(query, k).await {
        Ok(units) => units
            .iter()
            .map(|u| {
                let snippet: String = u.content.chars().take(200).collect();
                format!("[{}] {}: {}", u.unit_type, u.title, snippet)
            })
            .collect(),
        Err(e) => {
            warn!(error = %e, "retrieve_context: atom search failed");
            vec![]
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn retrieve_returns_empty_on_empty_stores() {
        let log = Arc::new(ChatLog::from_memory().unwrap());
        let topics = Arc::new(Topics::from_memory().unwrap());
        let ctx = retrieve_context(&log, &topics, 42, "anything", 10, 5).await;
        assert!(ctx.history.is_empty(), "expected empty history");
        assert!(ctx.atoms.is_empty(), "expected empty atoms");
    }

    #[tokio::test]
    async fn retrieve_finds_seeded_data() {
        let log = Arc::new(ChatLog::from_memory().unwrap());
        let topics = Arc::new(Topics::from_memory().unwrap());
        // Seed chat log
        log.log_user(99, "rust programming").await.unwrap();
        log.log_bot(99, "great choice").await.unwrap();
        // Seed topics
        topics.add_topic(99, "rust", "Rust language", "rust systems programming").await.unwrap();
        let ctx = retrieve_context(&log, &topics, 99, "rust", 10, 5).await;
        assert!(!ctx.history.is_empty(), "expected history entries after seeding");
        assert!(!ctx.atoms.is_empty(), "expected atom entries after seeding");
    }
}
