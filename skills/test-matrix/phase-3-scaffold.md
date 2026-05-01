# Phase 3 — Scaffold config + corpus + fixtures per cell

For each cell in `MATRIX`, generate the minimum-viable scaffold: one
dependency declaration, one example test, one fixture / seed corpus, one
local-run command. No over-scaffolding — just the "it runs" skeleton.

## 3a — Per-cell confirmation (AskUserQuestion, loop over cells)

For each cell, emit ONE AskUserQuestion:

```json
{
  "questions": [
    {
      "question": "Scaffold plan for [<type> × <lang>] — proceed?",
      "header": "<type>/<lang>",
      "multiSelect": false,
      "options": [
        {"label": "Proceed with default scaffold",     "description": "Apply the default files listed below"},
        {"label": "Minimal only (dep + 1 test)",        "description": "Skip CI + corpus; just prove the toolchain runs"},
        {"label": "Edit one file",                      "description": "Reply with one free-text path — that file only gets custom content"},
        {"label": "Skip this cell",                     "description": "Drop from MATRIX; next cell"}
      ]
    }
  ]
}
```

Preview the default scaffold BEFORE asking, so the user sees what "proceed"
means. Example for `[fuzz × Rust]`:

```
Default scaffold for [fuzz × Rust]:
  + fuzz/Cargo.toml           — cargo-fuzz manifest
  + fuzz/fuzz_targets/parse.rs — example fuzz_target!(|data: &[u8]| { ... })
  + fuzz/corpus/parse/seed_01  — one hand-picked valid input
  + fuzz/README.md             — local-run commands
Cite: _blocks/test-fuzz.md (corpus mgmt + triage + CI rules)
```

## 3b — Per-type default scaffolds

| Cell | Files |
|---|---|
| **fuzz × Rust** | `fuzz/Cargo.toml` (cargo-fuzz), `fuzz/fuzz_targets/<target>.rs`, `fuzz/corpus/<target>/seed_01` |
| **fuzz × Python** | `tests/fuzz/test_fuzz_<target>.py` (atheris OR hypothesis in fuzz mode), `tests/fuzz/corpus/` |
| **fuzz × JS/TS** | `test/fuzz/<target>.fuzz.ts` (fast-check with `numRuns: 10_000`) |
| **property × Rust** | `Cargo.toml` adds `proptest = "*"`, `tests/property_<name>.rs`, `.proptest-regressions` gitkeep |
| **property × Python** | `tests/property/test_<name>.py` with `@given`, `.hypothesis/` gitignored except `examples/` |
| **property × JS/TS** | `test/property/<name>.spec.ts` with `fc.assert(fc.property(...))` |
| **load × any** | `load/k6/baseline.js` with SLO thresholds; `load/README.md` with baseline→profile→fix loop |
| **e2e × any** | `e2e/playwright.config.ts`, `e2e/pages/login.page.ts`, `e2e/tests/login.spec.ts`, `e2e/README.md` |
| **mutation × Rust** | `.cargo-mutants.toml`, first run command in `tests/mutation/README.md` |
| **mutation × Python** | `mutmut` config in `setup.cfg` / `pyproject.toml`, runbook in `tests/mutation/README.md` |
| **mutation × JS/TS** | `stryker.conf.mjs` with sane `timeoutMS`, `mutate` glob narrowed to critical paths |

## 3c — Cite the block

Every scaffold file's header comment references the relevant `_blocks/`
file so the human reviewer can find the discipline rules:

```rust
// See _blocks/test-fuzz.md for corpus management + crash-triage rules.
// This file is the minimum skeleton; real targets expand from here.
```

## Verify-criterion

- For every `MATRIX` cell, user clicked `Proceed` / `Minimal` / explicit `Edit` / `Skip`.
- At least one file is written per non-skipped cell.
- `SCAFFOLDED` is a list of `{cell, files: [paths]}` entries.
- No file overwrites an existing one without explicit confirmation
  (a PreWrite check: if path exists, emit a second AskUserQuestion
  "overwrite / skip / rename" before writing).
