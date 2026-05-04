---
title: phase-2-identity-provider
path: auth-setup/phase-2-identity-provider.md
dna_hash: sha256:ca9dc3fc6a72b42d
language: markdown
size_loc: 79
generated: by-keidocs
---

# auth-setup/phase-2-identity-provider.md

## Public API

- `Phase 2 — Identity provider selection + env-var scaffold` — Only runs if `FLOWS` contains `OAuth / social` or `Enterprise SSO`. If
- `2a — Provider click (AskUserQuestion, multi-select)` — Reference: `_blocks/auth-oauth2-oidc.md`.
- `2b — Emit env-var scaffold (no AskUserQuestion)` — For EACH provider in `PROVIDERS`, emit the env-var rows the user must add
- `secrets/auth.env — add these, then `chmod 600` the file` — GOOGLE_CLIENT_ID=
- `2c — Library pick (emitted inline, no AskUserQuestion)` — Driven by `STACK` from Phase 1:
- `Verify-criterion` — - Every provider in `PROVIDERS` has its env-var scaffold printed.

## Related

- parent: `auth-setup`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
