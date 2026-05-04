---
title: error.rs
path: kei-prune/src/error.rs
dna_hash: sha256:fe37526b74e3d819
language: rust
size_loc: 25
generated: by-keidocs
---

# kei-prune/src/error.rs

Error type for kei-prune.

Constructor Pattern: one cube = one error enum. Keeps `rusqlite::Error`
wrapped so callers don't need to depend on rusqlite directly.

## Public API

- All failure modes produced by the kei-prune public API.
- Underlying SQLite error (schema DDL, query, insert).
- Agent id not present in the ledger `agents` table.
- JSON serialisation failure (CLI output only).

## Related

- parent: `kei-prune/Cargo.toml`
- imports: thiserror

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
