---
title: interactions.rs
path: kei-social-store/src/interactions.rs
dna_hash: sha256:10c013e0288bbee3
language: rust
size_loc: 58
generated: by-keidocs
---

# kei-social-store/src/interactions.rs

Interactions — append-only per-person event log.

Stays bespoke (not promoted to engine) because:
- `FOREIGN KEY(person_id) REFERENCES people(id) ON DELETE CASCADE`
is not expressible via `EntitySchema` fields.
- `interactions_for(person_id)` is a filter query by FK column,
not a generic `list` with offset/limit.
- `graph.rs::relationship_graph` runs `GROUP BY person_id,
target_id, channel` which is out of scope for engine verbs.
Table DDL still lives in `SOCIAL_SCHEMA::custom_migrations`.

## Related

- parent: `kei-social-store/Cargo.toml`
- imports: anyhow, chrono, crate, rusqlite, serde

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
