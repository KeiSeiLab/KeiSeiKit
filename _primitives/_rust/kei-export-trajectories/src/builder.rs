//! Convert a hydrated `TrajectoryRecord` into a ShareGPT `Trajectory`.
//!
//! Constructor Pattern: pure function cube — no I/O, no SQL. Lives in
//! the library so both the CLI binary and the integration test exercise
//! the same code path.
//!
//! Hermes normalization rules applied here:
//! - every `gpt` turn carries a `<think>` block (empty if no reasoning)
//! - system prompt is generated at save-time, NOT taken from the
//!   conversation source

use crate::builder_chatlog_parse::parse_chatlog_turns;
use crate::sharegpt::{From as ShareGptFrom, ShareGptMessage, ToolStats, Trajectory};
use crate::tool_stats::aggregate_tool_stats;
use crate::TrajectoryRecord;
use std::collections::BTreeMap;

/// Hermes-style canonical system prompt. Kept short — the real
/// production prompt should mirror the per-deployment tool registry.
/// Use [`system_prompt`] to read with `KEI_EXPORT_SYSTEM_PROMPT` env override.
pub const DEFAULT_SYSTEM_PROMPT: &str =
    "You are a KeiSei agent. Respond using <think> / <tool_call> XML conventions.";

/// Back-compat alias — was a const, now resolves at call-site via env.
#[deprecated(note = "use system_prompt() to honour KEI_EXPORT_SYSTEM_PROMPT env var")]
pub const SYSTEM_PROMPT: &str = DEFAULT_SYSTEM_PROMPT;

/// Read `KEI_EXPORT_SYSTEM_PROMPT` env var; fall back to [`DEFAULT_SYSTEM_PROMPT`].
pub fn system_prompt() -> String {
    std::env::var("KEI_EXPORT_SYSTEM_PROMPT")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| DEFAULT_SYSTEM_PROMPT.to_string())
}

/// Synthesize a multi-turn conversation (system + human + N gpt/tool turns)
/// plus metadata + tool_stats from a single record. Gpt/tool turn count is
/// derived from `<tool_call>` / `<tool_response>` markers in the chatlog;
/// markerless chatlogs collapse to a single legacy `gpt` turn. The
/// `tool_error_counts` mirror is filled later by `normalize_keys`.
pub fn record_to_trajectory(prompt_index: u64, r: &TrajectoryRecord) -> Trajectory {
    let convs = build_conversations(r);
    let stats: BTreeMap<String, ToolStats> = aggregate_tool_stats(&r.tool_events);
    let metadata = build_metadata(r);
    Trajectory {
        prompt_index,
        conversations: convs,
        completed: r.completed(),
        tool_stats: stats,
        tool_error_counts: BTreeMap::new(),
        metadata,
    }
}

fn build_conversations(r: &TrajectoryRecord) -> Vec<ShareGptMessage> {
    let mut out = Vec::with_capacity(3);
    out.push(ShareGptMessage {
        from: ShareGptFrom::System,
        value: system_prompt(),
    });
    out.push(ShareGptMessage {
        from: ShareGptFrom::Human,
        value: r.spec_text.clone(),
    });
    out.extend(parse_chatlog_turns(&r.chatlog_text));
    out
}

fn build_metadata(r: &TrajectoryRecord) -> serde_json::Map<String, serde_json::Value> {
    let mut m = serde_json::Map::new();
    m.insert("agent_id".into(), r.agent_id.clone().into());
    m.insert("branch".into(), r.branch.clone().into());
    m.insert("started_ts".into(), r.started_ts.into());
    if let Some(ft) = r.finished_ts {
        m.insert("finished_ts".into(), ft.into());
    }
    if let Some(ref s) = r.summary {
        m.insert("summary".into(), s.clone().into());
    }
    if let Some(ref d) = r.dna {
        m.insert("dna".into(), d.clone().into());
    }
    m
}

