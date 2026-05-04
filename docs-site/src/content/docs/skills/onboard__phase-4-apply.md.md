---
title: phase-4-apply
path: onboard/phase-4-apply.md
dna_hash: sha256:9604c52f7d345254
language: markdown
size_loc: 194
generated: by-keidocs
---

# onboard/phase-4-apply.md

## Public API

- `Phase 4 — Apply (branches on `MODE`)` — Three mode-specific branches. Each ends with `APPLIED` and `SKIPPED` lists
- `4-pre — Multi-project bulk shortcut (AskUserQuestion, conditional)` — Only emit if `len(PATHS) > 1` AND `GRANULARITY == "mixed"`. This lets the
- `4a — Full-auto branch (1 confirm total)` — ### Summary preview
- `4b — Step-by-step branch (≥N confirms)` — Iterate over `CANDIDATES` in order (agents first, then hooks, then
- `4c — Full-manual branch (per-candidate wizard walk)` — For EVERY candidate, fully delegate to the appropriate pipeline with
- `Verify-criterion (Phase 4 overall)` — - Exactly one branch ran per project (4a / 4b / 4c).

## Related

- parent: `onboard`

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
