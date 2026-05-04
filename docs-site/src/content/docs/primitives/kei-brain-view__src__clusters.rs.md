---
title: clusters.rs
path: kei-brain-view/src/clusters.rs
dna_hash: sha256:b7777a6e834f6989
language: rust
size_loc: 48
generated: by-keidocs
---

# kei-brain-view/src/clusters.rs

Cluster rendering over kei-dna-index groupings.

Constructor Pattern: one file = one responsibility (render clusters).
Pulls cluster groupings from `kei-dna-index` and decorates each member
with its current ledger status. Read-only — no schema mutation.

## Public API

- `pub fn render_clusters` — Render the cluster tree for `by` as an ASCII text block.
- Return the `agents.status` column for the row whose DNA matches.

## Related

- parent: `kei-brain-view/Cargo.toml`
- imports: crate, kei_dna_index, rusqlite

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
