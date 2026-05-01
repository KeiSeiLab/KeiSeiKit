# Phase 0 — Sleep mode pick

Ask the user to pick the execution mode for nightly sleep-layer
consolidation. This is the FIRST phase of the wizard — it runs BEFORE
Phase 1 and branches the entire pipeline.

## 0a — Mode click

Emit ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "How should sleep-layer consolidation run?",
      "header": "Sleep mode",
      "multiSelect": false,
      "options": [
        {"label": "Local-only",  "description": "macOS CronCreate on this Mac; full access to ~/.claude/memory/ and /self-audit. No git repo needed."},
        {"label": "Remote-only", "description": "Cloud agent via /schedule; git-repo based; morning git pull to read the report. Mac can sleep."},
        {"label": "Hybrid",      "description": "Both. Local cron does the deep analysis; remote is redundancy when Mac is asleep. Both paths are idempotent."}
      ]
    }
  ]
}
```

Store the pick as `SLEEP_MODE` ∈ {`local-only`, `remote-only`, `hybrid`}.

## 0b — Branching

Branch the remainder of the pipeline based on `SLEEP_MODE`:

- **`local-only`** — skip Phases 1, 2, 3, 4 entirely. No git provider,
  no SSH key, no repo URL, no deploy-key walkthrough, no test push.
  Jump directly to Phase 0b (time) → Phase 3b (deep-sleep cadence,
  adapted for local per phase-3b-deep-sleep.md) → Phase 5 (trigger,
  emits only CronCreate).
- **`remote-only`** — proceed through Phase 0b (time) → Phase 1 →
  Phase 2 → Phase 3 → Phase 3b → Phase 4 → Phase 5 (emits only
  `/schedule create`).
- **`hybrid`** — same full pipeline as `remote-only`, but Phase 5
  emits BOTH a CronCreate block AND a `/schedule create` block, with
  two sequential AskUserQuestions (one per trigger path).

## 0c — Implication note

Print a one-line reminder before continuing:

```
SLEEP_MODE = <pick>. Local cron uses this Mac; remote trigger uses a
cloud Claude Code agent. Hybrid runs both; the two paths write to
different paths and are idempotent.
```

No second AskUserQuestion — the pick is final. Re-running the wizard
lets the user change modes later.

## Verify-criterion

- Exactly ONE `AskUserQuestion` in this phase.
- `SLEEP_MODE ∈ {local-only, remote-only, hybrid}`.
- If `local-only`, Phases 1-4 MUST be skipped by the caller.
- If `remote-only` or `hybrid`, the full pipeline continues at Phase 0b.
