---
title: async_backend.rs
path: kei-store/src/async_backend.rs
dna_hash: sha256:f4b4f5a4741b47fe
language: rust
size_loc: 189
generated: by-keidocs
---

# kei-store/src/async_backend.rs

AsyncBackend — cloud-storage sub-trait + sync-over-async adapter.

v0.22 Track B. Adding a new cloud backend (GCS, Azure, Bunny, …) =
implement 4 async methods on `AsyncBackend` + `label()`. Runtime glue +
branch-prefix + path-validation + commit-manifest are all free.

Multi-thread shared runtime (2 workers, IO + time) avoids the N=2-Store
footgun of the previous per-instance `current_thread` runtimes — two
`AsyncBackendStore` instances in one process no longer risk `block_on`
panics when one instance's call runs on the other's runtime thread.

## Public API

- Process-global multi-thread tokio runtime.
- `pub fn validate_rel` — Reject absolute paths and `..` components. Keeps branch prefix
- `pub fn short_hash` — Tiny DJB2 — deterministic, avoids pulling sha2 just for manifest names.
- `pub fn is_manifest_key` — `manifest-<hex>.json` — format produced by `commit()` below.
- Cloud-storage backend trait. Impls deal with keys only.
- Single-level list — keys directly under `prefix`, no recursion.
- Full recursive list under `prefix`.
- Backend-specific label used by `MemoryStore::backend_name`.
- `pub struct AsyncBackendStore` — Sync wrapper: `MemoryStore` on top of any `AsyncBackend`.
- `pub fn wrap` — Wrap an already-constructed backend. Renamed from `new` to avoid a

## Related

- parent: `kei-store/Cargo.toml`
- imports: anyhow, async_trait, crate, std, tokio

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
