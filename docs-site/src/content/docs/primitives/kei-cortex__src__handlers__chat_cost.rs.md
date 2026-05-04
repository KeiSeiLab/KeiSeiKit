---
title: chat_cost.rs
path: kei-cortex/src/handlers/chat_cost.rs
dna_hash: sha256:2cb62b1da82209be
language: rust
size_loc: 201
generated: by-keidocs
---

# kei-cortex/src/handlers/chat_cost.rs

Cost-recording side-channel for the chat handler.

Constructor Pattern: one cube = one responsibility (turning per-turn
`TokenUsage` accumulation into a kei-ledger cost write at the end of
the agentic loop). Kept separate from `chat.rs` so neither file
exceeds the 200-LOC ceiling.

Wave 44c (2026-04-24) refactor:
* F-MED-4 — switched to micro-cents (1c = 1_000_000 µc) so 1-token
turns no longer ceil-charge a full cent. Display rounding happens
at the API boundary in `/usage`.
* F-HIGH-3 — switched to ADDITIVE accumulation in kei-ledger so a
10-turn conversation under one `conversation_id` charges all ten
turns, not just the last one.
* MISS-5 — replaced `expect("usage mutex poisoned")` with
`unwrap_or_else(|e| e.into_inner())` so a poisoned lock is
recoverable, not fatal to the streaming task.

Failure policy: every helper here is fire-and-forget. A SQLite
write error is logged to stderr but never surfaces to the SSE
client — the user already saw a successful chat turn, and a
missing ledger row is a `/usage` accuracy problem, not a chat-
correctness problem.

## Public API

- `pub fn compute_micro_cents` — Compute EXACT micro-cents (1 cent = 1_000_000 µc) for the given
- `pub fn display_cents` — Render a micro-cents accumulator as integer cents using half-up
- `pub fn wrap_invoker_with_usage_capture` — Wrap a `ModelInvoker` so that every successful turn's `usage` block
- Lock the accumulator, recovering the inner state if a previous panic
- `pub fn build_agent_id` — Build a stable agent_id for a chat turn. Prefers the caller-supplied
- Seconds since Unix epoch. Falls back to 0 only if the clock predates
- Bundle of values needed for a single ledger write. Passed by value
- `pub fn record_chat_cost` — Open ledger, ensure a row exists for `agent_id`, write cost. Logs to
- Insert a placeholder running-and-immediately-done row if one does
- SQLite `branch` cap is 256 chars (kei-ledger schema v3 trigger). We
- `pub fn provider_rates` — Look up a provider's per-MTok rates from a `LlmRouter`. Returns
- `pub fn snapshot` — Snapshot the accumulator into an owned `TokenUsage`. Used at the end

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
