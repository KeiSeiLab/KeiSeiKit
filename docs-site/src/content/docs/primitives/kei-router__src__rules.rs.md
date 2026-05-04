---
title: rules.rs
path: kei-router/src/rules.rs
dna_hash: sha256:3993c0c8fc9a5cd3
language: rust
size_loc: 35
generated: by-keidocs
---

# kei-router/src/rules.rs

Keyword rule type + `require` predicate model.

## Public API

- A dispatch rule: any matching keyword routes to `tool` if `require(extracted)` is true.
- A dynamic (runtime-added) rule — owned strings so caller can build at startup.

## Related

- parent: `kei-router/Cargo.toml`
- imports: crate

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
