# Pipeline 5-Phase Wizard Template (shared preamble)

> Shared contract for hub-and-spoke pipeline skills. Referenced by:
> `/ci-scaffold`, `/auth-setup`, `/observability-setup`, `/docs-scaffold`,
> `/schema-design`. Skill-specific phase tables stay inline in each skill.

## Hub-and-spoke pipeline contract

A pipeline skill is the **INDEX** ("hub"). Each phase lives in a separate
file in the skill directory ("spoke") and runs in a fixed order. Never skip
a phase. Never re-order. Each phase has exactly one `AskUserQuestion` call
(Phase 1 may batch multiple questions into one call via the `questions`
array).

## Minimum AskUserQuestion count

≥ 5 across a full session (one per phase). Phase 1 intake typically bundles
3–5 related questions into a single `AskUserQuestion` call per native
protocol — that still counts as one "call" for the minimum-5 contract.

## Phase conventions

| Phase | Purpose |
|---|---|
| 1 — Intake | Typed input (one-line description). Bundle 3-5 click-questions in one `AskUserQuestion`. |
| 2 — Decomposition | First structural decision (matrix / entities / identity / instrumentation). One click. |
| 3 — Generation | Emit primary artefact (workflow YAML / DDL / session-config / scaffold files / scrape-ship wiring). One click to approve/revise. |
| 4 — Integration | Secondary artefact wired to runtime (migrations / secrets / auth-z / dashboards). One click. |
| 5 — Verify / Hardening | Run linter / alert rules / threat checklist / seed / changelog init. One click per finding. |

## Final report format

Every pipeline skill emits a final report block after Phase 5:

```
=== <SKILL-NAME> REPORT ===
<Key: Value pairs, one per produced variable>
Next: <one-line handoff instruction>
```

## Universal rules (apply to all pipeline skills)

- **Pure-click contract** — only Phase 1 intake is typed. See
  `_blocks/rule-pure-click-contract.md`.
- **RULE 0.8 Secrets SSoT** — emit env-var NAMES only, never values.
  Storage path via `_blocks/domain-has-secrets.md`.
- **RULE 0.4 NO HALLUCINATION** — every cited primitive / block /
  dashboard ID must exist or be verified in-session.
- **NO DOWNGRADE (RULE -1)** — if a combination is unsafe, return 2-3
  constructive alternatives. Never "not supported".
- **Surgical scope** — write only the files the skill's `Rules` section
  lists. Never touch application source beyond the minimum init call.
- **Constructor Pattern** — `SKILL.md` < 200 LOC. Each phase file < 100 LOC.
- **Fail-closed default** — unknown inputs → no emission until user clicks.
