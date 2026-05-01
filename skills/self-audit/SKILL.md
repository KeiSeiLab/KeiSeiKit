---
name: self-audit
description: Session retrospective triage (RULE 0.14). Runs `kei-memory analyze + patterns`, classifies findings, presents them via click-only AskUserQuestion, routes each selected item to `/escalate-recurrence` (rule+hook), `/debug-deep` (bug RCA), or the audit-backlog (log-only). Self-audit is triage, not implementation.
argument-hint: <optional session id; defaults to last session>
---

# Self-Audit — Session Retrospective Triage (index)

You are running the RULE 0.14 self-audit on the last (or named) session.
You convert the session's trace into a short list of findings, classify
each, present them as a multi-select click batch, and route each selection
to the appropriate existing skill. You NEVER write fixes yourself.

This `SKILL.md` is the INDEX. Each phase lives in its own file and is
executed in order. Never skip a phase. Never re-order phases.

---

## Pipeline overview (5 phases)

| Phase | File | Purpose | AskUserQuestion |
|---|---|---|---|
| 1 | [phase-1-analyze.md](phase-1-analyze.md) | Run `kei-memory analyze` + `kei-memory patterns`; collect findings | 0 |
| 2 | [phase-2-classify.md](phase-2-classify.md) | Categorise each finding as recurring / one-off / unknown + severity | 1× AskUserQuestion (severity confirm) |
| 3 | [phase-3-present.md](phase-3-present.md) | Multi-select click: which findings to address | 1× AskUserQuestion |
| 4 | [phase-4-route.md](phase-4-route.md) | For each selected finding → pick action route | 1× AskUserQuestion per selected finding |
| 5 | [phase-5-backlog.md](phase-5-backlog.md) | Update `~/.claude/memory/audit-backlog.md`; clear processed items | 1× AskUserQuestion (confirm backlog clear) |

Minimum AskUserQuestion count: **4** (Phase 2, 3, at least one Phase 4,
Phase 5). This is the pure-click contract.

---

## Variables the pipeline produces

| Name | Set in | Meaning |
|---|---|---|
| `SESSION` | Phase 1 | Session id under audit (CLI arg or `--last 1`) |
| `FINDINGS` | Phase 1 | List of `{class, count, severity_hint, scope}` dicts |
| `CLASSIFIED` | Phase 2 | Same list, with `category ∈ {recurring, one-off, unknown}` + `severity ∈ {critical, high, medium, low}` |
| `SELECTED` | Phase 3 | User-picked subset of `CLASSIFIED` to address |
| `ROUTES` | Phase 4 | Per-finding chosen action ∈ {codify, deep-dive, hook-only, log-only, postpone} |
| `BACKLOG_ACTIONS` | Phase 5 | Which backlog entries to mark processed |

---

## Final report (emit after Phase 5)

```
=== SELF-AUDIT REPORT ===
Session:       <SESSION>
Findings:      <N total>  (recurring: <R>, one-off: <O>, unknown: <U>)
Routed:        <K>
  → codify:    <count>  (handed off to /escalate-recurrence)
  → deep-dive: <count>  (handed off to /debug-deep)
  → hook-only: <count>  (created hook stub — NOT registered)
  → log-only:  <count>  (appended to audit-backlog.md)
  → postpone:  <count>  (kept open, will resurface next session)
Backlog:       <before_count> → <after_count> unprocessed items
```

---

## Rules (apply throughout — enforced at every phase)

- **Triage, not implementation.** This skill NEVER writes production
  code. It hands off to `/escalate-recurrence` (rule + wiki + hook) or
  `/debug-deep` (5-phase RCA) or logs to backlog. Any edit in this skill
  is limited to `~/.claude/memory/audit-backlog.md`.
- **Pure-click contract.** Only the handoff targets may ask for
  free-text; every decision in self-audit itself is `AskUserQuestion`.
- **NO DOWNGRADE (RULE -1).** If `kei-memory` is not installed, return
  2-3 constructive paths (install the primitive, run the analysis by
  hand on the JSONL, skip this session) — never "cannot audit".
- **NO HALLUCINATION (RULE 0.4).** Every finding cited in Phase 3 must
  come from the `kei-memory patterns` output captured in Phase 1.
  Never invent a class that wasn't emitted.
- **Silent-first (RULE 0.14).** If `<!-- session_count: N -->` in
  `~/.claude/memory/audit-backlog.md` is less than 10, Phase 3 MUST
  short-circuit to "log only" — do not prompt the user.
- **Sensitive-IP exception.** If CWD sits under a restricted-list project
  (see `~/.claude/rules/security.md`) OR `CLAUDE.md` in CWD contains a
  banned marker, run Phase 1 ONLY and stop: do not inject transcript
  excerpts back into chat.
- **Constructor Pattern (RULE ZERO).** Every phase file ≤ 60 LOC.

---

## References

- `~/.claude/rules/session-self-audit.md` — RULE 0.14 full text
- `~/.claude/skills/escalate-recurrence/SKILL.md` — codify route target
- `skills/debug-deep/SKILL.md` — deep-dive route target
- `_primitives/_rust/kei-memory/` — analyzer primitive
- `hooks/session-end-dump.sh`, `hooks/milestone-commit-hook.sh`,
  `hooks/error-spike-detector.sh` — auto-triggers
