---
title: sharegpt.rs
path: kei-export-trajectories/src/sharegpt.rs
dna_hash: sha256:4479cc268244c959
language: rust
size_loc: 84
generated: by-keidocs
---

# kei-export-trajectories/src/sharegpt.rs

ShareGPT JSONL data-transfer types.

Constructor Pattern: pure DTOs + serde derives. No I/O, no SQL, no
filesystem. The Hermes trajectory format mandates a `from` discriminator
that only takes 4 string values (`system / human / gpt / tool`) — model
it as a typed enum so callers cannot emit a typo'd role.

Reference: tools/hermes-research/.../trajectory-format.md §"Conversations
Array (ShareGPT Format)".

## Public API

- Hermes / ShareGPT role discriminator. Names match the JSON value the
- System prompt — typically generated at save-time per Hermes spec.
- User-side input. ShareGPT calls this `human` (not `user`).
- Assistant turn (think + tool_call XML or final answer).
- Tool response, XML-wrapped per Hermes normalization rule.
- A single conversation turn. The `value` field is verbatim text — caller
- Per-tool invocation counters, normalized so EVERY trajectory in a JSONL
- Top-level JSONL line — the Hermes "batch runner" variant carries the
- Stable monotonic index across the JSONL file. Matches Hermes
- Conversation turns in chronological order.
- `true` iff the agent reached `status='done'` or `'merged'` in the
- Per-tool counters. Always carries the union-of-all-tools key set
- Mirror of `tool_stats[k].failure` — Hermes datasets sometimes
- Free-form metadata: agent_id, branch, dna, started_ts, finished_ts,
- `pub fn refresh_error_counts` — Build the `tool_error_counts` mirror from `tool_stats`. Keep this in

## Related

- parent: `kei-export-trajectories/Cargo.toml`
- imports: serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
