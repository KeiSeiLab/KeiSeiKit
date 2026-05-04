---
title: stats.rs
path: kei-prune/src/stats.rs
dna_hash: sha256:8472aeec33fa7855
language: rust
size_loc: 58
generated: by-keidocs
---

# kei-prune/src/stats.rs

Bucket counts for the `stats` verb.

Constructor Pattern: one cube = one aggregation DTO + one query.
Counts are computed with a single round-trip via CTEs to avoid the
drift that would happen if we summed four separate queries against
a table that could mutate between them.

## Public API

- Fleet-wide pruning summary.
- `pub fn stats` — Compute all four buckets in a single query.
- Run a `SELECT COUNT(*) ...` and return the scalar result.

## Related

- parent: `kei-prune/Cargo.toml`
- imports: crate, rusqlite, serde

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
