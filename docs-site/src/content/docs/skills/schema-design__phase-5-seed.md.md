---
title: phase-5-seed
path: schema-design/phase-5-seed.md
dna_hash: sha256:666718a120664358
language: markdown
size_loc: 92
generated: by-keidocs
---

# schema-design/phase-5-seed.md

## Public API

- `Phase 5 — Optional seed data + test fixtures` — Emit `db/seed.sql` with deterministic, safe-to-re-run seed rows, OR skip
- `5a — Seed decision (AskUserQuestion)` — ```json
- `5b — Generate `db/seed.sql` (inline, no AskUserQuestion)` — Rules, regardless of choice (unless Skip):
- `5c — Test-First hook (inline)` — If `SEED ≠ Skip`, emit a smoke-test snippet tailored to `DB`:
- `Smoke-test: load schema + seed, assert row counts.` — kei-migrate --database-url "$DATABASE_URL" --dir migrations up
- `Verify-criterion` — - If `SEED = Skip`: `db/seed.sql` is NOT created; state records "seed

## Related

- parent: `schema-design`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
