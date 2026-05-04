---
title: tests.rs
path: kei-store/src/s3_cloud/tests.rs
dna_hash: sha256:7e7842fd8d966bed
language: rust
size_loc: 306
generated: by-keidocs
---

# kei-store/src/s3_cloud/tests.rs

Unit tests for S3CloudStore — no network, mock-endpoint only.

These tests verify builder correctness + path-safety guards + SSRF /
IMDS-leak endpoint validation + explicit-credential wiring. They do
NOT exercise real S3 round-trips (that would require live AWS/MinIO
and would fail in CI without credentials). See `tests/s3_smoke.rs`
for the cross-crate smoke integration.

## Public API

- Set up the env for a local-mock build: allow-internal + allow-insecure,
- Compile-time assertion that `list_recursive` exists on the underlying
- Previously, each `S3CloudStore` built its own `current_thread` tokio
- The shared runtime is multi-thread (needed for the N=2-Store fix).

## Related

- parent: `kei-store/Cargo.toml`
- imports: crate

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
