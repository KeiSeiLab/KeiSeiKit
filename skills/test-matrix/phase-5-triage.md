# Phase 5 — Crash / regression triage workflow

Every matrix paradigm produces artifacts when it fails: fuzz crashes,
shrunk property counterexamples, load-SLO violations, E2E traces,
mutation survivors. Without a triage runbook, those artifacts rot.
This phase writes `docs/testing/triage.md` so the next failure is
actionable in ≤ 15 min.

## 5a — Confirm runbook generation (AskUserQuestion)

```json
{
  "questions": [
    {
      "question": "Write the triage runbook to docs/testing/triage.md?",
      "header": "Triage",
      "multiSelect": false,
      "options": [
        {"label": "Yes — full runbook",   "description": "Per-paradigm crash / regression flow + artifact paths + commit template"},
        {"label": "Yes — minimal",        "description": "One-page checklist only; skip per-paradigm deep-dives"},
        {"label": "Skip — team already has one", "description": "Finish without writing; final report notes the external link"}
      ]
    }
  ]
}
```

## 5b — Runbook template (full)

For every selected paradigm in `MATRIX`, emit a section:

```
## <paradigm> failure triage

1. Artifact: <fuzz/artifacts/ | .proptest-regressions | load/reports/ | test-results/ | mutation-report/>
2. Reproduce locally: <exact command from phase-3 scaffold>
3. Minimize: <tmin / shrink / trace-viewer / bisect>
4. Write a failing regression test using the minimized input.
5. Fix root cause (never the symptom — see RULE: No Patching).
6. Re-run the matrix cell. Green = commit with `fix:` + reference artifact SHA.
7. If flaky (not deterministic): quarantine with a ticket, never `retry: 5`.
```

Per-paradigm specifics are pulled from the citing `_blocks/test-*.md`:
- fuzz → `cargo fuzz tmin` / atheris replay flow (block §crash-triage)
- property → commit the shrunk counterexample as a normal unit test
- load → re-baseline after each fix; one variable at a time
- e2e → open `playwright show-trace`; never add `waitForTimeout`

## 5c — Commit template

The runbook ends with a ready-to-copy commit template:

```
fix(<paradigm>): <one-line symptom>

Reproducer: <minimized artifact path + SHA>
Root cause: <1-2 sentences>
Regression test: <path to new permanent test>

See docs/testing/triage.md §<paradigm> for the workflow used.
```

## Verify-criterion

- `TRIAGE_DOC` is set to `docs/testing/triage.md` (or skipped with reason).
- Every `MATRIX` paradigm has a section in the runbook.
- Every section lists artifact path + reproduce command + regression-test
  requirement + root-cause discipline + flake policy.
- Commit template present at end of doc.
