//! Ledger round-trip tests for `chat_cost.rs`. Split off from
//! `chat_cost_test.rs` so each test file stays under the 200-LOC
//! Constructor Pattern ceiling.

use super::super::*;
use tempfile::TempDir;

#[test]
fn record_chat_cost_roundtrips_through_ledger() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("ledger.sqlite");
    record_chat_cost(CostWrite {
        ledger_path: path.clone(),
        agent_id: "chat-conv-99".into(),
        provider: "anthropic".into(),
        model: "claude-haiku-4-5-20251001".into(),
        cents: 42,
        micro_cents: 42 * 1_000_000,
    });
    let conn = kei_ledger::open(&path).unwrap();
    let (cents, micro, provider, model) =
        kei_ledger::read_cost_micro(&conn, "chat-conv-99").unwrap().expect("row present");
    assert_eq!(cents, 42);
    assert_eq!(micro, 42_000_000);
    assert_eq!(provider, "anthropic");
    assert_eq!(model, "claude-haiku-4-5-20251001");
}

#[test]
fn record_chat_cost_accumulates_on_repeat_per_conversation() {
    // F-HIGH-3 regression: 10 chat turns under one conversation_id used
    // to bill only the last; now they accumulate.
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("ledger.sqlite");
    for cents in [10u64, 50, 30] {
        record_chat_cost(CostWrite {
            ledger_path: path.clone(),
            agent_id: "chat-conv-acc".into(),
            provider: "anthropic".into(),
            model: "claude-haiku".into(),
            cents,
            micro_cents: cents * 1_000_000,
        });
    }
    let conn = kei_ledger::open(&path).unwrap();
    let (cents, _, _) =
        kei_ledger::read_cost(&conn, "chat-conv-acc").unwrap().expect("row present");
    assert_eq!(cents, 90, "10 + 50 + 30 accumulates (was 30 last-write-wins)");
}

#[test]
fn record_chat_cost_micro_cents_accumulates_without_loss() {
    // F-MED-4: 100 micro-turns of 5 input tokens each @ 1c/MTok.
    // Each turn = 5 micro-cents. Total = 500 micro-cents over 100 turns.
    // Old code charged 100 cents ($1.00 — gross over-charge).
    // New code charges 0.0005 cents (under reporting threshold) but
    // the ledger keeps 500 µc visible for accurate aggregation.
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("ledger.sqlite");
    for _ in 0..100 {
        record_chat_cost(CostWrite {
            ledger_path: path.clone(),
            agent_id: "chat-micro".into(),
            provider: "anthropic".into(),
            model: "claude-haiku".into(),
            cents: 0,
            micro_cents: 5,
        });
    }
    let conn = kei_ledger::open(&path).unwrap();
    let (cents, micro, _, _) =
        kei_ledger::read_cost_micro(&conn, "chat-micro").unwrap().expect("row present");
    assert_eq!(cents, 0, "stays 0 cents — under threshold");
    assert_eq!(micro, 500, "but micro accumulator is exact");
}
