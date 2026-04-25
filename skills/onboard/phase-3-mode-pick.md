# Phase 3 — Mode Pick

Single AskUserQuestion. The user picks how much confirm-gating they want on
the Phase-4 apply step.

## 3a — Mode click (AskUserQuestion, ONE call)

```json
{
  "questions": [
    {
      "question": "How should Phase 4 apply the proposed candidates?",
      "header": "Mode",
      "multiSelect": false,
      "options": [
        {"label": "Full auto",     "description": "One approval → apply EVERY candidate (agents via /new-agent auto-flow, hooks via /escalate-recurrence auto-flow, primitives queued). Fastest path."},
        {"label": "Step-by-step",  "description": "Per-candidate click: apply / skip / modify. Medium friction — good for mixed-confidence batches."},
        {"label": "Full manual",   "description": "Each candidate fully delegated to /new-agent or /escalate-recurrence with scan-prefilled fields — you walk every wizard phase. Highest control."}
      ]
    }
  ]
}
```

Store as `MODE`. Value is one of `full-auto`, `step-by-step`, `full-manual`.

## 3b — Pre-Phase-4 sanity

Before leaving Phase 3, verify:

- `CANDIDATES` is non-empty OR the user explicitly chose `full-manual`
  (manual mode can start from scratch even if the scan produced nothing).
- `MODE` is set.
- If `CANDIDATES` is empty AND mode is `full-auto` or `step-by-step` →
  re-prompt the user: "Scan found no actionable candidates. Re-run with
  `full-manual` to walk `/new-agent` from scratch, or abort to adjust the
  path scope." Offer three constructive paths:
  (A) Switch to `full-manual` now
  (B) Re-run Phase 1 with a different path
  (C) Abort

## 3c — Multi-project note

If `len(PATHS) > 1` AND `GRANULARITY == "bulk-same-config"`:

- The mode applies at the BULK level: one mode decision covers all
  projects in `PATHS`.
- The Phase-4 apply loop iterates over `PATHS`, but the per-candidate
  click-batch is collapsed into one (e.g. "apply this agent to all 3
  matching projects?").

If `GRANULARITY == "mixed"`:

- Mode applies at per-project level: Phase 4 asks again for mode per
  project. This means the AskUserQuestion count grows — acceptable, user
  opted in.

If `GRANULARITY == "per-project"`:

- Mode is asked fresh per project at the TOP of Phase 4's per-project
  iteration (so each project can pick its own mode). This is NOT a
  per-project re-emit of Phase 3 — just that Phase 4 re-uses 3a's question
  text per project.

## Verify-criterion

- `MODE` is exactly one of the three labels.
- If the user picked a mode that requires at least one candidate, the
  candidate list is non-empty (or the user was re-prompted).
- The AskUserQuestion call count for Phase 3 is exactly 1 per mode
  decision — if multi-project mixed-granularity, it may fire N times
  total (once per project) across the full skill run, which is explicit
  and logged in the final report.
