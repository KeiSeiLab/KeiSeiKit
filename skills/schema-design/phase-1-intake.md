# Phase 1 — Intake (DB, ORM, scale, style, migration control)

One free-text paragraph, then ONE batched `AskUserQuestion` call with all
five click decisions. This is the only phase that accepts typed input.

## 1a — Ask for the app description

Emit a regular message (NOT AskUserQuestion):

> Describe the app in one paragraph: what is it, how many entities (rough
> count), any constraint I should know (existing DB, regulated data,
> multi-tenant, edge / serverless, expected row counts). Reply in one
> message.

Store the reply verbatim as `INTAKE`.

## 1b — Batched click (AskUserQuestion, 5 questions in ONE call)

The UI cap per `AskUserQuestion` call is 4–5 questions; emit all five at
once for a smooth click-through.

```json
{
  "questions": [
    {
      "question": "Which database engine?",
      "header": "DB",
      "multiSelect": false,
      "options": [
        {"label": "PostgreSQL 17", "description": "Default for multi-user / relational integrity. See _blocks/db-postgres.md"},
        {"label": "SQLite",        "description": "Single-node, edge-friendly, ~100k users ceiling. See _blocks/db-sqlite.md"},
        {"label": "MySQL / MariaDB", "description": "Existing stack compatibility; kei-migrate supports it"}
      ]
    },
    {
      "question": "ORM / query layer?",
      "header": "ORM",
      "multiSelect": false,
      "options": [
        {"label": "None (raw SQL)",  "description": "Hand-written queries; max control, no magic"},
        {"label": "Drizzle (TS)",    "description": "Schema-first or code-first; see _blocks/db-drizzle.md"},
        {"label": "SQLx (Rust)",     "description": "Compile-time checked queries; see _blocks/db-sqlx.md"},
        {"label": "Prisma (TS)",     "description": "Code-first; own migration engine (NOT kei-migrate)"},
        {"label": "SQLAlchemy (Py)", "description": "Alembic for migrations (NOT kei-migrate); legacy compat"}
      ]
    },
    {
      "question": "Target scale?",
      "header": "Scale",
      "multiSelect": false,
      "options": [
        {"label": "Solo prototype",          "description": "One dev, <1k rows, SQLite OK"},
        {"label": "Team dev",                "description": "Shared dev DB, staging, prod — standard"},
        {"label": "Production multi-replica", "description": "Leader-election required for migrations; zero-downtime patterns mandatory"}
      ]
    },
    {
      "question": "Design style?",
      "header": "Style",
      "multiSelect": false,
      "options": [
        {"label": "Schema-first (SQL → types)", "description": "Write DDL, generate types. Default with raw SQL / SQLx / Drizzle schema-first"},
        {"label": "Code-first (types → SQL)",   "description": "Define entities in code, generate DDL. Drizzle code-first / Prisma / SQLAlchemy"}
      ]
    },
    {
      "question": "Migration control?",
      "header": "MigCtl",
      "multiSelect": false,
      "options": [
        {"label": "Manual (human runs kei-migrate up)",  "description": "Safest; recommended for prod"},
        {"label": "Auto-on-deploy",                       "description": "CI runs migrations; single-replica only — NO DOWNGRADE warning if multi-replica"},
        {"label": "Hybrid (manual prod, auto dev)",       "description": "Recommended default — dev velocity + prod safety"}
      ]
    }
  ]
}
```

Store answers as `DB`, `ORM`, `SCALE`, `STYLE`, `MIGCTL`.

## Verify-criterion

- `INTAKE` non-empty.
- `DB`, `ORM`, `SCALE`, `STYLE`, `MIGCTL` each exactly one label.
- If `ORM ∈ {Prisma, SQLAlchemy}` → note in state: "Phase 4 will hand off
  to that tool's native migration runner (Prisma migrate / Alembic); the
  kei-migrate scaffold is skipped or wrapped." No downgrade — the skill
  still emits a working plan.
- If `MIGCTL = Auto-on-deploy` AND `SCALE = Production multi-replica` →
  warn "race condition risk — every replica tries to apply" (see
  `db-migration-hygiene.md`) and re-ask with the Hybrid option highlighted.
