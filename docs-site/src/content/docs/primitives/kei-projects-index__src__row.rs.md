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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
