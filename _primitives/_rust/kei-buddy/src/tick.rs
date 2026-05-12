// SPDX-License-Identifier: Apache-2.0
//! `run_tick` — one-shot digest generator for all tracked topics.
//!
//! Constructor Pattern: one responsibility — walk topics, collect recent
//! messages, call LLM, persist digests. Called from `kei-buddy-tick` bin.

use std::sync::Arc;

use crate::{
    chat_log::ChatLog,
    error::BuddyError,
    extractor::LlmExtractor,
    topics::Topics,
};

// ─── public types ────────────────────────────────────────────────────────────

/// Configuration for a `run_tick` invocation (file paths + tuning knobs).
pub struct TickConfig {
    pub buddy_db_path: String,
    pub chat_log_db_path: String,
    pub topics_db_path: String,
    /// Only messages newer than `now - since_hours * 3600` are included.
    pub since_hours: i64,
    pub max_messages_per_topic: i64,
    pub llm_proxy_url: Option<String>,
    pub llm_api_key: Option<String>,
    pub llm_model: Option<String>,
}

/// Summary returned after one tick run.
#[derive(Debug, Default)]
pub struct TickReport {
    pub topics_processed: usize,
    pub digests_created: usize,
    pub errors: Vec<String>,
}

// ─── public entry points ─────────────────────────────────────────────────────

/// Open DBs from `cfg` paths, discover chat_ids, run digests.
///
/// Exits 0 on per-topic errors — they are collected in `TickReport.errors`.
pub async fn run_tick(cfg: TickConfig) -> Result<TickReport, BuddyError> {
    let chat_log = Arc::new(ChatLog::from_path(&cfg.chat_log_db_path)?);
    let topics = Arc::new(Topics::from_path(&cfg.topics_db_path)?);
    let extractor = build_extractor(&cfg);
    let chat_ids = load_chat_ids(&cfg.buddy_db_path)?;
    run_tick_with(chat_log, topics, extractor, cfg.since_hours, cfg.max_messages_per_topic, chat_ids).await
}

/// Testable core: accepts pre-built instances and an explicit chat_id list.
pub async fn run_tick_with(
    chat_log: Arc<ChatLog>,
    topics: Arc<Topics>,
    extractor: Arc<dyn LlmExtractor>,
    since_hours: i64,
    max_messages: i64,
    known_chat_ids: Vec<i64>,
) -> Result<TickReport, BuddyError> {
    let mut report = TickReport::default();
    let cutoff = now_epoch() - since_hours * 3600;
    for chat_id in known_chat_ids {
        let topic_units = match topics.list_topics(chat_id).await {
            Ok(v) => v,
            Err(e) => { report.errors.push(format!("list_topics({chat_id}): {e}")); continue; }
        };
        for unit in topic_units {
            report.topics_processed += 1;
            process_topic(&chat_log, &topics, &*extractor, chat_id, &unit, cutoff, max_messages, &mut report).await;
        }
    }
    Ok(report)
}

/// Collect recent messages for one topic, call extractor, persist digest.
async fn process_topic(
    chat_log: &ChatLog,
    topics: &Topics,
    extractor: &dyn LlmExtractor,
    chat_id: i64,
    unit: &kei_sage::Unit,
    cutoff: i64,
    max_messages: i64,
    report: &mut TickReport,
) {
    let slug = slug_from_path(&unit.source_path);
    let msgs = match chat_log.search(&unit.title, Some(chat_id), max_messages).await {
        Ok(v) => v,
        Err(e) => { report.errors.push(format!("search({chat_id}, {}): {e}", unit.title)); return; }
    };
    let recent: Vec<_> = msgs.into_iter().filter(|m| m.created_at >= cutoff).collect();
    if recent.is_empty() { return; }
    let val = match extractor.extract(&digest_system_prompt(&unit.title), &concat_messages(&recent)).await {
        Ok(v) => v,
        Err(e) => { report.errors.push(format!("extract({chat_id}, {}): {e}", unit.title)); return; }
    };
    if val.is_null() || val.as_object().map(|m| m.is_empty()).unwrap_or(false) {
        report.errors.push(format!("extractor empty for '{}' in chat {chat_id}", unit.title));
        return;
    }
    let text = val.as_str().map(|s| s.to_string()).unwrap_or_else(|| val.to_string());
    match topics.add_digest(chat_id, slug, now_epoch(), &text).await {
        Ok(_) => report.digests_created += 1,
        Err(e) => report.errors.push(format!("add_digest({chat_id}, {slug}): {e}")),
    }
}

// ─── helpers ──────────────────────────────────────────────────────────────────

fn now_epoch() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn slug_from_path(source_path: &str) -> &str {
    source_path.rfind("topic/").map(|i| &source_path[i + 6..]).unwrap_or(source_path)
}

fn concat_messages(msgs: &[kei_chat_store::sessions::ChatMessage]) -> String {
    let mut out = String::new();
    for m in msgs {
        out.push_str(&m.content);
        out.push('\n');
        if out.len() > 2000 { break; }
    }
    out.truncate(2000);
    out
}

fn digest_system_prompt(title: &str) -> String {
    format!(
        "You are a digest writer. Summarise the following messages about \"{title}\" \
         into 2-4 bullet points in the user's language. Output plain text, no markdown \
         headers, no preamble."
    )
}

/// Read distinct chat_ids from buddy_state table.
fn load_chat_ids(buddy_db_path: &str) -> Result<Vec<i64>, BuddyError> {
    use crate::store::SqliteBuddyStore;
    let store = SqliteBuddyStore::from_path(buddy_db_path)?;
    load_chat_ids_from_store(&store)
}

/// Extract chat_ids from a BuddyStore (visible for testing via in-memory store).
pub fn load_chat_ids_from_store(store: &crate::store::SqliteBuddyStore) -> Result<Vec<i64>, BuddyError> {
    let conn = store.inner_conn();
    let mut stmt = conn.prepare("SELECT DISTINCT chat_id FROM buddy_state")
        .map_err(|e| BuddyError::Memory(e.to_string()))?;
    let ids: Vec<i64> = stmt
        .query_map([], |r| r.get(0))
        .map_err(|e| BuddyError::Memory(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();
    Ok(ids)
}

fn build_extractor(cfg: &TickConfig) -> Arc<dyn LlmExtractor> {
    #[cfg(feature = "extractor-openai")]
    {
        let proxy = cfg.llm_proxy_url.clone().or_else(|| Some("https://api.openai.com".to_string()));
        let key = cfg.llm_api_key.clone().or_else(|| std::env::var("OPENAI_API_KEY").ok());
        if let (Some(proxy_url), Some(api_key)) = (proxy, key) {
            use crate::extractor::openai::OpenAiExtractor;
            let ext = match &cfg.llm_model {
                Some(m) => OpenAiExtractor::new_with_model(proxy_url, api_key, m.clone()),
                None => OpenAiExtractor::new(proxy_url, api_key),
            };
            return Arc::new(ext);
        }
    }
    let _ = cfg; // suppress unused warning when feature off
    Arc::new(NoOpExtractor)
}

/// Fallback extractor when `extractor-openai` is not compiled or creds are absent.
struct NoOpExtractor;

#[async_trait::async_trait]
impl LlmExtractor for NoOpExtractor {
    async fn extract(&self, _system: &str, _user_text: &str) -> Result<serde_json::Value, BuddyError> {
        Ok(serde_json::Value::Null)
    }
}

