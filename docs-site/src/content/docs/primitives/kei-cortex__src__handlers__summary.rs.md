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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
