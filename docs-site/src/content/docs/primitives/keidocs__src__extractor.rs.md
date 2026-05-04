---
title: extractor.rs
path: keidocs/src/extractor.rs
dna_hash: sha256:d1b42313d42ffdad
language: rust
size_loc: 126
generated: by-keidocs
---

# keidocs/src/extractor.rs

Language-aware documentation extractors.

Each extractor returns a flat `Vec<Section>`; the markdown emitter is
responsible for grouping sections by `kind`.

## Public API

- `pub fn extract_rustdoc` — Parse rustdoc — module-level `//!` lines and item-level `///` blocks.
- `pub fn extract_jsdoc` — Parse jsdoc-style `/** ... */` blocks. Returns one Section per block.
- `pub fn extract_md_headers` — Treat `# ` and `## ` markdown headers + leading paragraph as sections.

## Related

- parent: `keidocs/Cargo.toml`
- imports: regex, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
