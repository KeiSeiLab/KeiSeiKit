---
title: phase-3-workflows
path: ci-scaffold/phase-3-workflows.md
dna_hash: sha256:00930afabe7ef7b2
language: markdown
size_loc: 113
generated: by-keidocs
---

# ci-scaffold/phase-3-workflows.md

## Public API

- `Phase 3 — Workflow generation` — Scaffold the YAML files under `.github/workflows/` (if `PLATFORM = GitHub Actions`) or `.forgejo/workflows/` (if `PLATFORM = Forgejo Actions`). Uses `_blocks/ci-github-actions.md` and `_blocks/ci-forgejo-actions.md` as the template source; uses `_blocks/ci-release-automation.md` for the release workflow; uses `_blocks/ci-security-gate.md` for the scanner workflow.
- `3a — Confirm generation scope (AskUserQuestion, multi-select)` — ```json
- `3b — Scaffold ci.yml` — Platform-specific base directory:
- `3c — Scaffold security.yml` — Uses `_blocks/ci-security-gate.md` as the authoritative template. Emits one job per selected scanner:
- `3d — Scaffold release.yml` — Template per `RELEASE`:
- `3e — Scaffold deploy.yml (per DEPLOY)` — - `aws-oidc` — `aws-actions/configure-aws-credentials@v4` with `role-to-assume: ${{ vars.AWS_ROLE_ARN }}`; environment `production` with required reviewer.
- `3f — Write files, print diff` — Emit each file path + the generated content as a fenced code block. DO NOT commit. Append to chat:
- `Verify-criterion` — - Every entry in `WORKFLOWS.selected` produced exactly one YAML file at the platform-correct path.

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
