---
title: mod.rs
path: kei-cleanup/src/scanners/mod.rs
dna_hash: sha256:ab45230f2729ae0e
language: rust
size_loc: 29
generated: by-keidocs
---

# kei-cleanup/src/scanners/mod.rs

Scanner registry — see CLEANUP-RUNTIME-SPEC.md §3.2.

Each scanner module exposes one `pub fn scan(workspace, cfg)` that
returns `Result<Vec<Finding>, CleanupError>`. The dispatcher in
`lib.rs` runs them in a fixed order and aggregates findings.

## Public API

- `pub const ALL` — Names of scanners shipped, in dispatch order.

## Related

- parent: `kei-cleanup/Cargo.toml`

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
