---
title: block.rs
path: kei-registry/src/block.rs
dna_hash: sha256:5596385cbd88ff73
language: rust
size_loc: 90
generated: by-keidocs
---

# kei-registry/src/block.rs

Block — registry record for one kit artefact.

Constructor Pattern: this cube owns the data shape only. SQL persistence
lives in `store.rs`; query helpers live in `registry.rs`. Block is what
flows over the JSON CLI surface.

## Public API

- Five recognised block types. Order is the canonical scan order and the
- `pub fn as_str` — Stable wire-format string. Used as the DNA `role` segment.
- `pub fn all` — All five recognised types in canonical scan order.
- Block — single registry record. Mirrors the SQLite `blocks` row plus
- `pub fn is_active` — True if no successor row points at this block.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: serde, std

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
