---
title: store.rs
path: kei-content-store/src/store.rs
dna_hash: sha256:79c1d0d49a88330d
language: rust
size_loc: 36
generated: by-keidocs
---

# kei-content-store/src/store.rs

Content store — thin shim over `kei_entity_store::Store`.

Multi-schema convergence (2026-04-23): both `content_units` and
`campaigns` are engine-owned. `Store::open` hands the engine
`ALL_SCHEMAS` so migrations for both tables run in a single
atomic transaction.

Verbs dispatch per-schema: callers that act on assets pass
`CONTENT_SCHEMA`, callers that act on campaigns pass
`CAMPAIGNS_SCHEMA`. Two bespoke SQL paths remain:
`prompts.rs::register_prompt` (hash-dedup) and
`campaigns.rs::{attach_asset,campaign_assets}` (composite PK).

## Related

- parent: `kei-content-store/Cargo.toml`
- imports: anyhow, crate, kei_entity_store, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
