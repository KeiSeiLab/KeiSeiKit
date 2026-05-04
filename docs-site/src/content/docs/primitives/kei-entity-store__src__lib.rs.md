---
title: lib.rs
path: kei-entity-store/src/lib.rs
dna_hash: sha256:b53fe59e4ae55cc2
language: rust
size_loc: 28
generated: by-keidocs
---

# kei-entity-store/src/lib.rs

kei-entity-store — Layer A verb-template engine.

Provides a schema-driven store that 6 sibling kei-*-store crates can
plug into instead of hand-rolling their own `Store::open` + CRUD
helpers. An `EntitySchema` declaratively describes one entity table
(fields, FTS columns, edge table, enabled verbs); verb modules
(`create`, `get`, `list`, `search`, `update`, `delete`, `link`,
`rank`) consume the schema and run parameterized SQL.

Pilot target: `kei-task` (see its `schema.rs` for an example usage).
Follow-up waves: kei-chat-store, kei-content-store, kei-social-store,
kei-sage, kei-crossdomain.

Per substrate schema v1 this crate stays library-only — no CLI, no
`bin`. Each sibling crate remains the user-facing binary.

## Related

- parent: `kei-entity-store/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
