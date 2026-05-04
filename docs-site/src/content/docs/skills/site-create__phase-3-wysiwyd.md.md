---
title: phase-3-wysiwyd
path: site-create/phase-3-wysiwyd.md
dna_hash: sha256:f11c0e942caf9970
language: markdown
size_loc: 133
generated: by-keidocs
---

# site-create/phase-3-wysiwyd.md

## Public API

- `Phase 3 — WYSIWYD Block-by-Block Build (CORE LOOP)` — > Goal: for each section in `SECTIONS`, generate the source file, render a
- `3.0 — One-time setup (first section only)` — Start the dev server via the `live-preview.sh` primitive:
- `3.1 — For each section in SECTIONS: generate` — Write `<project-root>/src/sections/<Name>.astro` (or `<Name>.tsx` for Next /
- `3.2 — Render mock` — ```bash
- `3.3 — Show + approve (1 AskUserQuestion per section)` — Display `mocks/<Name>.png` inline. Ask:
- `3.4 — Act on approval` — ### Approve
- `3.5 — WYSIWYD verify before any cross-section edit` — If a later phase (e.g. audit) would edit a locked section, you MUST first:
- `3.6 — Verify criterion (completion of Phase 3)` — All `SECTIONS[i]` must have `locked: true` in `site-state.json`.

## Related

- parent: `site-create`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
