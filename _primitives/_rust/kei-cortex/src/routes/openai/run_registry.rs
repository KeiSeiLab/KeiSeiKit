//! `RunRegistry` + `RunSlot` — process-wide store of in-flight `/v1/runs`.
//!
//! Lifted out of `runs.rs` so that file stays under the 200-LOC
//! Constructor-Pattern ceiling. The handlers in `runs.rs` mutate this
//! registry; the registry knows nothing about HTTP.
//!
//! P1.1.d (2026-04-28): cancel migrated from `Arc<Notify>` to
//! `tokio_util::sync::CancellationToken`. Reason: `agent_runner::
//! stream_events` already takes a `CancellationToken`, so a direct
//! field avoids the Notify→Token bridge spawn. `cancel()` calls
//! `token.cancel()` which is fire-once, observable via
//! `is_cancelled()` and `cancelled().await` — same semantics the
//! agent loop wants.

use super::sse::AgentChunk;
use dashmap::DashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// Per-process registry of in-flight runs.
#[derive(Clone, Default)]
pub struct RunRegistry {
    inner: Arc<DashMap<String, RunSlot>>,
}

/// One in-flight run. `rx` is held inside an `Arc<Mutex<Option<>>>`
/// so the events handler can `take()` it on first subscribe; `cancel`
/// is signalled by `/stop` so the agent loop short-circuits at its
/// next checkpoint.
#[derive(Clone)]
pub struct RunSlot {
    pub model: String,
    pub created_at: u64,
    pub status: String,
    pub rx: Arc<Mutex<Option<mpsc::Receiver<AgentChunk>>>>,
    pub cancel: CancellationToken,
}

impl RunRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&self, id: impl Into<String>, slot: RunSlot) {
        self.inner.insert(id.into(), slot);
    }

    /// Take the receiver out of the slot — first subscriber wins.
    /// Subsequent calls return `None` and the handler should 404.
    pub fn take_receiver(&self, id: &str) -> Option<mpsc::Receiver<AgentChunk>> {
        let slot = self.inner.get(id)?;
        let mut guard = slot.rx.lock().ok()?;
        guard.take()
    }

    pub fn cancel(&self, id: &str) -> bool {
        if let Some(slot) = self.inner.get(id) {
            slot.cancel.cancel();
            true
        } else {
            false
        }
    }

    pub fn mark(&self, id: &str, status: impl Into<String>) {
        if let Some(mut e) = self.inner.get_mut(id) {
            e.status = status.into();
        }
    }

    pub fn get(&self, id: &str) -> Option<RunSlot> {
        self.inner.get(id).map(|e| e.value().clone())
    }
}

/// Process-singleton accessor. Same pattern as `session::global()` —
/// keeps state coherent across all `/v1/runs/*` handlers regardless of
/// which router instance is serving the request.
pub fn global() -> RunRegistry {
    use once_cell::sync::Lazy;
    static REG: Lazy<RunRegistry> = Lazy::new(RunRegistry::new);
    REG.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slot() -> RunSlot {
        RunSlot {
            model: "kei-cortex".into(),
            created_at: 0,
            status: "queued".into(),
            rx: Arc::new(Mutex::new(None)),
            cancel: CancellationToken::new(),
        }
    }

    #[test]
    fn insert_then_get() {
        let r = RunRegistry::new();
        r.insert("r1", slot());
        assert!(r.get("r1").is_some());
    }

    #[test]
    fn cancel_unknown_returns_false() {
        let r = RunRegistry::new();
        assert!(!r.cancel("nope"));
    }

    #[test]
    fn mark_updates_status() {
        let r = RunRegistry::new();
        r.insert("r2", slot());
        r.mark("r2", "in_progress");
        assert_eq!(r.get("r2").unwrap().status, "in_progress");
    }

    #[test]
    fn cancel_known_fires_token() {
        let r = RunRegistry::new();
        r.insert("r3", slot());
        let slot = r.get("r3").unwrap();
        assert!(!slot.cancel.is_cancelled());
        assert!(r.cancel("r3"));
        let after = r.get("r3").unwrap();
        assert!(after.cancel.is_cancelled());
    }
}
