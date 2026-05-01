# Phase 4 — Migration scaffold + first migration + kei-migrate wiring

Package `db/schema.sql` (from Phase 3) into a proper
timestamp-prefixed migration pair under `migrations/`, and emit the
`kei-migrate` invocation the user should run.

## 4a — Create `migrations/` directory (no AskUserQuestion)

If `migrations/` does not yet exist in the repo, create it. Emit one
`.keep` file or rely on the first migration to anchor it.

If `ORM = Prisma`: the directory is `prisma/migrations/` and the runner is
`prisma migrate dev` — skill notes this and skips kei-migrate wiring
(reference the handoff but DO NOT overwrite Prisma's own layout).

If `ORM = SQLAlchemy`: the directory is `alembic/versions/` and the runner
is `alembic upgrade head` — same rule, skip kei-migrate wiring.

For every other `ORM` value (none / Drizzle / SQLx): use `migrations/`
with kei-migrate.

## 4b — Generate timestamp + filename

- Timestamp format: `YYYYMMDDHHMMSS` (matches `kei-migrate create`'s
  convention — see `_primitives/_rust/kei-migrate/src/cmd_create.rs`).
- Migration name: `init_schema`.
- Files:
  - `migrations/<ts>_init_schema.sql`        (up — full DDL from Phase 3)
  - `migrations/<ts>_init_schema.down.sql`   (down — `DROP TABLE` reverse order)

## 4c — Up migration content

Copy `db/schema.sql` contents into the up file verbatim, with a one-line
header:

```sql
-- kei-migrate: init_schema  (generated <YYYY-MM-DD>)
-- See db/schema.sql for the schema SSoT.
```

**Do not split one migration per table** — the initial schema ships as ONE
migration by convention. Subsequent changes each get their own timestamp.

## 4d — Down migration content

Emit `DROP TABLE IF EXISTS <name> CASCADE;` for every entity, in REVERSE
dependency order (children before parents — junctions first, then leaf
entities, then referenced entities last).

If any table is flagged `-- IRREVERSIBLE` by the user (e.g. contains
critical data once populated), replace the `DROP TABLE` line with:

```sql
-- IRREVERSIBLE: this table holds production data; manual restore required.
-- Abort reverse migration.
SELECT RAISE(FAIL, 'irreversible: init_schema') ;  -- or equivalent per DB
```

See `db-migration-hygiene.md` for the irreversible pattern.

## 4e — Wire kei-migrate (AskUserQuestion)

```json
{
  "questions": [
    {
      "question": "Add kei-migrate to the project?",
      "header": "Runner",
      "multiSelect": false,
      "options": [
        {"label": "Add to Cargo workspace as path dep", "description": "Rust projects — edit root Cargo.toml members. Skill will NOT edit; emits the snippet for you to paste."},
        {"label": "Install prebuilt binary (system-wide)", "description": "Any stack — `cargo install --path _primitives/_rust/kei-migrate` once; repo stays tool-agnostic"},
        {"label": "Use existing runner (Prisma / Alembic / Drizzle-kit / goose / Atlas)", "description": "Skill skips kei-migrate; records the handoff in the report"},
        {"label": "Decide later", "description": "Files land on disk; runner wiring deferred"}
      ]
    }
  ]
}
```

Store the answer as `RUNNER`.

## 4f — Emit the next-step command (inline, no AskUserQuestion)

Print a fenced code block tailored to `DB` + `RUNNER`:

```bash
# Load DB URL from SSoT (RULE 0.8)
set -a && source secrets/db.env && set +a

# Preview pending migrations
kei-migrate --database-url "$DATABASE_URL" --dir migrations status

# Apply
kei-migrate --database-url "$DATABASE_URL" --dir migrations up

# Revert the latest (dev only!)
kei-migrate --database-url "$DATABASE_URL" --dir migrations down 1
```

Reminder (once): `secrets/db.env` must be `chmod 600` and listed in
`.gitignore` BEFORE the first write. Template entry:

```bash
# secrets/db.env — chmod 600 before first write
DATABASE_URL=
```

No values. RULE 0.8 secrets SSoT.

## Verify-criterion

- `migrations/<ts>_init_schema.sql` exists and equals `db/schema.sql` body
  with the one-line header prepended.
- `migrations/<ts>_init_schema.down.sql` exists with DROP statements in
  reverse dependency order.
- Filenames use the `kei-migrate create` timestamp convention.
- If `ORM ∈ {Prisma, SQLAlchemy}` — kei-migrate files are NOT created;
  instead record a one-line handoff in state: "use `<native runner>` —
  schema.sql is the design SSoT, port it to the native format."
- Reminder about `secrets/db.env` emitted exactly once.
