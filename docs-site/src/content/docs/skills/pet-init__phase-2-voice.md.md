---
title: phase-2-voice
path: pet-init/phase-2-voice.md
dna_hash: sha256:4c8848e120ede9e8
language: markdown
size_loc: 124
generated: by-keidocs
---

# pet-init/phase-2-voice.md

## Public API

- `Phase 2 — Voice` — Gather the four `[voice]` fields: `tone_primary`, `tone_secondary`,
- `2a — Voice batch (AskUserQuestion, 1 batch with 4 questions)` — Emit a single `AskUserQuestion` call:
- `2b — Map clicks to variables` — `TONE_PRIMARY` — lowercase the chosen label:
- `2c — Consistency check` — If `HUMOR_STYLE == "none"` and `HUMOR_FREQUENCY != "rare"`, emit a regular
- `Verify-criterion` — - `TONE_PRIMARY` is one of `warm` / `neutral` / `formal` / `playful`
- `Failure modes (constructive paths)` — If the user bails mid-batch (closes without answering):

## Related

- parent: `pet-init`

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
