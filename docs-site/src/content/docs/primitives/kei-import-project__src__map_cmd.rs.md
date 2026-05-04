---
title: map_cmd.rs
path: kei-import-project/src/map_cmd.rs
dna_hash: sha256:ac6de2b468139340
language: rust
size_loc: 100
generated: by-keidocs
---

# kei-import-project/src/map_cmd.rs

map_cmd — build an architecture map of a repo by running the matcher per module.

Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

## Public API

- One row in the architecture map.
- `pub fn render_markdown` — Render entries as a markdown table.
- `pub fn render_json` — Render entries as a JSON array.

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: anyhow, crate, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
