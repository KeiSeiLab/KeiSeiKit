---
title: phase-4-route
path: self-audit/phase-4-route.md
dna_hash: sha256:d0eeae708273b45e
language: markdown
size_loc: 58
generated: by-keidocs
---

# self-audit/phase-4-route.md

## Public API

- `Phase 4 — Route` — For each finding in `SELECTED`, ask the user which action route to take.
- `4a — Per-finding click` — For EACH finding in `SELECTED`, emit ONE `AskUserQuestion`:
- `4b — Handoff rules` — Based on the click, append to `ROUTES` one of:
- `4c — Severity gate` — If a finding has `severity == critical` AND the user selected
- `Verify-criterion` — - `ROUTES` has exactly one entry per `SELECTED` finding.

## Related

- parent: `self-audit`

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
