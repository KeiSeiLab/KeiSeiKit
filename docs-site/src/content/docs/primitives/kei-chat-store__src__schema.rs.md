---
title: schema.rs
path: kei-chat-store/src/schema.rs
dna_hash: sha256:591800abd1c3199b
language: rust
size_loc: 89
generated: by-keidocs
---

# kei-chat-store/src/schema.rs

kei-chat-store EntitySchemas — declarative specs consumed by
`kei_entity_store::Store` and its verb templates.

Shape (multi-schema convergence, 2026-04-23):

- `MESSAGES_SCHEMA`: primary entity `chat_messages` (INTEGER PK;
engine-owned create/get/list/search + FTS reindex).
- `SESSIONS_SCHEMA`: second entity `chat_sessions` (TEXT UUID PK +
`TextArchiveEnum` status column, engine-owned create/get/archive).
Previously rode `custom_migrations`; now a first-class schema
since `Store::open` accepts a slice of schemas.
- `ALL_SCHEMAS`: the `&[&EntitySchema]` slice the `Store` wrapper
hands to the engine on open.

The session aggregates (`message_count`, `total_tokens`, `total_cost`)
are still updated via bespoke SQL in `sessions.rs` because the
engine has no `increment-on-related-insert` verb. That bespoke path
shrank from "whole row lifecycle" to "UPDATE counters only".

## Public API

- Keep the idx_cm_session index around — generic schema has no
- Legacy indexes on chat_sessions (project, status). `indexed` flag on

## Related

- parent: `kei-chat-store/Cargo.toml`
- imports: kei_entity_store

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
