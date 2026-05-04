---
title: keys.rs
path: kei-store/src/s3_cloud/keys.rs
dna_hash: sha256:752ef700509564ce
language: rust
size_loc: 40
generated: by-keidocs
---

# kei-store/src/s3_cloud/keys.rs

Key-path helpers for the S3 cloud backend.

v0.22 Track B: the actual logic moved one level up into
`crate::async_backend` (shared by every future cloud backend — GCS,
Azure Blob, Bunny, etc.). This module now re-exports the helpers for
backward source-compat and keeps the unit tests green.

## Related

- parent: `kei-store/Cargo.toml`

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
