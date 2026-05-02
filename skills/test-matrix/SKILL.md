---
name: test-matrix
description: "Use when a project needs testing BEYOND unit tests — fuzzing, property-based, load, E2E, or mutation. Five-phase hub-and-spoke pipeline composes the right mix per language × critical path × CI target, scaffolds configs + corpus + fixtures, wires CI jobs, and defines the crash/regression triage workflow. Pure-click: every decision except intake is an AskUserQuestion."
argument-hint: <free-text description of what needs testing and why>
---

# /test-matrix — Testing beyond unit tests (index)

## When to use

- A project needs testing beyond unit tests: fuzzing, property-based, load, E2E, or mutation testing.
- At project kickoff or when coverage gaps span multiple test paradigms across the stack.
- Use `/test-matrix` for project-wide strategy; use `/test-gen` for per-function unit tests.

You are designing a **testing matrix** for a project that already has (or
should have) unit-test coverage via `/test-gen`. This skill owns the
orthogonal axes:

- **Fuzzing** — input-space exploration at boundaries (parsers, deserializers, crypto)
- **Property-based** — invariants verified over generated inputs (pure functions, data structures)
- **Load** — SLO assertion under traffic (`k6`/`vegeta`/`oha`, baseline→profile→fix)
- **E2E** — browser-driven critical journeys (Playwright, page objects, trace viewer)
- **Mutation** — test-suite quality verification (mutmut / cargo-mutants / StrykerJS)

**Not duplicated here:** happy-path / edge / error unit tests (`/test-gen`
owns those). This skill links rather than re-implements.

This `SKILL.md` is the INDEX. Each phase lives in its own file, executed in
order. Never skip, never re-order.

---

## Pipeline overview (5 phases + final report)

| Phase | File | Purpose | AskUserQuestion count |
|---|---|---|---:|
| 1 | [phase-1-intake.md](phase-1-intake.md) | Language(s), coverage baseline, critical paths, CI target | 1× (multi-part) |
| 2 | [phase-2-matrix.md](phase-2-matrix.md) | Select test types × languages matrix | 1× multi-select |
| 3 | [phase-3-scaffold.md](phase-3-scaffold.md) | Generate config + corpus + fixtures per selected cell | 1× per cell |
| 4 | [phase-4-ci-wire.md](phase-4-ci-wire.md) | CI job per test type; artifacts; failure policy | 1× multi-select |
| 5 | [phase-5-triage.md](phase-5-triage.md) | Crash + regression triage workflow | 1× |

Minimum AskUserQuestion count across a full session: **5** (one per phase).
Higher when Phase 3 expands per selected cell. This is the pure-click
contract.

---

## Variables the pipeline produces

| Name | Set in | Meaning |
|---|---|---|
| `LANGS` | Phase 1 | Languages in scope (Rust / Python / JS-TS / Go / Swift / Flutter — multi) |
| `COVERAGE` | Phase 1 | Baseline unit-test coverage % (or "unknown") |
| `CRITICAL` | Phase 1 | Critical paths: auth / payment / data-integrity / perf / untrusted-input |
| `CI` | Phase 1 | github-actions / forgejo-actions / self-hosted / none |
| `MATRIX` | Phase 2 | Set of (test-type × language) cells to scaffold |
| `SCAFFOLDED` | Phase 3 | Files written per cell (paths + corpus seeds) |
| `CI_JOBS` | Phase 4 | CI workflow entries added per cell |
| `TRIAGE_DOC` | Phase 5 | Path to `docs/testing/triage.md` (or project-local equivalent) |

---

## Final report (emit after Phase 5)

```
=== TEST-MATRIX REPORT ===
Languages:        <LANGS>
Coverage (unit):  <COVERAGE>
Critical paths:   <CRITICAL>
Matrix cells:     <count> — <list (type × lang)>
Files written:    <count> (configs + corpus + fixtures)
CI jobs added:    <count> (<per-type failure policy>)
Triage doc:       <TRIAGE_DOC>
Next action:      Run <cmd> locally to verify the scaffold, then commit.
```

---

## Rules (apply throughout)

- **Pure-click contract.** Only the Phase 1 intake paragraph is free text.
  Everything else is `AskUserQuestion`. Count in the final report.
- **NO DOWNGRADE (RULE -1).** If a language × type cell has no good tool,
  return 2-3 constructive paths, never "not supported".
- **NO HALLUCINATION (RULE 0.4).** Every tool / library cited must exist
  and be current. When in doubt, mark `[UNVERIFIED — verify release page]`
  and surface in the report.
- **Plan Mode First (RULE 0.5).** This skill IS the plan; no writes before
  the corresponding phase's confirm click.
- **Constructor Pattern (RULE ZERO).** Block files (`_blocks/test-*.md`)
  stay ≤ 60 LOC. This SKILL.md ≤ 200 LOC; phase files ≤ 150 LOC each.
- **Surgical Changes.** Writes only to:
  - `<repo>/tests/`, `<repo>/fuzz/`, `<repo>/e2e/`, `<repo>/load/`
  - `<repo>/.github/workflows/` or `<repo>/.forgejo/workflows/`
  - `<repo>/docs/testing/triage.md`
  - No writes to `_blocks/` here (that's `compose-solution`'s Phase 6).
- **No duplication with `/test-gen`.** If the user really wants unit-test
  generation, Phase 1 detects it and hands off immediately.

---

## References

- [phase-1-intake.md](phase-1-intake.md) · [phase-2-matrix.md](phase-2-matrix.md) · [phase-3-scaffold.md](phase-3-scaffold.md) · [phase-4-ci-wire.md](phase-4-ci-wire.md) · [phase-5-triage.md](phase-5-triage.md)
- `skills/test-gen/SKILL.md` — unit-test generation (happy / edge / error).
  Phase 1 hands off there if intake reveals unit-test gap, not matrix gap.
- `_blocks/test-fuzz.md` · `_blocks/test-property.md` · `_blocks/test-load.md` · `_blocks/test-e2e.md` — per-paradigm reference blocks, composable into manifests.
- `_blocks/rule-test-first.md` — TDD / tests-with-code discipline (inherited).
- `skills/compose-solution/SKILL.md` — if you need a NEW block (e.g. mutation-specific), hand off there (Phase 6 block-augment).
