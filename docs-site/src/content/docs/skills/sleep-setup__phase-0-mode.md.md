---
title: phase-0-mode
path: sleep-setup/phase-0-mode.md
dna_hash: sha256:b0872371b72f1ab2
language: markdown
size_loc: 64
generated: by-keidocs
---

# sleep-setup/phase-0-mode.md

## Public API

- `Phase 0 — Sleep mode pick` — Ask the user to pick the execution mode for nightly sleep-layer
- `0a — Mode click` — Emit ONE `AskUserQuestion`:
- `0b — Branching` — Branch the remainder of the pipeline based on `SLEEP_MODE`:
- `0c — Implication note` — Print a one-line reminder before continuing:
- `Verify-criterion` — - Exactly ONE `AskUserQuestion` in this phase.

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
