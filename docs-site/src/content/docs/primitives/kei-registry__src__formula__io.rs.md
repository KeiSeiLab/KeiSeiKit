---
title: io.rs
path: kei-registry/src/formula/io.rs
dna_hash: sha256:f8545596189f6bc8
language: rust
size_loc: 119
generated: by-keidocs
---

# kei-registry/src/formula/io.rs

Formula persistence helpers — write and read against `blocks`,
`block_predicates`, `block_deps` (schema v2-v4).

Constructor Pattern: each helper is one SQL operation; the public
`register_formula` / `load_formula` in `mod.rs` compose them inside a
single transaction.

## Public API

- Update the four formula columns on a `blocks` row. Does NOT touch
- Replace all predicate rows for `block_id` with the supplied list.
- Replace all dep rows for `block_id` with the supplied set, marking each
- Read the four formula columns from `blocks`. Returns `None` when
- Read predicates for a block, ordered by `seq` so the round-trip order
- Read all dep DNAs for a block. Returns a `BTreeSet` so callers see them

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, rusqlite, std

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
