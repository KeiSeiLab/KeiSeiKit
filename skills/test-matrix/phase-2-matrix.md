# Phase 2 — Select the test-type × language matrix

Goal: turn `CRITICAL` + `LANGS` into the minimum set of `(test-type, language)`
cells to scaffold. Fewer cells, done well, beats many cells half-wired.

## 2a — Preview auto-recommendation

Apply these rules and emit a preview table in chat (markdown):

| Critical path | Recommended test types |
|---|---|
| Auth / crypto | fuzz + property |
| Payment | property + e2e + mutation |
| Data integrity | property + e2e |
| Performance SLO | load |
| Untrusted parsing | fuzz + property |
| User-facing UI | e2e |

Cross-product with `LANGS` → tentative `MATRIX_RECO`. Example output in chat:

```
Recommended cells (from CRITICAL × LANGS):
  [1] fuzz × Rust       — rationale: untrusted-parsing + Rust → cargo-fuzz
  [2] property × Rust   — rationale: data-integrity + Rust → proptest
  [3] e2e × TS          — rationale: user-facing UI → Playwright
  [4] load × Rust       — rationale: <100ms SLO → oha + k6
  [5] mutation × Rust   — rationale: payment → cargo-mutants for suite quality
```

Number each cell for the multi-select.

## 2b — Confirm / edit matrix (AskUserQuestion multi-select)

```json
{
  "questions": [
    {
      "question": "Which cells to scaffold this session?",
      "header": "Matrix",
      "multiSelect": true,
      "options": [
        {"label": "[1] fuzz × <lang>",      "description": "Generate fuzz target + seed corpus + CI nightly job"},
        {"label": "[2] property × <lang>",  "description": "Add property-test dependency + sample invariant test + regression cache"},
        {"label": "[3] e2e × <lang>",       "description": "Scaffold Playwright project + 1 page-object example + trace viewer"},
        {"label": "[4] load × <lang>",      "description": "k6/oha script + SLO thresholds + profile-loop runbook"},
        {"label": "[5] mutation × <lang>",  "description": "mutmut/cargo-mutants/StrykerJS config + baseline mutation score"},
        {"label": "Add a custom cell",       "description": "Free-text — e.g. contract tests, chaos tests, visual regression"},
        {"label": "Skip a reco",             "description": "Drop one of the recommended cells — free-text reason"}
      ]
    }
  ]
}
```

Options are GENERATED dynamically — one per `MATRIX_RECO` cell PLUS the two
catch-alls (`Add custom`, `Skip`). Substitute `<lang>` literally.

On `Add a custom cell` → single free-text line → regenerate preview →
re-ask. On `Skip a reco` → free-text reason (logged in final report) →
regenerate → re-ask.

## 2c — Budget check (soft cap)

If the final `MATRIX` has > 6 cells, emit a WARNING message (NOT
AskUserQuestion):

> WARNING: <N> cells selected. Scaffolding + CI wiring for each is ~30 min
> of human review per cell. Consider splitting into two sessions (critical
> cells now, rest next week). Continue? Reply "yes" or re-run Phase 2.

Store the final `MATRIX` as a list of `{type, lang, rationale}` objects.

## Verify-criterion

- `MATRIX` has ≥ 1 cell. Zero cells means nothing to do → stop with a
  message pointing at `/test-gen`.
- Every cell's `type` ∈ {fuzz, property, e2e, load, mutation, custom}.
- Every cell's `lang` ∈ `LANGS` (no phantom language).
- User explicitly confirmed the final matrix (not just auto-reco) — the
  multi-select click counts as the confirmation.
