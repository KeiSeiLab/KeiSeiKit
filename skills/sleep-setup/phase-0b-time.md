# Phase 0b — Sleep time picker

Ask the user to pick the local-time trigger for nightly consolidation.
Runs immediately after Phase 0 (mode pick), before any git setup.

## 0b.1 — Time click

Emit ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "When should nightly consolidation run (local time)?",
      "header": "Sleep time",
      "multiSelect": false,
      "options": [
        {"label": "03:00",  "description": "Classical REM peak — default"},
        {"label": "00:00",  "description": "Midnight — end of calendar day"},
        {"label": "05:00",  "description": "Pre-dawn — fresh morning report"},
        {"label": "23:00",  "description": "Late evening — before overnight sync"},
        {"label": "21:00",  "description": "Early evening — for early sleepers"},
        {"label": "Custom", "description": "Pick exact HH:MM (24h format) on next prompt"}
      ]
    }
  ]
}
```

## 0b.2 — Store or branch

- Non-custom pick → store as `SLEEP_TIME_LOCAL` verbatim (e.g. `03:00`).
- `Custom` → emit follow-up `AskUserQuestion` with `freeText`:

```json
{
  "questions": [
    {
      "question": "Enter the local trigger time in HH:MM (24h). Example: 04:15 for 4:15 AM.",
      "header": "Custom time",
      "freeText": true
    }
  ]
}
```

## 0b.3 — Validation

Validate with regex `^([01][0-9]|2[0-3]):[0-5][0-9]$`:

```bash
if ! echo "$SLEEP_TIME_LOCAL" | grep -qE '^([01][0-9]|2[0-3]):[0-5][0-9]$'; then
  # invalid — re-ask with the same freeText prompt
  # accept any leading zeros; reject "3:00" (must be "03:00"), "24:00", "12:60"
  echo "Invalid time '$SLEEP_TIME_LOCAL'. Expected HH:MM with leading zeros (e.g. 03:00, 23:59)."
  # loop back to 0b.2 Custom freeText prompt
fi
```

Retry loop: if invalid, re-emit the freeText prompt up to 3 times. After
3 failures, fall back to `03:00` and log `SLEEP_TIME_LOCAL defaulted to
03:00 after 3 invalid inputs`.

## 0b.4 — Confirmation line

Once validated, print:

```
SLEEP_TIME_LOCAL = <HH:MM> (this Mac's local time). Phase 5 will use
this value for the CronCreate expression and/or the UTC conversion for
the remote `/schedule` trigger.
```

## Verify-criterion

- At least ONE `AskUserQuestion` (two if Custom picked).
- `SLEEP_TIME_LOCAL` matches `^([01][0-9]|2[0-3]):[0-5][0-9]$`.
- No unclamped / unvalidated values stored. Invalid input either
  re-prompts or falls back to `03:00` with an audit line.
