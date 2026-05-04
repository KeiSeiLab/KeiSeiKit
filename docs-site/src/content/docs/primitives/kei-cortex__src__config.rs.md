---
title: config.rs
path: kei-cortex/src/config.rs
dna_hash: sha256:a6dccb12f319be36
language: rust
size_loc: 204
generated: by-keidocs
---

# kei-cortex/src/config.rs

Runtime configuration for the cortex daemon.

`AppConfig` is assembled once at startup from CLI arguments and handed to
the router via `AppState`. All paths are resolved to absolute at construct
time so handlers never have to re-resolve `~` or cwd.

## Public API

- `pub const DEFAULT_PORT` — Default listen port when `--port` is not provided.
- `pub const DEFAULT_CORS_ORIGIN` — Default CORS origin when `--cors-origin` is not provided.
- `pub const DEFAULT_PROVIDER` — Default LLM provider when `--default-provider` is not provided.
- Errors from config assembly — surface to `main.rs` as non-zero exit.
- Runtime configuration.
- TCP port for the local HTTP listener. Bound to 127.0.0.1 only.
- Single CORS origin the daemon will echo back. Exact-match; no wildcards.
- Path to the bearer-token file. Read once at startup.
- SQLite database holding the agent ledger (kei-ledger schema).
- Root directory holding `<user_id>.toml` pet manifests.
- SQLite database holding pet conversation memory (kei-pet schema).
- Directory containing bundled Cubism sample rigs (`haru/`, `mao/`,
- Working directory used by the chat handler to discover CLAUDE.md /
- Project root used for skill resolution (`<root>/.claude/skills/<name>/`)
- Default LLM provider name when the request lacks `?provider=`.
- SQLite database for per-turn token-event telemetry (kei-token-tracker
- Legacy 7-arg builder kept for tests. Panics on bad CORS.
- 11-arg constructor; returns a `Result` so `main` can render clean errors.
- Validate that `cors_origin` parses as a `HeaderValue` — we crash early
- Resolve the default live2d samples directory to an absolute path anchored
- `HOME` with a plain-`.` fallback so tests that unset `HOME` still work.
- Resolve the default cwd. Falls back to `.` when the OS refuses to give

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
