# Substrate Schema — LOCKED

**Locked on:** 2026-04-22
**Locked from commit:** feat/substrate-schema-v1 @ 5d089bc
**Schema SSoT:** [SUBSTRATE-SCHEMA.md](./SUBSTRATE-SCHEMA.md)

## Lock scope

The atom / capability / graph contracts defined in `SUBSTRATE-SCHEMA.md` are **immutable for 6 weeks** of parallel Stream A/B/C/D work (through ~2026-06-03).

## What "locked" means

**Non-breaking during lock** (allowed, standard git flow):
- New atom kinds beyond `command | query | stream | transform`
- New optional frontmatter fields
- New `side_effects.op` values
- New `stability` levels
- New JSON Schema field examples / constraints that don't break existing validators

**Breaking during lock** (requires revocation):
- Changing atom ID separator `::`
- Changing frontmatter shape
- Renaming `Cargo.toml [package.metadata.keisei]` fields
- Dropping JSON Schema draft-07 in favour of 2020-12
- Switching to shared error registry
- Any change that forces all 4 streams to rebase

## Revocation protocol

To break the lock within the 6-week window:

1. User explicitly states: "revoke substrate schema lock — reason: <...>"
2. All 4 stream agents paused
3. Schema edit committed on `feat/substrate-schema-v2` with revision log
4. All 4 stream worktrees rebased onto new schema
5. Ledger row: `kei-ledger fork <id> --reason "schema-revocation"` for audit
6. New lock marker committed

## Streams under lock

Per Stream interfaces in SUBSTRATE-SCHEMA.md:

- **Stream A** — `feat/stream-a-kei-forge` — web wizard UI that generates atoms
- **Stream B** — `feat/stream-b-atoms-refactor-kei-task` — pilot refactor of `kei-task` into 3 atoms
- **Stream C** — `feat/stream-c-kei-sage-substrate` — kei-sage walks `atoms/*.md`, resolves wikilinks
- **Stream D** — `feat/stream-d-kei-runtime` — new crate `kei-runtime` with `invoke`, `list-atoms`, `kei-schema-lint`

## Unlock condition

Automatic unlock on 2026-06-03 OR on `SCHEMA-UNLOCKED.md` commit by user (whichever comes first). Post-unlock, the next schema version (`SUBSTRATE-SCHEMA.md` v2) can iterate freely based on what the 4 streams learned.
