---
title: block_md.rs
path: kei-registry/src/scanners/block_md.rs
dna_hash: sha256:62fb654ddf11a35a
language: rust
size_loc: 83
generated: by-keidocs
---

# kei-registry/src/scanners/block_md.rs

Block scanner — walks `<kit-root>/_blocks/*.md`.

Constructor Pattern: this cube knows the flat `_blocks/` directory
convention. Body bytes = raw markdown; name = filename stem or H1;
maps to BlockType::Atom (atomic prompt fragment); caps = empty.

## Public API

- `pub struct BlockMdScanner` — `<kit-root>/_blocks/<name>.md` adapter.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, std

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
