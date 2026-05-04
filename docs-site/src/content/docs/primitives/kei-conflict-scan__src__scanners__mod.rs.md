---
title: mod.rs
path: kei-conflict-scan/src/scanners/mod.rs
dna_hash: sha256:a572c3aa015f7677
language: rust
size_loc: 10
generated: by-keidocs
---

# kei-conflict-scan/src/scanners/mod.rs

Per-category conflict scanners.

Each sub-module exposes `fn scan(root: &Path) -> Vec<Conflict>`.
The CLI in `main.rs` calls them based on `--only` or runs all.

## Related

- parent: `kei-conflict-scan/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
