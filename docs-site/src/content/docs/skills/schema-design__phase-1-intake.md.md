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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
