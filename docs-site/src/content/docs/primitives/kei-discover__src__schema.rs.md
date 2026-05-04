---
title: schema.rs
path: kei-discover/src/schema.rs
dna_hash: sha256:dd6ba6f329aa804b
language: rust
size_loc: 39
generated: by-keidocs
---

# kei-discover/src/schema.rs

`discover_index` EntitySchema — one row per announced primitive.

Fields follow task.toml spec: `slug` (unique indexed), `author`,
`source_url`, `description`, `installed` (0/1 stored as INTEGER —
SQLite has no native bool), `last_seen_ts`, `created_at`,
`updated_at`. A UNIQUE INDEX on `slug` is emitted via
`custom_migrations` so duplicate registrations fail at the SQL layer
(mapped to `DiscoverError::DuplicateSlug` by the `register` module).

FTS columns are `slug` + `description` — callers search by either.

## Related

- parent: `kei-discover/Cargo.toml`
- imports: kei_entity_store

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
