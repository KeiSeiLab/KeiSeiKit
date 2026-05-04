---
title: phase-3-session-strategy
path: auth-setup/phase-3-session-strategy.md
dna_hash: sha256:def632cb075c0499
language: markdown
size_loc: 79
generated: by-keidocs
---

# auth-setup/phase-3-session-strategy.md

## Public API

- `Phase 3 — Session strategy + cookie configuration` — Decides how the authenticated principal is carried across requests. Reads
- `3a — Strategy click (AskUserQuestion, single-select)` — ```json
- `3b — Emit cookie-flag checklist (inline, no AskUserQuestion)` — Apply ONLY when `SESSION` involves a browser cookie. For every cookie the
- `3c — Emit JWT-specific checklist (inline) — only if JWT chosen` — ```
- `3d — CSRF strategy (inline, driven by SESSION)` — - Cookie session + same-origin forms → `SameSite=Lax` is enough; plus a
- `Verify-criterion` — - `SESSION` set to exactly one strategy.

## Related

- parent: `auth-setup`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
