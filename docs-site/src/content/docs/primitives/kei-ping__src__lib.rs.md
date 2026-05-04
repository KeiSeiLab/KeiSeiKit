---
title: lib.rs
path: kei-ping/src/lib.rs
dna_hash: sha256:ccb5594d55c65f5c
language: rust
size_loc: 15
generated: by-keidocs
---

# kei-ping/src/lib.rs

`kei-ping` — cross-window agent heartbeat. Auto-selects backend.

Constructor Pattern: 1 trait + 2 impl-cubes (sqlite / redis) + 1 dispatcher.
Each cube ≤200 LOC, 1 responsibility.

## Related

- parent: `kei-ping/Cargo.toml`

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
