//! Periodic memory-review scheduler.
//!
//! Constructor Pattern: this cube owns the *trigger* decision only.
//! The actual review work lives in `memory_review_task`. The split
//! exists so the scheduler is unit-testable without spinning up a
//! tokio runtime: `should_trigger()` is sync and pure.
//!
//! Frozen-snapshot invariant (CRITICAL): the in-flight system prompt
//! of the parent agent is NEVER mutated by background reviews. Reviews
//! write only to the disk-backed memory store via `PersistTarget`.
//! The next session loads the new snapshot at startup; the running
//! session keeps its prefix-cache hits intact. This mirrors Hermes'
//! `_system_prompt_snapshot` discipline (memory_tool.py:122).

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{Mutex, RwLock};

use super::memory_review_task::{spawn_review, Invoker, PersistTarget, ReviewHandles};

/// One conversation turn — minimal stub the scheduler needs to count.
/// The real type lives in the chat-handler module; we depend only on
/// the *count* here, so duck-type via a thin record.
#[derive(Debug, Clone)]
pub struct Turn {
    pub role: String,
    pub content: String,
}

/// Snapshot of context the scheduler hands to the review task. Held
/// behind `Arc` so the conversation isn't deep-copied each turn.
///
/// `invoker` and `persist` are `Option` so the caller can wire each
/// independently — production wires both, tests typically wire the
/// invoker only and verify the call without touching disk.
pub struct AgentContext {
    pub session_id: String,
    pub turns: Arc<RwLock<Vec<Turn>>>,
    pub invoker: Option<Arc<dyn Invoker>>,
    pub persist: Option<PersistTarget>,
}

impl AgentContext {
    /// Construct a fresh context. Use `with_invoker`/`with_persist` to
    /// chain optional wiring; tests with no wiring stop at `new`.
    pub fn new(session_id: String, turns: Arc<RwLock<Vec<Turn>>>) -> Self {
        Self {
            session_id,
            turns,
            invoker: None,
            persist: None,
        }
    }

    pub fn with_invoker(mut self, inv: Arc<dyn Invoker>) -> Self {
        self.invoker = Some(inv);
        self
    }

    pub fn with_persist(mut self, target: PersistTarget) -> Self {
        self.persist = Some(target);
        self
    }
}

/// Scheduler state. `interval` is the number of *user* turns between
/// review nudges (Hermes default: 10). `counter` is the running count
/// of user turns since the last review fire. `last_review` tracks
/// when the last review actually ran — used as a cool-down guard so a
/// pathological burst of single-token turns can't kick off a review
/// every 30s.
pub struct MemoryNudgeScheduler {
    interval: u32,
    counter: AtomicU32,
    last_review: Mutex<Option<Instant>>,
    cooldown_secs: u64,
}

impl MemoryNudgeScheduler {
    pub fn new(interval: u32) -> Self {
        Self {
            interval: interval.max(1),
            counter: AtomicU32::new(0),
            last_review: Mutex::new(None),
            cooldown_secs: 60,
        }
    }

    /// Test/diagnostic constructor that lets us shrink the cooldown
    /// to make multi-trigger paths exercisable without sleeping.
    pub fn with_cooldown_secs(interval: u32, cooldown_secs: u64) -> Self {
        Self {
            interval: interval.max(1),
            counter: AtomicU32::new(0),
            last_review: Mutex::new(None),
            cooldown_secs,
        }
    }

    /// Register a new user turn and possibly fire a review. Returns
    /// `true` when a review was spawned.
    pub async fn maybe_trigger(&self, ctx: &AgentContext) -> bool {
        let count = self.counter.fetch_add(1, Ordering::SeqCst) + 1;
        if !self.should_trigger_count(count) {
            return false;
        }
        if !self.cooldown_elapsed().await {
            return false;
        }
        self.counter.store(0, Ordering::SeqCst);
        *self.last_review.lock().await = Some(Instant::now());
        let handles = ReviewHandles::from_context(ctx);
        spawn_review(handles);
        true
    }

    /// Pure predicate — exposed for tests so they don't need a runtime.
    pub fn should_trigger_count(&self, count: u32) -> bool {
        count >= self.interval && (count % self.interval) == 0
    }

    async fn cooldown_elapsed(&self) -> bool {
        let guard = self.last_review.lock().await;
        match *guard {
            Some(last) => last.elapsed().as_secs() >= self.cooldown_secs,
            None => true,
        }
    }

    /// Reset the counter — called e.g. when the session is cleared.
    pub fn reset(&self) {
        self.counter.store(0, Ordering::SeqCst);
    }

    /// Read-only counter accessor for diagnostics.
    pub fn current_count(&self) -> u32 {
        self.counter.load(Ordering::SeqCst)
    }
}

/// Build a scheduler with the Hermes default (every 10 user turns).
pub fn default_scheduler() -> MemoryNudgeScheduler {
    MemoryNudgeScheduler::new(10)
}

#[cfg(test)]
#[path = "memory_nudge_test.rs"]
mod tests;
