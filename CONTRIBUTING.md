# Contributing

This is a working monorepo (Rust + TypeScript + bash hooks + agent manifests). PRs accepted via Forgejo or GitHub.

## Setup

```bash
git clone <repo>
cd KeiSeiKit-public
cargo check --workspace
cd _ts_packages && pnpm install
```

## Before opening a PR

- `cargo check --workspace` clean
- `cargo test --workspace --no-fail-fast` all green
- `cargo audit` no critical CVEs
- Constructor Pattern: ≤200 LOC per file, ≤30 LOC per function
- Conventional commit prefix: `feat:` / `fix:` / `chore:` / `refactor:` / `docs:` / `test:`

## Code style

- Rust: `rustfmt` default, `clippy -W clippy::all`
- TypeScript: project-local `tsconfig.json`, no broad `any`
- Bash: `bash -n` syntax check, prefer POSIX `sh` when possible

## Security

Never commit secrets. All tokens live in `~/.claude/secrets/.env` or `<repo>/secrets/*.env` — referenced by env var name only. See `docs/SECURITY.md` for the secret-pattern detector spec.
