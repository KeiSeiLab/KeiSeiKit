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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
