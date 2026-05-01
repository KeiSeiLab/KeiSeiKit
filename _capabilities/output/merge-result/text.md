## Merge result output

Your return report MUST contain the following fields, each on its
own line, with exact key names:

- `COMMIT_SHA:` — the SHA-1 of the new commit on `main` (40 hex
  chars). If the merge produced multiple commits (e.g. squash vs
  merge-commit), report the tip of `main` after your work.
- `LEDGER_STATUS:` — exactly one of `done`, `failed`, or
  `still-running`. Reflects the ledger row for the fork you merged.
- `FORK_AGENT_ID:` — the agent-id of the writer whose fork you
  merged (or attempted to merge).
- `MERGE_METHOD:` — exactly one of `merge-no-ff`, `squash`,
  `rebase`, or `cherry-pick`. Whatever strategy you actually used.

Skeleton — success:

    COMMIT_SHA: e8b37c92d4a1f0...
    LEDGER_STATUS: done
    FORK_AGENT_ID: ag-edit-local-20260423-142033
    MERGE_METHOD: merge-no-ff

    blockers: none
    next: none

Skeleton — failure (fork diff did not apply):

    COMMIT_SHA: <none>
    LEDGER_STATUS: failed
    FORK_AGENT_ID: ag-edit-local-20260423-142033
    MERGE_METHOD: merge-no-ff

    blockers:
      - "3-way merge reported conflict in src/pipeline.rs line 42"
    next: "Orchestrator re-spawns writer with conflict hint"

Rules:

- `COMMIT_SHA:` — 40 hex chars on success, literal string `<none>`
  on failure. Do not paraphrase ("merged but no sha recorded" → FAIL).
- `LEDGER_STATUS:` — must match the actual ledger row. Cross-check
  with `kei-ledger show <agent-id>` before emitting.
- Merger MUST NOT close the ledger row if the merge failed; the
  `still-running` state is legitimate when the merge is deferred.
- If you had to rescue a half-merged state (`merge --abort` + retry),
  document the rescue in `blockers:` with the original sha + rescue
  sha, even on eventual success.
