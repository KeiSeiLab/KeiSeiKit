# CI — Forgejo Actions (self-hosted, Tailscale-only admin)

Forgejo Actions is GitHub-Actions compatible at the workflow-syntax layer (derived from Gitea Actions, which re-uses the `actions/*` runtime via `act`). A workflow that runs on GH usually runs on Forgejo with only the runner labels and registry URLs changed. Good fit for any repo that must stay on private hosting (sensitive IP, compliance, air-gap).

## Layout

Workflows live under `.forgejo/workflows/*.yml` (primary) — `.gitea/workflows/` also works for legacy repos. Keep the same narrow split as GH:

- `ci.yml` — build + test
- `release.yml` — tag-driven
- `security.yml` — scheduled scanners

## Self-hosted runner

Forgejo has no SaaS runner fleet — you provide the compute. Install `forgejo-runner` [VERIFIED: https://code.forgejo.org/forgejo/runner] on a node that is reachable ONLY over Tailscale.

Registration:

```bash
forgejo-runner register \
  --no-interactive \
  --instance http://<forgejo-host>:3000 \
  --name my-runner-01 \
  --labels "self-hosted,linux,x64,docker" \
  --token "$FORGEJO_RUNNER_TOKEN"       # from secrets/runner.env (RULE 0.8)
```

`FORGEJO_RUNNER_TOKEN` stays in `secrets/runner.env` — reference via env name only, never paste the literal value.

Target in workflow:

```yaml
jobs:
  build:
    runs-on: [self-hosted, linux, x64]
```

## GitHub-compat surface

Works out of the box: `actions/checkout@v4`, `actions/cache@v4`, `actions/setup-node@v4`, `Swatinem/rust-cache@v2`, shell/docker steps, matrix, reusable workflows (`uses: <forgejo-host>/<owner>/<repo>/.forgejo/workflows/<file>@<sha>`).

Does NOT work: `permissions:` block (Forgejo token is scoped at the runner level, not per-job), OIDC federation to AWS/GCP (no JWKS endpoint served by Forgejo), GitHub-Marketplace actions that call `api.github.com` directly.

Workaround for OIDC: for cloud deploys from Forgejo, prefer short-lived STS tokens minted by a bastion that has an IAM role, passed into the runner via a sealed env file rotated daily.

## Tailscale-only admin posture

Forgejo bound to a private interface (Tailscale/Wireguard/VPC); pick an address + SSH port per your topology. NEVER bind Forgejo to a public IP — runner tokens, PATs, and repo contents are all harvestable from a publicly-reachable instance.

## Secrets

Forgejo repo secrets (`Repo → Settings → Actions → Secrets`) mirror GH secrets syntactically: `${{ secrets.FOO }}`. Organisation-scope secrets also supported. Every secret still references the canonical `~/.claude/secrets/.env` / `secrets/*.env` source — repo secrets are cache copies, rotated when the source rotates.

## Forbidden

- Exposing Forgejo port 3000 or 2222 on a public IP
- Running `forgejo-runner` on a host that is also a production application node
- Mirroring a private Forgejo repo to github.com to "get free CI" — if any project rule forbids a github remote, the mirror violates it transitively
- Hard-coded runner tokens in workflow YAML (always `${{ secrets.* }}`)
