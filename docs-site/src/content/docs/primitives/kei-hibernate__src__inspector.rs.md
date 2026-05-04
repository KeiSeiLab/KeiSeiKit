---
title: inspector.rs
path: kei-hibernate/src/inspector.rs
dna_hash: sha256:513c935a3afb0af5
language: rust
size_loc: 42
generated: by-keidocs
---

# kei-hibernate/src/inspector.rs

Inspector — read-only preview of bundle contents.

Streams the archive, counts non-manifest entries, and returns
paths from the manifest (pre-computed, order-preserving). No
extraction, no side effects.

## Public API

- `pub fn inspect` — List bundle contents without extracting. Rejects missing manifest

## Related

- parent: `kei-hibernate/Cargo.toml`
- imports: crate, std

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
