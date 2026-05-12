// SPDX-License-Identifier: Apache-2.0
//! Topic classification helper — free function invoked after `OnboardState::Ready`.
//!
//! Constructor Pattern: one responsibility — LLM classify + Topics store, fire-and-forget.

use std::time::{SystemTime, UNIX_EPOCH};

use tracing::{error, warn};

use crate::{extractor::LlmExtractor, topics::Topics};

const CLASSIFY_PROMPT: &str = concat!(
    "You are a topic classifier. Output a single JSON object with two string fields: ",
    "\"slug\" (kebab-case, ascii, ≤30 chars, like \"work-meetings\") and ",
    "\"title\" (human-readable in the user's language, ≤50 chars). ",
    "Classify the following user message into ONE topic. ",
    "Output only the JSON, no prose, no markdown fences."
);

/// Classify `text` into a topic and store it in `topics`. Never panics; never returns `Err`.
pub async fn classify_and_store_topic(
    extractor: &dyn LlmExtractor,
    topics: &Topics,
    chat_id: i64,
    text: &str,
) {
    let val = match extractor.extract(CLASSIFY_PROMPT, text).await {
        Ok(v) => v,
        Err(e) => {
            warn!(chat_id, error = %e, "topic classifier LLM call failed");
            return;
        }
    };

    let slug = match val.get("slug").and_then(|v| v.as_str()) {
        Some(s) if !s.is_empty() => s.to_string(),
        _ => {
            warn!(chat_id, "topic classifier returned no slug field");
            return;
        }
    };
    let title = match val.get("title").and_then(|v| v.as_str()) {
        Some(s) if !s.is_empty() => s.to_string(),
        _ => {
            warn!(chat_id, "topic classifier returned no title field");
            return;
        }
    };

    if !is_valid_slug(&slug) {
        warn!(chat_id, slug = %slug, "topic slug failed validation; skipping");
        return;
    }

    if let Err(e) = topics.add_topic(chat_id, &slug, &title, text).await {
        error!(chat_id, slug = %slug, error = %e, "topics.add_topic failed");
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    if let Err(e) = topics.add_digest(chat_id, &slug, now, text).await {
        error!(chat_id, slug = %slug, error = %e, "topics.add_digest failed");
    }
}

fn is_valid_slug(slug: &str) -> bool {
    !slug.is_empty()
        && slug.len() <= 40
        && slug.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{extractor::MockExtractor, topics::Topics};
    use serde_json::json;

    async fn make_topics() -> Topics {
        Topics::from_memory().unwrap()
    }

    #[tokio::test]
    async fn classify_and_store_skips_invalid_slug() {
        let extractor = MockExtractor::new(json!({"slug": "has spaces", "title": "X"}));
        let topics = make_topics().await;
        classify_and_store_topic(&extractor, &topics, 1, "hello").await;
        assert!(topics.list_topics(1).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn classify_and_store_adds_topic_for_valid_slug() {
        let extractor = MockExtractor::new(json!({"slug": "work-stuff", "title": "Work Stuff"}));
        let topics = make_topics().await;
        classify_and_store_topic(&extractor, &topics, 1, "I have a meeting").await;
        assert_eq!(topics.list_topics(1).await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn classify_and_store_handles_missing_fields() {
        let extractor = MockExtractor::new(json!({}));
        let topics = make_topics().await;
        classify_and_store_topic(&extractor, &topics, 1, "any text").await;
        assert!(topics.list_topics(1).await.unwrap().is_empty());
    }
}
