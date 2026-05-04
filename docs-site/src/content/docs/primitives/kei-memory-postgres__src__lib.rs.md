---
title: lib.rs
path: kei-memory-postgres/src/lib.rs
dna_hash: sha256:aa9e7e17da988cd5
language: rust
size_loc: 31
generated: by-keidocs
---

# kei-memory-postgres/src/lib.rs

kei-memory-postgres — `MemoryBackend` impl over PostgreSQL via
`tokio-postgres`.

Suitable for:
- shared per-fleet memory store (multi-process, multi-host)
- JSONB payloads with GIN indexability on tags
- production durability + WAL replication

Out of scope:
- migrations beyond the single `apply_schema` idempotent bootstrap
(use a dedicated migration tool for richer schema evolution)
- `mirror_to_remote` returns `Provider` — git push is the
responsibility of `kei-sleep-sync.sh` per RULE 0.15.

Why tokio-postgres instead of sqlx: schema is small (one table,
two indexes), no compile-time query macros needed, fewer transitive
deps. Keeps the primitive tight.

## Related

- parent: `kei-memory-postgres/Cargo.toml`

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
