# Agent Substrate Schema — LOCKED

**Locked on:** 2026-04-23
**Locked from commit:** feat/agent-substrate-schema (decisions resolved)
**Schema SSoT:** [AGENT-SUBSTRATE-SCHEMA.md](./AGENT-SUBSTRATE-SCHEMA.md)
**Sibling SSoT:** [SUBSTRATE-SCHEMA.md](./SUBSTRATE-SCHEMA.md) (atoms, locked 2026-04-22)

## Lock scope

The capability triplet contract + role shape + task.toml shape + kei-agent-runtime Rust trait + CLI surface defined in `AGENT-SUBSTRATE-SCHEMA.md` are **immutable for 3 weeks** of parallel phase work (through ~2026-05-14).

## What "locked" means

**Non-breaking during lock** (allowed):
- New capability atoms beyond the initial 10
- New roles beyond the initial 5
- New optional fields on `capability.toml` / `role.toml` / `task.toml`
- New verify `run-mode` values
- New gate `severity` levels

**Breaking during lock** (requires revocation):
- Changing capability ID separator `::`
- Changing the Capability trait signature
- Switching capability definitions from Rust to another language
- Changing TOML → another config format
- Changing capability path layout `_capabilities/<category>/<slug>/`
- Changing the 8-decision values in §Decision log

## Phases under lock

| Phase | Branch | Start |
|---|---|---|
| 1 | `feat/phase-1-capability-library` | on lock |
| 2 | `feat/phase-2-role-matrix` | on lock |
| 3 | `feat/phase-3-kei-agent-runtime` | on lock |
| 4 | `feat/phase-4-hook-wiring` | after 1+3 |
| 5 | `feat/phase-5-agent-migration` | after 1+2+3+4 |

## Unlock

Automatic unlock on **2026-05-14** OR on `AGENT-SCHEMA-UNLOCKED.md` commit by user (whichever comes first). Post-unlock, schema v2 can iterate based on what the 5 phases learned.
