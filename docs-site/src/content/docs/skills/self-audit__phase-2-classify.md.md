---
title: phase-2-classify
path: self-audit/phase-2-classify.md
dna_hash: sha256:6e289c0abb65d4a6
language: markdown
size_loc: 55
generated: by-keidocs
---

# self-audit/phase-2-classify.md

## Public API

- `Phase 2 — Classify` — Tag each finding with a category and a severity; `CLASSIFIED` is produced.
- `2a — Category (automatic, no click)` — For each finding in `FINDINGS`:
- `2b — Severity hint (automatic heuristic)` — Grep the finding's `event_class`:
- `2c — Severity confirm click (single AskUserQuestion)` — Emit ONE `AskUserQuestion` batch grouping the severity confirm into a
- `Verify-criterion` — - Every finding has a `category` and a `severity`.

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
