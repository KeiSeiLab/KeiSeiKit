---
name: new-project
description: 4-phase pipeline for bootstrapping a new multi-agent project — intake, fork skeleton (branch + ledger row + sub-agent spawn), parallel execution with progress aggregation, and per-branch merge ceremony. Implements RULE 0.12 (agent git-model) at project scale. Hub-and-spoke — each phase lives in its own file and is executed in order.
argument-hint: <project name or one-line goal>
---

# New-Project — 4-Phase Pipeline (index)

You are bootstrapping a **new project** — research, code, theoretical, or
hybrid — that will run as a **main agent plus N parallel sub-agents** with
full git-model compliance (branch + 6-file artefact bundle + ledger row +
merge ceremony). This skill is the orchestrator wrapper; each phase lives
in its own file.

This skill does NOT itself write production code. It routes to
`compose-solution` (for each sub-task that needs a kit artefact) and to
`new-agent` (for each new specialist spawn) and records every fork in the
`kei-ledger` SQLite SSoT. Final merge decisions are user clicks.

---

## Pipeline overview (4 phases + final report)

| Phase | File | Purpose | Free-text? | AskUserQuestion |
|---|---|---|---|---|
| 1 | [phase-1-intake.md](phase-1-intake.md) | Project-shape intake (type / theory / parallelism / main-agent / DB) | 1 line (`GOAL`) | 1× batch of 5 questions |
| 2 | [phase-2-fork-skeleton.md](phase-2-fork-skeleton.md) | Create `project/<slug>` branch, ledger entry, theoretical sub-agent spawn | no | 1× (sub-agent kind confirm) |
| 3 | [phase-3-parallel-exec.md](phase-3-parallel-exec.md) | Poll `kei-ledger list --status running`, aggregate `progress.json` | no | 1× (continue / pause / add agent) |
| 4 | [phase-4-merge-ceremony.md](phase-4-merge-ceremony.md) | Per-branch merge decision — squash / no-ff / reject / defer | no | ≥ 1× per branch (multi-select) |

Minimum AskUserQuestion count across a full session: **≥ 6** — one batch
of five questions in Phase 1, one per spawn confirmation in Phase 2, one
polling click in Phase 3, and one per sub-branch in Phase 4 (≥ 2 branches
assumed). This is the pure-click contract: only the goal statement is
free-text.

---

## Variables the pipeline produces

| Name | Set in | Meaning |
|---|---|---|
| `GOAL` | Phase 1a | One-line project description |
| `PROJECT_TYPE` | Phase 1b | new-code / research / theoretical / hybrid / documentation |
| `THEORY_PART` | Phase 1b | none / math-derivation / prior-art / architecture-spec / paradigm-analysis |
| `FANOUT` | Phase 1b | single / up-to-3 / up-to-5 / up-to-10 |
| `MAIN_AGENT` | Phase 1b | meta-orchestrator / spawn specialist / compose-solution decides |
| `DB_MODE` | Phase 1b | file-only / sqlite-ledger / external-tool |
| `PROJECT_SLUG` | Phase 2a | kebab-case slug derived from `GOAL` |
| `PROJECT_BRANCH` | Phase 2a | `project/<slug>` |
| `LEDGER_ID` | Phase 2b | agent id issued via `kei-ledger fork` |
| `SUB_AGENTS` | Phase 2c | list of `{id, branch, kind}` spawned |
| `PROGRESS` | Phase 3 | per-sub-agent `{status, pct, last_summary}` aggregated |
| `MERGE_PLAN` | Phase 4 | per-branch verdict (merge / squash / reject / defer) |

---

## Final report (emit after Phase 4)

```
=== NEW-PROJECT REPORT ===
Goal:            <first 80 chars of GOAL>...
Project slug:    <PROJECT_SLUG>
Project branch:  <PROJECT_BRANCH>
Ledger root id:  <LEDGER_ID>
Sub-agents:      <N spawned, M done, K failed>
Merge verdicts:  <a merged / b squashed / c rejected / d deferred>
Artefact bundle: <per sub-agent — 6/6 present / missing list>
Next action:     <push / open PR / rerun failed / review deferred>
```

---

## Rules (apply throughout — enforced at every phase)

- **Pure-click contract.** Only `GOAL` (Phase 1a) is typed. Every other
  decision is an `AskUserQuestion` call.
- **NO DOWNGRADE (RULE -1).** Any phase that fails returns 2-3 constructive
  paths, never "can't be done". E.g. Phase 2 branch-conflict → (A) rename
  slug, (B) force new branch off HEAD, (C) abort — user clicks.
- **NO HALLUCINATION (RULE 0.4).** Every sub-agent kind referenced MUST
  exist in `_manifests/` or be scheduled for creation via `new-agent`
  before Phase 3 polling.
- **Plan Mode First (RULE 0.5).** This skill IS the plan; each phase file
  has its own verify-criterion. No Edit/Write to production scope before
  the corresponding phase's confirm click.
- **Constructor Pattern (RULE ZERO).** `SKILL.md` < 200 LOC, phase files
  < 150 LOC each.
- **RULE 0.12 compliance.** Every sub-agent spawn MUST:
  1. Create `project/<slug>/agent-<id>` branch OR worktree
  2. `kei-ledger fork <id> <branch> --parent <project-branch> --spec-sha <sha>`
  3. Produce 6-file bundle in `.claude/agents/<id>/`: spec.md, plan.md,
     progress.json, chatlog.md, handoffs.md, review.md
  4. `kei-ledger done <id>` OR `kei-ledger fail <id>` on completion
  5. `kei-ledger validate` before merge ceremony
- **Surgical scope.** new-project writes only to: ledger DB, `.claude/agents/<id>/`
  bundles it orchestrates, and a project manifest file it creates under
  `_manifests/project-<slug>.toml` if Phase 1b `MAIN_AGENT != "compose-solution decides"`.

---

## References

- [phase-1-intake.md](phase-1-intake.md) · [phase-2-fork-skeleton.md](phase-2-fork-skeleton.md) · [phase-3-parallel-exec.md](phase-3-parallel-exec.md) · [phase-4-merge-ceremony.md](phase-4-merge-ceremony.md)
- `skills/compose-solution/SKILL.md` — per-sub-task kit-artefact composer
- `skills/new-agent/SKILL.md` — 8-phase specialist manifest wizard
- `_primitives/_rust/kei-ledger/` — SQLite ledger CLI (`kei-ledger init / fork / done / fail / list / tree / validate`)
- `hooks/agent-fork-logger.sh` — PreToolUse:Agent advisory logger (auto-fork row)
- `~/.claude/rules/agent-git-model.md` — RULE 0.12 full text (fork / sub-fork / completion / merge ceremony)
