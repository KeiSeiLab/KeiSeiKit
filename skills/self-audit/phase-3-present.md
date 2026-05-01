# Phase 3 — Present

Show `CLASSIFIED` to the user as a multi-select click batch. User picks
which findings to address; selection becomes `SELECTED`.

## 3a — Silent-first guard

Read `~/.claude/memory/audit-backlog.md`. Parse the
`<!-- session_count: N -->` header. If `N < 10`:

- Log every finding to the backlog with a `[SELF-AUDIT SILENT]` prefix.
- Set `SELECTED = []` and SKIP to Phase 5.

This is the RULE 0.14 silent-first contract. Do NOT prompt the user.

## 3b — Sensitive-IP guard

If CWD sits under a banned project (`~/Projects/my-project`) OR a
`CLAUDE.md` in CWD contains a banned-marker line
matching `/restricted-project|sensitive-ip/i`:

- Log every finding to backlog with `[SELF-AUDIT OFFLINE]` prefix.
- Set `SELECTED = []` and SKIP to Phase 5.

Do NOT render transcript excerpts back to chat.

## 3c — Multi-select click

Emit ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "Which findings do you want to address this session?",
      "header": "Findings",
      "multiSelect": true,
      "options": [
        {"label": "<class-1>  ×<count> [severity]", "description": "<scope>"},
        {"label": "<class-2>  ×<count> [severity]", "description": "<scope>"},
        ...
        {"label": "None — just log to backlog",      "description": "Append all to backlog, pick up later"}
      ]
    }
  ]
}
```

Cap the option list at 8 findings (highest `count` first). If more exist,
add a trailing option `"Show full list"` that dumps all of them to stdout
and re-emits the click batch on the next turn.

## Verify-criterion

- Exactly one `AskUserQuestion` call was emitted (unless guard fired).
- `SELECTED` is a list of finding dicts (possibly empty).
- "None — just log to backlog" treated as `SELECTED = []`.
