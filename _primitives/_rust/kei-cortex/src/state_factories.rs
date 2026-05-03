//! Construction helpers extracted from `state.rs` to keep that file under
//! the Constructor-Pattern 200-LOC ceiling after the Wave 44d resource-cap
//! work added an LRU registry for per-user locks.
//!
//! Hosts:
//!   - `default_invoker_factory` — builds the Anthropic memory-review
//!     invoker fresh on every call so env-rotated API keys take effect.
//!   - `default_review_system` — short, stable system slot for review.
//!   - `open_token_tracker` — opens the token-event SQLite store and
//!     gracefully degrades to `None` when the host has no telemetry dir.

use crate::agent::anthropic_memory_invoker::AnthropicMemoryInvoker;
use crate::agent::memory_review_task::Invoker;
use crate::state::InvokerFactory;
use kei_token_tracker::Store as TokenTracker;
use std::sync::Arc;

/// Default Anthropic-backed invoker factory. Each call rebuilds the
/// invoker so the API key is re-read fresh — same discipline as
/// `anthropic::open_stream` (no client caching). The system slot
/// uses the review prompt's persona stub; the actual review prompt
/// is appended by `run_review` as a trailing user message.
pub(crate) fn default_invoker_factory() -> InvokerFactory {
    Arc::new(|| {
        Arc::new(AnthropicMemoryInvoker::new(default_review_system())) as Arc<dyn Invoker>
    })
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
pub(crate) fn open_token_tracker(
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
