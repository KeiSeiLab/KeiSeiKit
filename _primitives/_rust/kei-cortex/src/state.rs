//! Shared state passed to every handler via `axum::extract::State`.
//!
//! Holds the loaded configuration, the bearer token, the LLM provider router
//! (Wave 32 v0.40 multi-provider abstraction), and a per-user lock registry
//! used by expensive side-effecting handlers (portrait install) to serialize
//! work against the same `user_id`.

use crate::config::AppConfig;
use dashmap::DashMap;
use kei_router::LlmRouter;
use std::sync::Arc;
use tokio::sync::Mutex;

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
    pub fn with_router(config: AppConfig, token: String, router: Arc<LlmRouter>) -> Self {
        Self {
            inner: Arc::new(Inner {
                config,
                token,
                router,
                per_user_locks: DashMap::new(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn dummy_config() -> AppConfig {
        AppConfig::new(
            Some(9999),
            Some("https://example.test".into()),
            Some(PathBuf::from("/tmp/kc-tok")),
            Some(PathBuf::from("/tmp/kc-led")),
            Some(PathBuf::from("/tmp/kc-pets")),
            Some(PathBuf::from("/tmp/kc-mem.sqlite")),
            Some(PathBuf::from("/tmp/kc-live2d")),
        )
    }

    #[test]
    fn user_lock_is_stable_per_user() {
        let state = AppState::new(dummy_config(), "tok".into());
        let a = state.user_lock("alice");
        let b = state.user_lock("alice");
        assert!(Arc::ptr_eq(&a, &b));
    }

    #[test]
    fn user_lock_differs_per_user() {
        let state = AppState::new(dummy_config(), "tok".into());
        let a = state.user_lock("alice");
        let b = state.user_lock("bob");
        assert!(!Arc::ptr_eq(&a, &b));
    }

    #[test]
    fn router_is_present() {
        let state = AppState::new(dummy_config(), "tok".into());
        // Empty env yields zero providers, but the router itself exists.
        let _ = state.router();
    }
}
