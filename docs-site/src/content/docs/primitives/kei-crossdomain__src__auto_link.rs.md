---
title: auto_link.rs
path: kei-crossdomain/src/auto_link.rs
dna_hash: sha256:383bfbeb35abfdd3
language: rust
size_loc: 60
generated: by-keidocs
---

# kei-crossdomain/src/auto_link.rs

Auto-link heuristic — proposes edges based on URI-name component matching.
No-ML: intersect the last path segments (case-insensitive, normalized).

## Public API

- `pub fn auto_link` — Scan cross_edges for entities referenced from `uri` domain and propose

## Related

- parent: `kei-crossdomain/Cargo.toml`
- imports: anyhow, crate, rusqlite

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
