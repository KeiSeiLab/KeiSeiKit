---
title: phase-3-priority
path: sleep-on-it/phase-3-priority.md
dna_hash: sha256:3eb58c9c35693202
language: markdown
size_loc: 130
generated: by-keidocs
---

# sleep-on-it/phase-3-priority.md

## Public API

- `Phase 3 — Priority & time budget (click)` — Decide how much night-time the remote agent spends on this task.
- `3a — Click` — Emit ONE `AskUserQuestion`:
- `3b — Marathon confirmation` — If `LABEL == "Marathon"`, emit ONE more `AskUserQuestion` so the user
- `3c — Normalise` — Map the final label (after any marathon downgrade) to four variables:
- `3d — Cap check (informational)` — If `PRIORITY_LABEL ∈ {quick, standard, deep, marathon}` (i.e. this
- `3e — Advanced overrides (informational)` — After Phase 5 preview, explicit flags override the priority defaults:
- `Verify-criterion` — - `PRIORITY_LABEL ∈ {quick, standard, deep, marathon, weekly}`.

## Related

- parent: `sleep-on-it`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
