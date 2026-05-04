---
title: lib.rs
path: kei-graph-check/src/lib.rs
dna_hash: sha256:636d662fe52cd969
language: rust
size_loc: 10
generated: by-keidocs
---

# kei-graph-check/src/lib.rs

kei-graph-check — post-refactor reference-integrity gate.

Inputs: a directory root + an optional patch file (advisory only — we
detect file deletions/renames declared in the patch header and warn).
Output: list of broken references with file:line.

## Related

- parent: `kei-graph-check/Cargo.toml`

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
