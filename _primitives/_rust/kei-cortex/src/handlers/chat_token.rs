//! Token-event recording side-channel for the chat handlers.
//!
//! Constructor Pattern split (cleanup follow-up to Phase 2 wiring): the
//! `kei-token-tracker` write path was originally inlined in
//! `chat_cost.rs`, pushing that file to 263 LOC. This cube extracts the
//! token-event concern (TokenWrite bundle + record_token_event +
//! spawn_record_token_event + build_token_event helper) so each file
//! stays under the 200-LOC ceiling and the cost vs token concerns are
//! cleanly separated:
//!   - `chat_cost.rs` — kei-ledger cost writes (existed pre-Phase 2)
//!   - `chat_token.rs` (this file) — kei-token-tracker event writes
//!
//! Failure policy: every helper here is fire-and-forget. Tracker write
//! failures log to stderr; they never surface to the chat caller.

use crate::tool::loop_driver::TokenUsage;
use kei_token_tracker::{Store as TokenTracker, TokenEvent};
use std::sync::Arc;
use tracing::debug;
use std::time::{SystemTime, UNIX_EPOCH};

/// Bundle for one [`TokenEvent`] write. Held by the post-Done callback
/// path (`chat_stream::fire_post_done`, openai handlers' Done branches)
/// so the spawn_blocking closure can move it as a single Send value.
#[derive(Debug, Clone)]
pub struct TokenWrite {
    pub agent_id: String,
    pub conversation_id: Option<String>,
    pub model: String,
    pub role: String,
    pub source_kind: String,
    pub usage: TokenUsage,
    pub micro_cents: u64,
}

/// Record one token-event into the tracker. Fire-and-forget: any error
/// is logged to stderr; the chat call is unaffected. The handle is
/// `Option`al so callers don't have to branch — they just pass whatever
/// `AppState::token_tracker()` returned.
pub fn record_token_event(
    tracker: Option<Arc<std::sync::Mutex<TokenTracker>>>,
    write: TokenWrite,
) {
    let Some(t) = tracker else { return };
    let event = build_token_event(&write);
    let guard = match t.lock() {
        Ok(g) => g,
        Err(e) => e.into_inner(),
    };
    if let Err(e) = guard.record_event(&event) {
        debug!(
            "kei-cortex token-tracker: record_event({}) failed: {e}",
            write.agent_id
        );
    }
}

/// Build a [`TokenEvent`] from the per-turn `TokenWrite`. Saturating
/// `as u32` cast on token counts keeps the schema column happy on the
/// once-in-a-blue-moon turn that breaks 4 billion tokens.
fn build_token_event(w: &TokenWrite) -> TokenEvent {
    TokenEvent {
        ts: unix_now(),
        agent_id: w.agent_id.clone(),
        conversation_id: w.conversation_id.clone(),
        model: w.model.clone(),
        role: w.role.clone(),
        input_tokens: w.usage.input_tokens.min(u32::MAX as u64) as u32,
        output_tokens: w.usage.output_tokens.min(u32::MAX as u64) as u32,
        micro_cents: w.micro_cents,
        category: None,
        source_kind: Some(w.source_kind.clone()),
        latency_ms: None,
    }
}

/// Spawn a blocking task that performs the tracker write off the async
/// runtime's worker pool. Caller forgets the future; the closure owns
/// both the handle and the write bundle.
pub fn spawn_record_token_event(
    tracker: Option<Arc<std::sync::Mutex<TokenTracker>>>,
    write: TokenWrite,
) {
    tokio::task::spawn_blocking(move || record_token_event(tracker, write));
}

/// Local clock helper. Mirrors `chat_cost::unix_now` so this cube has
/// no inbound dep on chat_cost — the two are now siblings, not chained.
fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
