# Phase 4 — Secrets posture (OIDC vs PAT; RULE 0.8 scaffold)

Decides how CI obtains credentials. Default bias is OIDC (short-lived, no stored secret); fall back to PAT only when the provider has no OIDC (e.g. Forgejo → AWS, npm trusted-publishing not configured, custom SSH deploy). Every chosen secret is referenced by NAME ONLY per RULE 0.8 — this skill NEVER writes a value.

## 4a — Posture click (AskUserQuestion, single-select)

```json
{
  "questions": [
    {
      "question": "Credential posture for CI?",
      "header": "Secrets",
      "multiSelect": false,
      "options": [
        {"label": "OIDC-first (recommended)",
         "description": "Cloud roles trust token.actions.githubusercontent.com; no long-lived keys stored. Requires DEPLOY ∈ {aws-oidc, gcp-oidc} and PLATFORM = GitHub Actions."},
        {"label": "PAT fallback (when OIDC unavailable)",
         "description": "Long-lived scoped tokens stored in repo secrets. Rotation schedule mandatory (30–90 days). Used for Cloudflare, npm, DockerHub, custom SSH."},
        {"label": "Hybrid — OIDC where possible, PAT elsewhere",
         "description": "Most real setups. Skill emits both sections of the scaffold."},
        {"label": "No secrets (public CI tests only)",
         "description": "ci.yml + security.yml do not need credentials. deploy.yml / release.yml skipped."}
      ]
    }
  ]
}
```

Store as `SECRETS.posture`.

If `PLATFORM = Forgejo Actions` and the user picked `OIDC-first`, warn: "Forgejo does not serve a JWKS endpoint. Use the bastion pattern from `_blocks/ci-forgejo-actions.md` OR switch to PAT-fallback." Offer both constructive paths (NO DOWNGRADE) and re-ask.

## 4b — Enumerate required secrets (no AskUserQuestion; derived from DEPLOY + RELEASE)

Walk the matrix below. For each hit, add to `SECRETS.required`.

| DEPLOY / RELEASE | OIDC posture | PAT fallback posture |
|---|---|---|
| `aws-oidc` | `AWS_ROLE_ARN` (repo var, not secret); `AWS_REGION` | `AWS_ACCESS_KEY_ID` + `AWS_SECRET_ACCESS_KEY` (last-resort, rotate 30d) |
| `gcp-oidc` | `GCP_WORKLOAD_IDENTITY_PROVIDER` + `GCP_SERVICE_ACCOUNT` | `GCP_SA_KEY` JSON (avoid; Google deprecates static keys 2026) |
| `cloudflare` | (Workers OIDC preview; most prod still token) | `CLOUDFLARE_API_TOKEN` (scopes per `self-sufficiency.md`); `CLOUDFLARE_ACCOUNT_ID` |
| `modal` | n/a (Modal has its own token model) | `MODAL_TOKEN_ID` + `MODAL_TOKEN_SECRET`; cost tier check pre-launch |
| `registry (GHCR)` | built-in `GITHUB_TOKEN` write-packages | — |
| `registry (ECR)` | Uses AWS OIDC role | `AWS_ACCESS_KEY_ID` + `AWS_SECRET_ACCESS_KEY` |
| `registry (Forgejo)` | `FORGEJO_TOKEN` (built-in at runner) | — |
| `custom SSH` | — | `SSH_PRIVATE_KEY` (ed25519, generated fresh per repo), `SSH_HOST`, `SSH_USER` |
| `RELEASE=cargo-release` | crates.io trusted publishing (2025+) | `CARGO_REGISTRY_TOKEN` |
| `RELEASE=changesets` | npm trusted publishing (2026 preview) | `NPM_TOKEN` |

## 4c — Emit `secrets/ci.env` scaffold (inline; no file write)

Print as a fenced code block. Example when posture is OIDC-first + cargo-release:

```bash
# secrets/ci.env — paths and NAMES only. chmod 600 + .gitignore before writing values.
# RULE 0.8: reference by env-var name. NEVER paste a literal here.

# OIDC (no secrets stored; vars on the provider side)
AWS_ROLE_ARN=            # arn:aws:iam::<account>:role/gha-deployer — set as repo VAR, not secret
AWS_REGION=              # eu-north-1

# Release publishing
CARGO_REGISTRY_TOKEN=    # trusted-publishing preferred; fallback PAT only if TP unavailable
```

Append the reminder once:

> `secrets/ci.env` must be `chmod 600` AND listed in `.gitignore` BEFORE the first write. See `_blocks/domain-has-secrets.md`. Repo-level "Secrets and variables → Actions" is the deployment copy — rotate source `.env` when repo secret rotates, not the other way around.

## 4d — Confirm repo-side secret registration (AskUserQuestion, multi-select)

```json
{
  "questions": [
    {
      "question": "For each name I listed, confirm it is REGISTERED on the platform (Settings → Actions → Secrets or Repo Variables):",
      "header": "Registered",
      "multiSelect": true,
      "options": [
        {"label": "All names present and current (rotated within the last 90 days)", "description": "Proceed to Phase 5"},
        {"label": "Some names missing — I will register now and re-run", "description": "Skill exits; re-enter after registration"},
        {"label": "I use a secrets manager (Vault / 1Password CLI / Doppler) that syncs to the platform", "description": "Acceptable; confirm sync is green"},
        {"label": "None registered yet — show me the platform link",   "description": "Emit link per PLATFORM and exit"}
      ]
    }
  ]
}
```

Store the answer as `SECRETS.registration_status`. Any answer other than the first pauses Phase 5.

## Verify-criterion

- `SECRETS.posture` is exactly one choice.
- `SECRETS.required` is fully enumerated from `DEPLOY` + `RELEASE`; no `TODO` placeholders.
- The printed scaffold has NO literal values — every `=` is followed by whitespace or a `#` comment.
- Forgejo + OIDC combination has either the bastion pattern documented or the user opted into PAT-fallback.
- `SECRETS.registration_status` non-empty.
