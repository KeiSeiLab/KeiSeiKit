---
title: skeleton_table.rs
path: kei-import-project/src/skeleton_table.rs
dna_hash: sha256:b89def1bf28bc38d
language: rust
size_loc: 171
generated: by-keidocs
---

# kei-import-project/src/skeleton_table.rs

skeleton_table — static method-signature table for skeleton generation.

One `TraitMeta` entry per kei-runtime-core trait, with verbatim fn
signatures copied from the actual trait files.

Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

## Public API

- `pub struct MethodEntry` — One method entry: name, full fn signature opening brace, and a
- `pub struct TraitMeta` — Metadata for one runtime trait.
- `pub fn trait_meta` — Return the static trait metadata for `kind`.

## Related

- parent: `kei-import-project/Cargo.toml`
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
