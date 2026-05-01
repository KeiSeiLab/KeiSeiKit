# Philosophy — KeiSeiKit as a Living Neural Structure

> The README claims KeiSeiKit is "a living neural structure." This document
> is the long form of that claim: which biological properties we chose,
> why, how each one maps to code shipped in this repo, and what the
> tradeoffs are.

---

## The one-line thesis

A software toolkit accumulates features. A living neural structure
accumulates identity, memory, lineage, and the ability to recover from its
own mistakes. KeiSeiKit is built around the second list.

## The five properties

A neural system is distinguishable from a collection of functions by five
properties. Each one is load-bearing — remove it and the structure stops
being alive in the relevant sense.

1. **Identity** — each unit has a stable, reproducible name, not a random
   UUID or a human-friendly slug that drifts.
2. **Lineage** — each unit knows who produced it and from what.
3. **Memory** — the system remembers what it did yesterday, across
   restarts, across machines, across sessions.
4. **Consolidation** — memory is not raw logs; it is periodically replayed
   and compressed into patterns.
5. **Corrective learning** — mistakes the system notices in itself
   surface as explicit artefacts (rules, hooks, tests) the next session
   inherits.

The rest of this document maps each property to the shipped code.

---

## 1. Identity — DNA

Every agent invocation resolves to a deterministic 80-character string:

```
<role>::<caps-bitmap>::<scope-hash>::<body-hash>-<nonce>
```

- `role` — role slug (e.g. `edit-local`, `research-web`).
- `caps-bitmap` — the resolved capability list, encoded as ordered 2-char
  atom codes (`NG` = no-git-ops, `FW` = files-whitelist, `TG` = tests-green).
- `scope-hash` — 8 hex chars (32-bit) of SHA-256 over canonicalised scope.
- `body-hash` — 8 hex chars of SHA-256 over the task body text.
- `nonce` — 8 hex chars of `rand::random::<u32>()`, full 32-bit entropy.

The shape is enforced by `kei-agent-runtime::dna::parse`. Two invocations
with the same role, capability set, scope, and task body produce the same
`<role>::<caps>::<scope>::<body>` prefix, and differ only by nonce. That
makes DNA both deterministic (for reasoning) and unique (for collision
resistance — birthday threshold ≈ 65k agents per role+caps group).

**Why not UUIDs.** A UUID hides what the agent was supposed to do. A DNA
string is greppable: you can look at a ledger row and see the capability
bitmap without joining five tables.

**Why not slugs.** Slugs collide and drift. DNA is stable under renames
because the role slug is part of the hash input, not a sidecar.

Source: [`_primitives/_rust/kei-agent-runtime/src/dna.rs`](../_primitives/_rust/kei-agent-runtime/src/dna.rs).

## 2. Lineage — creator_id and fork_parent_id

Every row in the agent ledger carries two additional columns:

- `creator_id` — DNA or human id of whoever spawned this row.
- `fork_parent_id` — DNA of the agent this row was forked from, if any.

This is SQL-level lineage. Any artefact produced during a session can be
traced back to the agent that produced it, to the agent that spawned
*that* agent, to the human session the chain started from.

**Why this matters.** Software without lineage produces "where did this
file come from" questions at merge time. Lineage makes them disappear:
the merge-ceremony skill prints the DAG and the human picks which forks
merge, which squash, which defer.

Schema: [`_primitives/_rust/kei-ledger/src/schema.rs`](../_primitives/_rust/kei-ledger/src/schema.rs) migration v4.

## 3. Memory — three layers

The memory layer is deliberately three-tiered, mirroring the hippocampal
+ cortical split:

1. **Raw episodes** — session JSONL traces, append-only, one file per
   session. This is the hippocampus: fast, stateful, volatile (survives
   until the next full pull), not interpreted.
2. **Project memory** — `memory/{project}.md` one file per project,
   self-contained, constraints + stack + status + learnings with
   evidence grades.
3. **Index** — `MEMORY.md`, one line per project, ≤200 lines total. No
   inline data. Reading this file gives the shape of the world.

Any session's "what did we decide last time" is a read of the
corresponding project file, not a scroll through chat history. The index
guarantees that read is fast.

**Why this layering.** A single giant memory file stops being read
because it cannot be scanned. A million tiny files stop being read
because there is no entry point. Three layers — index → project file →
raw traces — give you O(1) navigation to the right detail.

Source: `~/.claude/rules/memory-protocol.md` (the full memory-protocol
rule, reusable across projects).

## 4. Consolidation — REM and NREM

Raw traces become patterns only if something replays them. KeiSeiKit's
sleep layer runs in three phases on a nightly schedule:

### Phase A — Incubation ("sleep on it")

During the day, you drop tasks into `/sleep-on-it`. Each task gets a
priority (quick 15 min / standard 60 min / deep 240 min / marathon
480 min) and optionally a checkpoint cadence. At 03:00 local a remote
Claude Code agent on Anthropic's cloud picks up the queue (up to 480
minutes total across ≤ 5 tasks, packed greedily in FIFO order) and
works until the budget or checkpoint fires.

Biological analog: the overnight consolidation of un-finished intentions
(Wagner et al. 2004, *Nature*). Things unsolved when you fell asleep are
often solved by morning not because the brain ran harder, but because
it ran offline.

### Phase B — REM consolidation

