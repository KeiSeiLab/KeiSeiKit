//! Background memory-review task.
//!
//! Constructor Pattern: this cube spawns a detached tokio task that
//! runs an ephemeral review agent. It does NOT own the agent surface
//! itself — it ports the Hermes `_spawn_background_review` shape
//! (run_agent.py:3267-3398) to a Rust async equivalent.
//!
//! Wiring contract (intentionally narrow):
//!   1. Caller hands us `ReviewHandles` (snapshot + memory-store Arc +
//!      invoker callable + optional persist target).
//!   2. We snapshot the conversation, append the review prompt, and
//!      hand the bundle to `invoker`.
//!   3. The invoker is responsible for the LLM round-trip. Reply text
//!      lands in `ReviewOutcome.raw_reply`. When a `PersistRequest`
//!      target is attached, a successful (non-short-circuit) reply is
//!      written to the disk-backed memory store via `memory_persist`.
//!
//! Tradeoffs:
//!   * `Arc<dyn Invoker>` instead of generics keeps this file <200 LOC
//!     and lets the scheduler treat all invokers uniformly. Cost: one
//!     virtual call per review. Reviews fire ~every 10 turns — cost
//!     is negligible vs the LLM round-trip dwarfing it.
//!   * Detached `tokio::spawn` means the task lives independently of
//!     the parent request. If the runtime shuts down mid-review, the
//!     write may be lost. That's acceptable — the next session's
//!     review will catch the same content. We deliberately do NOT
//!     join() the handle: blocking the parent on a memory write
//!     defeats the point of a background nudge.

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;

use kei_pet::memory::MemoryTag;

use super::memory_nudge::{AgentContext, Turn};
use super::memory_persist::{PersistOutcome, PersistRequest};
use super::memory_review_prompt::{is_nothing_to_save, REVIEW_PROMPT};

/// Result of a single review pass.
#[derive(Debug, Clone)]
pub struct ReviewOutcome {
    pub session_id: String,
    pub wrote_entries: usize,
    pub short_circuited: bool,
    pub raw_reply: String,
}

/// Trait for the actual LLM round-trip. Production wires
/// `kei-anthropic` here; tests provide an in-memory fake.
pub trait Invoker: Send + Sync + 'static {
    /// Hand the snapshot + appended review prompt to the model and
    /// receive the full reply. Must be cancel-safe — the scheduler
    /// may abort the task on shutdown.
    fn invoke(
        &self,
        snapshot: Vec<Turn>,
        prompt: String,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = String> + Send + '_>>;
}

/// Bundle handed to the spawned task. Cheap to construct — only
/// `Arc` clones, no deep copies.
pub struct ReviewHandles {
    pub session_id: String,
    pub turns: Arc<RwLock<Vec<Turn>>>,
    pub invoker: Option<Arc<dyn Invoker>>,
    /// Optional persistence target. When `Some`, a successful reply
    /// is written to the kei-pet memory store under
    /// `(persist.tag, role="memory_review")`. None disables writes
    /// (used by tests that drive the scheduler without a DB).
    pub persist: Option<PersistTarget>,
}

/// Subset of `PersistRequest` that's known at handle-build time —
/// the reply text itself is filled in after `invoke` returns.
#[derive(Debug, Clone)]
pub struct PersistTarget {
    pub db_path: std::path::PathBuf,
    pub tag: MemoryTag,
}

impl ReviewHandles {
    /// Build handles from a context. The invoker + persist target are
    /// set later by the caller (production wiring) — tests construct
    /// directly via the struct literal.
    pub fn from_context(ctx: &AgentContext) -> Self {
        let persist = ctx.persist.clone();
        Self {
            session_id: ctx.session_id.clone(),
            turns: ctx.turns.clone(),
            invoker: ctx.invoker.clone(),
            persist,
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

/// Spawn a detached review task. Returns immediately. Invoker absent
/// → the task logs and exits (used by smoke tests that exercise the
/// scheduler without an LLM).
pub fn spawn_review(handles: ReviewHandles) {
    tokio::spawn(async move {
        if handles.invoker.is_none() {
            return;
        }
        let _ = run_review(handles).await;
    });
}

/// Run one review pass synchronously (the body of the spawned task).
/// Exposed `pub` so smoke tests in the integration-test crate can
/// drive it without spawning a detached task.
pub async fn run_review(handles: ReviewHandles) -> ReviewOutcome {
    let snapshot = handles.turns.read().await.clone();
    let invoker = match handles.invoker.as_ref() {
        Some(i) => i.clone(),
        None => return absent_invoker_outcome(handles.session_id),
    };
    let reply = invoker.invoke(snapshot, REVIEW_PROMPT.to_string()).await;
    let short = is_nothing_to_save(&reply);
    let wrote = if short {
        0
    } else {
        persist_if_configured(&handles.persist, &reply).await
    };
    ReviewOutcome {
        session_id: handles.session_id,
        wrote_entries: wrote,
        short_circuited: short,
        raw_reply: reply,
    }
}

fn absent_invoker_outcome(session_id: String) -> ReviewOutcome {
    ReviewOutcome {
        session_id,
        wrote_entries: 0,
        short_circuited: true,
        raw_reply: String::new(),
    }
}

/// When a persist target is configured, fire a `spawn_blocking` write
/// and wait for its outcome. Returns the count of rows written
/// (0 on skip / failure).
async fn persist_if_configured(target: &Option<PersistTarget>, reply: &str) -> usize {
    let Some(t) = target else {
        return 0;
    };
    let req = PersistRequest {
        db_path: t.db_path.clone(),
        tag: t.tag.clone(),
        reply: reply.to_string(),
    };
    let out = match tokio::task::spawn_blocking(move || req.run()).await {
        Ok(o) => o,
        Err(e) => {
            debug!("memory_review persist join error: {e}");
            return 0;
        }
    };
    match out {
        PersistOutcome::Wrote(_) => 1,
        PersistOutcome::Failed(e) => {
            debug!("memory_review persist failed: {e}");
            0
        }
        _ => 0,
    }
}

#[cfg(test)]
#[path = "memory_review_task_test.rs"]
mod tests;
