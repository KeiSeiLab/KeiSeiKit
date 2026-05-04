---
title: dna.rs
path: keidocs/src/dna.rs
dna_hash: sha256:7dee204d408d3532
language: rust
size_loc: 70
generated: by-keidocs
---

# keidocs/src/dna.rs

DNA hash — sha256 of (path + sorted deps + content), truncated 16 hex chars.

Stable: same inputs → same hash. Sorting deps removes spurious diff noise.

## Public API

- `pub fn compute_dna` — Compute a deterministic content-addressable id for a source file.

## Related

- parent: `keidocs/Cargo.toml`
- imports: sha2

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
