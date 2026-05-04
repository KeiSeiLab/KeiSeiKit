---
title: phase-3-edge
path: pet-init/phase-3-edge.md
dna_hash: sha256:6ce3d4097d587f6f
language: markdown
size_loc: 131
generated: by-keidocs
---

# pet-init/phase-3-edge.md

## Public API

- `Phase 3 — Edge` — Gather the three `[edge]` fields (`directness`, `initiative`, `profanity`)
- `3a — Edge batch (AskUserQuestion, 1 batch with 3 questions)` — Emit a single `AskUserQuestion` call:
- `3b — Map clicks to variables` — `DIRECTNESS` — lowercase the chosen label:
- `3c — Forbidden topics (free text, optional)` — Emit a regular message (NOT AskUserQuestion):
- `3d — Consistency check (soft)` — If `DIRECTNESS == "blunt"` and `PROFANITY == "never"`, emit a regular
- `Verify-criterion` — - `DIRECTNESS` is one of `gentle` / `balanced` / `direct` / `blunt`
- `Failure modes (constructive paths)` — If the user seems confused by the Directness scale (asks "what does blunt

## Related

- parent: `pet-init`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
