---
title: phase-4-migrations
path: schema-design/phase-4-migrations.md
dna_hash: sha256:81d335fcc42ce786
language: markdown
size_loc: 121
generated: by-keidocs
---

# schema-design/phase-4-migrations.md

## Public API

- `Phase 4 — Migration scaffold + first migration + kei-migrate wiring` — Package `db/schema.sql` (from Phase 3) into a proper
- `4a — Create `migrations/` directory (no AskUserQuestion)` — If `migrations/` does not yet exist in the repo, create it. Emit one
- `4b — Generate timestamp + filename` — - Timestamp format: `YYYYMMDDHHMMSS` (matches `kei-migrate create`'s
- `4c — Up migration content` — Copy `db/schema.sql` contents into the up file verbatim, with a one-line
- `4d — Down migration content` — Emit `DROP TABLE IF EXISTS <name> CASCADE;` for every entity, in REVERSE
- `4e — Wire kei-migrate (AskUserQuestion)` — ```json
- `4f — Emit the next-step command (inline, no AskUserQuestion)` — Print a fenced code block tailored to `DB` + `RUNNER`:
- `Load DB URL from SSoT (RULE 0.8)` — set -a && source secrets/db.env && set +a
- `Preview pending migrations` — kei-migrate --database-url "$DATABASE_URL" --dir migrations status
- `Apply` — kei-migrate --database-url "$DATABASE_URL" --dir migrations up
- `Revert the latest (dev only!)` — kei-migrate --database-url "$DATABASE_URL" --dir migrations down 1
- `secrets/db.env — chmod 600 before first write` — DATABASE_URL=
- `Verify-criterion` — - `migrations/<ts>_init_schema.sql` exists and equals `db/schema.sql` body

## Related

- parent: `schema-design`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
