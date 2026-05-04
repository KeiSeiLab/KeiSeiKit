---
title: phase-1-identity
path: pet-init/phase-1-identity.md
dna_hash: sha256:bad682724bfe9609
language: markdown
size_loc: 106
generated: by-keidocs
---

# pet-init/phase-1-identity.md

## Public API

- `Phase 1 — Identity` — Gather the four `[identity]` fields: `pet_name`, `user_name`, `addressing`,
- `1a — Pet name (free text)` — Emit a regular message (NOT AskUserQuestion):
- `1b — User name (free text)` — Emit a regular message:
- `1c — Addressing + languages (AskUserQuestion, 1 batch)` — Emit a single `AskUserQuestion` call with TWO questions:
- `Verify-criterion` — - `PET_NAME` set, trimmed, 1-30 chars
- `Failure modes (constructive paths, NO DOWNGRADE)` — If the user declines to give a name:

## Related

- parent: `pet-init`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
