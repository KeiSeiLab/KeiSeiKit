---
title: people.rs
path: kei-social-store/src/people.rs
dna_hash: sha256:6e62f911eb968bc0
language: rust
size_loc: 86
generated: by-keidocs
---

# kei-social-store/src/people.rs

People + organizations.

`add_person` / `get_person` delegate to `kei_entity_store::verbs::*`
under `SOCIAL_SCHEMA`. Organizations live in a `custom_migrations`
table (name-keyed upsert semantics, not generic CRUD) and keep their
bespoke SQL path.

## Related

- parent: `kei-social-store/Cargo.toml`
- imports: anyhow, chrono, crate, kei_entity_store, rusqlite, serde, serde_json

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
