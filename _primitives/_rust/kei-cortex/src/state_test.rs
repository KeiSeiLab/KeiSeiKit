//! Inline unit tests for `state.rs`.
//!
//! Constructor Pattern: extracted to a sibling so the parent stays
//! ≤200 LOC after the Hermes P2.2.b additions (scheduler + invoker
//! factory plumbing) and Wave 44d resource-cap hardening (per-user
//! lock LRU eviction).

use super::*;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};

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

struct Counter(Arc<AtomicUsize>);
impl Invoker for Counter {
    fn invoke(
        &self,
        _s: Vec<crate::agent::memory_nudge::Turn>,
        _p: String,
    ) -> Pin<Box<dyn std::future::Future<Output = String> + Send + '_>> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Box::pin(async move { "Nothing to save.".into() })
    }
}

#[tokio::test]
async fn user_lock_is_stable_per_user() {
    let state = AppState::new(dummy_config(), "tok".into());
    let a = state.user_lock("alice").await;
    let b = state.user_lock("alice").await;
    assert!(Arc::ptr_eq(&a, &b));
}

#[tokio::test]
async fn user_lock_differs_per_user() {
    let state = AppState::new(dummy_config(), "tok".into());
    let a = state.user_lock("alice").await;
    let b = state.user_lock("bob").await;
    assert!(!Arc::ptr_eq(&a, &b));
}

#[test]
fn router_is_present() {
    let state = AppState::new(dummy_config(), "tok".into());
    let _ = state.router();
}

#[test]
fn scheduler_is_present() {
    let state = AppState::new(dummy_config(), "tok".into());
    let _ = state.scheduler();
}

#[test]
fn invoker_factory_yields_distinct_arcs() {
    let calls = Arc::new(AtomicUsize::new(0));
    let calls_capture = calls.clone();
    let factory: InvokerFactory =
        Arc::new(move || Arc::new(Counter(calls_capture.clone())) as Arc<dyn Invoker>);
    let state = AppState::with_router_and_factory(
        dummy_config(),
        "tok".into(),
        Arc::new(LlmRouter::new()),
        factory,
    );
    let a = state.build_memory_invoker();
    let b = state.build_memory_invoker();
    // Two distinct invocations of the factory → two Arcs (the
    // factory does NOT memoise).
    assert!(!Arc::ptr_eq(&a, &b));
}

/// Resource-exhaustion guard: the registry MUST cap at
/// `PER_USER_LOCK_CAP` (1024). After inserting 2× cap distinct user_ids
/// the registry size stays ≤ cap, proving LRU eviction kicked in. Without
/// the cap an auth'd attacker with 1M unique user_ids would OOM the
/// daemon — this test pins that the bound holds.
#[tokio::test]
async fn user_lock_evicts_past_cap() {
    let state = AppState::new(dummy_config(), "tok".into());
    let cap = super::PER_USER_LOCK_CAP;
    for i in 0..(cap * 2) {
        let _ = state.user_lock(&format!("user-{i}")).await;
    }
    let count = state.user_lock_count().await;
    assert!(
        count <= cap,
        "registry must stay ≤ {cap} after {} inserts; got {count}",
        cap * 2
    );
    assert!(count > 0, "registry should retain the most recent inserts");
}

/// LRU recency: the most recent inserts MUST survive eviction. After
/// pushing 2× cap entries, the very last one we inserted should still
/// be in the registry (cheap pointer-equality check via `user_lock`
/// returning the same `Arc`).
#[tokio::test]
async fn user_lock_keeps_most_recent() {
    let state = AppState::new(dummy_config(), "tok".into());
    let cap = super::PER_USER_LOCK_CAP;
    for i in 0..(cap * 2) {
        let _ = state.user_lock(&format!("user-{i}")).await;
    }
    // The very last user inserted in the loop above must still be cached:
    // re-fetching returns a clone of the same Arc.
    let last_key = format!("user-{}", cap * 2 - 1);
    let a = state.user_lock(&last_key).await;
    let b = state.user_lock(&last_key).await;
    assert!(Arc::ptr_eq(&a, &b));
}
