---
title: ddl_error.rs
path: kei-entity-store/src/ddl_error.rs
dna_hash: sha256:63d480e9f7ffc718
language: rust
size_loc: 25
generated: by-keidocs
---

# kei-entity-store/src/ddl_error.rs

`DdlError` — typed DDL-generation failures surfaced by the fallible
edge-table dispatcher in `ddl::try_edge_table_for`.

Split out of `ddl.rs` to keep each file inside the Constructor
Pattern 200-LOC cap (1 file = 1 responsibility). `ddl.rs` owns DDL
string emission; this module owns the error type only.

## Public API

- Typed DDL-generation failure. Surfaces caller-input problems (e.g.
- Caller passed a `FieldKind` that edge-column DDL cannot emit

## Related

- parent: `kei-entity-store/Cargo.toml`
- imports: thiserror

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
