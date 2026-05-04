---
title: aggregate.rs
path: kei-token-tracker/src/aggregate.rs
dna_hash: sha256:8cf7fe5c7e780b4d
language: rust
size_loc: 61
generated: by-keidocs
---

# kei-token-tracker/src/aggregate.rs

Per-model aggregation queries used by `sleep-report` and CLI.

## Public API

- Rolled-up usage for a single (model) since a unix epoch lower bound.
- `pub fn total_tokens` — Sum of input + output tokens. Convenience for the report renderer.
- `pub const MICRO_CENTS_PER_CENT` — 1 cent = 1_000_000 micro-cents — mirrors `kei_ledger::MICRO_CENTS_PER_CENT`.
- `pub fn format_usd` — Render a micro-cents value as a USD-display string with two decimals.
- `pub fn total_micro_cents` — Sum micro-cents across a slice of aggregates. Pure helper; saves the
- `pub fn total_tokens` — Sum total token counts (input + output) across a slice.
- `pub fn total_input_tokens` — Sum input tokens specifically — sleep-report header line item.
- `pub fn total_output_tokens` — Sum output tokens specifically — sleep-report header line item.
- `pub fn total_events` — Sum event counts across all aggregates.

## Related

- parent: `kei-token-tracker/Cargo.toml`
- imports: serde

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
