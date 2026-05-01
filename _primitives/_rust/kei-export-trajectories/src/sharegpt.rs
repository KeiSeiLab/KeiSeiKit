//! ShareGPT JSONL data-transfer types.
//!
//! Constructor Pattern: pure DTOs + serde derives. No I/O, no SQL, no
//! filesystem. The Hermes trajectory format mandates a `from` discriminator
//! that only takes 4 string values (`system / human / gpt / tool`) — model
//! it as a typed enum so callers cannot emit a typo'd role.
//!
//! Reference: tools/hermes-research/.../trajectory-format.md §"Conversations
//! Array (ShareGPT Format)".

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Hermes / ShareGPT role discriminator. Names match the JSON value the
/// trainer expects on the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum From {
    /// System prompt — typically generated at save-time per Hermes spec.
    System,
    /// User-side input. ShareGPT calls this `human` (not `user`).
    Human,
    /// Assistant turn (think + tool_call XML or final answer).
    Gpt,
    /// Tool response, XML-wrapped per Hermes normalization rule.
    Tool,
}

/// A single conversation turn. The `value` field is verbatim text — caller
/// owns Hermes-style normalization (`<think>` wrapping, `<tool_call>` XML).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareGptMessage {
    pub from: From,
    pub value: String,
}

/// Per-tool invocation counters, normalized so EVERY trajectory in a JSONL
/// file carries the SAME key set (Hermes constraint: HuggingFace Arrow
/// schema unification).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ToolStats {
    pub count: u64,
    pub success: u64,
    pub failure: u64,
}

/// Top-level JSONL line — the Hermes "batch runner" variant carries the
/// richer envelope (prompt_index, tool_stats, metadata) that downstream
/// trainers consume; we always emit that shape because it's a strict
/// superset of the CLI variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trajectory {
    /// Stable monotonic index across the JSONL file. Matches Hermes
    /// `batch_runner.py` field of the same name.
    pub prompt_index: u64,
    /// Conversation turns in chronological order.
    pub conversations: Vec<ShareGptMessage>,
    /// `true` iff the agent reached `status='done'` or `'merged'` in the
    /// ledger. `failed` / `running` / `rejected` → `false`.
    pub completed: bool,
    /// Per-tool counters. Always carries the union-of-all-tools key set
    /// for this export run, with zero defaults — enforced by
    /// `tool_stats::normalize_keys`.
    pub tool_stats: BTreeMap<String, ToolStats>,
    /// Mirror of `tool_stats[k].failure` — Hermes datasets sometimes
    /// reference this field directly. Same key set as `tool_stats`.
    pub tool_error_counts: BTreeMap<String, u64>,
    /// Free-form metadata: agent_id, branch, dna, started_ts, finished_ts,
    /// summary. Stored as JSON values so the schema stays open-ended for
    /// future ledger columns without breaking existing readers.
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

impl Trajectory {
    /// Build the `tool_error_counts` mirror from `tool_stats`. Keep this in
    /// one place so the two fields cannot drift.
    pub fn refresh_error_counts(&mut self) {
        self.tool_error_counts = self
            .tool_stats
            .iter()
            .map(|(k, v)| (k.clone(), v.failure))
            .collect();
    }
}
