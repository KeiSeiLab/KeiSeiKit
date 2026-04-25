# Phase 4 — Merge Ceremony

Per-branch verdict, ledger bookkeeping, integration into the project
branch. One AskUserQuestion **per sub-agent branch** (therefore the
skill's AskUserQuestion floor rises with fanout).

## 4a — Validate bundles first

For every `SUB_AGENTS[i]` where status ∈ {done, failed}:

```sh
kei-ledger validate "$agent_branch"
```

This verifies all 6 required artefacts exist under
`.claude/agents/<id>/`: spec.md, plan.md, progress.json, chatlog.md,
handoffs.md, review.md.

If any agent is MISSING artefacts — mark its verdict as "reject-bundle"
and skip the per-branch click below. The final report lists them.

## 4b — Per-branch merge click (AskUserQuestion, ONE per branch)

For each sub-agent whose bundle is complete, emit:

```json
{
  "questions": [
    {
      "question": "Merge verdict for <agent_id> on <agent_branch> (status=<s>, summary=<first 60 chars>)?",
      "header": "Verdict",
      "multiSelect": false,
      "options": [
        {"label": "merge --no-ff",   "description": "Preserve the sub-branch history — default for substantive work"},
        {"label": "squash",          "description": "Collapse into one commit on project branch — for small / fixup work"},
        {"label": "reject",          "description": "Do not merge — kei-ledger rejected; branch stays for audit"},
        {"label": "defer",           "description": "Leave for later — no merge, no rejection; re-enter skill next session"}
      ]
    }
  ]
}
```

Execute per click:

- `merge --no-ff` →
  ```sh
  git checkout "$PROJECT_BRANCH"
  git merge --no-ff "$agent_branch" -m "merge($agent_id): $summary"
  kei-ledger merged "$agent_id"
  ```
- `squash` →
  ```sh
  git checkout "$PROJECT_BRANCH"
  git merge --squash "$agent_branch"
  git commit -m "feat($agent_id): $summary (squashed)"
  kei-ledger merged "$agent_id"
  ```
- `reject` →
  ```sh
  kei-ledger fail "$agent_id" --reason "rejected at merge ceremony"
  # Update status table — ledger has no explicit 'rejected' from 'done',
  # so we log rejection via a `fail` with reason; the row stays as audit
  # evidence. (If the agent was already in 'failed' — leave as is.)
  ```
- `defer` → no git action, no ledger state change; record in `MERGE_PLAN`
  as deferred so the final report reminds the user.

## 4c — Final integration checkpoint

After every sub-branch has a verdict:

```sh
git checkout "$PROJECT_BRANCH"
git log --oneline -20
kei-ledger tree "$LEDGER_ID"
```

Emit a final NO-DOWNGRADE click if any sub-branch was rejected or
deferred — never silently close the project:

```json
{
  "questions": [
    {
      "question": "Project state has <N> rejected + <M> deferred branches. Next step?",
      "header": "Close",
      "multiSelect": false,
      "options": [
        {"label": "open PR as-is",          "description": "Push project branch, open PR — rejected work is audit-logged only"},
        {"label": "retry rejected",         "description": "Return to Phase 2c with the rejected sub-agents as fresh spawns"},
        {"label": "close and re-enter later", "description": "Leave project branch local; re-enter skill next session"}
      ]
    }
  ]
}
```

## Verify-criterion

- `kei-ledger list --status running` returns zero rows whose
  `parent_branch == $PROJECT_BRANCH`.
- Every `SUB_AGENTS[i]` has exactly one entry in `MERGE_PLAN` — one of
  `merge`, `squash`, `reject`, `defer`, `reject-bundle`.
- For every `merge` / `squash` verdict, `kei-ledger list --status merged`
  contains a matching row.
- Final report cites each verdict explicitly and does NOT gloss over
  rejected / deferred branches.
