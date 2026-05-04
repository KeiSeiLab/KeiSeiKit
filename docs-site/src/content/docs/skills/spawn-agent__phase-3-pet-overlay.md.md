---
title: phase-3-pet-overlay
path: spawn-agent/phase-3-pet-overlay.md
dna_hash: sha256:2e59a6d247b4b19b
language: markdown
size_loc: 181
generated: by-keidocs
---

# spawn-agent/phase-3-pet-overlay.md

## Public API

- `Phase 3 (pet-overlay) — Optional pet persona attached to this spawn` — > Goal: decide whether this spawn receives a pet persona overlay, and if so
- `3-pet.a — First AskUserQuestion: attach a pet?` — Send ONE `AskUserQuestion`:
- `3-pet.b — Discover available pets` — Run exactly one bash command (no chaining, so errors surface):
- `3-pet.c — Second AskUserQuestion: which pet?` — Build one option per discovered `.toml`. The `label` is the bare filename
- `3-pet.d — Validate the selected manifest` — Run exactly one command:
- `3-pet.e — Verify criterion` — - `PET_MANIFEST_PATH` is either `None` or an absolute filesystem path.
- `3-pet.f — Failure paths (NO DOWNGRADE)` — - (A) No pets on disk → offer `/new-pet`, NOT "skip silently". The user
- `Rules (inherit from SKILL.md)` — - **Pure-click contract.** At most one free-text entry in this phase, and

## Related

- parent: `spawn-agent`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
