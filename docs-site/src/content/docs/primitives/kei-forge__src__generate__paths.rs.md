---
title: paths.rs
path: kei-forge/src/generate/paths.rs
dna_hash: sha256:4644d3d68382688a
language: rust
size_loc: 174
generated: by-keidocs
---

# kei-forge/src/generate/paths.rs

Target-path resolution for atom scaffolding.

Given the repo root and a `ForgeRequest`, compute the five absolute
paths the generator will write, and the five relative template paths
it will read from. Decouples path arithmetic from I/O so tests can
assert directly on layout.

## Public API

- `pub fn resolve` — Build the five destination paths for `req` under `repo_root`.
- `pub fn pairs` — Return `(template-rel-path, absolute-dest-path)` pairs in the same
- `pub fn assert_none_exist` — Refuse to overwrite: error on the first extant target.
- `pub fn ensure_parent_dirs` — Create `atoms/`, `atoms/schemas/`, `src/atoms/`, `tests/` under

## Related

- parent: `kei-forge/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
