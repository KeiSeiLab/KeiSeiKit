# Phase 3 — Priority & time budget (click)

Decide how much night-time the remote agent spends on this task.
Priority maps to a wall-clock budget; the pipeline reads the budget
from frontmatter, so a hard equation can own the whole night while a
quick lookup is boxed to 15 minutes.

## 3a — Click

Emit ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "How much night-time should this task get?",
      "header": "Priority",
      "multiSelect": false,
      "options": [
        {
          "label": "Quick",
          "description": "15 min, this night — simple questions, fast lookups"
        },
        {
          "label": "Standard",
          "description": "60 min, this night — default, medium research"
        },
        {
          "label": "Deep",
          "description": "4 hour, this night — serious derivations, thorough prior-art"
        },
        {
          "label": "Marathon",
          "description": "Full night, 1 task only — hard equations, full autonomy; Phase B REM skipped this night"
        },
        {
          "label": "Weekly batch",
          "description": "60 min, processed next Sunday UTC — non-urgent research"
        }
      ]
    }
  ]
}
```

## 3b — Marathon confirmation

If `LABEL == "Marathon"`, emit ONE more `AskUserQuestion` so the user
consciously accepts the cost:

```json
{
  "questions": [
    {
      "question": "Marathon = this task owns the whole night. Phase B REM consolidation is skipped. Other queue tasks deferred to next night. Confirm?",
      "header": "Marathon",
      "multiSelect": false,
      "options": [
        {"label": "Yes, marathon", "description": "Take the full night; defer everything else"},
        {"label": "No, downgrade to Deep (4 hour)", "description": "Still a long run but Phase B and other tasks proceed"}
      ]
    }
  ]
}
```

If the user picks "No, downgrade to Deep (4 hour)", treat the effective
label as `Deep` for the rest of the pipeline.

## 3c — Normalise

Map the final label (after any marathon downgrade) to four variables:

| Label         | `PRIORITY_LABEL` | `TIME_BUDGET_MINUTES` | `CHECKPOINT_EVERY_MINUTES` | `MARATHON` |
|---------------|------------------|-----------------------|----------------------------|------------|
| Quick         | `quick`          | 15                    | 0 (off)                    | `false`    |
| Standard      | `standard`       | 60                    | 20                         | `false`    |
| Deep          | `deep`           | 240                   | 30                         | `false`    |
| Marathon      | `marathon`       | 480                   | 30                         | `true`     |
| Weekly batch  | `weekly`         | 60                    | 20                         | `false`    |

Store all four as phase-scoped variables. They flow to Phase 5, which
passes them to `kei-sleep-queue.sh add`.

## 3d — Cap check (informational)

If `PRIORITY_LABEL ∈ {quick, standard, deep, marathon}` (i.e. this
night), count current this-night pending tasks:

```bash
~/.claude/agents/_primitives/kei-sleep-queue.sh list --pending \
    | awk '$4 ~ /^(quick|standard|deep|marathon)$/' \
    | wc -l
```

Informational messages:

- **Marathon already queued this night:**
  > A marathon task is already pending for tonight. Submitting a second
  > marathon — or any this-night task — will be deferred to the next
  > night, because the marathon owns the whole window.

- **Greedy-pack near-full (≥ 480 min queued this-night):**
  > Tonight's this-night budget is nearly full (≥ 8 hours queued). New
  > this-night tasks will still be accepted but may be deferred to the
  > next night by the greedy-packing scheduler.

Do NOT re-prompt; the user may explicitly want overflow.

## 3e — Advanced overrides (informational)

After Phase 5 preview, explicit flags override the priority defaults:

```
kei-sleep-queue add --time-budget <N>m      # e.g. --time-budget 90m
                   --checkpoint-every <M>m  # e.g. --checkpoint-every 15m
                   --no-timeout             # time_budget_minutes: null
                   --marathon               # explicit marathon flag
```

The wizard does not emit these flags itself; they exist for power users
who call the helper directly.

## Verify-criterion

- `PRIORITY_LABEL ∈ {quick, standard, deep, marathon, weekly}`.
- `TIME_BUDGET_MINUTES ∈ {15, 60, 240, 480}` per the table.
- `CHECKPOINT_EVERY_MINUTES ∈ {0, 20, 30}` per the table.
- `MARATHON` is boolean and `true` iff `PRIORITY_LABEL == "marathon"`.
- At most TWO `AskUserQuestion` calls (second only on marathon path).
