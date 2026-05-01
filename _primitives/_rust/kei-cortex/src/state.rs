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

use crate::agent::anthropic_memory_invoker::AnthropicMemoryInvoker;
use crate::agent::memory_nudge::{default_scheduler, MemoryNudgeScheduler};
use crate::agent::memory_review_task::Invoker;
use crate::config::AppConfig;
use dashmap::DashMap;
use kei_router::LlmRouter;
use kei_token_tracker::Store as TokenTracker;
use std::sync::Arc;
use tokio::sync::Mutex;

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
    per_user_locks: DashMap<String, Arc<Mutex<()>>>,
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
                per_user_locks: DashMap::new(),
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
    /// `Arc<Mutex<()>>` is cloned — the entry stays alive in the map so
    /// subsequent calls for the same `user_id` share it.
    pub fn user_lock(&self, user_id: &str) -> Arc<Mutex<()>> {
        self.inner
            .per_user_locks
            .entry(user_id.to_string())
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone()
    }
}

/// Default Anthropic-backed invoker factory. Each call rebuilds the
/// invoker so the API key is re-read fresh — same discipline as
/// `anthropic::open_stream` (no client caching). The system slot
/// uses the review prompt's persona stub; the actual review prompt
/// is appended by `run_review` as a trailing user message.
fn default_invoker_factory() -> InvokerFactory {
    Arc::new(|| Arc::new(AnthropicMemoryInvoker::new(default_review_system())) as Arc<dyn Invoker>)
}

/// System-slot text for memory-review calls. Kept short and stable
/// across reviews so the model response is dominated by the snapshot
/// + review prompt rather than persona drift.
fn default_review_system() -> String {
    "You are a quiet observer reviewing a chat to surface memory-worthy facts."
        .to_string()
}

/// Try to open the token-event store at the configured path. Returns
/// `None` when the parent directory does not exist or the open fails —
/// startup must NOT fail just because telemetry isn't ready (a fresh
/// host without `~/.keisei/` is normal). Errors are logged to stderr
/// once at startup so an operator notices, but the daemon keeps running.
fn open_token_tracker(
    path: &std::path::Path,
) -> Option<Arc<std::sync::Mutex<TokenTracker>>> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            eprintln!(
                "kei-cortex: token-tracker parent dir {:?} missing — skipping store open",
                parent
            );
            return None;
        }
    }
    match TokenTracker::open(path) {
        Ok(s) => Some(Arc::new(std::sync::Mutex::new(s))),
        Err(e) => {
            eprintln!(
                "kei-cortex: token-tracker open {} failed: {e} — telemetry disabled",
                path.display()
            );
            None
        }
    }
}

#[cfg(test)]
#[path = "state_test.rs"]
mod tests;
