# Production Roadmap — to v1.0.0

> Tracking what blocks `v1.0.0 production-ready` for the substrate
> (`kei-arch-map` + `kei-cleanup` + `kei-registry` + `kei-arch-derive`).
>
> Updated: 2026-05-04. Drift expected — re-derive from git log + audits.

---

## Status by layer

| Layer | State |
|---|---|
| Code (compile, tests, wired) | substrate cargo check workspace clean; 23 tests in arch-derive, 3 e2e in pipeline-test, registry+map+cleanup tests passing |
| Self-validation runtime | `arch/PLAN.toml` 9/9 PASS mechanically verified |
| Self-validation content | inference shipped (`kei-arch-derive infer`); registry not yet repopulated on full workspace |
| Native deploy | 4/4 layers wired (install / commit / agent / release) |
| Security audit | Wave 4 closed 6 findings; Wave 6 re-audit raised 4 new HIGH (in flight) |
| Productionization | CI never executed end-to-end; 0 v* tags; cross-project pilot pending |

---

## Open gates to v1.0.0

### Gate 1 — Wave 7 HIGH security fixes (in flight)

Four parallel sub-agent worktrees:

- 7A — `hooks/arch-verify-precommit.sh:40` bypass regex anchor weakness.
- 7B — `kei-arch-map/src/evidence/cargo_check.rs` TOCTOU + `schema.rs`
  doc-vs-reality lie about behaviour.
- 7C — `keidocs/src/dna.rs:30` 64-bit truncation collision risk; variant
  scan across `kei-agent-runtime`, `kei-runtime-core`, `kei-shared`.
- 7D — `kei-cortex` comments POST 401 auth regression introduced by
  `1bbf79c feat(comments): sovereign comment system`.

Each fix lands as a separate commit by the orchestrator after agent
returns file-list (RULE 0.13 — agents write, orchestrator commits).
Verify-before-commit gate: `cargo check --workspace` + targeted test.

### Gate 2 — Wave 8 coverage uplift

Run `kei-arch-derive infer` on full workspace, commit resulting
`kei-registry` data. Expected ~239 / 587 blocks acquire formula
sidecar (~41% coverage) per B1 sub-agent's smoke run, not yet
re-validated this session. Sequential after Wave 7C (DNA hash change
would otherwise force re-roll).

### Gate 3 — Wave 10 re-audit (after Wave 7+8)

Five read-only agents in parallel:

- `validator` — every Wave 7+8 commit's claim verifies against code
- `security-auditor` — full re-sweep on the four fixed modules + new
  inference output path
- `kei-architect` — substrate coherence; pipeline carries data
  end-to-end after Wave 8
- `critic-tech-debt` — branch hygiene (21 stale `feat/*`), 19/54
  unregistered hooks
- `critic-bug` — variant analysis on the DNA truncation pattern
  globally

### Gate 4 — Tag v0.14.6 + CI verify

- `git tag v0.14.6 && git push --tags`
- Watch `release.yml` — first end-to-end test of `keigit publish` flow
- Companion: `KEIGIT_TOKEN` user-side rotation (1 click on keigit web)
- If CI fails: fix, re-tag v0.14.7, repeat

### Gate 5 — Cross-project pilot

- Install `kei-cleanup` in one real consumer (Recruiter or leadgen)
- Run on its real codebase, observe findings, confirm substrate works
  out-of-tree
- Capture any divergence vs unit-test behaviour as a HOTPATHS row

---

## Lower-priority lanes (parallelizable any time)

- P3 — branch hygiene; per-branch decision (merge / drop / defer)
- P5 — register the 19/54 missing hooks in `~/.claude/settings.json`
  via `update-config` skill
- Branch reduce — 8 stale local feat branches (substrate-v0.16-decomp,
  expand-skills-hooks-coverage, keiwiki-runtime, kei-cleanup-v0.1,
  p5/p6/p7-adapter)
- Documentation — keigit-published walkthrough; Berkheimer-style claim
  evidence section in README

---

## Pre-existing regressions (not from substrate work)

- `kei-cortex/tests/comments_smoke.rs::comments_lifecycle_post_list_react_delete`
  — POST returns 401 instead of 200; Bearer auth flow broken since
  `1bbf79c feat(comments)`. Wave 7D addresses.
- `docs-site/package.json` + `package-lock.json` — uncommitted
  `playwright` + `@axe-core/playwright` devDependencies from a prior
  `/visual-loop` setup; left in place pending owner-decision.

---

## Numeric anchors (RULE 0.18)

- 4 commits in this session: `9bbeaff`, `c4a647c`, `b915200`, `337f5ec`
  `[REAL: git log feat/kei-arch-map --oneline 2026-05-04]`.
- 23 tests pass in `kei-arch-derive`, 3 in `kei-pipeline-test`,
  428 in `kei-cortex` lib `[REAL: cargo test --workspace 2026-05-04]`.
- 1 pre-existing test fail (`comments_lifecycle_*`) verified as
  regression from `1bbf79c` not from this session
  `[REAL: git checkout afc825d -- _primitives/_rust/kei-cortex && cargo test -p kei-cortex --test comments_smoke 2026-05-04 — same failure]`.
- Production readiness ~75-78% post-Wave-7-9 commits
  `[ESTIMATE-HTC: weighted across the 6 layers in §"Status by layer"; not measured]`.
