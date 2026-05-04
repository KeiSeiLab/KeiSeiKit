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
