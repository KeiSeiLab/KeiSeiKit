//! Inline unit tests for `memory_nudge.rs`.
//!
//! Constructor Pattern: extracted to a sibling so the parent stays
//! ≤200 LOC after the AgentContext extension (invoker + persist).

use super::*;
use std::sync::atomic::AtomicUsize;

struct FakeInvoker {
    calls: Arc<AtomicUsize>,
}

impl Invoker for FakeInvoker {
    fn invoke(
        &self,
        _snapshot: Vec<Turn>,
        _prompt: String,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = String> + Send + '_>> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Box::pin(async move { "Nothing to save.".to_string() })
    }
}

#[test]
fn fires_at_threshold() {
    let s = MemoryNudgeScheduler::new(10);
    for i in 1..10 {
        assert!(!s.should_trigger_count(i), "count {i} should not fire");
    }
    assert!(s.should_trigger_count(10), "count 10 should fire");
    assert!(s.should_trigger_count(20), "count 20 should fire");
    assert!(!s.should_trigger_count(11), "count 11 should not fire");
}

#[test]
fn interval_one_fires_every_turn() {
    let s = MemoryNudgeScheduler::new(1);
    assert!(s.should_trigger_count(1));
    assert!(s.should_trigger_count(2));
}

#[test]
fn zero_interval_clamps_to_one() {
    let s = MemoryNudgeScheduler::new(0);
    assert!(s.should_trigger_count(1));
}

#[test]
fn reset_zeros_counter() {
    let s = MemoryNudgeScheduler::new(10);
    s.counter.store(7, Ordering::SeqCst);
    s.reset();
    assert_eq!(s.current_count(), 0);
}

#[tokio::test]
async fn maybe_trigger_fires_only_at_interval() {
    let s = MemoryNudgeScheduler::with_cooldown_secs(3, 0);
    let calls = Arc::new(AtomicUsize::new(0));
    let inv: Arc<dyn Invoker> = Arc::new(FakeInvoker {
        calls: calls.clone(),
    });
    let ctx = AgentContext::new("s".into(), Arc::new(RwLock::new(vec![])))
        .with_invoker(inv);
    assert!(!s.maybe_trigger(&ctx).await);
    assert!(!s.maybe_trigger(&ctx).await);
    let fired = s.maybe_trigger(&ctx).await;
    assert!(fired, "third call (count==interval) must fire");
    // Detached spawn — give it a moment to run the fake.
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn agent_context_carries_invoker_through_handles() {
    // Smoke regression for the dead-code state described in the
    // hermes-batch-2026-04-28 STATUS BANNER: from_context used to
    // return invoker=None unconditionally. Now the invoker plumbed
    // into AgentContext flows through ReviewHandles correctly.
    let calls = Arc::new(AtomicUsize::new(0));
    let inv: Arc<dyn Invoker> = Arc::new(FakeInvoker {
        calls: calls.clone(),
    });
    let ctx = AgentContext::new("s".into(), Arc::new(RwLock::new(vec![])))
        .with_invoker(inv);
    let h = crate::agent::memory_review_task::ReviewHandles::from_context(&ctx);
    assert!(
        h.invoker.is_some(),
        "from_context must propagate invoker (dead-code regression)"
    );
}
