# Phase 1 — Intake (platform, languages, deploy, release)

One free-text paragraph, then four click batches. This is the only phase that accepts typed input.

## 1a — Ask for the repo description

Emit a regular message (NOT AskUserQuestion):


Store the reply verbatim as `REPO`.

## 1b — Platform click (AskUserQuestion, single-select)

```json
{
  "questions": [
    {
      "question": "CI platform?",
      "header": "Platform",
      "multiSelect": false,
      "options": [
        {"label": "GitHub Actions",                  "description": "github.com-hosted runners; OIDC to AWS/GCP/CF; see _blocks/ci-github-actions.md"},
        {"label": "Neither / unsure",                "description": "Skill defaults to Forgejo (safer for unpublished work); override later"}
      ]
    }
  ]
}
```

Store as `PLATFORM`. If `Both` is selected, emit a one-line confirm: "You understand — only public-safe code ever pushes to GitHub?" and wait for a `y` typed reply before proceeding.

## 1c — Languages click (AskUserQuestion, multi-select)

```json
{
  "questions": [
    {
      "question": "Which language toolchains must CI build + test?",
      "header": "Languages",
      "multiSelect": true,
      "options": [
        {"label": "Rust",    "description": "cargo build/test + Swatinem/rust-cache@v2 + cargo-audit + cargo-deny"},
        {"label": "Node / TypeScript", "description": "pnpm or npm; actions/setup-node@v4 with cache; npm audit / pnpm audit"},
        {"label": "Python", "description": "actions/setup-python@v5 + pip cache; pip-audit; hatch/poetry/uv as lock source"},
        {"label": "Go",     "description": "actions/setup-go@v5 + cache; go vet + govulncheck; goreleaser for release"},
        {"label": "Flutter","description": "subosito/flutter-action@v2; flutter analyze + flutter test before any build"},
        {"label": "Swift",  "description": "SPM on macos-14 runner (GH) or self-hosted mac (Forgejo); codesign outside CI"},
        {"label": "Docker image only", "description": "No language toolchain in CI; buildx builds the image + SBOM"}
      ]
    }
  ]
}
```

Store as `LANGS`. Empty selection → re-ask.

## 1d — Deploy target click (AskUserQuestion, single-select)

```json
{
  "questions": [
    {
      "question": "Where does CI deploy the artefact?",
      "header": "Deploy",
      "multiSelect": false,
      "options": [
        {"label": "None — CI only tests",         "description": "Skip all deploy jobs; still run build + security gate"},
        {"label": "AWS via OIDC",                 "description": "aws-actions/configure-aws-credentials@v4; role trust policy pinned to repo+ref"},
        {"label": "GCP via OIDC (WIF)",           "description": "google-github-actions/auth@v2 + Workload Identity Federation"},
        {"label": "Cloudflare (Workers/Pages/R2)","description": "wrangler deploy; CLOUDFLARE_API_TOKEN with scopes from self-sufficiency.md"},
        {"label": "Modal (GPU)",                  "description": "modal deploy; cost tiers enforced (see _blocks/deploy-modal.md + RULE api-cost-guard)"},
        {"label": "Container registry (GHCR / ECR / GAR / Forgejo)", "description": "Build + push image, optionally sign with cosign; SBOM attached"},
        {"label": "Custom / on-prem via SSH",     "description": "appleboy/ssh-action@v1 with an ephemeral key minted per run"}
      ]
    }
  ]
}
```

Store as `DEPLOY`.

## 1e — Release strategy click (AskUserQuestion, single-select)

```json
{
  "questions": [
    {
      "question": "Release / versioning tool?",
      "header": "Release",
      "multiSelect": false,
      "options": [
        {"label": "release-please",  "description": "Conventional Commits → Release-PR; polyglot; recommended monorepo default"},
        {"label": "changesets",      "description": "JS/TS; per-PR .changeset/*.md; best for npm publishing"},
        {"label": "cargo-release",   "description": "Rust crates.io; sign-tag, cargo publish with trusted-publishing token"},
        {"label": "goreleaser",      "description": "Go; tag push → build matrix + archives + checksums + SBOM"},
        {"label": "Manual tags / none", "description": "No release automation; CI builds + tests only"}
      ]
    }
  ]
}
```

Store as `RELEASE`.

## Verify-criterion

- `REPO` non-empty.
- `PLATFORM` exactly one label.
- `LANGS` has ≥1 entry (or exactly `Docker image only`).
- `DEPLOY`, `RELEASE` each exactly one label.
- If `PLATFORM = Both`, explicit user `y` confirm captured ().
- If `DEPLOY = AWS via OIDC` (or GCP/WIF) and `PLATFORM = Forgejo`, warn: "Forgejo has no OIDC JWKS — the AWS role must be assumed via a bastion. Continue?" and offer NO DOWNGRADE alternatives before proceeding.
