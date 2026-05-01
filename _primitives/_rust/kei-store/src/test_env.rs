//! Shared `ENV_LOCK` for kei-store tests that mutate process-wide env vars.
//!
//! Constructor Pattern: single responsibility — one global `Mutex<()>` that
//! every test serialising on `KEI_STORE_*` and related env variables takes
//! before `set_var` / `remove_var`. Prevents the cargo-test default parallel
//! runner from racing multiple tests on the same env state.
//!
//! Exposed under `#[cfg(any(test, feature = "s3"))]` so:
//!   - in-crate unit tests (`github.rs`, `s3_cloud/*`) can use it
//!   - the out-of-crate smoke test (`tests/s3_smoke.rs`) can import it via
//!     the `s3` feature gate (same gate the smoke test already sits behind)
//!
//! NOT exposed in normal release builds — this is a test-only hygiene shim.

use std::sync::{Mutex, MutexGuard};

pub static ENV_LOCK: Mutex<()> = Mutex::new(());

/// Take the lock, recovering from a poisoned guard (another test panicked
/// while holding it). Poisoning is fine for the env-var use case — the
/// guarded data is `()`.
pub fn env_lock() -> MutexGuard<'static, ()> {
    ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner())
}
