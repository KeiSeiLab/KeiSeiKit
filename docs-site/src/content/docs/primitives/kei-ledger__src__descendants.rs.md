---
title: descendants.rs
path: kei-ledger/src/descendants.rs
dna_hash: sha256:b6627497a5d25483
language: rust
size_loc: 24
generated: by-keidocs
---

# kei-ledger/src/descendants.rs

`descendants()` — lineage walker over `fork_parent_id` + `creator_id`.

Constructor Pattern: one cube = one query. Single public fn under 30 LOC.
RULE 0.12 v4 lineage lookup: find every agent that was forked-from OR
spawned-by a given DNA.

## Public API

- `pub fn descendants` — Return every row whose `fork_parent_id == dna` OR `creator_id == dna`.

## Related

- parent: `kei-ledger/Cargo.toml`
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
