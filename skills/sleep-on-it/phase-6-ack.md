# Phase 6 — Acknowledgment (click)

Show the user what was written, how to inspect it, and when the agent
picks it up.

## 6a — Resolve next-run ETA

Read the cron expression the `/schedule` trigger uses. The helper keeps
the URL in `sync-repo/.keisei-sync.toml`; the cron target itself is in
the user's Claude Code `/schedule list`. We do not parse the scheduler
from this skill (no portable API) — instead, print the canonical line:

```bash
grep -E '^schedule_utc_cron' "$REPO_PATH/.keisei-sync.toml" 2>/dev/null \
    || printf 'schedule_utc_cron = "unknown — run /schedule list to verify"\n'
```

If the key is absent (older sync-repo), print the fallback line
verbatim. Do not fabricate a time (RULE 0.4).

## 6b — Print acknowledgment block

Emit this block to chat:

```
Queued.

  UUID:        <UUID>
  File:        <QUEUE_PATH>
  Type:        <TASK_TYPE>
  Priority:    <PRIORITY>
  Format:      <FORMAT>
  Next run:    <cron line from 6a>
  Results at:  <REPO_PATH>/sleep-results/<UUID>.md (after the next run)

Inspect:  `kei-sleep-queue show <UUID>`
List all: `kei-sleep-queue list --pending`
Cancel:   delete the file at the path above before the next run
```

## 6c — Click (final)

Emit ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "What now?",
      "header": "Done",
      "multiSelect": false,
      "options": [
        {"label": "Show queue",    "description": "Run `kei-sleep-queue list --pending`"},
        {"label": "Submit another", "description": "Restart the wizard"},
        {"label": "Done",          "description": "Close the wizard"}
      ]
    }
  ]
}
```

Handle each option:

- `Show queue`     → shell out to `kei-sleep-queue list --pending` and
                     print the table; then re-emit this click.
- `Submit another` → restart the wizard from Phase 1.
- `Done`           → emit the final report from `SKILL.md` and exit.

## Verify-criterion

- The acknowledgment block in 6b was printed with real values (no `<UUID>`
  placeholders left).
- Exactly ONE `AskUserQuestion` in this phase (plus loops on "Show queue").
- The final report block from `SKILL.md` was emitted on `Done`.
