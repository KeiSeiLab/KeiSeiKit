---
title: prompts.rs
path: kei-content-store/src/prompts.rs
dna_hash: sha256:be3a03d3fd4f81dd
language: rust
size_loc: 66
generated: by-keidocs
---

# kei-content-store/src/prompts.rs

Prompts — hash-deduplicated prompt registry.

Stays bespoke (not promoted to engine) because `register_prompt`
uses `INSERT OR IGNORE` + re-query by `UNIQUE(prompt_hash, model)`
to collapse duplicate text+model submissions to the same id.
The engine `create` verb is plain `INSERT` (no OR IGNORE), which
would break `prompt_dedup_by_hash` semantics. The table DDL still
lives in `CONTENT_SCHEMA::custom_migrations`.

## Related

- parent: `kei-content-store/Cargo.toml`
- imports: anyhow, chrono, crate, rusqlite, serde, sha2

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
