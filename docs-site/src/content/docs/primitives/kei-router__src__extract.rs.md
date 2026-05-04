---
title: extract.rs
path: kei-router/src/extract.rs
dna_hash: sha256:0936d26eb5acc1f5
language: rust
size_loc: 167
generated: by-keidocs
---

# kei-router/src/extract.rs

Param extraction — regex scans the raw query for path / limit / id / URI / KV.

Ported from LBM pkg/keirouter/extract.go.

## Public API

- `pub fn extract_params` — Parse a raw NL query into structured [`Extracted`] params.

## Related

- parent: `kei-router/Cargo.toml`
- imports: regex, std

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
