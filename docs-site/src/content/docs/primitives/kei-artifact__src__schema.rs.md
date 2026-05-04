---
title: schema.rs
path: kei-artifact/src/schema.rs
dna_hash: sha256:907bf0030be9f738
language: rust
size_loc: 50
generated: by-keidocs
---

# kei-artifact/src/schema.rs

SQL schema DDL + migrations for the artifact store.

Two tables:
- `schemas`   — registered JSON Schemas by name (SSoT for validation).
- `artifacts` — typed content + metadata + parent pointer for handoff chain.

## Public API

- `pub const MIGRATIONS` — Ordered migrations. Index = schema version. Append only; never reorder.
- `pub fn migrate` — Apply pending migrations. Uses pragma `user_version` as the version cursor.
- `pub const KNOWN_SCHEMAS` — Canonical list of artifact schema names shipped with this primitive.

## Related

- parent: `kei-artifact/Cargo.toml`
- imports: rusqlite

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
