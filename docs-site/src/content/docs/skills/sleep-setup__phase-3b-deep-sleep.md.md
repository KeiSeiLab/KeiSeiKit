---
title: phase-3b-deep-sleep
path: sleep-setup/phase-3b-deep-sleep.md
dna_hash: sha256:d84a700673a9f92c
language: markdown
size_loc: 157
generated: by-keidocs
---

# sleep-setup/phase-3b-deep-sleep.md

## Public API

- `Phase 3b — Deep-sleep NREM configuration (v0.13.0)` — Collect three pure-click decisions for Phase C (system consolidation):
- `Mode-dependent behaviour` — If `SLEEP_MODE == local-only` (set in Phase 0), the fork mode question
- `3b.1 — Deep-sleep cadence` — Emit ONE `AskUserQuestion`:
- `3b.2 — Fork output mode` — For `SLEEP_MODE ∈ {remote-only, hybrid}`, emit ONE `AskUserQuestion`
- `3b.3 — Memory-repo backend` — Emit ONE `AskUserQuestion`:
- `3b.4 — Write store config` — Call `kei-store init <STORE_BACKEND> --url <REPO_URL>` which writes
- `3b.5 — Verify-criterion` — - `DEEP_SLEEP_CRON_DAYS ∈ {0,1,3,7,14, or 1..=90}` for custom.

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
