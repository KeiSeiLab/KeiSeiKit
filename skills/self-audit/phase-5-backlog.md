# Phase 5 — Backlog

Update `~/.claude/memory/audit-backlog.md`: increment the session counter,
mark processed findings as done, leave postponed ones open.

## 5a — Increment session counter

Read the `<!-- session_count: N -->` header. Rewrite the same line with
`N + 1`. This advances the silent-first threshold by one session.

## 5b — Append per-finding notes

For each finding, append one line based on its `ROUTES` entry:

- `codify` → `- [CODIFIED yyyy-mm-dd] <class> ×<count>  → /escalate-recurrence`
- `deep-dive` → `- [DEEP-DIVE yyyy-mm-dd] <class>  → /debug-deep`
- `create hook` → `- [HOOK-ONLY yyyy-mm-dd] <class>  → /escalate-recurrence (hook branch)`
- `skip` → `- [LOGGED yyyy-mm-dd] <class> ×<count>`
- `postpone` → `- [POSTPONE yyyy-mm-dd] <class> ×<count>  (resurface next session)`

If Phase 3 short-circuited (silent-first OR private-content guard), append all
findings with the `[SELF-AUDIT SILENT]` or `[SELF-AUDIT OFFLINE]` prefix.

## 5c — Clear processed items click

Emit ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "Clear processed (codified / deep-dived / logged) items from backlog?",
      "header": "Clear",
      "multiSelect": false,
      "options": [
        {"label": "Yes — clear now",      "description": "Mark processed=1 in kei-memory backlog table"},
        {"label": "No — keep for review", "description": "Review later before clearing"}
      ]
    }
  ]
}
```

On "Yes" — run `kei-memory backlog --clear`. On "No" — no-op.

## 5d — Emit final report

Print the final report (format from `SKILL.md`).

## Verify-criterion

- `<!-- session_count: N -->` header incremented by exactly 1.
- Every finding has a backlog line (appended in 5b).
- Exactly one `AskUserQuestion` call in 5c.
- Final report printed after backlog write.
