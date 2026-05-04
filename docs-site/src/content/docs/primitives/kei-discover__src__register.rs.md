---
title: register.rs
path: kei-discover/src/register.rs
dna_hash: sha256:79551f372547161a
language: rust
size_loc: 70
generated: by-keidocs
---

# kei-discover/src/register.rs

`register` — insert one primitive announcement.

Dispatches to `kei_entity_store::verbs::create` with a JSON
payload assembled from the typed arguments. `last_seen_ts` is stamped
to the current Unix timestamp; `installed` defaults to 0.

Duplicate-slug detection: the schema emits a UNIQUE INDEX on `slug`
(see `schema.rs`). A duplicate INSERT surfaces as a SQLite constraint
error which we map back to `DiscoverError::DuplicateSlug(slug)` so
callers get a typed signal with the offending slug included.

## Public API

- Map a `VerbError` to `DiscoverError`, re-attaching the offending slug

## Related

- parent: `kei-discover/Cargo.toml`
- imports: crate, kei_entity_store, rusqlite, serde_json, std

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
