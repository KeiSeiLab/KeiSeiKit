---
name: spawn-agent
description: Prepare a composed Agent-tool invocation via kei-spawn. Click-only wizard — pick role, describe task, set scope. Emits ready-to-paste prompt + subagent_type + isolation + DNA.
argument-hint: (no arguments)
---

# /spawn-agent — Click-only Agent-tool composer (index)

You convert a user's intent into a ready-to-paste `Agent`-tool invocation
via the `kei-spawn` CLI. Every decision is a click; only the task description
(Phase 2) is typed. No git, no side-effects on disk beyond the task.toml the
CLI consumes.

This `SKILL.md` is the INDEX. Each phase lives in its own file and is
executed in order. Never skip a phase. Never re-order phases.

---

## Pipeline overview

| Phase | File | Purpose | AskUserQuestion |
|---|---|---|---|
| 1 | [phase-1-role.md](phase-1-role.md) | Pick agent role (capability tier) | 1× AskUserQuestion |
| 2 | [phase-2-task.md](phase-2-task.md) | Free-text task description (ONE paragraph) | 0× (only free-text in the skill) |
| 3 | [phase-3-scope.md](phase-3-scope.md) | Files whitelist glob patterns | 2× AskUserQuestion |
| 4 | [phase-4-emit.md](phase-4-emit.md) | Write task.toml, run kei-spawn, emit Agent-tool invocation | 1× AskUserQuestion (confirm-emit) |

**Minimum AskUserQuestion count: 4** — pure-click contract.

---

## Variables the pipeline produces

| Name | Set in | Meaning |
|---|---|---|
| `ROLE` | Phase 1 | One of `read-only` / `explorer` / `edit-local` / `edit-shared` |
| `TASK` | Phase 2 | Free-text task description (1-3 sentences) |
| `WHITELIST` | Phase 3 | List of glob patterns (relative to repo root) |
| `DENYLIST` | Phase 3 | Optional list of glob patterns (auto-derived from role if skipped) |
| `TASK_TOML` | Phase 4 | Path `tasks/<uuid>.toml` relative to current repo |
| `AGENT_ID` | Phase 4 | UUID returned by `kei-spawn spawn` |
| `DNA` | Phase 4 | SHA-256 fingerprint returned by `kei-spawn spawn` |
| `SUBAGENT_TYPE` | Phase 4 | Resolved by kei-spawn from ROLE (e.g. `code-implementer` / `researcher`) |
| `ISOLATION` | Phase 4 | Resolved by kei-spawn from ROLE (`worktree` or `shared`) |
| `WORKTREE_PATH` | Phase 4 | Returned when isolation=worktree |

---

## Role → defaults map (LOAD-BEARING)

| ROLE | subagent_type | isolation | Bash? | Writes? |
|---|---|---|---|---|
| read-only | researcher | shared | no | no |
| explorer | researcher | shared | read-only bash | no |
| edit-local | code-implementer | worktree | cargo/test only | yes (whitelist) |
| edit-shared | code-implementer | worktree | cargo/test only | yes (whitelist) |

`kei-spawn` owns the resolution — this table is reference only. The skill
passes the ROLE string verbatim; the CLI returns the resolved values.

---

## Final report (emit after Phase 4)

```
=== /SPAWN-AGENT REPORT ===
Role:            <ROLE>
Task (first 80): <first 80 chars of TASK>...
Whitelist:       <N globs> — <first 3 shown>
Denylist:        <auto / explicit — N globs>
Agent ID:        <UUID>
DNA:             <sha256:first-12>...
subagent_type:   <SUBAGENT_TYPE>
isolation:       <ISOLATION>
Worktree:        <WORKTREE_PATH or n/a>

Ready-to-paste Agent-tool invocation:

  <JSON block — see phase-4-emit.md §4 for exact shape>

Next step on return:
  kei-spawn verify <AGENT_ID> <WORKTREE_PATH>
```

---

## Runtime binary resolution

`kei-spawn` must be on `PATH` OR reachable via `$KEI_RUNTIME_BIN_DIR`. The
phases call `kei-spawn <cmd>` directly; if the shell returns `command not
found`, fall back to `"$KEI_RUNTIME_BIN_DIR/kei-spawn" <cmd>`. If both fail,
STOP and surface the error to the user with three constructive paths:

- (A) Build `kei-spawn` from the kit source (`cd _primitives/_rust/kei-spawn && cargo build --release`).
- (B) Export `KEI_RUNTIME_BIN_DIR` to point at an existing build.
- (C) Install via `install.sh` which wires both.

Never silently substitute a mock.

---

## Rules (enforced at every phase)

- **Pure-click contract.** Only `TASK` (Phase 2) is typed. Every other
  decision is an `AskUserQuestion`. Count them in the final report.
- **NO DOWNGRADE (RULE -1).** Any phase that fails returns 2-3 constructive
  paths, never "can't be done".
- **NO HALLUCINATION (RULE 0.4).** Never fabricate a DNA or agent_id — they
  come from `kei-spawn spawn --format=json` verbatim. Never emit an Agent
  invocation without a successful spawn call first.
- **Plan Mode First (RULE 0.5).** This skill IS the plan; each phase file
  has its own verify-criterion. No `kei-spawn spawn` call before Phase 4's
  confirm click.
- **Orchestrator branch first (RULE 0.13).** The skill NEVER invokes git.
  The emitted prompt MUST contain the phrase "MUST NOT invoke git" so the
  spawned agent respects the rule. Phase 4 inserts this automatically.
- **Constructor Pattern (RULE ZERO).** One file per phase. This SKILL.md
  stays <200 LOC; each phase file stays <200 LOC.
- **Surgical Changes.** The skill only writes `tasks/<uuid>.toml`. It does
  not modify project source, does not commit, does not push.

---

## References

- [phase-1-role.md](phase-1-role.md) · [phase-2-task.md](phase-2-task.md) · [phase-3-scope.md](phase-3-scope.md) · [phase-4-emit.md](phase-4-emit.md)
- `_primitives/_rust/kei-spawn/` — CLI source (task.toml schema + spawn/verify commands)
- `skills/new-agent/SKILL.md` — sister skill: generates agent *manifests*; this skill spawns *instances* of existing agents
- `skills/hooks-control/SKILL.md` — reference click-only pattern
- RULE 0.12 (agent-git-model) — ledger row auto-written by `kei-spawn spawn`
- RULE 0.13 (orchestrator-branch-first) — ban-phrase injected by Phase 4
