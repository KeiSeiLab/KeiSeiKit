# Phase 2 — Task type (click)

Map the free-text task to one of five dispatch categories. The remote
agent's Phase A uses this to pick the right tool chain.

## 2a — Click

Emit ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "How should the nightly agent approach this task?",
      "header": "Type",
      "multiSelect": false,
      "options": [
        {
          "label": "Deep research",
          "description": "WebSearch + WebFetch + synthesis — 3+ searches, 2+ page fetches, structured report"
        },
        {
          "label": "Pipeline design",
          "description": "Architect + critic sequence — 5-7 phases with verify-criteria and tradeoffs"
        },
        {
          "label": "Pattern analysis",
          "description": "Query kei-memory + past reports — extract trends across sessions, propose action"
        },
        {
          "label": "Comparative study",
          "description": "Pros/cons matrix across N options the user lists — weighted recommendation"
        },
        {
          "label": "Custom",
          "description": "Follow the task text verbatim — no dispatch tool, free-form response"
        }
      ]
    }
  ]
}
```

## 2b — Normalise

Map the clicked label to a compact token the queue file stores:

| Label | Token |
|---|---|
| Deep research | `deep` |
| Pipeline design | `pipeline` |
| Pattern analysis | `pattern` |
| Comparative study | `compare` |
| Custom | `custom` |

Store as `TASK_TYPE`.

## 2c — Soft nudge on mismatch

If `TASK_TYPE == "custom"` AND `TASK_TEXT` contains any of
`should I | compare | trade[- ]off | which is better`, print a soft hint:

> Your task text looks like a comparison — `compare` or `deep` usually
> produce a stronger result than `custom`. Proceeding anyway.

Do NOT re-ask. One nudge, user keeps control.

## Verify-criterion

- `TASK_TYPE ∈ {deep, pipeline, pattern, compare, custom}`.
- Exactly ONE `AskUserQuestion` in this phase.
