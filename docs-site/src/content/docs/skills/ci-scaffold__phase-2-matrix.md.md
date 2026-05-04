---
title: phase-2-matrix
path: ci-scaffold/phase-2-matrix.md
dna_hash: sha256:a9578f82d1293dc7
language: markdown
size_loc: 83
generated: by-keidocs
---

# ci-scaffold/phase-2-matrix.md

## Public API

- `Phase 2 — Build matrix (OS × version × target)` — Decide how the build fans out. Matrix minimum: OS × primary-language version. Max reasonable: 3 OS × 3 versions × 3 targets = 27 cells — beyond that CI time-to-green kills iteration speed.
- `2a — Matrix click (AskUserQuestion, multi-select across three axes)` — The question encodes three axes in one screen to keep the click contract tight. Each selection is stored as a set; the cartesian product becomes `MATRIX`.
- `2b — Sanity check (no AskUserQuestion)` — Compute `N = |os| × |versions| × |targets|`. Print inline:
- `2c — Fail-fast click inferred (inline, no extra AskUserQuestion)` — - PR matrix: `fail-fast: false` (user wants to see ALL failing cells at once).
- `Verify-criterion` — - `MATRIX.os` has ≥1 entry.

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
