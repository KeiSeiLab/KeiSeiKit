---
title: row.rs
path: kei-projects-index/src/row.rs
dna_hash: sha256:736b038fcc1aadcd
language: rust
size_loc: 29
generated: by-keidocs
---

# kei-projects-index/src/row.rs

`ProjectRow` — value type mirroring one row of the `projects` table.

Constructor Pattern: one cube = one struct + its serde derive. Kept
separate from `index.rs` so the orchestrator stays under the 120-LOC
cap and the schema's row shape lives in a single, easily-diffable cube.

## Public API

- One row of the `projects` table. Mirrors the SQL schema verbatim.

## Related

- parent: `kei-projects-index/Cargo.toml`
- imports: serde

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
