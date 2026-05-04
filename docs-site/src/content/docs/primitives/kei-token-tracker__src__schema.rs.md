---
title: schema.rs
path: kei-token-tracker/src/schema.rs
dna_hash: sha256:e6ba405054a0a665
language: rust
size_loc: 68
generated: by-keidocs
---

# kei-token-tracker/src/schema.rs

SQLite schema runner. One table + three indexes, applied at `open`.

## Public API

- `pub fn migrate` — Apply pending migrations. Idempotent: re-running on an up-to-date

## Related

- parent: `kei-token-tracker/Cargo.toml`
- imports: crate, rusqlite

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
