// SPDX-License-Identifier: Apache-2.0
//! Tests for `machine::handle_step`.
//! Extracted from machine.rs to keep it within the 260-LOC exception budget.
//!
//! TopicResearch-specific tests live in the sibling module
//! `machine_tests_topic_research` (Constructor Pattern: split by concern).

use serde_json::json;

use crate::extractor::MockExtractor;
use crate::machine::handle_step;
use crate::state::OnboardState;

mod machine_tests_topic_research;

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Runtime::new().unwrap()
}

#[test]
fn intro_to_ask_language() {
    rt().block_on(async {
        let mock = MockExtractor::new(json!({}));
        let out = handle_step(&OnboardState::Intro, "hi", &json!({}), &mock)
            .await
            .unwrap();
        // Intro now transitions to AskLanguage, not AskName.
        assert_eq!(out.next_state, OnboardState::AskLanguage);
        assert!(!out.response_text.is_empty(), "intro response must not be empty");
    });
}

#[test]
fn ask_language_en_advances_to_ask_name() {
    rt().block_on(async {
        let mock = MockExtractor::new(json!({}));
        let out = handle_step(&OnboardState::AskLanguage, "en", &json!({}), &mock)
            .await
            .unwrap();
        assert_eq!(out.next_state, OnboardState::AskName);
        assert_eq!(
            out.persona_patch["language"].as_str(),
            Some("en"),
            "persona_patch must contain language=en"
        );
        assert!(
            out.response_text.contains("What's your name"),
            "response must contain English ask_name phrase, got: {:?}",
            out.response_text
        );
    });
}

#[test]
fn ask_language_ru_advances_to_ask_name() {
    rt().block_on(async {
        let mock = MockExtractor::new(json!({}));
        let out = handle_step(&OnboardState::AskLanguage, "ru", &json!({}), &mock)
            .await
            .unwrap();
        assert_eq!(out.next_state, OnboardState::AskName);
        assert_eq!(
            out.persona_patch["language"].as_str(),
            Some("ru"),
            "persona_patch must contain language=ru"
        );
        assert!(
            out.response_text.contains("называть"),
            "response must contain Russian ask_name phrase, got: {:?}",
            out.response_text
        );
    });
}

#[test]
fn ask_language_invalid_stays_in_state() {
    rt().block_on(async {
        let mock = MockExtractor::new(json!({}));
        let out = handle_step(&OnboardState::AskLanguage, "blah", &json!({}), &mock)
            .await
            .unwrap();
        assert_eq!(out.next_state, OnboardState::AskLanguage, "invalid input must loop");
        assert!(
            out.response_text.contains("en") && out.response_text.contains("ru"),
            "error response must mention both options, got: {:?}",
            out.response_text
        );
    });
}

#[test]
fn migration_sets_ru_when_language_missing() {
    rt().block_on(async {
        // Persona has no `language` key — simulates a chat started before this commit.
        let mock = MockExtractor::new(json!({ "name": "Denis" }));
        let persona = json!({});
        let out = handle_step(&OnboardState::AskName, "Denis", &persona, &mock)
            .await
            .unwrap();
        assert_eq!(
            out.persona_patch["language"].as_str(),
            Some("ru"),
            "migration must inject language=ru when key is missing"
        );
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
