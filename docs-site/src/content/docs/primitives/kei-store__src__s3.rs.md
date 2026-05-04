---
title: s3.rs
path: kei-store/src/s3.rs
dna_hash: sha256:4310607cad47638d
language: rust
size_loc: 171
generated: by-keidocs
---

# kei-store/src/s3.rs

S3Store — object-storage backend (MVP stub; v0.14.1 local-only).

This is a local-manifest-based implementation intended as an offline MVP.
Reads/writes go to `cache_path`; `commit` serialises a
`manifest-<hash>.json` listing the current file tree + content hash;
`push`/`pull` are NO-OPs in stub mode.

v0.14.1: because the backend does NOT actually reach S3, the factory
now refuses to build an `S3Store` unless `KEI_STORE_ALLOW_S3_STUB=1`
is set. Previously users who configured S3 were silently writing to a
local cache with no remote push. See `factory.rs` for the guard.

v0.14.1 hardening: `full()` rejects absolute paths and `..` components
(same CVE class as `filesystem.rs` — user-supplied `rel` could escape
the cache root).

Production S3/R2/MinIO support is planned via `aws-sdk-s3` behind a
feature flag — see README §Store backends. This stub keeps the trait
surface honest so downstream code can exercise the full kei-store
API without pulling a ~20 MB AWS SDK at install time.

## Related

- parent: `kei-store/Cargo.toml`
- imports: anyhow, crate, std

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
