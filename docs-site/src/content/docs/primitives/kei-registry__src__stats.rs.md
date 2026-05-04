---
title: stats.rs
path: kei-registry/src/stats.rs
dna_hash: sha256:0d01aee484b7d30f
language: rust
size_loc: 75
generated: by-keidocs
---

# kei-registry/src/stats.rs

Aggregate counts over the registry.

Constructor Pattern: pure SQL aggregation, no I/O beyond the connection.
Returns a `Stats` struct with per-type counts and supersede ratios.

## Public API

- Per-type and global aggregate counts.
- Per-block-type counts and timestamp bracket.
- `pub fn compute_stats` — Compute aggregates for the entire `blocks` table.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, rusqlite, serde, std

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
