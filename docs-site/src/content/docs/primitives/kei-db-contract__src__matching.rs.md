---
title: matching.rs
path: kei-db-contract/src/matching.rs
dna_hash: sha256:8562ecd488632119
language: rust
size_loc: 86
generated: by-keidocs
---

# kei-db-contract/src/matching.rs

Pair SQL tables with TS types using name heuristics:
`users` ≈ `User`, `magic_tokens` ≈ `MagicToken`, `auth_sessions` ≈ `AuthSession`.

## Public API

- One paired (or orphan) result from the matching step.
- `pub fn pair_tables_with_types` — Pair every SQL table with at most one TS type and vice versa.
- `pub fn canonicalize_table` — Strip plural -s, normalize separators, lowercase. `magic_tokens` → `magictoken`.
- `pub fn canonicalize_type` — Lowercase + strip trailing -s. `MagicToken` → `magictoken`.

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
