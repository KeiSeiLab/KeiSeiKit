---
title: archive.rs
path: kei-entity-store/src/verbs/archive.rs
dna_hash: sha256:139d0117b88328cd
language: rust
size_loc: 138
generated: by-keidocs
---

# kei-entity-store/src/verbs/archive.rs

`archive` verb — soft-delete. If the configured `archived_field`
column has kind `TextArchiveEnum`, writes the column's
`archived` sentinel string; otherwise flips an INTEGER column to 1.
A sibling `<archived_field>_at` column is stamped with the current
Unix timestamp when present.

Required schema configuration: `archived_field: Some("<col>")`.
Without it the verb errors with `InvalidInput` — the engine does NOT
fall back to legacy `archived` heuristics (those remain in
`delete.rs` soft-path only).

FTS semantics: archiving means "hidden from active listing". The verb
therefore DELETEs the row from `fts_<table>` inside the same
transaction as the UPDATE, so `search` will no longer return the
archived row. If a future caller flips the column back to active
(unarchive), it MUST reinsert the row into the FTS index — the
current contract does not auto-reindex on unarchive. "Keep
searchable while archived" is a future feature (`search
--include-archived`), NOT today.

Input: `{ id: <int|string> }`.
Output: `{ id, archived_at }` — `archived_at` is the stamped
timestamp when a `<field>_at` column exists, else `null`.

## Public API

- UPDATE the archived column (+ stamp) and DELETE the row from FTS (if
- Pick the SQL value written to the archived column. `TextArchiveEnum`

## Related

- parent: `kei-entity-store/Cargo.toml`
- imports: crate, rusqlite, serde_json

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
