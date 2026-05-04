---
title: phase-1-intake
path: schema-design/phase-1-intake.md
dna_hash: sha256:62b32fa7e725f1f7
language: markdown
size_loc: 92
generated: by-keidocs
---

# schema-design/phase-1-intake.md

## Public API

- `Phase 1 — Intake (DB, ORM, scale, style, migration control)` — One free-text paragraph, then ONE batched `AskUserQuestion` call with all
- `1a — Ask for the app description` — Emit a regular message (NOT AskUserQuestion):
- `1b — Batched click (AskUserQuestion, 5 questions in ONE call)` — The UI cap per `AskUserQuestion` call is 4–5 questions; emit all five at
- `Verify-criterion` — - `INTAKE` non-empty.

## Related

- parent: `schema-design`

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
