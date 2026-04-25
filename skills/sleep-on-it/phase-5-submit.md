# Phase 5 — Preview and submit (click)

Show the user exactly what will be written, then submit via the helper.

## 5a — Render preview

Print a fenced block with the frontmatter + body preview:

```
---
uuid: <generated-after-submit>
submitted_at: <generated-after-submit>
type: <TASK_TYPE>
priority: <PRIORITY_LABEL>
format: <FORMAT>
time_budget_minutes: <TIME_BUDGET_MINUTES>
checkpoint_every_minutes: <CHECKPOINT_EVERY_MINUTES>
marathon: <MARATHON>
status: pending
---

<TASK_TEXT>
```

Then print a one-line wall-clock estimate:

```
estimated wall-clock: <TIME_BUDGET_MINUTES> min
```

If `MARATHON == true`, append an explicit warning line beneath the
estimate:

```
marathon: Phase B REM consolidation will be SKIPPED the night this
task runs, and other queue items will be deferred to the next night.
```

Tell the user the `uuid` and `submitted_at` fields are assigned by the
helper on submit — the preview leaves them as placeholders.

## 5b — Click

Emit ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "Submit this to the nightly queue?",
      "header": "Submit",
      "multiSelect": false,
      "options": [
        {"label": "Submit",  "description": "Write queue file + push to memory-repo"},
        {"label": "Edit",    "description": "Go back to Phase 1 and re-enter the task text"},
        {"label": "Abort",   "description": "Drop the draft; nothing is written"}
      ]
    }
  ]
}
```

Store the pick as `SUBMIT_ACTION`.

## 5c — Dispatch

- `SUBMIT_ACTION == "Edit"` → restart from Phase 1 (clears all variables).
- `SUBMIT_ACTION == "Abort"` → print `submission cancelled` and exit.
- `SUBMIT_ACTION == "Submit"` → call the helper (see 5d).

## 5d — Invoke `kei-sleep-queue.sh add`

Write the task text to a temp file, then:

```bash
PROMPT_FILE="$(mktemp)"
printf '%s\n' "$TASK_TEXT" > "$PROMPT_FILE"
MARATHON_FLAG=""
[ "$MARATHON" = "true" ] && MARATHON_FLAG="--marathon"
OUTPUT="$(
  ~/.claude/agents/_primitives/kei-sleep-queue.sh add \
    --type "$TASK_TYPE" \
    --priority "$PRIORITY_LABEL" \
    --format "$FORMAT" \
    --time-budget "${TIME_BUDGET_MINUTES}m" \
    --checkpoint-every "${CHECKPOINT_EVERY_MINUTES}m" \
    $MARATHON_FLAG \
    --prompt-file "$PROMPT_FILE" 2>&1
)"
STATUS=$?
rm -f "$PROMPT_FILE"
```

The helper prints two lines on success:

```
<uuid>
<absolute path of queue file>
```

Capture `UUID = first line`, `QUEUE_PATH = second line`.

On non-zero exit, surface stderr verbatim. Common causes:

- **write failed** (disk / permissions) → print the error; exit.
- **sync push failed after local write succeeded** → not an error; the
  queue file IS committed locally and will push on next session end.

## Verify-criterion

- `SUBMIT_ACTION ∈ {Submit, Edit, Abort}`.
- If `Submit`, `UUID` is a non-empty string and `QUEUE_PATH` ends in
  `.md` under `sync-repo/sleep-queue/`.
- Exactly ONE `AskUserQuestion` in this phase.
