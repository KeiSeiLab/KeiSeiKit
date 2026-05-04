---
title: encyclopedia.rs
path: kei-registry/src/encyclopedia.rs
dna_hash: sha256:96bd2fed59435dc3
language: rust
size_loc: 109
generated: by-keidocs
---

# kei-registry/src/encyclopedia.rs

Encyclopedia renderer — public API for markdown and JSON output.

Constructor Pattern: this cube owns wire types + orchestration only.
Section builders live in `encyclopedia_render`; date formatting in
`encyclopedia_time`. No I/O beyond the return value.

## Public API

- One flattened entry for the JSON surface.
- Top-level JSON envelope.
- `pub fn to_entries` — Convert a `Block` slice into sorted `EncyclopediaEntry` values.
- `pub fn render_json` — Render entries as pretty JSON.
- `pub fn render_markdown` — Render active entries as a markdown encyclopedia.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
