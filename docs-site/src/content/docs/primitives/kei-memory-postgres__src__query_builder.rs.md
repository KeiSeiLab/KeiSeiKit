---
title: query_builder.rs
path: kei-memory-postgres/src/query_builder.rs
dna_hash: sha256:b3401c94b1e181a9
language: rust
size_loc: 111
generated: by-keidocs
---

# kei-memory-postgres/src/query_builder.rs

Dynamic WHERE composition for [`MemoryQuery`]. Returns a parameterised
SQL string + a parallel parameter vector typed for tokio-postgres.

Kept in its own module so `backend.rs` stays under the 200-LOC cube
limit and the SQL composition can be unit-tested in isolation.

## Public API

- `pub struct BuiltQuery` — Output of [`build_select`]: the full statement plus the boxed
- `pub fn build_select` — Compose a SELECT for the given query. `LIMIT` defaults to 1000 when

## Related

- parent: `kei-memory-postgres/Cargo.toml`
- imports: kei_runtime_core, tokio_postgres

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
