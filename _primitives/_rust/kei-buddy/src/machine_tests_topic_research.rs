// SPDX-License-Identifier: Apache-2.0
//! TopicResearch FSM arm tests — split from machine_tests.rs (Constructor Pattern: ≤200 LOC).

use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

use crate::error::BuddyError;
use crate::extractor::{LlmExtractor, MockExtractor};
use crate::machine::handle_step;
use crate::state::OnboardState;

/// Returns responses in sequence: responses[0] on call 0, responses[1] on call 1, etc.
/// After exhaustion repeats the last element.
pub(super) struct SequenceMockExtractor {
    responses: Arc<Mutex<Vec<Value>>>,
    call_idx: Arc<Mutex<usize>>,
}

impl SequenceMockExtractor {
    pub(super) fn new(responses: Vec<Value>) -> Self {
        Self {
            responses: Arc::new(Mutex::new(responses)),
            call_idx: Arc::new(Mutex::new(0)),
        }
    }
}

#[async_trait]
impl LlmExtractor for SequenceMockExtractor {
    async fn extract(&self, _system: &str, _user_text: &str) -> Result<Value, BuddyError> {
        let mut idx = self.call_idx.lock().unwrap();
        let responses = self.responses.lock().unwrap();
        let resp = responses.get(*idx).or_else(|| responses.last()).cloned()
            .unwrap_or_else(|| json!({}));
        if *idx + 1 < responses.len() {
            *idx += 1;
        }
        Ok(resp)
    }
}

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Runtime::new().unwrap()
}

fn topic_research_persona() -> Value {
    json!({
        "language": "en",
        "current_topic": { "name": "Rust", "kind": "interest" },
        "current_topic_specifics": ["async", "traits"],
        "current_topic_defer": false,
        "__topic_state": { "queue": [], "index": 0 }
    })
}

/// User says "yes" and LLM returns sources → next=TopicSources, sources in response and patch.
#[test]
fn topic_research_yes_proposes_sources() {
    rt().block_on(async {
        let sources_resp = json!({
            "sources": [
                { "name": "S1", "url": "https://a.com", "why": "x" }
            ]
        });
        let mock = SequenceMockExtractor::new(vec![json!({ "yes": true }), sources_resp]);
        let out = handle_step(&OnboardState::TopicResearch, "yes", &topic_research_persona(), &mock)
            .await
            .unwrap();
        assert_eq!(out.next_state, OnboardState::TopicSources, "must advance to TopicSources");
        assert!(out.response_text.contains("S1"), "response must mention source name S1, got: {:?}", out.response_text);
        assert!(out.response_text.contains("https://a.com"), "response must include URL, got: {:?}", out.response_text);
        let proposed = out.persona_patch["current_topic_proposed"].as_array().unwrap();
        assert_eq!(proposed.len(), 1, "one proposed source must be stored in patch");
    });
}

/// User says "yes" but LLM returns empty sources → fallback message, still next=TopicSources.
#[test]
fn topic_research_yes_empty_sources_still_advances() {
    rt().block_on(async {
        let mock = SequenceMockExtractor::new(vec![
            json!({ "yes": true }),
            json!({ "sources": [] }),
        ]);
        let out = handle_step(&OnboardState::TopicResearch, "yes", &topic_research_persona(), &mock)
            .await
            .unwrap();
        assert_eq!(out.next_state, OnboardState::TopicSources, "must still enter TopicSources");
        let proposed = out.persona_patch["current_topic_proposed"].as_array().unwrap();
        assert!(proposed.is_empty(), "proposed must be empty in patch");
        let lower = out.response_text.to_lowercase();
        assert!(
            lower.contains("suggest") || lower.contains("предложи") || lower.contains("предложить"),
            "fallback must ask user to suggest a source, got: {:?}", out.response_text
        );
    });
}

/// User says "no" → TopicSources is skipped entirely, advances past it.
#[test]
fn topic_research_no_skips_topic_sources() {
    rt().block_on(async {
        let mock = MockExtractor::new(json!({ "yes": false }));
        let out = handle_step(&OnboardState::TopicResearch, "no", &topic_research_persona(), &mock)
            .await
            .unwrap();
        assert_ne!(
            out.next_state, OnboardState::TopicSources,
            "\"no\" must skip TopicSources, got: {:?}", out.next_state
        );
    });
}
