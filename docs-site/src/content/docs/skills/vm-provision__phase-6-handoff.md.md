---
title: phase-6-handoff
path: vm-provision/phase-6-handoff.md
dna_hash: sha256:7cd9eac31b450ea0
language: markdown
size_loc: 123
generated: by-keidocs
---

# vm-provision/phase-6-handoff.md

## Public API

- `Phase 6 — Handoff + Final Report` — > Goal: emit a single, complete report and (optionally) hand off to
- `6.a — Artefact ledger` — Collect and surface:
- `6.b — Final report` — ```
- `6.c — Handoff (no AskUserQuestion; next-skill dispatch inferred)` — If `TLS_HOST` was set AND the caller's intent mentions deploying an app
- `6.d — Memory save (RULE memory-protocol)` — Append to `memory/{project-or-infra}.md`:
- `6.e — Verify criterion` — - [ ] Report emitted.
- `6.f — Rollback instructions (always include in the report)` — ```
- `destroy the VM + all its resources (idempotent)` — _primitives/provision-<PROVIDER>.sh destroy <VM_NAME> --force
- `purge local artefacts (plan, logs, captured configs)` — rm -rf <run-dir>

## Related

- parent: `vm-provision`

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
