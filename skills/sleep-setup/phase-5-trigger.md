# Phase 5 — Emit trigger (CronCreate and/or `/schedule create`)

Render the ready-to-paste nightly trigger(s) and ask how to register
them. Branches on `SLEEP_MODE` (set in Phase 0) and uses
`SLEEP_TIME_LOCAL` (set in Phase 0b).

## 5a — Load template (remote path only)

If `SLEEP_MODE ∈ {remote-only, hybrid}`, read the cloud-agent template
from the kit install path:

```
~/.claude/agents/_primitives/templates/sleep-trigger-prompt.md
```

If the file is missing (older kit version), fall back to the inline
template in this phase file (see 5f).

For `SLEEP_MODE == local-only` skip 5a entirely — no remote template is
rendered.

## 5b — Parse local time

`SLEEP_TIME_LOCAL` has format `HH:MM` (validated in Phase 0b). Convert
to minutes-past-midnight. The `10#` prefix prevents bash from
interpreting `08` / `09` as invalid octal:

```bash
hh=${SLEEP_TIME_LOCAL%:*}
mm=${SLEEP_TIME_LOCAL#*:}
local_minutes=$((10#$hh * 60 + 10#$mm))
```

## 5c — Compute UTC cron (remote path only)

Only needed if `SLEEP_MODE ∈ {remote-only, hybrid}`. CronCreate on the
Mac uses local time directly, so `local-only` skips this block.

```bash
# macOS / GNU date — detect local TZ offset in minutes
offset_min=$(date +%z | awk '{ s=substr($0,1,1); h=substr($0,2,2); m=substr($0,4,2); print (s=="-" ? 1 : -1) * (h*60+m) }')
utc_minutes=$(( (local_minutes + offset_min + 1440) % 1440 ))
utc_hour=$(( utc_minutes / 60 ))
utc_min=$(( utc_minutes % 60 ))
SLEEP_CRON_UTC=$(printf '%d %d * * *' "$utc_min" "$utc_hour")
```

## 5d — Render blocks per mode

### Mode: `local-only`

Render ONE fenced `CronCreate` snippet (no `/schedule`). The cron
expression uses the user's local time directly — CronCreate runs on
this Mac, not in UTC:

```
CronCreate expression: <mm> <hh> * * *   (local time on this Mac)
Prompt body:
  Run /self-audit --cross-session on ~/.claude/memory/traces/.
  Duration budget: 60 min max.
  Always write summary to ~/.claude/memory/sleep-report-YYYY-MM-DD.md.
  If >=3 recurring patterns detected, append a dated block to
  ~/.claude/memory/audit-backlog.md (section per RULE 0.14).
  Invariants: append-only traces; no fabricated findings; skip
  analysis if CWD was under a restricted-project path.
```

Where `<mm>` and `<hh>` are the values from 5b. Emit ONE
`AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "Register the local CronCreate now?",
      "header": "Local cron",
      "multiSelect": false,
      "options": [
        {"label": "Create CronCreate now", "description": "Invoke CronCreate with the rendered body"},
        {"label": "Copy, create later",   "description": "I'll register the cron myself with the snippet above"},
        {"label": "Skip (no local cron)", "description": "No scheduled analysis — manual /self-audit only"}
      ]
    }
  ]
}
```

Store `SCHEDULE_ACTION`:
- `Create CronCreate now` → `local-cron-created`
- `Copy, create later`   → `local-cron-copy-later`
- `Skip`                 → `local-cron-skipped`

### Mode: `remote-only`

Render ONE fenced `/schedule create` block using the template from 5a.
Replace `{REPO_URL}` with `REPO_URL` and `{UTC_CRON}` with
`SLEEP_CRON_UTC`. Emit ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "How should we register the nightly REM trigger?",
      "header": "Schedule",
      "multiSelect": false,
      "options": [
        {"label": "Run /schedule now",   "description": "Invoke /schedule create with the rendered prompt"},
        {"label": "Copy, run later",     "description": "I'll paste into /schedule create myself"},
        {"label": "Skip (local-only)",   "description": "Just push traces; no nightly consolidation"}
      ]
    }
  ]
}
```

Store `SCHEDULE_ACTION`:
- `Run /schedule now`  → `remote-run-now`
- `Copy, run later`    → `remote-copy-later`
- `Skip`               → `remote-skipped`

### Mode: `hybrid`

Render BOTH blocks (CronCreate first, then `/schedule create`). Emit
TWO sequential `AskUserQuestion` batches — first the local question
from mode `local-only` (section 5d.local), then the remote question
from mode `remote-only` (section 5d.remote).

Store `SCHEDULE_ACTION` as a composite, e.g.
`local-cron-created+remote-run-now`,
`local-cron-copy-later+remote-skipped`,
`local-cron-skipped+remote-copy-later`, etc.

## 5e — Render placeholders (remote path only)

For `remote-only` / `hybrid`: replace `{REPO_URL}` and `{UTC_CRON}` in
the template. Print the rendered prompt inside a fenced code block so
the user can one-click-copy.

For `local-only`: no placeholders to render — the CronCreate body is
self-contained in 5d.

## 5f — Fallback inline template (remote path, if kit missing file)

If `~/.claude/agents/_primitives/templates/sleep-trigger-prompt.md` is
absent, use this minimal inline prompt:

```
Clone: <REPO_URL>
At UTC <SLEEP_CRON_UTC>:
  1. Clone shallow, read traces/ since reports/last-run.txt
  2. Write reports/sleep-<date>.md with session + tool + error summary
  3. If >=3 cross-session patterns, prepend to backlog.md
  4. Commit + push to main
Invariants: append-only traces; no fabricated findings; never
paraphrase author's flagged content into report bodies.
```

Note the fallback is strictly less capable — loudly log "template file
missing from kit install; using fallback" so the user can re-install.

## Verify-criterion

- `local-only`: exactly ONE `AskUserQuestion`; exactly ONE fenced
  CronCreate block; no `/schedule` rendered.
- `remote-only`: exactly ONE `AskUserQuestion`; exactly ONE fenced
  `/schedule create` block; no CronCreate rendered.
- `hybrid`: exactly TWO `AskUserQuestion` batches (local first,
  remote second); both blocks rendered.
- Cron expression on local path uses `SLEEP_TIME_LOCAL` directly (no
  UTC conversion).
- Cron expression on remote path uses `SLEEP_CRON_UTC` from 5c.
- Rendered prompt contains no placeholder (`{REPO_URL}` / `{UTC_CRON}`).
- `SCHEDULE_ACTION` set per mode rules above.
- The final report block from SKILL.md is emitted with real values,
  including `Mode: <SLEEP_MODE>` and `Time (local): <SLEEP_TIME_LOCAL>`.
