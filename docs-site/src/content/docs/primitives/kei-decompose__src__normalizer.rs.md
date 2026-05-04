---
title: normalizer.rs
path: kei-decompose/src/normalizer.rs
dna_hash: sha256:bd5cf2377b54791e
language: rust
size_loc: 140
generated: by-keidocs
---

# kei-decompose/src/normalizer.rs

Unified Action struct + severity helpers.

Every parser in the registry yields `Action` regardless of source format.
Downstream (emitter, dispatcher) consumes only `Action`, so adding a new
format means adding a parser — never touching the consumers.

## Public API

- `pub type SourceFormat` — Source-format tag carried alongside each Action.
- Single canonical shape across all formats.

## Related

- parent: `kei-decompose/Cargo.toml`
- imports: serde

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
