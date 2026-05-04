---
title: exec.rs
path: kei-provision/src/exec.rs
dna_hash: sha256:bcc2054d3c42c332
language: rust
size_loc: 134
generated: by-keidocs
---

# kei-provision/src/exec.rs

Shared subprocess helper for backend adapters.

Centralises `std::process::Command` so both Hetzner and Vultr backends
have a single JSON-exec path. Makes test-time PATH injection uniform.

DO NOT pass secrets as CLI args — env-only per RULE 0.8. Error
messages redact argv to `<bin> <N args>` and truncate stderr to 200
chars to avoid info-disclosure in logs (future-proofing against
accidental `--api-key $X` refactors + vultr-cli stderr leaking
request URL query params).

## Public API

- Max stderr length retained in error messages before truncation.
- Redact CLI args to `<bin> <N args>` — never echo full argv.
- Truncate stderr to `STDERR_MAX` chars, UTF-8 safe (char-boundary aware).
- `pub fn run_json` — Run `bin args…` and return parsed JSON on exit code 0.
- `pub fn run_json_strict` — Run `bin args…` and fail loudly on non-zero (create/delete paths).
- `pub fn run_void` — Plain void run — success = ok, failure = err with stderr context.
- `pub fn require_cli` — Assert a CLI binary is on PATH (friendly error).
- `pub fn require_env` — Assert an env var is set + non-empty (friendly error).

## Related

- parent: `kei-provision/Cargo.toml`
- imports: anyhow, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
