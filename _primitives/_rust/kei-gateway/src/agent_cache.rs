//! LRU-cached AIAgent handles, keyed by session.
//!
//! Hermes pattern: each session_key owns a long-lived agent process. The cache
//! is bounded (memory pressure) and TTL-aware (idle eviction).

use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::{Duration, Instant};

use lru::LruCache;
use tokio::sync::Mutex;

/// Cached agent record. The `agent_handle` is intentionally type-erased — the
/// gateway crate doesn't depend on the agent crate. Consumers parameterise via
/// trait objects or downcast through `Any`.
pub struct CachedAgent {
    /// Opaque handle the gateway forwards to the runner. The actual type is
    /// supplied by the caller when constructing the cache (e.g. an
    /// `Arc<dyn AgentLike>` or a channel sender).
    pub agent_handle: Arc<dyn std::any::Any + Send + Sync>,
    /// Hash / fingerprint of the (model, system prompt, toolset) tuple. Used
    /// to invalidate stale entries after `/reload`.
    pub config_signature: String,
    pub created_at: Instant,
    pub last_used: Instant,
}

impl CachedAgent {
    pub fn new(handle: Arc<dyn std::any::Any + Send + Sync>, signature: String) -> Self {
        let now = Instant::now();
        Self {
            agent_handle: handle,
            config_signature: signature,
            created_at: now,
            last_used: now,
        }
    }

    pub fn touch(&mut self) {
        self.last_used = Instant::now();
    }

    pub fn is_idle(&self, ttl: Duration) -> bool {
        self.last_used.elapsed() > ttl
    }
}

/// Bounded LRU agent cache with idle TTL.
#[derive(Clone)]
pub struct AgentCache {
    inner: Arc<Mutex<LruCache<String, CachedAgent>>>,
    ttl: Duration,
}

impl AgentCache {
    pub fn new(capacity: usize, ttl: Duration) -> Self {
        let cap = NonZeroUsize::new(capacity.max(1)).unwrap();
        Self {
            inner: Arc::new(Mutex::new(LruCache::new(cap))),
            ttl,
        }
    }

    /// Insert or replace an agent for `session_key`.
    pub async fn put(&self, session_key: &str, agent: CachedAgent) {
        self.inner.lock().await.put(session_key.to_string(), agent);
    }

    /// Fetch a fresh-enough agent. Returns `None` if missing OR stale.
    pub async fn get(&self, session_key: &str) -> Option<Arc<dyn std::any::Any + Send + Sync>> {
        let mut guard = self.inner.lock().await;
        let entry = guard.get_mut(session_key)?;
        if entry.is_idle(self.ttl) {
            guard.pop(session_key);
            return None;
        }
        entry.touch();
        Some(entry.agent_handle.clone())
    }

    /// Compare a stored agent's `config_signature` against `expected`. If they
    /// differ (e.g. config changed), evict and return false.
    pub async fn check_signature(&self, session_key: &str, expected: &str) -> bool {
        let mut guard = self.inner.lock().await;
        match guard.peek(session_key) {
            Some(c) if c.config_signature == expected => true,
            Some(_) => {
                guard.pop(session_key);
                false
            }
            None => false,
        }
    }

    /// Drop every entry whose `last_used` exceeds `ttl`. Returns count purged.
    pub async fn evict_idle(&self) -> usize {
        let mut guard = self.inner.lock().await;
        let stale: Vec<String> = guard
            .iter()
            .filter(|(_, v)| v.is_idle(self.ttl))
            .map(|(k, _)| k.clone())
            .collect();
        for key in &stale {
            guard.pop(key);
        }
        stale.len()
    }

    pub async fn len(&self) -> usize {
        self.inner.lock().await.len()
    }
}
