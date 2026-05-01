# Phase 3 — Parallel Execution

Poll the ledger, aggregate progress, let the user steer the fleet mid-run.

## 3a — Aggregate current state

Every ~60s (or whenever the user asks "status"):

```sh
kei-ledger list --status running
kei-ledger list --status done
kei-ledger list --status failed
```

For each row, attempt to read
`.claude/agents/<id>/progress.json` (written by the child every ≥ 30s per
`feedback_agent_observability`). Shape:

```json
{
  "pct": 0..100,
  "last_step": "short free text",
  "wall_time_s": int,
  "last_updated": iso8601
}
```

Aggregate into `PROGRESS = {id: {status, pct, last_summary, stale_s}}`.
`stale_s = now - last_updated`. Flag `stale_s > 300` as "possibly hung".

Render one-line-per-agent table:

```
<id>  <status>  <pct>%  <last_step>  stale=<s>s
```

## 3b — Steering click (AskUserQuestion, ONE)

After displaying the table:

```json
{
  "questions": [
    {
      "question": "Fleet state — how to proceed?",
      "header": "Steer",
      "multiSelect": false,
      "options": [
        {"label": "continue polling",    "description": "Wait and re-poll — default if > 0 agents are running"},
        {"label": "add sub-agent",       "description": "Spawn another sub-agent — returns to Phase 2c"},
        {"label": "kill stale",          "description": "Call kei-ledger fail on agents with stale_s > 300"},
        {"label": "proceed to merge",    "description": "All required agents done — jump to Phase 4"},
        {"label": "pause and review",    "description": "Stop polling — user reviews manually, re-enter later"}
      ]
    }
  ]
}
```

Route:
- `continue polling` → re-run 3a after a user-initiated "status" request
  (do NOT busy-loop on a timer — let the user drive polling cadence)
- `add sub-agent` → jump to Phase 2c, add new row, return to Phase 3
- `kill stale` → for each `stale_s > 300` running row: `kei-ledger fail <id> --reason "stale > 300s, killed by orchestrator"`
- `proceed to merge` → Phase 4 (verify no `running` rows remain first)
- `pause and review` → emit final report with current state, user re-enters
  the skill later (ledger survives)

## Verify-criterion

- Ledger-polling code actually called `kei-ledger list` at least once per
  Phase-3 entry (unless `DB_MODE == "file-only"` — in which case the
  orchestrator reads each `progress.json` directly).
- Every running sub-agent either has a fresh `progress.json`
  (`stale_s < 300`) or has been flagged as "possibly hung" in the
  displayed table.
- If `proceed to merge` is clicked while `kei-ledger list --status running`
  is non-empty — block the transition and re-ask with the list shown.
- NO-DOWNGRADE: if every child has failed, DO NOT close the project silently.
  Emit a 3-path recovery click (retry failed / re-fanout / abort with audit).
