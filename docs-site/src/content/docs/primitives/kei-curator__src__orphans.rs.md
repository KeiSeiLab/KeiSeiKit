---
title: orphans.rs
path: kei-curator/src/orphans.rs
dna_hash: sha256:78cd6812fd367c95
language: rust
size_loc: 22
generated: by-keidocs
---

# kei-curator/src/orphans.rs

Prune orphan URIs — those that appear in `cross_edges` but have no in-edges.
Conservative: only removes edges where the tail URI has no other incoming edge.

## Related

- parent: `kei-curator/Cargo.toml`
- imports: anyhow, rusqlite

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
