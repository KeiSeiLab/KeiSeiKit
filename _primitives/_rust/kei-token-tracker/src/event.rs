//! [`TokenEvent`] data shape — one row per LLM turn / tool call.
//!
//! `micro_cents` matches the `kei-ledger` cost unit (1 cent = 1_000_000
//! micro-cents) so cross-store rollups stay coherent without conversion
//! tables. The optional `category` / `source_kind` fields allow the
//! sleep-report to bucket usage by call site without forcing every
//! caller to populate them.

use serde::{Deserialize, Serialize};

/// One LLM turn worth of telemetry. `record_event` accepts a borrowed
/// reference; the store copies fields into prepared-statement params,
/// so the caller keeps ownership of the struct after recording.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenEvent {
    pub ts: i64,
    pub agent_id: String,
    pub conversation_id: Option<String>,
    pub model: String,
    pub role: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub micro_cents: u64,
    pub category: Option<String>,
    pub source_kind: Option<String>,
    pub latency_ms: Option<u32>,
}

impl TokenEvent {
    /// Convenience constructor for the common chat-turn shape — fills in
    /// optional columns as `None`. Tests + CLI seeding paths use this so
    /// they don't have to spell out every Option<…> at every call site.
    pub fn chat_turn(
        ts: i64,
        agent_id: impl Into<String>,
        model: impl Into<String>,
        role: impl Into<String>,
        input_tokens: u32,
        output_tokens: u32,
        micro_cents: u64,
    ) -> Self {
        Self {
            ts,
            agent_id: agent_id.into(),
            conversation_id: None,
            model: model.into(),
            role: role.into(),
            input_tokens,
            output_tokens,
            micro_cents,
            category: None,
            source_kind: None,
            latency_ms: None,
        }
    }
}
