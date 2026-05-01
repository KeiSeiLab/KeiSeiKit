//! Smoke test for the periodic memory-nudge scheduler.
//!
//! Constructor Pattern: one file = one scenario per test fn.
//! Drives the scheduler through 12 simulated turns, asserts:
//!   * trigger fires at turn 10 (Hermes default interval),
//!   * counter resets after fire,
//!   * "Nothing to save." short-circuit is recognised by the
//!     review-task path (no memory writes spawned for it).

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

use kei_cortex::agent::memory_nudge::{AgentContext, MemoryNudgeScheduler, Turn};
use kei_cortex::agent::memory_review_prompt::is_nothing_to_save;
use kei_cortex::agent::memory_review_task::{
    run_review, Invoker, ReviewHandles,
};

struct CountingInvoker {
    reply: String,
    calls: Arc<AtomicUsize>,
}

impl Invoker for CountingInvoker {
    fn invoke(
        &self,
        _snapshot: Vec<Turn>,
        _prompt: String,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = String> + Send + '_>>
    {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let r = self.reply.clone();
        Box::pin(async move { r })
    }
}

fn fake_ctx(session_id: &str) -> AgentContext {
    let turns: Vec<Turn> = (0..12)
        .map(|i| Turn {
            role: if i % 2 == 0 { "user".into() } else { "assistant".into() },
            content: format!("turn {i}"),
        })
        .collect();
    AgentContext::new(session_id.to_string(), Arc::new(RwLock::new(turns)))
}

#[test]
fn pure_predicate_fires_at_turn_10() {
    let s = MemoryNudgeScheduler::new(10);
    let mut fires_at = vec![];
    for i in 1..=12u32 {
        if s.should_trigger_count(i) {
            fires_at.push(i);
        }
    }
    assert_eq!(fires_at, vec![10]);
}

#[tokio::test]
async fn maybe_trigger_resets_counter_after_fire() {
    let s = MemoryNudgeScheduler::new(10);
    let ctx = fake_ctx("smoke-1");
    let mut fired = 0;
    for _ in 0..12 {
        if s.maybe_trigger(&ctx).await {
            fired += 1;
        }
    }
    // Cooldown is 60s — second trigger at turn 20 will be suppressed
    // by the cooldown guard. Counter resets each fire either way.
    assert_eq!(fired, 1);
    // After the single fire, two more turns leave counter at 2.
    assert_eq!(s.current_count(), 2);
}

#[tokio::test]
async fn nothing_to_save_short_circuits() {
    let calls = Arc::new(AtomicUsize::new(0));
    let invoker = Arc::new(CountingInvoker {
        reply: "Nothing to save.".to_string(),
        calls: calls.clone(),
    });
    let ctx = fake_ctx("smoke-2");
    let handles = ReviewHandles::from_context(&ctx).with_invoker(invoker);
    let outcome = run_review(handles).await;
    assert!(outcome.short_circuited, "should recognise short-circuit");
    assert_eq!(outcome.wrote_entries, 0);
    assert!(is_nothing_to_save(&outcome.raw_reply));
    assert_eq!(calls.load(Ordering::SeqCst), 1, "invoker called exactly once");
}

#[tokio::test]
async fn substantive_reply_does_not_short_circuit() {
    let invoker = Arc::new(CountingInvoker {
        reply: "User prefers Rust; saved to memory.".to_string(),
        calls: Arc::new(AtomicUsize::new(0)),
    });
    let ctx = fake_ctx("smoke-3");
    let handles = ReviewHandles::from_context(&ctx).with_invoker(invoker);
    let outcome = run_review(handles).await;
    assert!(!outcome.short_circuited);
    assert!(outcome.raw_reply.contains("Rust"));
}
