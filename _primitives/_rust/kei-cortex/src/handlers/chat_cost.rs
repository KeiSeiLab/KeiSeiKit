//! Cost-recording side-channel for the chat handler.
//!
//! Constructor Pattern: one cube = one responsibility (turning per-turn
//! `TokenUsage` accumulation into a kei-ledger cost write at the end of
//! the agentic loop). Kept separate from `chat.rs` so neither file
//! exceeds the 200-LOC ceiling.
//!
//! Wave 44c (2026-04-24) refactor:
//!   * F-MED-4 — switched to micro-cents (1c = 1_000_000 µc) so 1-token
//!     turns no longer ceil-charge a full cent. Display rounding happens
//!     at the API boundary in `/usage`.
//!   * F-HIGH-3 — switched to ADDITIVE accumulation in kei-ledger so a
//!     10-turn conversation under one `conversation_id` charges all ten
//!     turns, not just the last one.
//!   * MISS-5 — replaced `expect("usage mutex poisoned")` with
//!     `unwrap_or_else(|e| e.into_inner())` so a poisoned lock is
//!     recoverable, not fatal to the streaming task.
//!
//! Failure policy: every helper here is fire-and-forget. A SQLite
//! write error is logged to stderr but never surfaces to the SSE
//! client — the user already saw a successful chat turn, and a
//! missing ledger row is a `/usage` accuracy problem, not a chat-
//! correctness problem.

use crate::tool::loop_driver::{ModelInvoker, TokenUsage};
use std::path::PathBuf;
use tracing::debug;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

/// Compute EXACT micro-cents (1 cent = 1_000_000 µc) for the given
/// `(input, output)` token pair and per-MTok cents rates. No rounding
/// loss — the underlying multiplication is u128. Use
/// `display_cents` to render this as integer cents at the API boundary.
pub fn compute_micro_cents(usage: &TokenUsage, in_cents_per_m: u32, out_cents_per_m: u32) -> u64 {
    kei_ledger::compose_micro_cents(
        usage.input_tokens,
        usage.output_tokens,
        in_cents_per_m,
        out_cents_per_m,
    )
}

/// Render a micro-cents accumulator as integer cents using half-up
/// rounding. The kei-ledger `cost_cents` column gets this value while
/// `cost_micro_cents` keeps the exact accumulator.
pub fn display_cents(micro_cents: u64) -> u64 {
    kei_ledger::display_cents_from_micro(micro_cents)
}

/// Wrap a `ModelInvoker` so that every successful turn's `usage` block
/// is added to a shared accumulator. Failed turns leave the accumulator
/// untouched. The wrapper is `Send + Sync + 'static` like the input.
pub fn wrap_invoker_with_usage_capture(
    inner: ModelInvoker,
    accum: Arc<Mutex<TokenUsage>>,
) -> ModelInvoker {
    Arc::new(move |messages, tool_defs| {
        let inner = inner.clone();
        let accum = accum.clone();
        Box::pin(async move {
            let result = inner(messages, tool_defs).await;
            if let Ok(turn) = &result {
                if let Some(u) = &turn.usage {
                    let mut guard = lock_recover(&accum);
                    guard.input_tokens = guard.input_tokens.saturating_add(u.input_tokens);
                    guard.output_tokens = guard.output_tokens.saturating_add(u.output_tokens);
                }
            }
            result
        })
    })
}

/// Lock the accumulator, recovering the inner state if a previous panic
/// poisoned the mutex. Streaming tasks must NOT abort — a stale row in
/// kei-ledger is preferable to a 500 to the SSE client (MISS-5).
fn lock_recover<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Build a stable agent_id for a chat turn. Prefers the caller-supplied
/// `conversation_id` so multi-turn conversations accumulate cost on a
/// single row (Wave 44c F-HIGH-3 — was last-write-wins, now ADD).
/// Falls back to `chat-{user_id}-{epoch_seconds}` so the row is still
/// addressable when the client did not send a conversation_id.
pub fn build_agent_id(conversation_id: Option<&str>, user_id: &str) -> String {
    if let Some(c) = conversation_id {
        if !c.is_empty() {
            return format!("chat-{c}");
        }
    }
    format!("chat-{user_id}-{}", unix_now())
}

