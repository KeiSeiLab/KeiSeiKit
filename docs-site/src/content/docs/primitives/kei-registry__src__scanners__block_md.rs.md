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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
