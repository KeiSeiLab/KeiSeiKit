# Phase 4 — Route

For each finding in `SELECTED`, ask the user which action route to take.
Each selection becomes one entry in `ROUTES`.

## 4a — Per-finding click

For EACH finding in `SELECTED`, emit ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "Action for: <class>  ×<count> [<severity>]?",
      "header": "Route",
      "multiSelect": false,
      "options": [
        {"label": "codify via /escalate-recurrence", "description": "Make this a permanent rule + wiki entry + optional hook (recurring patterns)"},
        {"label": "deep-dive via /debug-deep",       "description": "Run the 5-phase RCA skill (one-off or unknown bugs)"},
        {"label": "create hook",                     "description": "Hand off to /escalate-recurrence but force the hook-only branch (mechanical blocks/enforce/warn)"},
        {"label": "skip (just log)",                 "description": "Append to audit-backlog with a note; no further action this session"},
        {"label": "postpone (remind next session)",  "description": "Keep open; re-surface at the start of the next self-audit"}
      ]
    }
  ]
}
```

## 4b — Handoff rules

Based on the click, append to `ROUTES` one of:

| Click label             | Action                                                            |
|-------------------------|-------------------------------------------------------------------|
| codify                  | Run `/escalate-recurrence` with `CLASS=<event_class>` prefilled   |
| deep-dive               | Run `/debug-deep` with the class as the error description         |
| create hook             | Run `/escalate-recurrence` and select "hook-only" in its phase 2  |
| skip (just log)         | Append `[LOGGED yyyy-mm-dd] <class>` to backlog; no skill handoff |
| postpone                | Append `[POSTPONE yyyy-mm-dd] <class>` to backlog; no handoff     |

Self-audit itself does not perform the handoff action — it emits the
`/<slash-skill>` invocation as a suggested next step. The user runs it.

## 4c — Severity gate

If a finding has `severity == critical` AND the user selected
"postpone" or "skip" — echo one reminder line:

> "Critical finding <class> postponed/skipped. It will resurface next
> session but consider addressing before close."

Do not block; this is advisory only.

## Verify-criterion

- `ROUTES` has exactly one entry per `SELECTED` finding.
- One `AskUserQuestion` call per finding, no batching.
- No actual fixes written — only suggested handoffs printed.
