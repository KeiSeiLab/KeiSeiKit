---
title: ddl.rs
path: kei-entity-store/src/ddl.rs
dna_hash: sha256:51392ca136457eb3
language: rust
size_loc: 91
generated: by-keidocs
---

# kei-entity-store/src/ddl.rs

DDL-string generators split out of `engine.rs` to keep that file
under the Constructor-Pattern 200-LOC cap. One function per emitted
`CREATE` statement; the engine's `run_migrations` orchestrates the
calls and stamps `user_version`.

Edge-table DDL lives in `ddl_edge.rs` and is re-exported below;
`DdlError` lives in `ddl_error.rs`. Split preserves the 200-LOC cap
per Constructor Pattern.

## Public API

- Deterministic SQL literal for an f64 — always has a decimal point,

## Related

- parent: `kei-entity-store/Cargo.toml`
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
