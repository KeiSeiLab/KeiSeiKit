# Schema Locks — REVOKED

**Revoked on:** 2026-04-23
**Revoked by:** user (explicit request "Мы проекты за часы делаем — недели abstraction")
**Replaces:** `docs/SCHEMA-LOCKED.md` and `docs/AGENT-SCHEMA-LOCKED.md`

## Revocation rationale

Both schema locks (atom substrate v1 and agent substrate v1) were originally declared for calendar windows:
- Atom substrate: 6 weeks (through 2026-06-03)
- Agent substrate: 3 weeks (through 2026-05-14)

Reality check: the phases those locks were intended to protect shipped in **hours**, not weeks:
- Atom substrate phases 1-5 (4 parallel streams + integration) — landed in ~2 hours
- Agent substrate phases 1-5 — landed in ~30 minutes
- Convergence pre-unlock wave (U1+U2+U3) — landed in ~25 minutes

**The locks were calibrated for a timeline that was 30-100× slower than actual execution.** Calendar-week windows are corporate-ritual pacing, not real-velocity pacing. Keeping the locks would delay breaking consolidations (path-filter merge, cargo-green merge, /audit target, verb-template refactor) that can now safely ship.

## What revocation enables

Immediate (post-revocation, in-flight wave):
- Verb templates (CRUD as data) — Layer A from convergence plan
- Schema fragments ($ref'd JSON) — Layer B
- PatternGate unified trait — Layer C
- CommandVerify unified trait — Layer D
- Role expression (extends/adds/relaxes) — Layer E
- DNA identity — new Layer G (agent ID encodes composition)

Follow-up:
- `scope::path-filter` (consolidate whitelist + denylist)
- `quality::cargo-green` (consolidate check + tests)
- `/audit <target>` (collapse 6 skills + checklist registry)
- kei-runtime-core extraction (shared infrastructure across 3+ runtime crates)

## What stays locked (nothing)

Neither SUBSTRATE-SCHEMA.md nor AGENT-SUBSTRATE-SCHEMA.md is locked. Breaking changes allowed from this commit forward with standard review gate (audit agents + integration tests pass).

## Process change

Future schema-lock declarations:
- Express duration in **agent-hours-of-work**, not calendar weeks
- Default: "locked until explicitly revoked OR all declared-dependent phases land"
- Revocation requires: ledger row + reason + this doc updated

No more calendar-week locks.

## Ledger entry

```
2026-04-23  schema_lock_revoked  atom,agent  reason=calendar-overestimate,phases-landed
```
