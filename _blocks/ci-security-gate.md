# CI — Security gate (secrets, SCA, SBOM, semgrep, licenses)

Every PR passes through this gate before merge. Every scheduled run re-scans `main`. Pair with `ci-github-actions.md` / `ci-forgejo-actions.md` (the shell) and RULE 0.8 (secrets SSoT) /  () — the gate enforces both.

## Scanner set (one job each, matrix is fine)

| Concern | Tool | Trigger | Fail threshold |
|---|---|---|---|
| Leaked secrets | gitleaks [VERIFIED: https://github.com/gitleaks/gitleaks] | PR + push | any finding |
| Rust SCA | cargo-audit [VERIFIED: https://github.com/rustsec/rustsec] | PR + cron daily | any `Vulnerability` |
| Node SCA | `npm audit` / `pnpm audit` (native) | PR + cron | `high` and above |
| Python SCA | pip-audit [VERIFIED: https://github.com/pypa/pip-audit] | PR + cron | any CVE |
| SBOM generation | syft [VERIFIED: https://github.com/anchore/syft] | release only | CycloneDX JSON as artefact |
| SAST / patterns | semgrep [VERIFIED: https://github.com/semgrep/semgrep] | PR | any `ERROR` severity |
| License policy | cargo-deny [VERIFIED: https://github.com/EmbarkStudios/cargo-deny] (Rust) / license-checker (JS) | PR | disallowed SPDX ID |

## gitleaks — secrets scan (always first)

Runs before any build step so that a detected secret aborts the job without ever shipping a binary that used it.

```yaml
- uses: gitleaks/gitleaks-action@v2          # [VERIFIED: https://github.com/gitleaks/gitleaks-action]
  env:
    GITLEAKS_LICENSE: ${{ secrets.GITLEAKS_LICENSE }}   # orgs only; free for ≤25 users
```

Custom rules in `.gitleaks.toml` at repo root — mirror the patterns from `~/.claude/rules/secrets-single-source.md` (sk-, ghp_, sk-ant-, Telegram bot, AWS access key, etc.). Any hit FAILS the run. No "informational" severity for secrets.

## cargo-audit / pip-audit / npm audit

Daily cron to catch CVEs published after merge. Fail-fast on HIGH/CRITICAL; report MEDIUM to a tracking issue rather than blocking the PR.

```yaml
- run: cargo audit --deny warnings --deny unmaintained --deny yanked
```

Pin the advisory-DB commit in vendored copies; upstream can get taken down.

## SBOM via syft

Generate CycloneDX JSON for every published artefact. Attach to the GitHub Release (see `ci-release-automation.md`) and to the container image as an OCI annotation.

```yaml
- uses: anchore/sbom-action@v0               # [VERIFIED: https://github.com/anchore/sbom-action]
  with:
    format: cyclonedx-json
    artifact-name: sbom.cdx.json
```

SLSA provenance (`slsa-framework/slsa-github-generator`) is an optional upgrade; required when shipping to any customer under a supply-chain contract.

## semgrep — SAST

`p/default` + `p/secrets` + `p/owasp-top-ten` + any language pack relevant to the repo. Custom rules under `.semgrep/*.yaml` for project-specific patterns (e.g. "no `unwrap()` in request handlers").

```yaml
- uses: semgrep/semgrep-action@v1            # [VERIFIED: https://github.com/semgrep/semgrep-action]
  env:
    SEMGREP_RULES: p/default p/secrets p/owasp-top-ten
```

## License policy

`cargo-deny` `deny.toml` declares allowed SPDX identifiers (`MIT`, `Apache-2.0`, `BSD-3-Clause`, `ISC`, `Unicode-DFS-2016`). Anything else FAILS the PR. GPL / AGPL / SSPL in a commercial repo = hard stop. For JS, `license-checker --failOn 'GPL;AGPL;SSPL'`.

## Scheduling

```yaml
on:
  pull_request:
  push: { branches: [main] }
  schedule:
    - cron: "17 3 * * *"      # daily 03:17 UTC — off-hour, avoids global burst
```

## Forbidden

- Running the security gate AFTER build/test (secret must block before the secret-using binary exists)
- Allowing "informational" severity on secrets scans (gitleaks = binary; 0 or 1)
- Skipping `cargo-audit` / `pip-audit` on release workflows (a CVE published yesterday ships today without it)
- Uploading SBOM to a public artefact store from a RULE-0.1 repo (internal artefact store only)
- Copy-pasting a secret detected by gitleaks into the chat to "discuss" — rotate at provider FIRST, then discuss
