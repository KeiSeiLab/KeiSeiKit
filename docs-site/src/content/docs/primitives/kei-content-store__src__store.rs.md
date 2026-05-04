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
