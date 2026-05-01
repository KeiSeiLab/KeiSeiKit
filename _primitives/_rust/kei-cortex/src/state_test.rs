//! Inline unit tests for `state.rs`.
//!
//! Constructor Pattern: extracted to a sibling so the parent stays
//! ≤200 LOC after the Hermes P2.2.b additions (scheduler + invoker
//! factory plumbing).

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
