---
title: path_safety.rs
path: kei-atom-discovery/src/path_safety.rs
dna_hash: sha256:100544177c279f2e
language: rust
size_loc: 81
generated: by-keidocs
---

# kei-atom-discovery/src/path_safety.rs

Path-traversal-safe base+rel join.

`safe_join` is the authoritative base+rel path-join: rejects absolute
components and `..`, canonicalises, asserts base containment (including
post-canonicalise symlink escapes).

## Public API

- `pub fn safe_join` — Safe base+rel path join.

## Related

- parent: `kei-atom-discovery/Cargo.toml`
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
