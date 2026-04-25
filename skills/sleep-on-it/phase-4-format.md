# Phase 4 — Output format (click)

Tell the remote agent how to shape `sync-repo/sleep-results/<uuid>.md`.

## 4a — Click

Emit ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "What format should the result use?",
      "header": "Format",
      "multiSelect": false,
      "options": [
        {
          "label": "Structured markdown report",
          "description": "Sections + findings + sources — default, best for research / pattern analysis"
        },
        {
          "label": "ADR-style decision record",
          "description": "Context / Decision / Consequences — best for pipeline-design output"
        },
        {
          "label": "Checklist / action items",
          "description": "`- [ ] item` list — best when you want a morning TODO"
        },
        {
          "label": "Pros/cons table",
          "description": "Markdown table with weighted criteria — best for comparative study"
        }
      ]
    }
  ]
}
```

## 4b — Normalise

| Label | Token |
|---|---|
| Structured markdown report | `md` |
| ADR-style decision record | `adr` |
| Checklist / action items | `checklist` |
| Pros/cons table | `table` |

Store as `FORMAT`.

## 4c — Coherence hint

Soft hint only (no re-ask), when type and format drift apart:

- `TASK_TYPE == compare` and `FORMAT != table` → hint that `table` is the usual pick.
- `TASK_TYPE == pipeline` and `FORMAT != adr` → hint that `adr` is the usual pick.
- `TASK_TYPE == pattern` and `FORMAT != checklist` → hint that `checklist` often reads best.

Format: single line, does not block.

## Verify-criterion

- `FORMAT ∈ {md, adr, checklist, table}`.
- Exactly ONE `AskUserQuestion` in this phase.
