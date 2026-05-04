---
title: module_source.rs
path: kei-import-project/src/module_source.rs
dna_hash: sha256:0c5a749a20043a3e
language: rust
size_loc: 106
generated: by-keidocs
---

# kei-import-project/src/module_source.rs

module_source — lightweight source-file bundle consumed by the trait matcher.

Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.
A1.2 will adapt RepoWalk → ModuleSource; until then this is standalone.

## Public API

- `pub struct ModuleSource` — A named module with its pre-loaded Rust source files.
- `pub fn from_content` — Build from in-memory content — used in unit tests.
- `pub fn from_dir` — Walk `dir`, read every `.rs` file, return a `ModuleSource`.
- Recursively collect all `.rs` files under `dir`.

## Related

- parent: `kei-import-project/Cargo.toml`
- imports: std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
