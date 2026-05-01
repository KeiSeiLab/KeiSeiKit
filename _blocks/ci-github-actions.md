# CI — GitHub Actions (OIDC, matrix, cache, reusable workflows)

Pipeline platform for code hosted on (or mirrored to) github.com. This block ships the defaults; pair with `ci-security-gate.md` for scanners and `ci-release-automation.md` for tags.

## Workflow layout

Keep workflow files narrow: ONE responsibility each under `.github/workflows/`.

- `ci.yml` — build + test on every push/PR
- `release.yml` — tag-driven release automation (see `ci-release-automation.md`)
- `security.yml` — scheduled scanners (see `ci-security-gate.md`)
- `deploy-*.yml` — per-environment deploys, each behind a GitHub Environment with required reviewers

## OIDC — cloud deploy WITHOUT long-lived keys

GitHub Actions mints a short-lived JWT per run; the cloud provider trusts `token.actions.githubusercontent.com` and issues temporary credentials. **Never** store `AWS_SECRET_ACCESS_KEY` / `GCP_SA_KEY` in repo secrets.

```yaml
permissions:
  id-token: write        # mandatory for OIDC
  contents: read
jobs:
  deploy:
    runs-on: ubuntu-24.04
    steps:
      - uses: actions/checkout@v4                  # [VERIFIED: https://github.com/actions/checkout]
      - uses: aws-actions/configure-aws-credentials@v4   # [VERIFIED: https://github.com/aws-actions/configure-aws-credentials]
        with:
          role-to-assume: arn:aws:iam::${{ vars.AWS_ACCOUNT_ID }}:role/gha-deployer
          aws-region: eu-north-1
```

Cloud-side role trust policy pins `repo:<org>/<repo>:ref:refs/heads/main` — wildcards invite cross-repo impersonation.

## Least-privilege GITHUB_TOKEN

Default token permissions at the workflow level, then widen per-job:

```yaml
permissions:
  contents: read          # read-only at top level
jobs:
  build:
    # inherits read-only
  release:
    permissions:
      contents: write     # only the release job gets write
      id-token: write
```

Org-level default should be `read` (Settings → Actions → Workflow permissions). Any job requiring write must opt in explicitly.

## Matrix builds

Fan out across OS × language version × target; `fail-fast: false` prevents one red cell from cancelling the whole matrix.

```yaml
strategy:
  fail-fast: false
  matrix:
    os:   [ubuntu-24.04, macos-14]
    rust: [stable, 1.80]              # MSRV pin
```

## Cache hygiene

- Lock-file as key, never branch name: `key: cargo-${{ hashFiles('**/Cargo.lock') }}`.
- `restore-keys` is a PREFIX fallback — safe for cold PRs.
- `actions/cache@v4` [VERIFIED: https://github.com/actions/cache] for generic; language-specific actions (`actions/setup-node@v4`, `Swatinem/rust-cache@v2`) manage cache internally — don't double-cache.
- Cache POISONING check: never cache directories that contain your built artefacts alongside downloaded deps.

## Reusable workflows

Shared logic lives in one repo and is called by `uses: <org>/<repo>/.github/workflows/<file>.yml@<sha>`. Pin by SHA, not tag — tags are mutable. `workflow_call` contract:

```yaml
on:
  workflow_call:
    inputs:
      rust-version: { required: true, type: string }
    secrets:
      CARGO_TOKEN: { required: false }
```

## Pinning third-party actions

Pin by full commit SHA, not tag: `uses: foo/bar@3a4b5c6d7e8f9012...` with a comment `# v2.1.0`. Dependabot updates SHAs the same way — supply-chain hijack via tag-overwrite is a documented class (e.g. `tj-actions/changed-files` 2025). [E2]

## Forbidden

- `secrets.AWS_SECRET_ACCESS_KEY` in any workflow (use OIDC)
- `permissions: write-all` at workflow level
- Third-party action pinned by tag
- `pull_request_target` with `checkout` of PR head + secrets access (classic pwn-request)
- Caching `target/` or `node_modules/` alongside `.git` or user config
