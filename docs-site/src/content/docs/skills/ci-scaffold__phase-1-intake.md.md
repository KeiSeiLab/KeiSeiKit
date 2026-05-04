---
title: phase-1-intake
path: ci-scaffold/phase-1-intake.md
dna_hash: sha256:f4ebcc596ab349cf
language: markdown
size_loc: 112
generated: by-keidocs
---

# ci-scaffold/phase-1-intake.md

## Public API

- `Phase 1 — Intake (platform, languages, deploy, release)` — One free-text paragraph, then four click batches. This is the only phase that accepts typed input.
- `1a — Ask for the repo description` — Emit a regular message (NOT AskUserQuestion):
- `1b — Platform click (AskUserQuestion, single-select)` — ```json
- `1c — Languages click (AskUserQuestion, multi-select)` — ```json
- `1d — Deploy target click (AskUserQuestion, single-select)` — ```json
- `1e — Release strategy click (AskUserQuestion, single-select)` — ```json
- `Verify-criterion` — - `REPO` non-empty.

## Related

- parent: `ci-scaffold`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
