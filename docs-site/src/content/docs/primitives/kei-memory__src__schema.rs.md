---
title: schema.rs
path: kei-memory/src/schema.rs
dna_hash: sha256:d2cec70682d4c2b7
language: rust
size_loc: 95
generated: by-keidocs
---

# kei-memory/src/schema.rs

SQL schema for the kei-memory offline analyzer.

Constructor Pattern: schema + migration runner, no business logic.
DB default path: `~/.claude/memory/kei-memory.sqlite`.
Any structural change MUST append a new migration; never edit history.

## Public API

- `pub const MIGRATIONS` — Ordered migrations. Index = schema version. Never reorder.
- `pub fn migrate` — Apply all pending migrations. Stores version in `PRAGMA user_version`.

## Related

- parent: `kei-memory/Cargo.toml`
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
