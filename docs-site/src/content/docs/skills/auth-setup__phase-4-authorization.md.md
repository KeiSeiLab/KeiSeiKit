---
title: phase-4-authorization
path: auth-setup/phase-4-authorization.md
dna_hash: sha256:fde99a4f2cf6329c
language: markdown
size_loc: 84
generated: by-keidocs
---

# auth-setup/phase-4-authorization.md

## Public API

- `Phase 4 — Authorization model + permission matrix` — Decides who-can-do-what after authentication. Reads
- `4a — Model click (AskUserQuestion, single-select)` — ```json
- `4b — Emit permission matrix skeleton (inline)` — For `RBAC` / `RBAC + ownership`, emit a table stub the user must fill in
- `4c — Enforcement-point rule (inline)` — - Middleware, not handlers. Every authenticated request runs an authz
- `4d — Policy-engine pick (inline, driven by AUTHZ)` — - `RBAC` / `RBAC + ownership` → in-code match on `Permission` enum; no
- `Verify-criterion` — - `AUTHZ` is exactly one choice.

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
