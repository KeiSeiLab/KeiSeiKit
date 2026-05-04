---
title: cost.rs
path: kei-ledger/src/cost.rs
dna_hash: sha256:3adfa832c17d15c1
language: rust
size_loc: 181
generated: by-keidocs
---

# kei-ledger/src/cost.rs

`record_cost` — write cost-tracking columns for an existing agent row.

Constructor Pattern: one cube = one mutator. Wave 44c (2026-04-24)
flipped the semantics from last-write-wins to ADDITIVE — the chat
handler under-counted multi-turn conversations because each turn
UPDATEd the same `conversation_id`-derived row, overwriting the
prior turn's cents. Now consecutive `record_cost` calls accumulate;
a separate `replace_cost` exists for the (rare) explicit-overwrite
flow that previously used `record_cost`.

Schema dependency: requires the v7 migration (`cost_cents` /
`cost_micro_cents` / `provider` / `model` columns on the `agents`
table). Caller is expected to have opened the ledger via
`ledger::open()` so migrations run before this function is reachable.

Why additive (vs MAX or last-write): every chat turn under the same
`conversation_id` shares a row. With last-write-wins, a 10-turn
conversation billed only the final turn — silent under-charge in
`/usage` aggregation. ADD avoids both that under-charge AND the
double-count risk of legacy retry overwrites by exposing
`replace_cost` as the explicit "I am rewriting, not adding" call.

## Public API

- `pub const MICRO_CENTS_PER_CENT` — 1 cent = 1_000_000 micro-cents. Public so external tooling (the
- `pub fn record_cost` — Add cost-tracking metadata to `agent_id`. Callers that ALREADY have
- `pub fn record_cost_micro` — Same as `record_cost` but accepts the EXACT micro-cents accumulator.
- `pub fn replace_cost` — Explicit OVERWRITE of cost columns. Use ONLY for retry / amend
- `pub fn replace_cost_micro` — Same as `replace_cost` but with explicit micro-cents value (no
- `pub fn read_cost` — Read back the (cost_cents, provider, model) tuple for `agent_id`.
- `pub fn read_cost_micro` — Read both the cents and micro-cents columns alongside provider and
- `pub fn compose_micro_cents` — Compute EXACT micro-cents from a token usage and per-MTok cents
- `pub fn display_cents_from_micro` — Render a micro-cents accumulator as a display cents value. Uses

## Related

- parent: `kei-ledger/Cargo.toml`
- imports: rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
