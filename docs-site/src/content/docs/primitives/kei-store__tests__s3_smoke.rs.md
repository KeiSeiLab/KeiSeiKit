---
title: s3_smoke.rs
path: kei-store/tests/s3_smoke.rs
dna_hash: sha256:1d4aa101e5740757
language: rust
size_loc: 143
generated: by-keidocs
---

# kei-store/tests/s3_smoke.rs

Smoke tests for the S3 cloud backend (behind `s3` feature).

These tests never hit real AWS. They verify:
- the `S3CloudStore` builder accepts a mock endpoint without panic
- the library re-exports `s3_cloud` when the feature is enabled
- path-safety guards reject traversal attempts

Run with: `cargo test -p kei-store --features s3 --test s3_smoke`.
Without the feature, this file compiles to an empty crate — harmless.

v0.21.1: builder now rejects loopback endpoints + plain HTTP unless the
caller opts in via `KEI_STORE_S3_ALLOW_INTERNAL=1` +
`KEI_STORE_S3_ALLOW_INSECURE=1`, and requires explicit `access_key_env`
/ `secret_key_env` whenever a custom endpoint is set (H2 SSRF guard).
Each test sets both env vars + mock creds under the shared `env_lock`
so `cargo test` parallelism can't race on the process env.

## Related

- parent: `kei-store/tests`
- imports: kei_store

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
