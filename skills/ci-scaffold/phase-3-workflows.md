# Phase 3 — Workflow generation

Scaffold the YAML files under `.github/workflows/` (if `PLATFORM = GitHub Actions`) or `.forgejo/workflows/` (if `PLATFORM = Forgejo Actions`). Uses `_blocks/ci-github-actions.md` and `_blocks/ci-forgejo-actions.md` as the template source; uses `_blocks/ci-release-automation.md` for the release workflow; uses `_blocks/ci-security-gate.md` for the scanner workflow.

## 3a — Confirm generation scope (AskUserQuestion, multi-select)

```json
{
  "questions": [
    {
      "question": "Which workflow files to generate?",
      "header": "Workflows",
      "multiSelect": true,
      "options": [
        {"label": "ci.yml — build + test + lint (from MATRIX)",        "description": "Runs on push + PR; uses fail-fast:false for PRs"},
        {"label": "security.yml — gitleaks + SCA + semgrep",            "description": "From _blocks/ci-security-gate.md; PR trigger + daily cron"},
        {"label": "release.yml — tag / publish (from RELEASE)",         "description": "Only if RELEASE != 'Manual tags / none'"},
        {"label": "deploy.yml — per DEPLOY target",                     "description": "Only if DEPLOY != 'None — CI only tests'; guarded by GitHub Environment + reviewers"},
        {"label": "sbom.yml — syft CycloneDX on release",               "description": "Attaches SBOM artefact to release; from ci-security-gate.md"}
      ]
    }
  ]
}
```

Store as `WORKFLOWS.selected`. Default-include the first two if the user clicks "nothing selected" — fail-closed.

## 3b — Scaffold ci.yml

Platform-specific base directory:

- `PLATFORM = GitHub Actions` → `.github/workflows/ci.yml`
- `PLATFORM = Forgejo Actions` → `.forgejo/workflows/ci.yml`

Template (filled from `MATRIX`, `LANGS`):

```yaml
name: ci
on:
  push:
    branches: [main]
  pull_request:
permissions:
  contents: read         # least-privilege top-level (ci-github-actions.md R2)
jobs:
  build-test:
    strategy:
      fail-fast: false
      matrix:
        os: [<MATRIX.os>]
        # version axis injected per language
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4         # [VERIFIED: https://github.com/actions/checkout]
      # language setup steps injected from LANGS
      - name: Test
        run: <per-lang test command>
```

Per-language step injection (one `setup-*` + one cache strategy each):

| Lang | setup action | cache action | test command |
|---|---|---|---|
| Rust | `actions-rust-lang/setup-rust-toolchain@v1` [VERIFIED: https://github.com/actions-rust-lang/setup-rust-toolchain] | `Swatinem/rust-cache@v2` [VERIFIED: https://github.com/Swatinem/rust-cache] | `cargo test --workspace --locked` |
| Node | `actions/setup-node@v4` [VERIFIED: https://github.com/actions/setup-node] | built-in `cache: pnpm` | `pnpm install --frozen-lockfile && pnpm test` |
| Python | `actions/setup-python@v5` [VERIFIED: https://github.com/actions/setup-python] | built-in `cache: pip` | `pip install -e .[test] && pytest` |
| Go | `actions/setup-go@v5` [VERIFIED: https://github.com/actions/setup-go] | built-in (>=setup-go v4) | `go test ./...` |
| Flutter | `subosito/flutter-action@v2` [VERIFIED: https://github.com/subosito/flutter-action] | built-in | `flutter analyze && flutter test` |
| Swift | `maxim-lobanov/setup-xcode@v1` [VERIFIED: https://github.com/maxim-lobanov/setup-xcode] | n/a | `swift test` |

All `uses:` tags above correspond to published versions on the linked repos — RULE 0.4: never invent a tag. If the repo's latest major is unknown at scaffold time, pin by SHA with a `# v<major>` comment.

## 3c — Scaffold security.yml

Uses `_blocks/ci-security-gate.md` as the authoritative template. Emits one job per selected scanner:

- `secrets-scan` — gitleaks (first job, before build)
- `sca-<lang>` — one job per `LANGS` entry (`cargo audit`, `pnpm audit`, `pip-audit`, `govulncheck`)
- `sast` — semgrep with `p/default p/secrets p/owasp-top-ten`
- `licenses` — cargo-deny (Rust) or `license-checker --failOn 'GPL;AGPL;SSPL'` (Node)

Trigger: `on: { pull_request:, push: { branches: [main] }, schedule: [{ cron: '17 3 * * *' }] }`.

## 3d — Scaffold release.yml

Template per `RELEASE`:

- `release-please` → `googleapis/release-please-action@v4` [VERIFIED: https://github.com/googleapis/release-please-action]
- `changesets` → `changesets/action@v1` [VERIFIED: https://github.com/changesets/action]
- `cargo-release` → step runs `cargo publish --locked`, token via OIDC trusted-publishing (or `CARGO_REGISTRY_TOKEN` from Phase 4)
- `goreleaser` → `goreleaser/goreleaser-action@v6` [VERIFIED: https://github.com/goreleaser/goreleaser-action]

Permissions set at job level only: `contents: write`, `id-token: write`, `pull-requests: write` (release-please).

## 3e — Scaffold deploy.yml (per DEPLOY)

- `aws-oidc` — `aws-actions/configure-aws-credentials@v4` with `role-to-assume: ${{ vars.AWS_ROLE_ARN }}`; environment `production` with required reviewer.
- `gcp-oidc` — `google-github-actions/auth@v2` [VERIFIED: https://github.com/google-github-actions/auth] with `workload_identity_provider`.
- `cloudflare` — `cloudflare/wrangler-action@v3` [VERIFIED: https://github.com/cloudflare/wrangler-action] with `apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}`.
- `modal` — `pip install modal && modal deploy` with `MODAL_TOKEN_ID` + `MODAL_TOKEN_SECRET` (Phase 4 registers the env names; RULE api-cost-guard before first run).

## 3f — Write files, print diff

Emit each file path + the generated content as a fenced code block. DO NOT commit. Append to chat:

> Scaffold written. Review, then `git add <paths>` + commit. Phase 5 will run `kei-ci-lint` before you push.

## Verify-criterion

- Every entry in `WORKFLOWS.selected` produced exactly one YAML file at the platform-correct path.
- Every `uses:` line has a VERIFIED cite in the surrounding block reference OR is pinned by 40-hex SHA.
- No `secrets.*` variable is populated with a literal in the YAML (Phase 4 owns names only).
- No workflow has `permissions: write-all` at top level.
