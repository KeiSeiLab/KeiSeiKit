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
