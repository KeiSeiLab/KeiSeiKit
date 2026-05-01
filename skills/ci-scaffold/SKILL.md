---
name: ci-scaffold
description: Hub-and-spoke pipeline that produces a production-grade CI/CD plan and scaffolds the workflow files for a new or existing repo — platform choice (GitHub Actions vs Forgejo Actions), build matrix, OIDC-vs-token secrets posture, release automation, and a security gate — via pure-click decisions across five phases. Emits `.github/workflows/*.yml` or `.forgejo/workflows/*.yml`, a secrets-env scaffold (RULE 0.8), and runs `kei-ci-lint` before handing off. Never writes secret values.
argument-hint: <one-line repo description, e.g. "Rust axum service, deploys to AWS via OIDC, crates.io publish on tag">
---

# CI-Scaffold — CI/CD Pipeline Generator (index)

> See `_blocks/pipeline-5phase-template.md` for the 5-phase wizard contract
> and `_blocks/rule-pure-click-contract.md` for the AskUserQuestion rule.
> Skill-specific phase tables are inline below.

You are converting "I need CI for repo X" into a reviewable, concrete plan plus generated workflow files: which platform (GH Actions vs Forgejo), what build matrix, how secrets flow (OIDC vs PAT), which release tool, and which security scanners block merge. Every decision is a click; the only typed input is the Phase 1 intake paragraph.

This skill scaffolds workflow YAML. It does NOT commit on the user's behalf and NEVER writes secret values. After Phase 5 it runs `_primitives/kei-ci-lint.sh` and walks the user through any violations via AskUserQuestion (fix / skip / abort).

The skill reads four companion blocks heavily — every phase references at least one:

- `_blocks/ci-github-actions.md` — GH Actions: OIDC, matrix, cache, reusable, least-privilege token.
- `_blocks/ci-forgejo-actions.md` — Forgejo (GH-compat) self-hosted runner, Tailscale-only admin.
- `_blocks/ci-release-automation.md` — release-please / changesets / cargo-release / goreleaser.
- `_blocks/ci-security-gate.md` — gitleaks, cargo-audit, npm/pip-audit, syft SBOM, semgrep, licenses.

---

## Pipeline overview (5 phases, ≥5 AskUserQuestion calls)

| Phase | File | Purpose | AskUserQuestion |
|---|---|---|---|
| 1 | [phase-1-intake.md](phase-1-intake.md) | Platform / languages / deploy target / release strategy | 4× |
| 2 | [phase-2-matrix.md](phase-2-matrix.md) | Build matrix: OS × version × target | 1× |
| 3 | [phase-3-workflows.md](phase-3-workflows.md) | Generate `.github/workflows/*.yml` or `.forgejo/workflows/*.yml` | 1× |
| 4 | [phase-4-secrets.md](phase-4-secrets.md) | OIDC vs PAT; RULE 0.8 env-var scaffold | 1× |
| 5 | [phase-5-verify.md](phase-5-verify.md) | Run `kei-ci-lint`; fix/skip/abort on each finding | 1× per finding (≥0) |

Minimum AskUserQuestion count across a full session: **8** (4 Phase 1 + 1 each Phases 2–5). Exceeds the ≥5 hub-and-spoke contract. Phase 5 adds one AskUserQuestion PER lint finding — typically 0–3.

---

## Variables the pipeline produces

| Name | Set in | Meaning |
|---|---|---|
| `REPO` | Phase 1 | Free-text one-liner: stack + deploy target |
| `PLATFORM` | Phase 1 | github-actions / forgejo-actions / both |
| `LANGS` | Phase 1 | subset of {rust, node, python, go, flutter, swift} |
| `DEPLOY` | Phase 1 | none / aws-oidc / gcp-oidc / cloudflare / modal / docker-registry / custom |
| `RELEASE` | Phase 1 | release-please / changesets / cargo-release / goreleaser / none |
| `MATRIX` | Phase 2 | {os, lang-version, target} tuple list |
| `WORKFLOWS` | Phase 3 | list of generated YAML filenames |
| `SECRETS` | Phase 4 | env var NAMES + storage path; NEVER values |
| `LINT` | Phase 5 | pass / warn-with-overrides / fail |

---

## Final report (emit after Phase 5)

```
=== CI-SCAFFOLD REPORT ===
Repo:       <REPO one-liner>
Platform:   <PLATFORM>
Languages:  <LANGS>
Deploy:     <DEPLOY>
Release:    <RELEASE tool>
Matrix:     <os count> × <version count> × <target count> = N cells
Workflows:  <list of generated file paths>
Secrets:    <N> env VAR names written to secrets/ci.env scaffold (RULE 0.8)
Lint:       <kei-ci-lint status> (<N findings, M fixed, K skipped>)
Next:       review diff → commit → push to feat/<name>-ci branch
```

---

## Rules (apply throughout)

- **Pure-click contract.** Only Phase 1 intake is typed. Every other decision is `AskUserQuestion`.
- **RULE 0.8 Secrets SSoT.** Emit env VARIABLE NAMES only (`AWS_ROLE_ARN`, `CARGO_REGISTRY_TOKEN`, ...). NEVER echo a token value. Storage path is `<repo>/secrets/ci.env` per `_blocks/domain-has-secrets.md`.
- **RULE 0.4 NO HALLUCINATION.** Every `uses:` value cites a real repo — tags used are those actually published on the action's release page at scaffold time (`actions/checkout@v4`, `actions/cache@v4`, `Swatinem/rust-cache@v2`, etc.). If unsure, prefer pin-by-SHA with a comment; never invent a version.
- ** NO GITHUB PUSH.** If `PLATFORM=forgejo-actions` the skill REFUSES to also emit `.github/workflows/` files. Mixed posture allowed only with explicit user confirmation.
- **NO DOWNGRADE.** If a Phase-5 finding blocks, the skill returns 2–3 constructive fixes (not "skip it").
- **Fail-closed default.** Unknown stack → no matrix generated until user clicks; missing OIDC role → block deploy job scaffold with a typed TODO.
- **Surgical scope.** Writes ONLY under `.github/workflows/` or `.forgejo/workflows/` and prints the `secrets/ci.env` scaffold to chat (never writes `secrets/*.env` itself).

---

## References

- `_blocks/ci-github-actions.md`, `_blocks/ci-forgejo-actions.md`,
  `_blocks/ci-release-automation.md`, `_blocks/ci-security-gate.md`.
- `_blocks/domain-has-secrets.md` — storage path + loading convention.
- `_blocks/rule-pre-dev-gate.md` — analogue check before inventing a new workflow.
- `_primitives/kei-ci-lint.sh` — workflow YAML validator (R1–R7 rules).
- Evidence grade [E2] — mirrors GitHub Actions security hardening guide + Forgejo Actions docs as of 2026-04-21.
