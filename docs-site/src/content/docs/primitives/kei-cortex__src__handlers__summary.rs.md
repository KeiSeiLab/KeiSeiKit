---
title: summary.rs
path: kei-cortex/src/handlers/summary.rs
dna_hash: sha256:55696498a40ddfdc
language: rust
size_loc: 107
generated: by-keidocs
---

# kei-cortex/src/handlers/summary.rs

`GET /api/v1/cortex/summary` — aggregate counters over ledger + pets.

The endpoint is intentionally cheap: a couple of indexed COUNTs + a
directory scan. It exists so the UI can render a landing page without
hitting four separate endpoints.

## Public API

- JSON body returned by `/summary`.
- Handler entry point.
- Blocking helper: opens the ledger DB, runs 3 queries, lists the pet dir.
- Run a single scalar COUNT query against the ledger DB if present. Missing
- Return max(started_ts) from the agents table, or `None` if table is empty.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate, rusqlite, serde, std

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
