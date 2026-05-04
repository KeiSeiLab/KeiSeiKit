---
title: stats.rs
path: kei-brain-view/src/stats.rs
dna_hash: sha256:6b99274d1d96b382
language: rust
size_loc: 52
generated: by-keidocs
---

# kei-brain-view/src/stats.rs

Bucket counts over the hydrated graph.

Constructor Pattern: one cube = `Stats` struct + `compute_stats` fn +
text renderer. Deterministic output order (sorted keys) so downstream
diffing / snapshot tests stay stable across runs.

## Public API

- status -> count (e.g. "running" -> 3, "done" -> 12)
- has-dna count (non-NULL dna column)
- top-level roots (no parent_branch or parent outside the ledger)
- non-root forks
- `pub fn compute_stats` — Build `Stats` from an in-memory graph — pure, no I/O.
- `pub fn render_stats` — Human-readable text report. Ordering: total, roots/forks, with_dna,

## Related

- parent: `kei-brain-view/Cargo.toml`
- imports: crate, serde, std

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
