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
