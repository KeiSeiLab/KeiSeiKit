# Phase 1 — Intake (language, coverage, critical paths, CI)

One free-text paragraph + one AskUserQuestion multi-part batch.

## 1a — Ask for the testing-gap description

Emit a regular message (NOT AskUserQuestion):

> Describe in one paragraph: what are you testing (project name / stack),
> what gap is `/test-gen` not solving (fuzz? load? E2E? mutation? all?),
> and what failure mode would be worst (prod crash? data loss? latency
> regression? auth bypass?). Reply in one message.

Store verbatim as `INTAKE`.

If `INTAKE` mentions ONLY "unit tests" / "missing tests for function X"
(unit-level gap, not matrix gap), emit:

```
DETECTION: this is a /test-gen task, not /test-matrix.
Handing off to `skills/test-gen/SKILL.md`. Re-run /test-matrix later
when fuzz / property / load / E2E / mutation coverage is needed.
```

…and STOP. Do not proceed.

## 1b — Multi-part intake click (one AskUserQuestion call)

```json
{
  "questions": [
    {
      "question": "Language(s) in scope?",
      "header": "Languages",
      "multiSelect": true,
      "options": [
        {"label": "Rust",        "description": "cargo-fuzz, proptest, cargo-mutants, oha"},
        {"label": "Python",      "description": "hypothesis, atheris, mutmut, schemathesis"},
        {"label": "JavaScript/TypeScript", "description": "fast-check, StrykerJS, Playwright"},
        {"label": "Go",          "description": "built-in fuzz (go test -fuzz), gopter, vegeta"},
        {"label": "Swift",       "description": "SwiftCheck, XCUITest — limited fuzz tooling"},
        {"label": "Flutter/Dart", "description": "glados property, flutter integration_test"}
      ]
    },
    {
      "question": "Baseline unit-test coverage?",
      "header": "Coverage",
      "multiSelect": false,
      "options": [
        {"label": "High (≥ 80%)",        "description": "Matrix tests layer on top of solid unit base"},
        {"label": "Medium (40-80%)",     "description": "Run /test-gen in parallel, don't skip unit gaps"},
        {"label": "Low (< 40%)",         "description": "Strongly recommend /test-gen FIRST — fuzz+load on buggy code wastes CI"},
        {"label": "Unknown — need to measure", "description": "Phase 3 will add a coverage job before scaffolding"}
      ]
    },
    {
      "question": "Critical paths (multi-select)?",
      "header": "Critical",
      "multiSelect": true,
      "options": [
        {"label": "Auth / session / crypto",         "description": "Fuzz + property mandatory on token parsers + signature verify"},
        {"label": "Payment / money-in-motion",        "description": "E2E + property (invariants: no negative balance, idempotency) mandatory"},
        {"label": "Data integrity (DB / serialization)", "description": "Property-based round-trips + migration E2E"},
        {"label": "Performance-sensitive (< 100ms SLO)", "description": "Load tests with k6/oha mandatory; set SLO thresholds in CI"},
        {"label": "Untrusted-input parsing",          "description": "Fuzz mandatory (cargo-fuzz / atheris / jsfuzz)"},
        {"label": "User-facing UI flows",             "description": "E2E with Playwright on 5-15 critical journeys"}
      ]
    },
    {
      "question": "CI target?",
      "header": "CI",
      "multiSelect": false,
      "options": [
        {"label": "GitHub Actions",       "description": "workflow file under .github/workflows/"},
        {"label": "Forgejo Actions",      "description": "workflow file under .forgejo/workflows/ (kit default —  compatible)"},
        {"label": "Self-hosted / custom", "description": "Emit portable YAML + shell scripts; wire manually"},
        {"label": "None — local only",    "description": "Generate Makefile / justfile targets, no CI"}
      ]
    }
  ]
}
```

Store as `LANGS`, `COVERAGE`, `CRITICAL`, `CI`.

## Verify-criterion

- `INTAKE` is non-empty.
- `LANGS` has ≥ 1 entry.
- `CRITICAL` has ≥ 1 entry (zero-critical-path tasks are unit-test-only — redirect to /test-gen).
- `CI` is exactly one value.
- On failure, re-ask the failing input only. Never fall through.
