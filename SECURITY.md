# Security Policy

## Reporting a vulnerability

Email `info@greendragon.info` with a description and reproduction steps. PGP key available on request.

## Threat model

- Secrets handling: see RULE 0.8 — all tokens via env vars, hardcoding blocked at PreToolUse:Edit by `hooks/secrets-pre-guard.sh` and Rust binary `_primitives/_rust/secrets-guard/`.
- Banned-project leak guard: `_primitives/_rust/kei-leak-matrix/` runs on every push attempt to flag known patent / IP markers.
- Public-push gate: RULE 0.1 triple-confirm via `hooks/no-github-push.sh` before any push to publicly-reachable remote.

## Supported versions

Latest `v0.38.x` tag. Older versions accept fixes for CVEs only.

## Audit

See `docs/SECURITY.md` for the secret-pattern detector regex set used by `secrets-guard`.

### Known transitive-dependency advisories (2026-05-12 audit)

`cargo audit` flags 9 RUSTSEC advisories from transitive deps (not used directly):

- `rsa 0.9.10` — RUSTSEC-2023-0071 (Marvin Attack timing sidechannel). Path: vendored RSA used by S3/auth crates.
- `rustls-webpki 0.101.7 + 0.102.8` — RUSTSEC-2026-{0049,0098,0099,0104}. Path: TLS in HTTP/auth deps.
- `sqlx 0.8.0` — RUSTSEC-2024-0363 (Binary Protocol Misinterpretation). Path: postgres clients.
- `async-std 1.13.2` — RUSTSEC-2025-0052 (discontinued).
- `lru 0.12.5` — RUSTSEC-2026-0002 (unsound `IterMut`).
- `fxhash 0.2.1`, `instant 0.1.13` — unmaintained.

Resolution requires major-version bumps in direct deps (sqlx 0.9, rustls 0.23+, rsa 0.10). Tracked separately; non-blocker for current dev usage (no untrusted RSA-decrypt path, no untrusted TLS-cert validation against malicious URI/wildcard names in current code-paths).
