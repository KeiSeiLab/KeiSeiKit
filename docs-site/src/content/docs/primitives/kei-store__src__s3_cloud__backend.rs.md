---
title: backend.rs
path: kei-store/src/s3_cloud/backend.rs
dna_hash: sha256:0be64ef06d67eb90
language: rust
size_loc: 120
generated: by-keidocs
---

# kei-store/src/s3_cloud/backend.rs

S3AsyncBackend — `AsyncBackend` impl over `aws-sdk-s3::Client`.

v0.22 Track B. Holds only the S3-specific pieces: the `aws-sdk-s3`
client, the bucket name, and the ListObjectsV2 paginator. Branch-prefix
+ path-safety + commit-manifest semantics live in
`crate::async_backend::AsyncBackendStore<S3AsyncBackend>`.

## Public API

- Build the backend. Requires `cfg.bucket` to be set.
- Shared ListObjectsV2 paginator. `delim_slash=true` → delimiter="/"

## Related

- parent: `kei-store/Cargo.toml`
- imports: anyhow, async_trait, aws_sdk_s3, crate

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
