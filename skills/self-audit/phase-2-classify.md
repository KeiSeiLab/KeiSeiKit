# Phase 2 — Classify

Tag each finding with a category and a severity; `CLASSIFIED` is produced.

## 2a — Category (automatic, no click)

For each finding in `FINDINGS`:

- `scope == "in-session"` AND `count ≥ 2` → `category = recurring`
- `scope == "cross-session"` AND `count ≥ 2` → `category = recurring`
- `scope == "in-session"` AND `count == 1` → `category = one-off`
- otherwise → `category = unknown`

## 2b — Severity hint (automatic heuristic)

Grep the finding's `event_class`:

| Contains                             | Severity   |
|--------------------------------------|------------|
| `permission_denied`, `panic`, `security` | `critical` |
| `error`, `failed`, `timeout`, `worktree_error` | `high`     |
| `cargo_workspace`, `tool_use:*`      | `medium`   |
| anything else                        | `low`      |

## 2c — Severity confirm click (single AskUserQuestion)

Emit ONE `AskUserQuestion` batch grouping the severity confirm into a
single question:

```json
{
  "questions": [
    {
      "question": "Confirm severity for top finding?",
      "header": "Severity",
      "multiSelect": false,
      "options": [
        {"label": "critical", "description": "Security / data loss / irreversible"},
        {"label": "high",     "description": "Blocks work or leaks to production"},
        {"label": "medium",   "description": "Slows work; fix this week"},
        {"label": "low",      "description": "Nice to fix; not urgent"}
      ]
    }
  ]
}
```

Apply the user's pick only to the TOP finding (highest `count`). All
other findings keep their heuristic severity.

## Verify-criterion

- Every finding has a `category` and a `severity`.
- `CLASSIFIED` is the full list with those two fields added.
- Exactly one `AskUserQuestion` call was emitted.
