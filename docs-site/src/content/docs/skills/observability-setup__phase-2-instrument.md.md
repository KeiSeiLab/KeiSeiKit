---
title: phase-2-instrument
path: observability-setup/phase-2-instrument.md
dna_hash: sha256:6a70238cd41fe946
language: markdown
size_loc: 81
generated: by-keidocs
---

# observability-setup/phase-2-instrument.md

## Public API

- `Phase 2 — Code-side instrumentation (SDK + config diff)` — Decide WHICH SDK to wire per language, emit the init-call diff, and cite the
- `2a — Detect languages in the target service` — Run (via Bash):
- `2b — Emit AskUserQuestion (one call)` — ```json
- `2c — Per-language SDK table (reference, no user click)` — | Lang | Logs | Metrics | Traces |
- `2d — Generate init diffs` — For each language in `LANGUAGES`, emit a unified-diff patch to the target
- `Verify-criterion` — - `LANGUAGES` non-empty.

## Related

- parent: `observability-setup`

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
