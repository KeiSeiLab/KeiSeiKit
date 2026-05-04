---
title: skeleton.rs
path: kei-import-project/src/skeleton.rs
dna_hash: sha256:4b6f0070bfbd24a7
language: rust
size_loc: 133
generated: by-keidocs
---

# kei-import-project/src/skeleton.rs

skeleton — generate Rust impl-skeleton files from a TraitKind.

Public entry-point: `render_skeleton`. Static trait metadata lives in
`skeleton_table` to keep this file ≤200 LOC.

Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

## Public API

- `pub fn render_skeleton` — Generate a Rust impl-skeleton for `module_name` implementing `target_trait`.
- `pub fn module_name_to_type` — Convert kebab-case module name to PascalCase with `Foreign` prefix.

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
