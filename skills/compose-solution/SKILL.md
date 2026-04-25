---
name: compose-solution
description: Meta-orchestrator — converts a free-text task description into the right artefact(s) (agent, skill, hook, rule, or block) by composing existing KeiSeiKit primitives. Pure-click decision chain except the single intake field. Enriches `_blocks/` over time via Phase 6 block-augmentation — the kit gets smarter with every invocation.
argument-hint: <free-text task description>
---

# Compose-Solution — Meta-Orchestrator (index)

You are converting an arbitrary user task ("I want to solve X") into the
right durable KeiSeiKit artefact — an agent manifest, a skill, a hook, a
rule, or a new behavioural block. You decompose, grep prior art,
gap-analyse, compose, and assemble. Every decision is a click; only the
intake description (and an optional free-text edit in Phase 6) is typed.

This skill is the **meta-creator**: it does not itself write production
code. It routes to `new-agent` (agent branch), to `escalate-recurrence`
(hook/rule branch), or composes a new skill/block in-place.

This `SKILL.md` is the INDEX. Each phase lives in its own file and is
executed in order. Never skip a phase. Never re-order phases.

---

## Pipeline overview (7 phases + final report)

| Phase | File | Purpose | Free-text? | AskUserQuestion |
|---|---|---|---|---|
| 1 | [phase-1-intake.md](phase-1-intake.md) | Intake + target-type click | 1 line (`DESC`) | 1× AskUserQuestion |
| 2 | [phase-2-decompose.md](phase-2-decompose.md) | Wave-based decomposition | no | 1× AskUserQuestion |
| 3 | [phase-3-prior-art.md](phase-3-prior-art.md) | Prior-art grep sweep | no | 0 |
| 4 | [phase-4-gap-analysis.md](phase-4-gap-analysis.md) | Gap analysis (multi-select) | no | 1× AskUserQuestion |
| 5 | [phase-5-architecture.md](phase-5-architecture.md) | Architecture (math-first) | no | 1× AskUserQuestion |
| 6 | [phase-6-block-augment.md](phase-6-block-augment.md) | Block augmentation | optional per-block | 1× AskUserQuestion per new block |
| 7 | [phase-7-assemble.md](phase-7-assemble.md) | Recipe assembly (branches on `T`) | no | 1× AskUserQuestion |

Minimum AskUserQuestion count across a full session: **6** — one each in
Phases 1, 2, 4, 5, at least one in Phase 6 (if any gaps selected), and one
in Phase 7. This is the pure-click contract: only intake is free-text.

---

## Variables the pipeline produces

| Name | Set in | Meaning |
|---|---|---|
| `DESC` | Phase 1a | The user's one-paragraph task description |
| `T` | Phase 1b | Target artefact type — Agent / Skill / Hook / Rule / Block / Auto-detect |
| `COMPONENTS` | Phase 2 | 2-5 orthogonal components, each with 3-5 grep keywords |
| `CLASSIFICATION` | Phase 3 | Per-component: REUSE / ADAPT / CREATE / EXTERNAL + evidence grade |
| `GAPS` | Phase 4 | User-selected subset of components that need Phase-6 augmentation |
| `ARCHITECTURE` | Phase 5 | Math-first composition expression + block list + Constructor-Pattern check |
| `BLOCKS_WRITTEN` | Phase 6 | Names of newly persisted `_blocks/*.md` files (possibly empty) |
| `FINAL_NAME` | Phase 7 | Path of the assembled artefact (or handoff target) |

---

## Final report (emit after Phase 7)

```
=== COMPOSE-SOLUTION REPORT ===
Intake:         <first 80 chars of DESC>...
Target type:    <T (after auto-detect resolution, if applicable)>
Decomposition:  <N components>
Prior-art:      <M reused, K adapted, L created, X external>
Blocks written: <names>  (kit: <before_count> → <after_count>)
Assembled:      <artefact path or "handed off to <skill>">
Next action:    <what user should run / review / commit>

Future invocations benefit from the K new blocks — kit is now smarter by K blocks.
```

---

## Rules (apply throughout — enforced at every phase)

- **Pure-click contract.** Only `DESC` (Phase 1a) and optional per-block
  edits (Phase 6b "Edit") are typed. Every other decision is an
  `AskUserQuestion` call. Count them in the final report.
- **NO DOWNGRADE.** Any phase that fails returns 2-3 constructive paths,
  never "can't be done".
- **NO HALLUCINATION (RULE 0.4).** Every block / skill / agent / bridge
  name referenced in the session MUST exist on disk. Phase 3 greps, Phase 5
  architecture listing, and Phase 7 handoffs all verify before citing. If
  grep returns nothing — the component class is CREATE, report it, never
  invent a phantom match.
- **Plan Mode First (RULE 0.5).** This skill IS the plan; each phase file
  has its own verify-criterion. No Edit/Write before the corresponding
  phase's confirm click.
- **Constructor Pattern (RULE ZERO).** Every new block is single-concern,
  20-40 LOC, hard-capped at 60 LOC → split. Every new skill phase is < 30
  LOC of imperative prose. This `SKILL.md` index file itself must stay
  < 200 LOC; phase files each < 150 LOC.
- **Surgical Changes.** Compose-solution writes only to:
  - `_blocks/<slug>.md` (Phase 6, user-approved)
  - `skills/<slug>/SKILL.md` (Phase 7c, user-approved)
  - Hands off to `new-agent` (Phase 7b) or `escalate-recurrence`
    (Phase 7d/e) — no direct writes to `_manifests/`, `~/.claude/rules/`,
    or `~/.claude/hooks/`.
- **Kit-enrichment feedback loop.** Phase 6 is the virtuous cycle: every
  missing block becomes a new permanent block, so the next invocation of
  compose-solution (or `new-agent`) finds more prior art in Phase 3.
  Report the before/after block count in every session that touched Phase
  6 — this makes the loop visible.

---

## References

- [phase-1-intake.md](phase-1-intake.md) · [phase-2-decompose.md](phase-2-decompose.md) · [phase-3-prior-art.md](phase-3-prior-art.md) · [phase-4-gap-analysis.md](phase-4-gap-analysis.md) · [phase-5-architecture.md](phase-5-architecture.md) · [phase-6-block-augment.md](phase-6-block-augment.md) · [phase-7-assemble.md](phase-7-assemble.md)
- `skills/research/SKILL.md` — Variant C "Deep decomposition" wave pattern
  (Phase 2 delegation target for heavy tasks)
- `skills/new-agent/SKILL.md` — 8-phase wizard (Phase 7b handoff target)
- `~/.claude/skills/escalate-recurrence/SKILL.md` — hook + rule + wiki
  pipeline (Phase 7d/e handoff target)
- `~/.claude/skills/architecture/SKILL.md` — optional, for heavy
  architectural decomposition if `research` is overkill
- `_blocks/baseline.md`, `_blocks/rule-math-first.md` — block templates
  (Phase 6a shape references)
- `_manifests/kei-*.toml` — 12 kit agents (Phase 7b handoff references)
- `_bridges/*.tmpl` — 11 tool bridges (architecture Phase 5 may reference
  them for agent-creation flows)
