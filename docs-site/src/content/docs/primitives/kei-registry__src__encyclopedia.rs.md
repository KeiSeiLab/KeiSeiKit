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
