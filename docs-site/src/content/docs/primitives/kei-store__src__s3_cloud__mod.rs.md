---
title: mod.rs
path: kei-store/src/s3_cloud/mod.rs
dna_hash: sha256:a50fb7e8a55a4716
language: rust
size_loc: 43
generated: by-keidocs
---

# kei-store/src/s3_cloud/mod.rs

S3CloudStore — real object-storage backend via `aws-sdk-s3`.

v0.22 Track B refactor: this module now contains ONLY the S3-specific
construction and re-exports. The sync-over-async runtime bridge,
branch-prefix handling, path validation, and commit-manifest semantics
all live in `crate::async_backend::AsyncBackendStore`, which is a
generic wrapper over any `AsyncBackend` impl.

Extension seam: to add a new cloud backend (GCS, Azure Blob, Bunny),

1. Create `src/gcs_cloud/backend.rs` with a struct that impls
`crate::async_backend::AsyncBackend` (4 async methods + `label`).
2. Add `pub type GcsCloudStore = AsyncBackendStore<GcsAsyncBackend>;`.
3. Wire a constructor that calls `AsyncBackendStore::wrap(backend)`.
4. `factory::build_store` dispatches on `cfg.active.backend`.

The shared tokio runtime + MemoryStore impl are free.

## Public API

- `pub type S3CloudStore` — Public API: unchanged from v0.21 — `S3CloudStore::new(cfg)` still works.
- `pub fn new` — Build a cloud-S3 backend. `bucket` MUST be configured.

## Related

- parent: `kei-store/Cargo.toml`
- imports: anyhow, crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
