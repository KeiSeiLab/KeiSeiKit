//! Per-session run guard (port of Hermes asyncio.Event pattern).
//!
//! Two messages arriving for the same `session_key` while an agent is mid-run
//! must serialise — Hermes uses an `asyncio.Event` per session; we use a
//! `tokio::sync::Notify` keyed on the session_key, with stale-lock heal.

use std::sync::Arc;
use std::time::{Duration, Instant};

use dashmap::DashMap;
use tokio::sync::{Notify, OwnedSemaphorePermit, Semaphore};

const STALE_LOCK_TIMEOUT: Duration = Duration::from_secs(30);

/// Internal record for one session's lock.
struct LockEntry {
    sem: Arc<Semaphore>,
    notify: Arc<Notify>,
    /// When the active permit was issued, for stale detection.
    acquired_at: Instant,
}

impl LockEntry {
    fn fresh() -> Self {
        Self {
            sem: Arc::new(Semaphore::new(1)),
            notify: Arc::new(Notify::new()),
            acquired_at: Instant::now(),
        }
    }
}

/// Tracks one in-flight agent run per session_key.
#[derive(Clone, Default)]
pub struct SessionGuard {
    active: Arc<DashMap<String, LockEntry>>,
}

impl SessionGuard {
    pub fn new() -> Self {
        Self {
            active: Arc::new(DashMap::new()),
        }
    }

    /// Acquire the lock for `session_key`. Blocks until any concurrent run on
    /// the same key completes — or 30s pass and we declare the prior holder
    /// dead and steal it.
    pub async fn acquire(&self, session_key: &str) -> SessionLock {
        // Heal stale lock first (cheap peek under DashMap shard lock).
        self.heal_stale(session_key);

        let entry_sem = self
            .active
            .entry(session_key.to_string())
            .or_insert_with(LockEntry::fresh)
            .sem
            .clone();

        // owned permit so the SessionLock can keep it across awaits
        let permit = entry_sem
            .acquire_owned()
            .await
            .expect("session semaphore never closed");

        // bump acquired_at on successful acquisition
        if let Some(mut e) = self.active.get_mut(session_key) {
            e.acquired_at = Instant::now();
        }

        SessionLock {
            _permit: permit,
            session_key: session_key.to_string(),
            map: self.active.clone(),
        }
    }

    fn heal_stale(&self, session_key: &str) {
        if let Some(entry) = self.active.get(session_key) {
            if entry.acquired_at.elapsed() > STALE_LOCK_TIMEOUT
                && entry.sem.available_permits() == 0
            {
                // Drop and recreate; the previous holder (likely panicked) loses.
                drop(entry);
                self.active
                    .insert(session_key.to_string(), LockEntry::fresh());
            }
        }
    }

    /// True if any session is currently held. Test helper.
    pub fn is_held(&self, session_key: &str) -> bool {
        self.active
            .get(session_key)
            .map(|e| e.sem.available_permits() == 0)
            .unwrap_or(false)
    }
}

/// RAII handle. Dropping releases the permit and notifies waiters.
pub struct SessionLock {
    _permit: OwnedSemaphorePermit,
    session_key: String,
    map: Arc<DashMap<String, LockEntry>>,
}

impl Drop for SessionLock {
    fn drop(&mut self) {
        if let Some(entry) = self.map.get(&self.session_key) {
            entry.notify.notify_waiters();
        }
    }
}
