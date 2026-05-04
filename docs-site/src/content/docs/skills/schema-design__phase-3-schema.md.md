---
title: phase-3-schema
path: schema-design/phase-3-schema.md
dna_hash: sha256:a8c1bd3878e8c806
language: markdown
size_loc: 115
generated: by-keidocs
---

# schema-design/phase-3-schema.md

## Public API

- `Phase 3 — Generate SQL DDL (tables, indexes, FKs, constraints)` — Emit a full `db/schema.sql` file based on `DB`, `ORM`, `STYLE`, and
- `3a — Pick primary-key strategy (inline, no AskUserQuestion — deterministic)` — - Postgres 17: `id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY`
- `3b — Per-entity DDL generation rules` — For each entity in `ENTITIES`:
- `3c — Foreign keys` — For each `relations` entry (kind ≠ `None`):
- `3d — Indexes + constraints` — - Unique: `email`, `slug`, `username`, any field user marked `UNIQUE` →
- `3e — Emit the file` — Write `db/schema.sql` with a top-level comment:
- `3f — Review click (AskUserQuestion)` — ```json
- `Verify-criterion` — - `db/schema.sql` exists on disk and is non-empty.

## Related

- parent: `schema-design`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
