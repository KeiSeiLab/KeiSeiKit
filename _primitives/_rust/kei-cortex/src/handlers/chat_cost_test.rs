//! Inline unit tests for `chat_cost.rs` — the pure-helper subset.
//!
//! Constructor Pattern: ledger-roundtrip tests live in
//! `chat_cost_test_ledger.rs` so each test file stays under the 200-LOC
//! ceiling after Wave 44c added micro-cents + accumulation + poison
//! recovery test coverage.

use super::*;
use crate::tool::loop_driver::{ContentBlock, ModelInvoker, ModelTurn, TokenUsage};
use std::sync::Arc;

#[test]
fn compute_micro_cents_anthropic_mid_load() {
    // 1234 in @ 100c/MTok = 123_400 micro-cents
    // 567 out @ 500c/MTok = 283_500 micro-cents
    // Total = 406_900 micro-cents (= 0.4069 cents — under 1 cent).
    let usage = TokenUsage { input_tokens: 1234, output_tokens: 567 };
    assert_eq!(compute_micro_cents(&usage, 100, 500), 406_900);
}

#[test]
fn compute_micro_cents_zero_zero_yields_zero() {
    let usage = TokenUsage::default();
    assert_eq!(compute_micro_cents(&usage, 100, 500), 0);
}

#[test]
fn compute_micro_cents_large_load_is_exact() {
    // 1.5M in @ 100c/MTok = 150M µc, 0.5M out @ 500c/MTok = 250M µc
    // Total = 400M µc = 400 cents exactly (no ceil).
    let usage = TokenUsage { input_tokens: 1_500_000, output_tokens: 500_000 };
    let micro = compute_micro_cents(&usage, 100, 500);
    assert_eq!(micro, 400_000_000);
    assert_eq!(display_cents(micro), 400);
}

#[test]
fn compute_micro_cents_one_token_does_not_round_up() {
    // F-MED-4 fix: 1 token @ 100c/MTok = 100 micro-cents = 0.0001 cents.
    // Old code charged 1 full cent (ceil-div). New code charges 0.
    let usage = TokenUsage { input_tokens: 1, output_tokens: 0 };
    let micro = compute_micro_cents(&usage, 100, 500);
    assert_eq!(micro, 100, "100 micro-cents = 0.0001 cents");
    assert_eq!(display_cents(micro), 0, "0.0001 cents rounds DOWN to 0");
}

#[test]
fn compute_micro_cents_handles_overflow_via_saturation() {
    // u64::MAX tokens × 1000 cents-per-MTok would overflow naive mul.
    // Saturating arithmetic prevents panic — result is just clamped.
    let usage = TokenUsage { input_tokens: u64::MAX, output_tokens: u64::MAX };
    let _ = compute_micro_cents(&usage, 1000, 1000);
}

#[test]
fn build_agent_id_prefers_conversation_id() {
    assert_eq!(build_agent_id(Some("abc-123"), "alice"), "chat-abc-123");
}

#[test]
fn build_agent_id_falls_back_to_user_plus_timestamp() {
    let id = build_agent_id(None, "alice");
    assert!(id.starts_with("chat-alice-"), "got {id}");
    let suffix = id.trim_start_matches("chat-alice-");
    assert!(suffix.parse::<i64>().is_ok(), "suffix not numeric: {suffix}");
}

#[test]
fn build_agent_id_treats_empty_conversation_id_as_absent() {
    let id = build_agent_id(Some(""), "bob");
    assert!(id.starts_with("chat-bob-"));
}

#[test]
fn provider_rates_unknown_provider_yields_zero() {
    let r = kei_router::LlmRouter::new();
    assert_eq!(provider_rates(&r, "no-such-provider"), (0, 0));
}

#[tokio::test]
async fn wrap_invoker_with_usage_capture_accumulates_across_turns() {
    let accum: Arc<std::sync::Mutex<TokenUsage>> =
        Arc::new(std::sync::Mutex::new(TokenUsage::default()));

    // Inner invoker emits a different usage block each call.
    let counter: Arc<std::sync::Mutex<u64>> = Arc::new(std::sync::Mutex::new(0));
    let counter_clone = counter.clone();
    let inner: ModelInvoker = Arc::new(move |_msgs, _tools| {
        let counter = counter_clone.clone();
        Box::pin(async move {
            let mut c = counter.lock().unwrap();
            *c += 1;
            let n = *c;
            Ok(ModelTurn {
                content: vec![ContentBlock::Text("k".into())],
                stop_reason: "end_turn".into(),
                usage: Some(TokenUsage {
                    input_tokens: 100 * n,
                    output_tokens: 10 * n,
                }),
            })
        })
    });

    let wrapped = wrap_invoker_with_usage_capture(inner, accum.clone());
    // Drive 3 turns.
    for _ in 0..3 {
        let _ = wrapped(vec![], vec![]).await.unwrap();
    }
    let final_usage = snapshot(&accum);
    assert_eq!(final_usage.input_tokens, 100 + 200 + 300);
    assert_eq!(final_usage.output_tokens, 10 + 20 + 30);
}

#[tokio::test]
async fn wrap_invoker_skips_accumulation_on_error() {
    let accum: Arc<std::sync::Mutex<TokenUsage>> =
        Arc::new(std::sync::Mutex::new(TokenUsage::default()));
    let inner: ModelInvoker = Arc::new(|_, _| {
        Box::pin(async move { Err::<ModelTurn, String>("boom".into()) })
    });
    let wrapped = wrap_invoker_with_usage_capture(inner, accum.clone());
    let _ = wrapped(vec![], vec![]).await;
    let final_usage = snapshot(&accum);
    assert_eq!(final_usage, TokenUsage::default(), "errored turns should not bump accumulator");
}

#[test]
fn snapshot_recovers_from_poisoned_mutex() {
    // MISS-5: a panic inside another lock-holder must NOT abort the
    // streaming task. Verify snapshot survives a poisoned mutex.
    let accum: Arc<std::sync::Mutex<TokenUsage>> =
        Arc::new(std::sync::Mutex::new(TokenUsage {
            input_tokens: 7,
            output_tokens: 3,
        }));
    let accum_clone = accum.clone();
    let _ = std::thread::spawn(move || {
        let _g = accum_clone.lock().unwrap();
        panic!("intentional poison");
    })
    .join();
    // Mutex is now poisoned. snapshot must still return the inner state.
    let usage = snapshot(&accum);
    assert_eq!(usage.input_tokens, 7);
    assert_eq!(usage.output_tokens, 3);
}

// Ledger-round-trip tests live in `chat_cost_test_ledger.rs`.
#[path = "chat_cost_test_ledger.rs"]
mod ledger_tests;
