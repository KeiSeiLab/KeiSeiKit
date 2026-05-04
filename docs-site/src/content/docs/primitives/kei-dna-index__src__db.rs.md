---
title: db.rs
path: kei-dna-index/src/db.rs
dna_hash: sha256:f2ca7e2a6bbf7619
language: rust
size_loc: 63
generated: by-keidocs
---

# kei-dna-index/src/db.rs

Read-only SQLite access to the kei-ledger agents table.

Constructor Pattern: one file = one responsibility (DB row loading).

## Public API

- One row of the `agents` table, with its DNA already parsed.
- `pub fn open_read_only` — Open ledger in read-only mode. No schema mutation.
- `pub fn load_rows` — Load all rows with non-null DNA. Malformed DNAs are skipped silently.
- `pub fn find_target` — Find the row matching a given DNA string exactly.

## Related

- parent: `kei-dna-index/Cargo.toml`
- imports: crate, rusqlite, std

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
