---
title: types_map.rs
path: kei-db-contract/src/types_map.rs
dna_hash: sha256:25ec1887476def27
language: rust
size_loc: 73
generated: by-keidocs
---

# kei-db-contract/src/types_map.rs

SQL → TypeScript type compatibility table.
Conservative allow-list: anything not listed surfaces as drift.

## Public API

- `pub fn sql_ts_compatible` — Returns true when the SQL column type is compatible with the TS field type.
- `pub fn strip_null_union` — Filter out `null` / `undefined` from a TS union to get the core type set.
- `pub fn null_compatible` — SQL column nullable + TS field shape ⇒ compatible? NOT NULL columns always pass.

## Related

- parent: `kei-db-contract/Cargo.toml`
- imports: crate

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
