# CI — Release automation (SemVer, changelog, tagging)

Automates "merge to main → versioned release" so the next step (build artefact, publish, deploy) has a predictable trigger. Picks ONE tool per repo — mixing release-please with cargo-release creates duplicate tags. Pair with `ci-github-actions.md` / `ci-forgejo-actions.md` for the workflow shell.

## Tool picks per ecosystem

| Stack | Tool | Trigger | Changelog source |
|---|---|---|---|
| Monorepo / polyglot / apps | release-please [VERIFIED: https://github.com/googleapis/release-please] | merge to main | Conventional Commits |
| JS/TS packages (npm publish) | changesets [VERIFIED: https://github.com/changesets/changesets] | merge of `.changeset/*.md` | Explicit changeset files |
| Rust crates (crates.io) | cargo-release [VERIFIED: https://github.com/crate-ci/cargo-release] | manual `cargo release` | git log + Conventional Commits |
| Go modules | goreleaser [VERIFIED: https://github.com/goreleaser/goreleaser] | tag push | git log + `.goreleaser.yaml` |

## SemVer contract

- `MAJOR` — breaking change to public API, wire format, on-disk schema, config file keys
- `MINOR` — additive feature, no breakage, new optional fields
- `PATCH` — bug fix, performance, docs, dep bump without API change

Conventional Commits mapping: `feat!:` / `BREAKING CHANGE:` → MAJOR; `feat:` → MINOR; `fix:` / `perf:` / `refactor:` → PATCH; `checkpoint:` / `audit:` / `chore:` → no-bump (ignored by release-please).

## release-please minimal config

`.github/workflows/release.yml` (or `.forgejo/workflows/release.yml`):

```yaml
on:
  push:
    branches: [main]
permissions:
  contents: write            # create tags + releases
  pull-requests: write       # update the Release-PR
jobs:
  release-please:
    runs-on: ubuntu-24.04
    steps:
      - uses: googleapis/release-please-action@v4   # [VERIFIED: https://github.com/googleapis/release-please-action]
        with:
          release-type: rust                        # or node, python, go, simple, etc.
          token: ${{ secrets.GITHUB_TOKEN }}
```

release-please opens a long-lived "Release PR" that updates `CHANGELOG.md` + version file on every main merge; merging that PR creates the tag and GitHub Release. No human writes the changelog.

## changesets minimal config (JS/TS monorepo)

```yaml
- uses: changesets/action@v1       # [VERIFIED: https://github.com/changesets/action]
  with:
    publish: pnpm release           # runs `changeset publish`
  env:
    NPM_TOKEN: ${{ secrets.NPM_TOKEN }}
```

Each PR that changes a package ships a `.changeset/<name>.md` describing the bump. CI blocks merge without one (`changeset status --since=origin/main`).

## cargo-release minimal config (Rust crates.io)

`release.toml` at repo root:

```toml
sign-tag = true
push = true
tag-message = "{{crate_name}} {{version}}"
pre-release-commit-message = "release: {{version}}"
```

Publish workflow runs on tag push: `cargo publish --token "$CARGO_REGISTRY_TOKEN"` where the token is minted just-in-time from the `ci-security-gate.md` trusted-publishing flow.

## Lock-file discipline

`Cargo.lock` / `package-lock.json` / `pnpm-lock.yaml` / `pubspec.lock` / `go.sum` — ALWAYS committed (RULE git-conventions). Release workflows must FAIL if the lock file is stale: `cargo update --locked --dry-run`, `pnpm install --frozen-lockfile`, `go mod verify`.

## Forbidden

- Manual `git tag vX.Y.Z && git push --tags` when a release tool is configured (drift between CHANGELOG and tag)
- Two release tools in the same repo (release-please + cargo-release both tagging)
- Publishing from a `pull_request` trigger (never — only from `push` to main or `workflow_dispatch`)
- Forcing a tag with `git push --force origin refs/tags/*` — breaks every consumer that pinned by SHA
- Stale lock files passing CI (must be a hard fail, not a warning)
