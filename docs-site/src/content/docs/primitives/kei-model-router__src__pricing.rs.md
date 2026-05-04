---
title: pricing.rs
path: kei-model-router/src/pricing.rs
dna_hash: sha256:84efd2456a57672c
language: rust
size_loc: 168
generated: by-keidocs
---

# kei-model-router/src/pricing.rs

Verified Claude API pricing constants.

Source: <https://platform.claude.com/docs/en/docs/about-claude/pricing>
Verified: 2026-04-30 (RULE 0.4 — primary source fetched in same session).

All prices in microcents per 1M tokens (`u64` to avoid float drift in
cost arithmetic). 1 microcent = 1e-6 USD = 1e-4 cents. Aligns with
`kei-ledger.cost_micro_cents` column.

Constructor Pattern: pricing is one cube. The decision rule (`select.rs`)
reads constants from here and never duplicates them.

## Public API

- Per-model token pricing (microcents per 1M tokens).
- `pub const OPUS_47_TOKENIZER_OVERHEAD` — Tokenizer density relative to baseline (Sonnet/Haiku tokenizer).
- `pub const HAIKU_45` — Claude Haiku 4.5 — cheapest, simple lookup / formatting / single-edit.
- `pub const SONNET_46` — Claude Sonnet 4.6 — multi-step reasoning, code edits, summarization.
- `pub const OPUS_47` — Claude Opus 4.7 — architecture, novel reasoning, math derivation.
- Discrete model identifier. Order matches escalation ladder
- `pub fn next_tier` — Next-tier (escalation). Returns None if already at top.
- `pub fn cost_micro_cents` — Cost in microcents for a single (input, output) token pair on `model`.

## Related

- parent: `kei-model-router/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
