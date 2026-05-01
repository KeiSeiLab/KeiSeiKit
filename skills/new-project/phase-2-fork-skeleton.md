# Phase 2 — Fork Skeleton

Create the project branch, the root ledger entry, and the first
theoretical sub-agent spawn(s) per Phase 1's `THEORY_PART` choice.

## 2a — Project branch

Resolve `PROJECT_BRANCH = project/<PROJECT_SLUG>`.

Run (via Bash tool):

```sh
git fetch origin main
git checkout -b "$PROJECT_BRANCH" origin/main
```

Failure modes — emit a NO-DOWNGRADE recovery AskUserQuestion:

```json
{
  "questions": [
    {
      "question": "Branch creation failed — how to proceed?",
      "header": "Recovery",
      "multiSelect": false,
      "options": [
        {"label": "rename slug",              "description": "Use a suffix (-v2, -alt) and retry"},
        {"label": "reuse existing branch",    "description": "Check out the existing branch and append this work"},
        {"label": "abort project",            "description": "Stop before touching the ledger"}
      ]
    }
  ]
}
```

## 2b — Ledger root entry

Skip when `DB_MODE == "file-only"`. Otherwise:

```sh
spec_sha=$(printf '%s' "$GOAL" | shasum -a 256 | cut -c1-16)
LEDGER_ID="project-${PROJECT_SLUG}-$(date +%s)"
kei-ledger init
kei-ledger fork "$LEDGER_ID" "$PROJECT_BRANCH" \
    --parent main \
    --spec-sha "$spec_sha"
```

Also write the root bundle under
`.claude/agents/$LEDGER_ID/{spec.md, plan.md, progress.json, chatlog.md, handoffs.md, review.md}`.
`spec.md` = `GOAL` + 5 Phase-1 answers; others start empty / scaffolded.

## 2c — Theoretical sub-agent spawn (AskUserQuestion, ONE)

Branch on `THEORY_PART`. Emit this confirmation click:

```json
{
  "questions": [
    {
      "question": "Confirm theoretical sub-agent fan-out (derived from Phase 1)?",
      "header": "Spawn",
      "multiSelect": true,
      "options": [
        {"label": "physics-deriver",     "description": "Math derivation agent (only if THEORY_PART = math derivation)"},
        {"label": "research sweep",      "description": "Prior-art research sub-agent (only if THEORY_PART = prior-art research)"},
        {"label": "architect",           "description": "Architecture spec agent (only if THEORY_PART = architecture spec)"},
        {"label": "paradigm-classifier", "description": "Observable classification (observable-classification)"},
        {"label": "skip theory",         "description": "No theoretical sub-agent — straight to implementation fan-out"}
      ]
    }
  ]
}
```

For each selected label (except `skip theory`):

1. Derive `agent_id = <kind>-<ts>`, `agent_branch = project/$PROJECT_SLUG/agent-$agent_id`
2. `git worktree add .claude/worktrees/$agent_id -b $agent_branch` (when fanout > single)
3. `kei-ledger fork "$agent_id" "$agent_branch" --parent "$PROJECT_BRANCH" --spec-sha "$spec_sha"` (skip if file-only)
4. Invoke the Agent tool with the matching `subagent_type` and
   `isolation: "worktree"` (the `agent-fork-logger.sh` hook will emit a
   second fork row — OK, ledger de-duplicates by primary key and the hook
   attempt returns nonzero silently)
5. Append `{id, branch, kind}` to `SUB_AGENTS`

## Verify-criterion

- `git branch --show-current` returns `$PROJECT_BRANCH`.
- `kei-ledger list --status running` returns ≥ 1 row whose id == `LEDGER_ID`
  (unless `DB_MODE == "file-only"`).
- Every entry in `SUB_AGENTS` has a corresponding `kei-ledger list` row
  with `parent_branch == $PROJECT_BRANCH`.
- If any spawn failed: emit NO-DOWNGRADE recovery click (retry / skip / abort).