After Phase A, the same agent reads the last 24 h of JSONL traces, diffs
them against the previous report, and writes
`reports/sleep-YYYY-MM-DD.md`. Cross-session patterns (≥ 3 occurrences
across ≥ 2 distinct sessions) are prepended to `backlog.md`.

Biological analog: REM dream-state. Pattern extraction, not raw replay.

### Phase C — NREM deep sleep

Every seven days (by default; configurable to zero to disable) the
pipeline also runs `kei-conflict-scan` → `kei-refactor-engine` → optional
`kei-graph-check`. The output is a **plan-only** markdown file or a
**plan + fork** branch (`deep-sleep/YYYY-MM-DD`) with `git apply`-ready
changes. Ambiguous conflicts are excluded from any auto-patch and listed
explicitly for human decision.

Biological analog: NREM slow-wave sleep. System-level consolidation.
Integrating, not just reviewing.

### The no-feedback-loop invariant

Nothing the cloud agent writes is ever auto-injected into a Claude Code
session. The morning report is for human review. Any rule or hook that
emerges from it is installed by hand via `/escalate-recurrence`. This
is a deliberate architectural choice: auto-learning loops without human
signoff are how models drift.

Source: [`docs/SLEEP-LAYER.md`](./SLEEP-LAYER.md) and `~/.claude/rules/sleep-layer.md`.

## 5. Corrective learning — self-audit

Three passive hooks run during any session:

- `session-end-dump` — on Stop, archives the session trace and ingests
  it into `kei-memory`.
- `milestone-commit-hook` — on `feat:` / `refactor:` / merge commits,
  appends a one-line summary to `audit-backlog.md`.
- `error-spike-detector` — when three or more errors occur in the last
  twenty tool calls, tags the pattern.

These feed the `/self-audit` skill, which classifies recurring problems
and surfaces them via click-only `AskUserQuestion`. The user can
route a finding to:

- `/escalate-recurrence` — codify as rule + wiki + optional hook.
- `/debug-deep` — 5-phase root-cause analysis.
- hook-only — mechanical block / enforce / warn / remind.
- backlog — log, surface next session.
- postpone — keep open, resurface later.

**Silent-first mode.** The first ten sessions log only — no prompts.
This prevents false-positive fatigue while the memory store is still
empty. Session 11 onward, the self-audit starts surfacing items.

Source: `~/.claude/rules/session-self-audit.md` (RULE 0.14) and
`skills/self-audit/`.

---

## Growth — the sixth property, emergent

A substrate that satisfies properties 1–5 can support a sixth that is
harder to design for directly: **growth**. New primitives, new blocks,
new agents, new projects enter through user-driven commands and
accumulate in a way the next session can find.

- **New primitive.** `/compose-solution` decomposes a free-text problem,
  greps existing atoms for prior art, and drafts a block if nothing
  matches. The draft is persisted on user click, discoverable thereafter.
- **New agent.** `/spawn-agent` emits a manifest + DNA + ledger row. The
  assembler hook rebuilds the markdown Claude Code picks up.
- **New project.** `/new-project` is a 4-phase skill: intake, fork
  skeleton, parallel execution (orchestrator owns git per RULE 0.13),
  merge ceremony.

Growth is not a feature we implemented. It is what the other five
properties produce when you compose them.

---

## What this is not

A neural network. The name "neural structure" is an analogy about
properties (identity, lineage, memory, consolidation, corrective
learning), not a claim about weights or gradients. Nothing in KeiSeiKit
trains on your data. The cloud agent in the sleep layer is a standard
Claude Code session with scheduled triggers — it reads your traces to
write a report, not to fine-tune itself.

A federation. As of v0.24, KeiSeiKit ships as a single-user substrate
installed next to Claude Code. Cross-user signing, marketplace publishing
of blocks, and federation are on the roadmap but not yet shipped. If a
doc claims otherwise, that doc is stale.

A framework. A framework tells you how to structure your application. A
substrate gives your agents identity, lineage, memory, and sleep —
nothing about it dictates the application. You can delete every skill
in this repo and the substrate still works; you can also add fifty more
and it still works.

---

## The constraints that shaped this

Three constraints, made explicit because they push back against common
defaults:

- **Constructor Pattern.** One file, one class, one concern. Files
  greater than 200 lines are decomposed. Functions greater than 30
  lines are split. No mixins, no DI containers, no abstract factories.
  This keeps the graph readable by both humans and Claude.
- **Rust-first default.** New primitive code is Rust unless there is a
  cited exception (ML training > 10M params / existing-language project
  / platform UI / browser-DOM / one-off < 50 lines / external binding
  only / explicit user override). The reason is not performance — it is
  that the Rust type system catches the class of mistakes LLMs most
  often introduce (`None` vs `[]`, missing `.await`, unhandled
  `Result`) at compile time.
- **Local-first.** Nothing is pushed anywhere by default. The sleep
  layer's memory-repo is user-owned, on whatever remote the user
  chose (or no remote — everything works locally).

If these constraints feel restrictive, they are — deliberately. They
are the shape of the substrate, not decorations.

---

## Further reading

- [ARCHITECTURE.md](./ARCHITECTURE.md) — build pipeline + bridges + meta-composer
- [SLEEP-LAYER.md](./SLEEP-LAYER.md) — Phase A / B / C in depth
- [TAXONOMY.md](./TAXONOMY.md) — the seven-facet vocabulary
- [SUBSTRATE-SCHEMA.md](./SUBSTRATE-SCHEMA.md) — atom contract
- [WHY.md](./WHY.md) — the full origin story
