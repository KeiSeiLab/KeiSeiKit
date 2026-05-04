---
title: backend.rs
path: kei-memory-postgres/src/backend.rs
dna_hash: sha256:ea7ced4c8c134e84
language: rust
size_loc: 170
generated: by-keidocs
---

# kei-memory-postgres/src/backend.rs

`MemoryBackend` impl over `PgStore`. One backend = one DNA. Many
backends can share the same `Arc<PgStore>`.

## Public API

- `pub fn new` — Build with a fresh DNA. `body` defaults to `b"pg-v16"` to

## Related

- parent: `kei-memory-postgres/Cargo.toml`
- imports: crate, kei-sleep-sync.sh, kei_runtime_core, std, tokio_postgres

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
