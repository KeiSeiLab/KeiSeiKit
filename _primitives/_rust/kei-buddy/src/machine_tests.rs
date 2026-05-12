// SPDX-License-Identifier: Apache-2.0
//! Tests for `machine::handle_step`.
//! Extracted from machine.rs to keep it within the 250-LOC exception budget.

use serde_json::json;

use crate::extractor::MockExtractor;
use crate::machine::handle_step;
use crate::state::OnboardState;

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Runtime::new().unwrap()
}

#[test]
fn intro_to_ask_name() {
    rt().block_on(async {
        let mock = MockExtractor::new(json!({}));
        let out = handle_step(&OnboardState::Intro, "hi", &json!({}), &mock)
            .await
            .unwrap();
        assert_eq!(out.next_state, OnboardState::AskName);
        assert!(!out.response_text.is_empty(), "intro response must not be empty");
    });
}

#[test]
fn ask_name_extracts_and_advances() {
    rt().block_on(async {
        let mock = MockExtractor::new(json!({ "name": "Denis" }));
        let out = handle_step(&OnboardState::AskName, "Denis", &json!({}), &mock)
            .await
            .unwrap();
        assert_eq!(out.next_state, OnboardState::AskTone);
        assert_eq!(out.persona_patch["name"].as_str(), Some("Denis"));
    });
}

#[test]
fn ask_tone_extracts_and_advances() {
    rt().block_on(async {
        let mock = MockExtractor::new(json!({ "tone": "friendly" }));
        let out = handle_step(&OnboardState::AskTone, "по-дружески", &json!({}), &mock)
            .await
            .unwrap();
        assert_eq!(out.next_state, OnboardState::AskInterests);
        assert_eq!(out.persona_patch["tone"].as_str(), Some("friendly"));
    });
}

#[test]
fn ask_interests_seeds_topic_queue() {
    rt().block_on(async {
        let mock = MockExtractor::new(json!({ "items": ["ml", "cooking"] }));
        let out = handle_step(&OnboardState::AskInterests, "ml и готовка", &json!({}), &mock)
            .await
            .unwrap();
        assert_eq!(out.next_state, OnboardState::AskHobbies);
        let interests = out.persona_patch["interests"].as_array().unwrap();
        assert_eq!(interests.len(), 2);
        assert_eq!(interests[0].as_str(), Some("ml"));
    });
}

#[test]
fn ask_hobbies_seeds_topic_queue_from_interests_and_hobbies() {
    rt().block_on(async {
        let mock = MockExtractor::new(json!({ "items": ["surfing"] }));
        let persona = json!({ "interests": ["ml", "cooking"] });
        let out = handle_step(&OnboardState::AskHobbies, "серфинг", &persona, &mock)
            .await
            .unwrap();
        // current_topic = "ml" (first), queue = ["cooking", "surfing"]
        assert_eq!(out.next_state, OnboardState::TopicSpecifics);
        let queue = out.persona_patch["__topic_state"]["queue"].as_array().unwrap();
        assert_eq!(queue.len(), 2, "queue must have [cooking, surfing]");
        assert_eq!(queue[0]["name"].as_str(), Some("cooking"));
    });
}

#[test]
fn ready_is_idempotent() {
    rt().block_on(async {
        let mock = MockExtractor::new(json!({}));
        let out = handle_step(&OnboardState::Ready, "hello", &json!({}), &mock)
            .await
            .unwrap();
        assert_eq!(out.next_state, OnboardState::Ready);
        assert!(out.response_text.is_empty());
        assert_eq!(out.persona_patch, json!({}));
    });
}

#[test]
fn ask_tone_falls_back_to_friendly_on_unknown() {
    rt().block_on(async {
        let mock = MockExtractor::new(json!({ "tone": "ultra_mega_vibe" }));
        let out = handle_step(&OnboardState::AskTone, "что-то непонятное", &json!({}), &mock)
            .await
            .unwrap();
        assert_eq!(out.persona_patch["tone"].as_str(), Some("friendly"));
    });
}
