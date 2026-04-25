---
name: onboard
description: Scan a project (or scope of projects) and propose candidate agents, hooks, and primitives based on detected stack, CI, deploy, tests, and env surface. Three modes — Full auto, Step-by-step, Full manual — reuse the same scan+propose foundation and differ only in confirm-gate count. Delegates to /new-agent, /escalate-recurrence, and kei-sleep-queue.
argument-hint: <project-path or glob scope>
---

# Onboard — Auto Project Analysis (index)

You are analysing an existing project (or a scope of projects) and proposing
the right kit artefacts for it: project-specialist agents, stack-specific
hooks, and install-time primitives.

This skill is a **proposer + applier**, not a manifest writer. It
scans, scores, proposes, and then delegates each accepted candidate to the
existing pipeline:

- **Agents** → handoff to `/new-agent` (the 8-phase wizard)
- **Hooks + rules** → handoff to `/escalate-recurrence`
- **Primitives** → `kei-sleep-queue add` (or direct `install.sh --add=`
  suggestion in the final report)

This `SKILL.md` is the INDEX. Each phase lives in its own file and runs in
strict order. Never skip or re-order phases.

---

## Prerequisite check

Before Phase 1, verify the kit baseline is present:

- `~/.claude/skills/new-agent/SKILL.md` (or the kit-shipped
  `skills/new-agent/SKILL.md`) — required for agent delegation
- `~/.claude/skills/escalate-recurrence/SKILL.md` — required for hook/rule
  delegation
- `~/.claude/skills/compose-solution/SKILL.md` — required only when a
  proposed candidate crosses artefact boundaries

If any are missing, stop and tell the user: "Install KeiSeiKit v0.12+ first
(run `install.sh --profile=dev` from the kit repo)." Do not fall through.

---

## Pipeline overview (5 phases + final report)

| Phase | File | Purpose | AskUserQuestion |
|---|---|---|---|
| 1 | [phase-1-scan.md](phase-1-scan.md) | Free-text intake + Bash scan of artefacts | 1× (scope granularity, multi-project only) |
| 2 | [phase-2-propose.md](phase-2-propose.md) | Analyse scan → propose N agents, M hooks, K primitives | 0 |
| 3 | [phase-3-mode-pick.md](phase-3-mode-pick.md) | Pick Full auto / Step-by-step / Full manual | 1× |
| 4 | [phase-4-apply.md](phase-4-apply.md) | Apply by mode (each mode has its own confirm-gates) | 1-N× |
| 5 | [phase-5-report.md](phase-5-report.md) | Summary + suggested next steps | 0 |

Minimum AskUserQuestion count across the 5 phases: **6** — 1 (Phase 1
scope-granularity if multi-project), 1 (Phase 3 mode), at least 4 inside
Phase 4 (mode-specific confirms: Full-auto has 1; Step-by-step has ≥N per
candidate; Full manual delegates per candidate, each wizard emitting its
own AskUserQuestion calls).

---

## Variables the pipeline produces

| Name | Set in | Meaning |
|---|---|---|
| `PATHS` | Phase 1a | One or more project root paths (glob-resolved) |
| `SCAN` | Phase 1b | Per-project scan summary: stack, CI, deploy, tests, README-purpose, recent activity, env-var surface |
| `CANDIDATES` | Phase 2 | List of (kind, name, confidence, rationale) tuples — kind ∈ {agent, hook, primitive} |
| `MODE` | Phase 3 | full-auto / step-by-step / full-manual |
| `APPLIED` | Phase 4 | Candidates that were applied (path + delegation target) |
| `SKIPPED` | Phase 4 | Candidates the user declined |

---

## Final report (emit after Phase 5)

```
=== ONBOARD REPORT ===
Scope:       <1 project | N projects>
Scan:        <stack(s) | CI | deploy | tests | env-vars>
Proposed:    <N agents | M hooks | K primitives>
Mode:        <full-auto | step-by-step | full-manual>
Applied:     <N agents | M hooks | K primitives>
Skipped:     <count>
Next:        <e.g. "run install.sh --profile=dev to enable /schema-design if DB detected">
```

---

## Rules (apply throughout)

- **RULE 0.4 (NO HALLUCINATION).** Phase-1 scan uses exact grep/ls output.
  If grep returns nothing for a framework, mark it as "not detected". Never
  invent a framework based on directory names or README prose alone.
  Confidence grade (E3-E4 for scan-derived, never E1-E2).
- **RULE 0.8 (SECRETS).** Never read actual `.env` or `secrets/*.env` files.
  Read only `.env.example`, `.env.template`, and schema files. The env-var
  surface in Phase 1b is the list of KEY names, never values.
- **RULE 0.13 (NO GIT).** This skill does not invoke git. The orchestrator
  (or the user in a follow-up turn) handles commits.
- **NO DOWNGRADE.** Any phase that fails returns 2-3 constructive paths,
  never "can't be done". Example: scan finds no lockfile → propose
  "(A) Ask user to specify stack, (B) Scan for code-file extensions, (C)
  Skip stack-specialisation, propose generic agent only".
- **Plan Mode First.** This skill IS the plan; each phase has a
  verify-criterion. No Edit/Write before the corresponding phase's
  confirm click.
- **Constructor Pattern.** Each phase file is a single cube. This index
  stays ≤200 LOC; phase files each ≤150 LOC. Candidates that exceed
  complexity budget at application time are split — never stuffed into
  one manifest.
- **Surgical Changes.** Onboard writes nothing directly — all writes are
  delegated to `/new-agent`, `/escalate-recurrence`, or `kei-sleep-queue`.
  The only in-place artefacts onboard produces are ephemeral scan summaries
  displayed in the conversation.

---

## References

- [phase-1-scan.md](phase-1-scan.md) · [phase-2-propose.md](phase-2-propose.md)
  · [phase-3-mode-pick.md](phase-3-mode-pick.md) · [phase-4-apply.md](phase-4-apply.md)
  · [phase-5-report.md](phase-5-report.md)
- `skills/new-agent/SKILL.md` — agent delegation target (Phase 4)
- `~/.claude/skills/escalate-recurrence/SKILL.md` — hook/rule delegation
  target (Phase 4)
- `skills/compose-solution/SKILL.md` — cross-artefact delegation (Phase 4
  fallback)
- `_primitives/MANIFEST.toml` — primitive catalogue (Phase 2 lookup source)
- `_primitives/kei-sleep-queue.sh` — primitive install queueing
- `_blocks/stack-*.md`, `_blocks/deploy-*.md`, `_blocks/ci-*.md` — Phase 2
  block suggestions per detected stack
- `_manifests/kei-*.toml` — 12 kit agents (Phase 2 handoff references)
