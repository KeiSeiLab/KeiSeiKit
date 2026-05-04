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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
