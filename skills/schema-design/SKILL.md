---
name: schema-design
description: Hub-and-spoke pipeline that converts "I need a database for app X" into a designed relational schema, a generated first migration, and optional seed/fixture data — via pure-click decisions across five phases. Emits SQL DDL, a kei-migrate-shaped migrations directory, and a library/ORM pick; never writes production secrets.
argument-hint: <one-line app description, e.g. "multi-tenant B2B SaaS, 6-8 entities, Postgres + Drizzle">
---

# Schema-Design — Relational Schema & Migration Pipeline (index)

## When to use

- Designing a relational database schema for a new or evolving app (Postgres, SQLite).
- Generating SQL DDL, migrations directory, and optional seed/fixture data from a click-driven design session.
- Choosing the right ORM, migration tool, and indexing strategy before writing any code.

> See `_blocks/pipeline-5phase-template.md` for the 5-phase wizard contract
> and `_blocks/rule-pure-click-contract.md` for the AskUserQuestion rule.
> Skill-specific phase tables are inline below.

You are converting "I need a database for app X" into a concrete, reviewable
design: chosen DB + ORM, entity list + relations, SQL DDL with indexes and
FKs, a scaffolded migrations directory with the first migration, and (if
asked) seed data for tests and dev. Every decision is a click; the only
typed inputs are the one-line app description in Phase 1 and the entity
list in Phase 2.

This skill does NOT run migrations or touch production. It produces files
under `db/schema.sql`, `migrations/<ts>_init.sql` (+ `.down.sql`), and
optionally `db/seed.sql`. Applying them is a separate command (`kei-migrate
up`), owned by the project's code-implementer.

The skill reads the five database blocks heavily — every phase references
at least one of them:

- `_blocks/db-postgres.md` — PG 17 patterns, indexing, pooling.
- `_blocks/db-sqlite.md` — single-node / edge pragmas.
- `_blocks/db-sqlx.md` — Rust query + migration flow.
- `_blocks/db-drizzle.md` — TS schema-first ORM.
- `_blocks/db-migration-hygiene.md` — universal up/down + checksum rules.

Primitive used for scaffolding: `_primitives/_rust/kei-migrate` (universal
Postgres / SQLite / MySQL migration runner — create + up + down + status).

---

## Pipeline overview (5 phases, ≥5 AskUserQuestion calls)

| Phase | File | Purpose | AskUserQuestion |
|---|---|---|---|
| 1 | [phase-1-intake.md](phase-1-intake.md) | DB, ORM, scale, style, migration control | 5× (batched) |
| 2 | [phase-2-entities.md](phase-2-entities.md) | Entity list + relations matrix | 1× |
| 3 | [phase-3-schema.md](phase-3-schema.md) | Generate DDL + indexes + FKs + constraints; review/revise | 1× |
| 4 | [phase-4-migrations.md](phase-4-migrations.md) | Scaffold `migrations/` + first migration + kei-migrate wiring | 1× |
| 5 | [phase-5-seed.md](phase-5-seed.md) | Optional seed + test fixtures | 1× |

Minimum AskUserQuestion count across a full session: **9** (5 in Phase 1 +
1 each in Phases 2–5). Exceeds the ≥5 hub-and-spoke contract.

---

## Variables the pipeline produces

| Name | Set in | Meaning |
|---|---|---|
| `INTAKE` | Phase 1 | one-paragraph app description (verbatim) |
| `DB` | Phase 1 | Postgres / SQLite / MySQL |
| `ORM` | Phase 1 | none (raw SQL) / Drizzle / SQLx / Prisma / SQLAlchemy |
| `SCALE` | Phase 1 | solo-prototype / team-dev / production-multi-replica |
| `STYLE` | Phase 1 | schema-first (SQL → types) / code-first (types → SQL) |
| `MIGCTL` | Phase 1 | manual / auto-on-deploy / hybrid (manual prod, auto dev) |
| `ENTITIES` | Phase 2 | list of entities + fields + relations matrix |
| `DDL` | Phase 3 | generated SQL (tables, indexes, FKs, constraints) |
| `MIGDIR` | Phase 4 | path of migrations dir + first migration filenames |
| `SEED` | Phase 5 | seed-data plan (or "skipped") |

---

## Final report (emit after Phase 5)

```
=== SCHEMA-DESIGN REPORT ===
App:          <first 80 chars of INTAKE>...
DB / ORM:     <DB> + <ORM>  (style: <STYLE>)
Scale:        <SCALE>       migration control: <MIGCTL>
Entities:     <N> tables, <M> relations
Schema:       db/schema.sql (<LOC> lines, <I> indexes, <F> FKs, <C> constraints)
Migrations:   migrations/<ts>_init.sql (+ .down.sql)  runner: kei-migrate
Seed:         <SEED summary or "skipped">
Libraries:    <ORM pick + driver crate/package, one line>
Next:         run `kei-migrate up` against dev DB, then hand off to code-implementer
```

---

## Rules (apply throughout)

- **Pure-click contract.** Only the Phase 1 intake paragraph and the Phase 2
  entity list are typed. Every other decision is an `AskUserQuestion` call.
- **RULE 0.8 Secrets SSoT.** The skill never emits a live `DATABASE_URL`
  value, never writes to `secrets/*.env`, never hard-codes credentials in
  DDL or seed. It emits ENV-VAR NAMES only; storage path is
  `<repo>/secrets/db.env` per `domain-has-secrets.md`.
- **NO DOWNGRADE.** If the chosen combination is unsafe (e.g.
  `auto-on-deploy` + multi-replica without leader-election) the skill
  returns 2–3 constructive alternatives, never "not supported".
- **Migration hygiene enforced.** Every migration emitted is
  timestamp-prefixed, has a `.down.sql` counterpart, and uses
  `IF NOT EXISTS` / `IF EXISTS` where safe. See `db-migration-hygiene.md`.
- **Test-First.** If Phase 5 is selected, seed includes at minimum a
  smoke-test fixture (one row per entity) to verify schema loads.
- **Surgical scope.** Reads the five db-* blocks; writes only to
  `db/schema.sql`, `migrations/<ts>_init.{sql,down.sql}`, and optionally
  `db/seed.sql`. Never touches application code.

---

## References

- `_blocks/db-postgres.md`, `_blocks/db-sqlite.md`, `_blocks/db-sqlx.md`,
  `_blocks/db-drizzle.md`, `_blocks/db-migration-hygiene.md`.
- `_primitives/_rust/kei-migrate` — universal migration runner
  (autodetects Postgres / SQLite / MySQL from `DATABASE_URL`).
- `_blocks/domain-has-secrets.md` — DB URL storage convention.
- `_blocks/rule-pre-dev-gate.md` — check existing schema before inventing.
- Evidence grade [E4] — pipeline mirrors standard relational-modelling
  practice (Codd 1NF-3NF, surrogate keys, FK-first indexing).
