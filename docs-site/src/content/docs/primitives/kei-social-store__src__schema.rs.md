---
title: schema.rs
path: kei-social-store/src/schema.rs
dna_hash: sha256:0ad3fc143f8b0245
language: rust
size_loc: 84
generated: by-keidocs
---

# kei-social-store/src/schema.rs

kei-social-store EntitySchemas — Layer A convergence.

Shape (multi-schema audit, 2026-04-23):

- `SOCIAL_SCHEMA`: primary entity `person` (table `people`; INTEGER
PK; engine-owned create/get/search/list + FTS).
- `ALL_SCHEMAS`: the `&[&EntitySchema]` slice for `Store::open`.

Secondary tables stay in `custom_migrations` and keep bespoke SQL
paths — **none were promotable** on this pass:

- `organizations` — uses `INSERT OR IGNORE` + re-query by
`UNIQUE(name)` for idempotent name-keyed upsert (`orgs_idempotent`
test relies on `add_org` returning the SAME id for repeat names).
The engine `create` verb is plain INSERT, not OR-IGNORE, and
would break that semantic. Sibling: `people.rs::add_org`.
- `interactions` — append-only log with `FOREIGN KEY ... ON DELETE
CASCADE` on `person_id`, filter query `WHERE person_id=?`, and
aggregate `GROUP BY person_id, target_id, channel` used by
`graph.rs::relationship_graph`. None of these are covered by
the engine's generic verbs. Sibling: `interactions.rs` + `graph.rs`.

FTS columns cover name, handle, email, bio — search verb routes
through `fts_people`. The legacy `fts_social` virtual table is
replaced; FTS is rebuilt on first open against the new name.

## Public API

- Aggregate slice for `Store::open`. Currently a single-element slice;

## Related

- parent: `kei-social-store/Cargo.toml`
- imports: kei_entity_store

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
