---
title: scan.rs
path: kei-gdrive-import/src/scan.rs
dna_hash: sha256:2e78e45d5ccc79ad
language: rust
size_loc: 78
generated: by-keidocs
---

# kei-gdrive-import/src/scan.rs

Walk-tree scanner.

Two backends:
* local FS (`std::fs::read_dir`, no `walkdir` dep)
* remote rclone (shell out to `rclone lsjson <remote> --dirs-only`)

Depth: one level under root. The wizard recurses by re-invoking
`scan-tree` on subfolders the user marks AMBIGUOUS — keeps the
primitive flat and predictable.

## Related

- parent: `kei-gdrive-import/Cargo.toml`
- imports: anyhow, crate, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
