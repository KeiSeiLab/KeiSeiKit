# KeiSeiKit Architectural Decisions

> ADR-style log. Each entry: context → decision → consequences. New entries
> at the top. Cross-link from `_primitives/_rust/<crate>/README.md` when a
> decision is crate-local.

---

## 2026-05-25 — Opt-in hook packs + stack profiles (public-prep posture)

### Context

The kit force-activated every hook via `settings-snippet.json`, including the
author's personal research discipline (numeric-claims evidence markers,
no-downgrade, citation-verify, rust-first / no-python). For a public,
general-audience kit that is presumptuous — users bring their own stack and do
not need a Rust-only policy or evidence-marker enforcement by default.

### Decision

- Posture: **safety hooks on by default; all discipline packs opt-in.** Packs:
  `safety` (always), `evidence`, `observability`, `epistemic`, `orchestration`,
  `git-guard`, `stack-rust`. SSoT = `_primitives/hook-packs.toml`.
- **Stack profiles** (minimal / web / ml / systems / mobile) pull a set of
  discipline packs AND an agent set. `rust-first` / `no-python` live only in
  `stack-rust`, which only the `systems` stack enables. `git-guard`
  (no-github-push) is opt-in only and pulled by NO stack — a general kit must
  not block a user's normal `git push` to github.
- Mechanism: install-time **filter** of the snippet by selected packs
  (`filter_snippet_by_packs`) + **prune** of kit-owned hooks on reconfigure
  (`prune_kit_hooks`, foreign hooks preserved). Selection persists to
  `~/.claude/config/onboarding.toml`; re-runnable via `kei configure`.
- Non-interactive / `--yes` / CI default = minimal (safety + cosmetic only),
  all agents (back-compat for power users).

### Consequences

- Gate wiring (`_lib/gate.sh`) added to the 8 highest-friction discipline hooks
  for runtime toggling via the `hooks-control` skill; remaining cosmetic/event
  hooks deferred (install-time filtering already gives "off by default", so the
  runtime gate is a convenience, not a correctness requirement).
- Agent-set changes via `kei configure` apply on the next `./install.sh`
  (reconfigure re-applies hooks fully but does not remove already-installed
  agent manifests — they are harmless extra `.md` files).
- `_toml_array` extracted from `lib-profile.sh:profile_members` as the shared
  one-line-array TOML reader (no new dependency).

## 2026-04-28 — Three scheduling abstractions in workspace

### Context

After Hermes import (P4.2 `kei-cron-scheduler`) the KeiSeiKit workspace
contains **three** scheduler-like primitives. A naive audit reads this as
duplication; in practice each occupies a distinct layer of the stack and
removing any one would break a downstream consumer. This ADR documents the
boundary so a future reader does not consolidate them by mistake.

### The three primitives

| Crate | Storage | Concurrency | Owns runner? | Canonical use |
|---|---|---|---|---|
| `kei-scheduler` | `rusqlite` (sync, metadata-only) | sync | **no** | per-call queryable schedule index |
| `kei-cron-scheduler` | JSON-on-disk + `fcntl` advisory lock | `tokio` async | **yes** | Hermes parity (`/schedule` parser + cron loop) |
| `kei-pipe` cron triggers | embedded in pipe TOML | driven by pipe runtime | depends on pipe | pipeline-level cron embedded in a pipe definition |

### Decision

**Keep all three. Do not consolidate.** Each abstraction encodes a
different ownership contract and a different blast radius on failure.

### Rationale, primitive by primitive

#### `kei-scheduler` — synchronous metadata-only store

Synchronous `rusqlite` schedule store. Stores cron expression, next-run
timestamp, owner, payload pointer. Does **not** dispatch — the caller asks
"what should I run between t and t+Δ" and the caller is responsible for
execution.

This separation matters because two callers want exactly that contract:

- `kei-pipe` queries the schedule from the pipe-runtime loop (already its
  own scheduler) — it must not have a competing async runner inside the
  store.
- `cron-wrapper-agent` test harness wants deterministic, blocking lookups
  with no background tasks. A `tokio` runtime would fight the harness.

A SQLite-backed metadata store is the smallest abstraction that satisfies
both callers. Any `tokio` infrastructure inside this crate would force its
contract on the harness and break determinism.

#### `kei-cron-scheduler` — async runner for Hermes parity

Async `tokio`-based runner. JSON-on-disk persistence (one file per job),
`fcntl` advisory lock to keep multiple binaries from racing the same job
file, owns its own loop, supports interval + standard 5-field cron. This
is the surface imported from Hermes (HERMES-MIGRATION-PLAN P4.2) — the
contract is "set-and-forget recurring scheduler with the runner inside the
crate."

A SQLite-only store like `kei-scheduler` cannot satisfy this contract:

- File-per-job is the unit of `fcntl` locking; a single SQLite file would
  serialise all locks through the SQLite write mutex.
- The runner is part of the public surface — Hermes callers expect to
  hand the crate a job and walk away. Splitting the runner into a
  separate crate would re-litigate the contract on every consumer.

#### `kei-pipe` cron triggers — pipeline-level cron embedded in a pipe

Pipes (KeiSei pipeline definitions, TOML) can declare a cron trigger
inline. The pipe runtime evaluates the trigger as part of the pipe's own
state machine, alongside event triggers, file-watch triggers, and HTTP
triggers. The cron trigger is **not** a separate scheduler — it is a
trigger source within the pipe runtime, which is itself the scheduler.

Re-implementing this on top of `kei-cron-scheduler` would either (a)
duplicate the pipe runtime's lifecycle into the cron crate, or (b) split
a single pipe's triggers across two runtimes, which loses the atomic
"trigger-fired-and-pipe-started" guarantee the pipe runtime provides.

### Consequences

- **Choosing the right primitive for a new caller.** Decision tree:
  - Need a recurring background runner with `fcntl` durability and
    minimal blast radius if a single binary crashes? → `kei-cron-scheduler`.
  - Need a queryable index of "what should I run", with execution owned
    elsewhere? → `kei-scheduler`.
  - Trigger is one of many in a pipe definition, lives next to the data
    flow, dies with the pipe? → `kei-pipe` cron trigger.
- **Fail-loud overlap.** If you find yourself porting a feature from one
  to another (e.g. "let `kei-scheduler` also dispatch"), STOP — that is
  the No-Patching/No-Overlay smell from the umbrella rules. Add the
  feature to the right primitive instead, or write a new one.
- **Audit signal.** A future audit may flag "three schedulers" as a code
  smell. This ADR is the canonical answer; link here from any review
  comment that surfaces the question again.

### References

- `_primitives/_rust/kei-scheduler/`
- `_primitives/_rust/kei-cron-scheduler/`
- `_primitives/_rust/kei-pipe/` (cron trigger source)
- `HERMES-MIGRATION-PLAN.md` §P4.2 — Hermes parity import
