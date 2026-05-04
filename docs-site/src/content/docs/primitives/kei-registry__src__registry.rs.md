---
title: registry.rs
path: kei-registry/src/registry.rs
dna_hash: sha256:be33e1e7ff657c8c
language: rust
size_loc: 182
generated: by-keidocs
---

# kei-registry/src/registry.rs

Block CRUD over the registry SQLite store.

Constructor Pattern: this cube owns the SQL row mapping + register
idempotency rule. Schema lives in `store.rs`; DNA composition in
`dna_block.rs`. The supersede chain is the only non-trivial state
transition — see `register` below.

## Public API

- `pub fn register` — Register a block. Idempotency rule:
- `pub fn mark_superseded` — Set `superseded_by` on the row with id `old_id` to `new_dna`. Touches
- `pub fn get` — Fetch a block by integer id.
- `pub fn get_by_dna` — Fetch a block by full DNA string.
- `pub fn list` — List active blocks (or all if `include_superseded` is true), bounded by `limit`.
- `pub fn list_by_type` — List all active blocks of one type (no limit; caller paginates if needed).
- `pub fn find_by_path` — Find the active row at `path`, if any.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, rusqlite, std

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
