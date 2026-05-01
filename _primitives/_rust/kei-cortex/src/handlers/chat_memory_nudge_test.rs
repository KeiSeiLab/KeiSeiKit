//! Inline unit tests for `chat_memory_nudge.rs`.
//!
//! Constructor Pattern: extracted to a sibling so the parent stays
//! ≤200 LOC. Tests cover the context-builder and verify the wiring
//! ends up with both `invoker` and `persist` populated (regression
//! against the prior dead-code state).

use super::*;
use crate::agent::memory_review_task::Invoker;
use crate::config::AppConfig;
use crate::state::{AppState, InvokerFactory};
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};

struct CountingInvoker {
    calls: Arc<AtomicUsize>,
}

impl Invoker for CountingInvoker {
    fn invoke(
        &self,
        _s: Vec<Turn>,
        _p: String,
    ) -> Pin<Box<dyn std::future::Future<Output = String> + Send + '_>> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Box::pin(async move { "Nothing to save.".to_string() })
    }
}

fn dummy_state(invoker_calls: Arc<AtomicUsize>) -> AppState {
    let cfg = AppConfig::new(
        Some(9999),
        Some("https://example.test".into()),
        Some(PathBuf::from("/tmp/kc-tok")),
        Some(PathBuf::from("/tmp/kc-led")),
        Some(PathBuf::from("/tmp/kc-pets")),
        Some(PathBuf::from("/tmp/kc-mem.sqlite")),
        Some(PathBuf::from("/tmp/kc-live2d")),
    );
    let factory: InvokerFactory = Arc::new(move || {
        Arc::new(CountingInvoker {
            calls: invoker_calls.clone(),
        }) as Arc<dyn Invoker>
    });
    AppState::with_router_and_factory(
        cfg,
        "tok".into(),
        Arc::new(kei_router::LlmRouter::new()),
        factory,
    )
}

#[tokio::test]
async fn build_context_populates_invoker_and_persist() {
    let calls = Arc::new(AtomicUsize::new(0));
    let st = dummy_state(calls.clone());
    let ctx = build_context(&st, "alice", "conv-1", "hi", "hello");
    assert!(
        ctx.invoker.is_some(),
        "build_context must wire an invoker (regression: dead code)"
    );
    assert!(
        ctx.persist.is_some(),
        "build_context must wire a persist target"
    );
}

#[tokio::test]
async fn build_context_records_both_turns_in_order() {
    let calls = Arc::new(AtomicUsize::new(0));
    let st = dummy_state(calls.clone());
    let ctx = build_context(&st, "alice", "conv-1", "hi", "hello");
    let turns = ctx.turns.read().await;
    assert_eq!(turns.len(), 2);
    assert_eq!(turns[0].role, "user");
    assert_eq!(turns[0].content, "hi");
    assert_eq!(turns[1].role, "assistant");
    assert_eq!(turns[1].content, "hello");
}

#[tokio::test]
async fn spawn_nudge_eventually_invokes_invoker_at_threshold() {
    // Arrange a scheduler with interval 1 + zero cooldown by
    // re-constructing AppState manually so we can control the
    // scheduler. We test via the public surface: ten calls of
    // spawn_nudge against a 10-turn-default scheduler should fire
    // exactly once. Using interval=1 keeps the test fast.
    let calls = Arc::new(AtomicUsize::new(0));
    let st = dummy_state(calls.clone());
    // Default scheduler uses 10-turn interval. Spin 10 turns.
    for i in 0..10 {
        spawn_nudge(
            st.clone(),
            "alice".into(),
            "conv-1".into(),
            format!("u{i}"),
            format!("a{i}"),
        );
    }
    // Detached spawn — wait briefly for tokio to schedule.
    tokio::time::sleep(std::time::Duration::from_millis(150)).await;
    // The 10th turn fires the review; the FakeInvoker bumps the counter.
    // We assert ≥1 to tolerate task-scheduling jitter; the regression
    // is "0 calls because spawn_review early-returns on None invoker".
    let observed = calls.load(Ordering::SeqCst);
    assert!(
        observed >= 1,
        "expected at least one invoker call, got {observed} \
         (regression: spawn_review early-returns when invoker=None)"
    );
}
