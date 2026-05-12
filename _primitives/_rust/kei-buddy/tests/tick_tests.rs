// SPDX-License-Identifier: Apache-2.0
//! Integration tests for `tick::run_tick_with`.

use std::sync::Arc;

use kei_buddy::{
    chat_log::ChatLog,
    extractor::{LlmExtractor, MockExtractor},
    run_tick_with,
    topics::Topics,
};
use serde_json::json;

fn mock(v: serde_json::Value) -> Arc<dyn LlmExtractor> {
    Arc::new(MockExtractor::new(v))
}

#[tokio::test]
async fn tick_no_topics_returns_zero() {
    let chat_log = Arc::new(ChatLog::from_memory().unwrap());
    let topics = Arc::new(Topics::from_memory().unwrap());
    let report = run_tick_with(chat_log, topics, mock(json!("digest")), 24, 50, vec![42])
        .await
        .unwrap();
    assert_eq!(report.topics_processed, 0);
    assert_eq!(report.digests_created, 0);
    assert!(report.errors.is_empty());
}

#[tokio::test]
async fn tick_with_topic_and_messages_creates_digest() {
    let chat_log = Arc::new(ChatLog::from_memory().unwrap());
    let topics = Arc::new(Topics::from_memory().unwrap());

    // Title "rust" so FTS search on "rust" matches messages containing "rust".
    topics.add_topic(42, "rust", "rust", "about rust").await.unwrap();
    chat_log.log_user(42, "I love rust borrow checker").await.unwrap();
    chat_log.log_user(42, "rust ownership is great").await.unwrap();

    let report = run_tick_with(
        chat_log,
        topics.clone(),
        mock(json!("• Rust is great\n• Ownership rules")),
        24,
        50,
        vec![42],
    )
    .await
    .unwrap();

    assert_eq!(report.topics_processed, 1, "errors: {:?}", report.errors);
    assert_eq!(report.digests_created, 1, "errors: {:?}", report.errors);

    let digests = topics.digests_for(42, "rust").await.unwrap();
    assert_eq!(digests.len(), 1);
}

#[tokio::test]
async fn tick_skips_topic_without_recent_messages() {
    let chat_log = Arc::new(ChatLog::from_memory().unwrap());
    let topics = Arc::new(Topics::from_memory().unwrap());

    topics.add_topic(42, "go", "Go Programming", "about go").await.unwrap();
    // No messages added — topic has 0 recent messages, skip digest.

    let report = run_tick_with(
        chat_log,
        topics,
        mock(json!("• Go is fine")),
        24,
        50,
        vec![42],
    )
    .await
    .unwrap();

    assert_eq!(report.topics_processed, 1);
    assert_eq!(report.digests_created, 0);
}
