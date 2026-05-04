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
