---
title: event.rs
path: kei-token-tracker/src/event.rs
dna_hash: sha256:8b63f22cacd5f7c1
language: rust
size_loc: 56
generated: by-keidocs
---

# kei-token-tracker/src/event.rs

[`TokenEvent`] data shape — one row per LLM turn / tool call.

`micro_cents` matches the `kei-ledger` cost unit (1 cent = 1_000_000
micro-cents) so cross-store rollups stay coherent without conversion
tables. The optional `category` / `source_kind` fields allow the
sleep-report to bucket usage by call site without forcing every
caller to populate them.

## Public API

- One LLM turn worth of telemetry. `record_event` accepts a borrowed
- `pub fn chat_turn` — Convenience constructor for the common chat-turn shape — fills in

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
