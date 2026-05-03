//! Shared state passed to every handler via `axum::extract::State`.
//!
//! Holds the loaded configuration, the bearer token, the LLM provider router
//! (Wave 32 v0.40 multi-provider abstraction), and a per-user lock registry
//! used by expensive side-effecting handlers (portrait install) to serialize
//! work against the same `user_id`.
//!
//! Hermes P2.2.b: the memory-review scheduler + an invoker factory live
//! here so the chat handler can fire `maybe_trigger` after each turn
//! without re-discovering wiring. The factory closure rebuilds the
//! Anthropic invoker per call so the API key is read fresh (env-rotation
//! friendly, mirrors `anthropic::open_stream`).

use crate::agent::memory_nudge::{default_scheduler, MemoryNudgeScheduler};
use crate::agent::memory_review_task::Invoker;
use crate::config::AppConfig;
use crate::state_factories::{default_invoker_factory, open_token_tracker};
use kei_router::LlmRouter;
use kei_token_tracker::Store as TokenTracker;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Hard cap on how many distinct `user_id` mutexes we keep alive. Anything
/// past this is LRU-evicted. Memory bound: cap × (~80 bytes per entry +
/// `Arc<Mutex<()>>`). At 1024 the registry stays well under 200 KiB even
/// if every slot is hot. The eviction is safe because callers hold their
/// own `Arc<Mutex<()>>` clone for the duration of the critical section —
/// dropping the cache entry only retires the *registry's* reference.
const PER_USER_LOCK_CAP: usize = 1024;

/// Type alias for the per-call invoker factory. Each `()` invocation
/// returns a freshly-built invoker so env mutations between turns
/// (`ANTHROPIC_API_KEY` rotation) are picked up automatically.
pub type InvokerFactory = Arc<dyn Fn() -> Arc<dyn Invoker> + Send + Sync>;

/// Read-only handler state (cheaply cloneable via `Arc`).
#[derive(Clone)]
pub struct AppState {
    inner: Arc<Inner>,
}

struct Inner {
    config: AppConfig,
    token: String,
    router: Arc<LlmRouter>,
    /// Bounded LRU registry of per-user mutexes. Capped at
    /// `PER_USER_LOCK_CAP` to prevent unbounded growth from auth'd
    /// attackers who present a fresh `user_id` on every call. Wrapped
    /// in `Mutex` because `LruCache::get` mutates the recency list and
    /// is `!Sync` by design.
    per_user_locks: Mutex<LruCache<String, Arc<Mutex<()>>>>,
    scheduler: Arc<MemoryNudgeScheduler>,
    invoker_factory: InvokerFactory,
    /// Token-event store. `None` when the configured path could not be
    /// opened at startup (parent dir missing on a fresh host, etc) —
    /// callers treat the absence as a no-op and never panic. The store
    /// holds an owned [`Connection`]; we wrap it in `Mutex` because
    /// rusqlite's `Connection` is not `Sync`. Each `record_event` call
    /// is fast (single INSERT) so contention is negligible.
    token_tracker: Option<Arc<std::sync::Mutex<TokenTracker>>>,
}

impl AppState {
    /// Construct new state from a validated config and bearer token.
    /// The LLM router is built from environment (`ANTHROPIC_API_KEY` etc.)
    /// at construction time — providers without keys are silently skipped.
    pub fn new(config: AppConfig, token: String) -> Self {
        let router = Arc::new(LlmRouter::from_env());
        Self::with_router(config, token, router)
    }

    /// Test-friendly constructor that takes an explicit router (e.g.
    /// pre-registered fakes). Production code path goes through `new`.
    /// Wires the default Anthropic memory invoker factory.
    pub fn with_router(config: AppConfig, token: String, router: Arc<LlmRouter>) -> Self {
        let factory = default_invoker_factory();
        Self::with_router_and_factory(config, token, router, factory)
    }

    /// Full-control constructor: caller passes router AND invoker
    /// factory. Tests use this to inject a `FakeInvoker` so the
    /// scheduler's `maybe_trigger` can be exercised end-to-end without
    /// touching Anthropic.
    pub fn with_router_and_factory(
        config: AppConfig,
        token: String,
        router: Arc<LlmRouter>,
        invoker_factory: InvokerFactory,
    ) -> Self {
        let token_tracker = open_token_tracker(&config.token_tracker_db);
        Self::with_router_factory_and_tracker(
            config, token, router, invoker_factory, token_tracker,
        )
    }

    /// Test-only escape hatch: caller supplies the token-tracker handle
    /// directly (e.g. an in-memory store). Lets the integration test
    /// drive an end-to-end chat turn against a counted-event tracker
    /// without writing a real SQLite file.
    pub fn with_router_factory_and_tracker(
        config: AppConfig,
        token: String,
        router: Arc<LlmRouter>,
        invoker_factory: InvokerFactory,
        token_tracker: Option<Arc<std::sync::Mutex<TokenTracker>>>,
    ) -> Self {
        Self {
            inner: Arc::new(Inner {
                config,
                token,
                router,
                per_user_locks: Mutex::new(LruCache::new(
                    NonZeroUsize::new(PER_USER_LOCK_CAP)
                        .expect("PER_USER_LOCK_CAP > 0"),
                )),
                scheduler: Arc::new(default_scheduler()),
                invoker_factory,
                token_tracker,
            }),
        }
    }

    /// Borrow the configuration.
    pub fn config(&self) -> &AppConfig {
        &self.inner.config
    }

    /// Borrow the bearer token.
    pub fn token(&self) -> &str {
        &self.inner.token
    }

    /// Borrow the LLM provider router (cheap clone via `Arc`).
    pub fn router(&self) -> Arc<LlmRouter> {
        self.inner.router.clone()
    }

    /// Borrow the memory-review scheduler. Cheap clone via `Arc`.
    pub fn scheduler(&self) -> Arc<MemoryNudgeScheduler> {
        self.inner.scheduler.clone()
    }

    /// Build a fresh memory-review invoker via the configured factory.
    pub fn build_memory_invoker(&self) -> Arc<dyn Invoker> {
        (self.inner.invoker_factory)()
    }

    /// Borrow the token-event tracker. `None` when the configured DB
    /// path could not be opened at startup — handlers must treat the
    /// absence as fire-and-forget no-op (token recording is observability,
    /// not critical path).
    pub fn token_tracker(&self) -> Option<Arc<std::sync::Mutex<TokenTracker>>> {
        self.inner.token_tracker.clone()
    }

    /// Return the per-user mutex, creating it on first access. The returned
    /// `Arc<Mutex<()>>` is cloned — when the LRU has spare capacity the
    /// entry stays in the registry so the next call for the same `user_id`
    /// shares it. If the registry has evicted the slot under load, a new
    /// mutex is created (acceptable: the prior critical section is already
    /// fenced by the caller's own `Arc` clone).
    pub async fn user_lock(&self, user_id: &str) -> Arc<Mutex<()>> {
        let mut cache = self.inner.per_user_locks.lock().await;
        if let Some(existing) = cache.get(user_id) {
            return existing.clone();
        }
        let new_lock = Arc::new(Mutex::new(()));
        cache.put(user_id.to_string(), new_lock.clone());
        new_lock
    }

    /// Test-only accessor: current size of the per-user-lock registry.
    /// Exposed for the eviction integration test (`state_test.rs`).
    #[cfg(test)]
    pub async fn user_lock_count(&self) -> usize {
        self.inner.per_user_locks.lock().await.len()
    }
}

#[cfg(test)]
#[path = "state_test.rs"]
mod tests;
