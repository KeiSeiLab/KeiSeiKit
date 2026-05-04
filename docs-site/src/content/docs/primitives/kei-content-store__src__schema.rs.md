---
title: schema.rs
path: kei-content-store/src/schema.rs
dna_hash: sha256:a3a533990b3e083d
language: rust
size_loc: 105
generated: by-keidocs
---

# kei-content-store/src/schema.rs

kei-content-store EntitySchemas — declarative specs consumed by
`kei_entity_store::Store` and its verb templates.

Shape (multi-schema convergence, 2026-04-23):

- `CONTENT_SCHEMA`: primary entity `content_units` (assets; INTEGER
PK; engine-owned create/get/list/search/update/delete + FTS).
- `CAMPAIGNS_SCHEMA`: plain-CRUD INTEGER-PK table promoted to engine
on this pass (create/get only — no idempotency or dedup).
- `ALL_SCHEMAS`: the `&[&EntitySchema]` slice `Store::open` hands
to the engine.

Secondary tables that stay in `custom_migrations` (on CONTENT_SCHEMA)
and keep bespoke SQL in their sibling modules:

- `prompts` — hash-dedup via `INSERT OR IGNORE` + re-query by
`UNIQUE(prompt_hash, model)`; engine `create` is plain INSERT,
would break `prompt_dedup_by_hash` test. Sibling: `prompts.rs`.
- `campaign_assets` — composite `(campaign_id, asset_id)` PK, no
single-column PK; engine schemas require one PK field. Also uses
`INSERT OR IGNORE` for idempotent attach. Sibling: `campaigns.rs`.

## Public API

- Secondary DDL co-located with `content_units` — indexes on the

## Related

- parent: `kei-content-store/Cargo.toml`
- imports: kei_entity_store

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
