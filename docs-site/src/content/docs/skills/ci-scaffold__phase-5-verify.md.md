---
title: phase-5-verify
path: ci-scaffold/phase-5-verify.md
dna_hash: sha256:0808dae8d8a60d98
language: markdown
size_loc: 98
generated: by-keidocs
---

# ci-scaffold/phase-5-verify.md

## Public API

- `Phase 5 — Verify via kei-ci-lint, then final report` — Close the pipeline by validating every generated workflow with `_primitives/kei-ci-lint.sh`. The lint has seven rules (R1–R7); each finding drives one AskUserQuestion for fix/skip/abort.
- `5a — Run the linter` — Execute:
- `or, if PLATFORM = Forgejo Actions:` — sh _primitives/kei-ci-lint.sh --dir .forgejo/workflows
- `5b — Per-finding triage (AskUserQuestion, single-select per finding)` — For EACH `FAIL` line, emit:
- `5c — Fix recipes (applied inline when user picks "Fix now")` — | Rule | Fix |
- `5d — Re-run linter after fixes` — After all fixes applied, re-run `kei-ci-lint` once. If still failing, enter the 3-Level Escalation (dev-workflow.md): after 2 automatic fix attempts, STOP and escalate — present the remaining findings to the user with a numbered plan (NO DOWNGRADE: alternative scaffolds, not "accept the violation").
- `5e — Emit final report` — Template (from SKILL.md):
- `5f — Handoff` — If `LINT` is `PASS` or `WARN-only`, advise:
- `Verify-criterion` — - `kei-ci-lint` was executed against the generated files.

## Related

- parent: `ci-scaffold`

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
