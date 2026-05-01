//! Wiring between the chat handler and the memory-nudge scheduler.
//!
//! Constructor Pattern: this cube owns ONE responsibility — assembling
//! an `AgentContext` from a completed chat turn and firing
//! `MemoryNudgeScheduler::maybe_trigger`. Kept separate from
//! `chat_stream.rs` so neither file exceeds the 200-LOC ceiling.
//!
//! Frozen-snapshot invariant: the conversation `Vec<Turn>` we pass to
//! the scheduler is freshly constructed on each call from the
//! user-message + assistant-text pair. We do NOT keep a running
//! conversation here — the stream handler is stateless across requests
//! and the scheduler treats each call independently. The Hermes
//! "snapshot" abstraction is satisfied by the read-only `RwLock`
//! handed to the review task at trigger time.

use std::sync::Arc;
use tokio::sync::RwLock;

use kei_pet::memory::MemoryTag;

use crate::agent::memory_nudge::{AgentContext, Turn};
use crate::agent::memory_review_task::PersistTarget;
use crate::state::AppState;

/// Default pet-name used for the persist target when the request
/// doesn't carry an explicit pet selector. Matches the pet-root
/// convention used by `handlers/memory.rs` (callers may override
/// in the future via a query param).
pub const DEFAULT_PET_NAME: &str = "default";

/// Build an `AgentContext` for a completed (user, assistant) turn
/// pair and return it ready for `scheduler.maybe_trigger`.
pub fn build_context(
    state: &AppState,
    user_id: &str,
    conversation_id: &str,
    user_msg: &str,
    assistant_msg: &str,
) -> AgentContext {
    let turns = Arc::new(RwLock::new(vec![
        Turn {
            role: "user".to_string(),
            content: user_msg.to_string(),
        },
        Turn {
            role: "assistant".to_string(),
            content: assistant_msg.to_string(),
        },
    ]));
    let invoker = state.build_memory_invoker();
    let persist = PersistTarget {
        db_path: state.config().memory_db.clone(),
        tag: MemoryTag {
            user_id: user_id.to_string(),
            pet_name: DEFAULT_PET_NAME.to_string(),
        },
    };
    AgentContext::new(conversation_id.to_string(), turns)
        .with_invoker(invoker)
        .with_persist(persist)
}

/// Spawn `maybe_trigger` as a detached task. The chat handler does
/// NOT await — the SSE client already received `done` and the
/// scheduler / reviewer run in the background. Returning `()` keeps
/// the caller signature trivial.
pub fn spawn_nudge(
    state: AppState,
    user_id: String,
    conversation_id: String,
    user_msg: String,
    assistant_msg: String,
) {
    tokio::spawn(async move {
        let ctx = build_context(
            &state,
            &user_id,
            &conversation_id,
            &user_msg,
            &assistant_msg,
        );
        let _fired = state.scheduler().maybe_trigger(&ctx).await;
    });
}

#[cfg(test)]
#[path = "chat_memory_nudge_test.rs"]
mod tests;
