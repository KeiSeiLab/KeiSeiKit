---
title: path_resolve.rs
path: kei-arch-map/src/evidence/path_resolve.rs
dna_hash: sha256:26e98a6bc41f4eb2
language: rust
size_loc: 80
generated: by-keidocs
---

# kei-arch-map/src/evidence/path_resolve.rs

## Public API

- `pub fn repo_root` — Resolve repo root: parent of PLAN.toml's parent dir.
- `pub fn resolve` — Resolve a claim-relative path against `root`. Absolute paths pass through.
- `pub fn resolve_confined` — Resolve `input` relative to `root` AND verify it stays within the
- `pub fn confine_out` — Confine `out` such that its canonicalized parent stays within `root`.

## Related

- parent: `kei-arch-map/Cargo.toml`
- imports: anyhow, std

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
