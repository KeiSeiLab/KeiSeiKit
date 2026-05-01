# Phase 4 — CI wiring per cell (artifacts + failure policy)

Each scaffolded cell gets exactly one CI job. Different paradigms have
different failure-budget rules — wire them explicitly, never "all tests
block merge by default".

## 4a — Per-type failure policy (preview)

Emit a table in chat showing the default policy per `MATRIX` cell:

| Cell | Trigger | Duration | Failure policy |
|---|---|---|---|
| fuzz (short) | PR | 60 s per target | **block merge** on any crash |
| fuzz (nightly) | cron | 1-4 h per target | **artifact + issue**, do not block PRs |
| property | PR | ~30 s | **block merge** (failures are real bugs) |
| load (smoke) | PR | 30-60 s | **block merge** if SLO thresholds fail |
| load (full) | nightly / manual | 10-30 min | **artifact + dashboard**, do not block PRs |
| e2e (critical) | PR | 2-5 min | **block merge** (retry×2 max) |
| e2e (full) | nightly | 15-30 min | **artifact + trace**, do not block PRs |
| mutation | weekly / manual | hours | **dashboard + report**, NEVER block PRs |

Rationale written inline: fuzz and load have two lanes (fast smoke on PR,
deep nightly). Mutation testing is too slow to block PRs. E2E uses retries
but keeps the retry count honest (max 2).

## 4b — Confirm CI jobs (AskUserQuestion multi-select)

```json
{
  "questions": [
    {
      "question": "Which CI jobs to generate this session?",
      "header": "CI Jobs",
      "multiSelect": true,
      "options": [
        {"label": "fuzz-smoke (PR)",       "description": "60s per target per PR; blocks merge on crash"},
        {"label": "fuzz-nightly (cron)",   "description": "1-4h deep fuzz; artifacts uploaded; non-blocking"},
        {"label": "property (PR)",         "description": "~30s; blocks merge; PROPTEST_CASES=10000 in CI"},
        {"label": "load-smoke (PR)",       "description": "30-60s; blocks merge if k6 SLO thresholds fail"},
        {"label": "load-full (nightly)",   "description": "10-30m; uploads HTML report; non-blocking"},
        {"label": "e2e-critical (PR)",     "description": "5-15 critical journeys; blocks merge; retry×2 max"},
        {"label": "e2e-full (nightly)",    "description": "full suite; non-blocking; traces on failure"},
        {"label": "mutation (weekly)",     "description": "full mutation run; emits HTML + score; never blocks PRs"},
        {"label": "coverage gate",         "description": "add a coverage-diff gate so /test-gen output is measurable"}
      ]
    }
  ]
}
```

Options are GENERATED — only show the cell types actually present in
`MATRIX`. Adding `mutation` to options only if at least one `mutation × _`
cell was selected in Phase 2.

## 4c — Write the workflow file(s)

Based on `CI` from Phase 1:

- **GitHub Actions** → `.github/workflows/test-matrix.yml` with jobs as
  selected. One matrix-strategy job per paradigm (language matrix inside).
- **Forgejo Actions** → `.forgejo/workflows/test-matrix.yml` (same schema
  as GH Actions, compatible syntax). KeiSeiKit default ().
- **Self-hosted / custom** → emit portable YAML + a `Makefile` / `justfile`
  with the same job commands so humans can wire into any CI.
- **None — local only** → write only `Makefile` / `justfile` targets
  (`make fuzz-smoke`, `make load-smoke`, etc.) and a `docs/testing/ci.md`
  note explaining how to wire them into CI later.

## 4d — Artifact discipline

Every job uploads one artifact directory, never loose files:

- `fuzz` → `fuzz/artifacts/` (crash inputs + minimized reproducers)
- `load` → `load/reports/` (HTML, JSON summaries, Grafana links)
- `e2e` → `test-results/` (traces, videos, screenshots — Playwright default)
- `mutation` → `mutation-report/` (HTML + JSON)

Retention: 30 days default; 90 days for nightly + weekly jobs. Never
infinite — CI storage costs compound.

## Verify-criterion

- `CI_JOBS` has ≥ 1 entry (else redirect to local-only Makefile path).
- Workflow file writes to the correct path per `CI` from Phase 1.
- Every job declares explicit `timeout-minutes` (no unbounded runs).
- Every job uploads artifacts on failure (not just on success).
- No job `continue-on-error: true` for PR-blocking lanes.
