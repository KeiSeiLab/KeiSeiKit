---
title: phase-4-secrets
path: ci-scaffold/phase-4-secrets.md
dna_hash: sha256:9ee62ae18f92354d
language: markdown
size_loc: 98
generated: by-keidocs
---

# ci-scaffold/phase-4-secrets.md

## Public API

- `Phase 4 — Secrets posture (OIDC vs PAT; RULE 0.8 scaffold)` — Decides how CI obtains credentials. Default bias is OIDC (short-lived, no stored secret); fall back to PAT only when the provider has no OIDC (e.g. Forgejo → AWS, npm trusted-publishing not configured, custom SSH deploy). Every chosen secret is referenced by NAME ONLY per RULE 0.8 — this skill NEVER writes a value.
- `4a — Posture click (AskUserQuestion, single-select)` — ```json
- `4b — Enumerate required secrets (no AskUserQuestion; derived from DEPLOY + RELEASE)` — Walk the matrix below. For each hit, add to `SECRETS.required`.
- `4c — Emit `secrets/ci.env` scaffold (inline; no file write)` — Print as a fenced code block. Example when posture is OIDC-first + cargo-release:
- `secrets/ci.env — paths and NAMES only. chmod 600 + .gitignore before writing values.` — 
- `RULE 0.8: reference by env-var name. NEVER paste a literal here.` — 
- `OIDC (no secrets stored; vars on the provider side)` — AWS_ROLE_ARN=            # arn:aws:iam::<account>:role/gha-deployer — set as repo VAR, not secret
- `Release publishing` — CARGO_REGISTRY_TOKEN=    # trusted-publishing preferred; fallback PAT only if TP unavailable
- `4d — Confirm repo-side secret registration (AskUserQuestion, multi-select)` — ```json
- `Verify-criterion` — - `SECRETS.posture` is exactly one choice.

## Related

- parent: `ci-scaffold`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
