# Phase 5 — Verify via kei-ci-lint, then final report

Close the pipeline by validating every generated workflow with `_primitives/kei-ci-lint.sh`. The lint has seven rules (R1–R7); each finding drives one AskUserQuestion for fix/skip/abort.

## 5a — Run the linter

Execute:

```
sh _primitives/kei-ci-lint.sh --dir .github/workflows
# or, if PLATFORM = Forgejo Actions:
sh _primitives/kei-ci-lint.sh --dir .forgejo/workflows
```

Capture stdout + stderr. Parse output — one line per finding, format `FAIL <file> <R#> <message>` or `WARN …`.

## 5b — Per-finding triage (AskUserQuestion, single-select per finding)

For EACH `FAIL` line, emit:

```json
{
  "questions": [
    {
      "question": "Lint finding: <R#> in <file> — <message>. Action?",
      "header": "Lint",
      "multiSelect": false,
      "options": [
        {"label": "Fix now (skill applies the recommended patch)",
         "description": "Skill edits the YAML file inline; next lint run must show clean"},
        {"label": "Skip (add to allowlist with justification)",
         "description": "Skill prompts for a 1-line justification; stored as a YAML comment + ledger line"},
        {"label": "Abort (stop the pipeline; user investigates manually)",
         "description": "Scaffold stays in place, but final report marks the skill run as INCOMPLETE"}
      ]
    }
  ]
}
```

Store each answer under `LINT.triage[<finding-key>]`.

## 5c — Fix recipes (applied inline when user picks "Fix now")

| Rule | Fix |
|---|---|
| R1 missing `name:` / `on:` / `jobs:` | Insert the field with a sensible default (`name` from filename; `on: { pull_request: , push: { branches: [main] } }`) |
| R2 no top-level `permissions:` | Insert `permissions: { contents: read }` at top level |
| R2 `permissions: write-all` | Replace with `contents: read`; move `write` to the single job that needs it |
| R3 OIDC + AWS keys present | Ask AskUserQuestion: "Which one do you keep? OIDC / keys" — remove the other |
| R4 `key: github.ref` | Replace with `key: <name>-${{ hashFiles('<lockfile>') }}` |
| R5 action pinned by tag | Look up the tag's commit SHA on the action's repo, replace `@vX.Y` with the full 40-hex SHA, add `# vX.Y` comment. If lookup fails, leave the tag with a TODO comment and ABORT rather than inventing a SHA (RULE 0.4) |
| R6 `::set-output` / `::save-state` | Replace with `$GITHUB_OUTPUT` / `$GITHUB_STATE` redirect (GH docs 2023+) |
| R6 `actions/checkout@v1` / `v2` | Upgrade to `@v4` (and re-run R5 pin-by-SHA) |
| R7 `pull_request_target` + PR-head checkout | Either remove `pull_request_target` (prefer) OR remove the `ref: ${{ github.event.pull_request.head.sha }}` line. Present both to user |

## 5d — Re-run linter after fixes

After all fixes applied, re-run `kei-ci-lint` once. If still failing, enter the 3-Level Escalation (dev-workflow.md): after 2 automatic fix attempts, STOP and escalate — present the remaining findings to the user with a numbered plan (NO DOWNGRADE: alternative scaffolds, not "accept the violation").

## 5e — Emit final report

Template (from SKILL.md):

```
=== CI-SCAFFOLD REPORT ===
Repo:       <REPO>
Platform:   <PLATFORM>
Languages:  <LANGS joined>
Deploy:     <DEPLOY>
Release:    <RELEASE>
Matrix:     <|os|> × <|versions|> × <|targets|> = <N> cells
Workflows:  <paths, one per line>
Secrets:    <|SECRETS.required|> env names scaffolded to secrets/ci.env (posture: <SECRETS.posture>)
Lint:       <PASS | WARN-<N> | FAIL-<N>> — <fixes applied count>/<skips count>/<aborts count>
Next:       git diff → review → commit on feat/<name>-ci → PR

Citations used (RULE 0.4):
  - actions/checkout@v4                   [VERIFIED: https://github.com/actions/checkout]
  - actions/cache@v4                      [VERIFIED: https://github.com/actions/cache]
  - <one line per every uses: in generated files>
```

## 5f — Handoff

If `LINT` is `PASS` or `WARN-only`, advise:

> Scaffold complete. Next: `git add .github/ .forgejo/ secrets/ci.env` (NOT the secret values — just the scaffold), commit on a `feat/ci-*` branch, push, and request review.

If `LINT = FAIL` after 2 fix passes, advise the user to invoke `compose-solution` with the remaining findings as new components — the meta-orchestrator may find missing `_blocks/` or suggest a new primitive.

## Verify-criterion

- `kei-ci-lint` was executed against the generated files.
- Every `FAIL` line produced exactly one AskUserQuestion triage.
- No action tag was invented to satisfy R5 — unresolvable SHA lookups must ABORT with a TODO (RULE 0.4 hard).
- Final report lists every citation used in every generated workflow.
- `Next:` line tells the user exactly what to stage, where to branch, and where to PR.
