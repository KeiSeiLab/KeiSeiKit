//! Side-channel state captured by `chat_stream::run_loop_stream` for
//! the post-`Done` callbacks.
//!
//! Constructor Pattern: extracted into a sibling cube so
//! `chat_stream.rs` can stay under the 200-LOC ceiling now that the
//! Hermes P2.2.b memory-nudge wiring is in place. Two structs live
//! here: `ChatCostCtx` (cost-recording row) and `MemoryNudgeCtx`
//! (memory-review trigger). Both are `Clone` so the `stream!` macro
//! can move them into the captured-state set without lifetime gymnastics.

use crate::state::AppState;
use crate::tool::loop_driver::TokenUsage;
use kei_token_tracker::Store as TokenTracker;
use std::sync::{Arc, Mutex};

/// Captures all post-loop side-channel state for cost recording.
/// Owned by `run_loop_stream` and threaded through `build_event_stream`.
#[derive(Clone)]
pub(super) struct ChatCostCtx {
    pub(super) accum: Arc<Mutex<TokenUsage>>,
    pub(super) ledger_path: std::path::PathBuf,
    pub(super) agent_id: String,
    pub(super) provider: String,
    pub(super) model: String,
    pub(super) rates: (u32, u32),
    /// Conversation id passed by the client (raw, not the chat-prefixed
    /// agent_id). Stored separately so the token-event row keeps the
    /// caller's original id while the ledger row keeps the
    /// `chat-<conv>` form for cost addressability.
    pub(super) conversation_id: Option<String>,
    /// Token-event tracker handle. `None` when the configured DB path
    /// failed to open at startup; helpers no-op on absence.
    pub(super) token_tracker: Option<Arc<std::sync::Mutex<TokenTracker>>>,
}

/// Side-channel state for the post-`Done` memory-nudge call. Owned by
/// `run_loop_stream` so `build_event_stream` doesn't need to know
/// about `AppState` directly. Mirrors `ChatCostCtx` shape for symmetry
/// — both fire after the `Done` event.
#[derive(Clone)]
pub(super) struct MemoryNudgeCtx {
    pub(super) state: AppState,
    pub(super) user_id: String,
    pub(super) conversation_id: String,
    pub(super) user_msg: String,
}
