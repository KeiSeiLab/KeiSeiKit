---
title: phase-0b-time
path: sleep-setup/phase-0b-time.md
dna_hash: sha256:4739864e2087897b
language: markdown
size_loc: 79
generated: by-keidocs
---

# sleep-setup/phase-0b-time.md

## Public API

- `Phase 0b — Sleep time picker` — Ask the user to pick the local-time trigger for nightly consolidation.
- `0b.1 — Time click` — Emit ONE `AskUserQuestion`:
- `0b.2 — Store or branch` — - Non-custom pick → store as `SLEEP_TIME_LOCAL` verbatim (e.g. `03:00`).
- `0b.3 — Validation` — Validate with regex `^([01][0-9]|2[0-3]):[0-5][0-9]$`:
- `0b.4 — Confirmation line` — Once validated, print:
- `Verify-criterion` — - At least ONE `AskUserQuestion` (two if Custom picked).

## Related

- parent: `sleep-setup`

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