/// Seconds since Unix epoch. Falls back to 0 only if the clock predates
/// 1970 (impossible on a healthy host but cheaper than panicking).
fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Bundle of values needed for a single ledger write. Passed by value
/// to the `tokio::task::spawn_blocking` closure so the SQLite call is
/// OFF the async runtime's worker pool. `micro_cents` is the EXACT
/// accumulator; `cents` is the rounded display value (kept coherent
/// inside the ledger row).
#[derive(Debug, Clone)]
pub struct CostWrite {
    pub ledger_path: PathBuf,
    pub agent_id: String,
    pub provider: String,
    pub model: String,
    pub cents: u64,
    pub micro_cents: u64,
}

/// Open ledger, ensure a row exists for `agent_id`, write cost. Logs to
/// stderr on any SQL failure. Designed for `spawn_blocking`.
pub fn record_chat_cost(write: CostWrite) {
    let conn = match kei_ledger::open(&write.ledger_path) {
        Ok(c) => c,
        Err(e) => {
            debug!(
                "kei-cortex chat_cost: open ledger {} failed: {e}",
                write.ledger_path.display()
            );
            return;
        }
    };
    if let Err(e) = ensure_row(&conn, &write.agent_id) {
        debug!("kei-cortex chat_cost: ensure_row({}): {e}", write.agent_id);
        return;
    }
    if let Err(e) = kei_ledger::record_cost_micro(
        &conn,
        &write.agent_id,
        write.cents,
        write.micro_cents,
        &write.provider,
        &write.model,
    ) {
        debug!("kei-cortex chat_cost: record_cost({}): {e}", write.agent_id);
    }
}

/// Insert a placeholder running-and-immediately-done row if one does
/// not already exist for `agent_id`. We use INSERT OR IGNORE so a
/// follow-up chat turn under the same conversation_id reuses the
/// existing row, which `record_cost_micro` then ADDS to.
fn ensure_row(conn: &rusqlite::Connection, agent_id: &str) -> rusqlite::Result<usize> {
    let now = unix_now();
    let branch = format!("chat-stream-{agent_id}");
    conn.execute(
        "INSERT OR IGNORE INTO agents
         (id, branch, parent_branch, spec_sha, status, started_ts, finished_ts, summary)
         VALUES (?1, ?2, NULL, 'chat-handler', 'done', ?3, ?3, 'chat turn')",
        rusqlite::params![agent_id, truncate_branch(&branch), now],
    )
}

/// SQLite `branch` cap is 256 chars (kei-ledger schema v3 trigger). We
/// truncate to 200 to leave headroom for the `chat-stream-` prefix.
/// Walks back to the nearest char boundary so UTF-8 input never panics.
fn truncate_branch(s: &str) -> String {
    const CAP: usize = 200;
    if s.len() <= CAP {
        return s.to_string();
    }
    let mut end = CAP;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    s[..end].to_string()
}

/// Look up a provider's per-MTok rates from a `LlmRouter`. Returns
/// `(input_cents, output_cents)`; falls back to (0, 0) when the
/// provider is not registered (e.g. tests with empty router).
pub fn provider_rates(router: &kei_router::LlmRouter, name: &str) -> (u32, u32) {
    match router.pick(name) {
        Ok(p) => (p.cost_per_m_tok_input_cents(), p.cost_per_m_tok_output_cents()),
        Err(_) => (0, 0),
    }
}

/// Snapshot the accumulator into an owned `TokenUsage`. Used at the end
/// of the loop where we need a Send-able value for `spawn_blocking`.
/// Recovers from a poisoned mutex (MISS-5).
pub fn snapshot(accum: &Arc<Mutex<TokenUsage>>) -> TokenUsage {
    lock_recover(accum).clone()
}

// Token-event recording moved to sibling cube `chat_token.rs` (cleanup
// follow-up to Phase 2 wiring) to keep both files under the 200-LOC
// Constructor Pattern ceiling. Re-exported via handlers::chat_token::*.

#[cfg(test)]
#[path = "chat_cost_test.rs"]
mod tests;
