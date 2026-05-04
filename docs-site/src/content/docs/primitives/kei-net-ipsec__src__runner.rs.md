---
title: runner.rs
path: kei-net-ipsec/src/runner.rs
dna_hash: sha256:427b0e25e1f680ca
language: rust
size_loc: 139
generated: by-keidocs
---

# kei-net-ipsec/src/runner.rs

Runner trait — the seam every `swanctl` invocation goes through.

Constructor Pattern: ALL subprocess invocation lives here. The
[`crate::network::IpsecMode`] cube accepts an `Arc<dyn Runner + Send +
Sync>` so unit tests substitute [`MockRunner`] without spawning real
`swanctl` and without root privileges.

Mirrors the W59 `kei-llm-mlx::runner` pattern (sync trait, sanitized
fixture stems, in-memory override map). The trait stays sync because
every `swanctl` shell-out is whole-output capture (no streaming).

## Public API

- Captured one-shot subprocess result. `code = None` means the child was
- `pub trait Runner` — Single seam. Implementors: [`SystemRunner`] (real host) or
- `pub struct SystemRunner` — Real-host runner. ONLY production user of `std::process::Command::new`
- `pub fn fixture_stem` — Sanitize `(cmd, args)` into a fixture-stem key. Bytes outside
- `pub struct MockRunner` — In-memory fixture-backed runner for tests. Overrides keyed by
- `pub fn recorded` — Snapshot of `(cmd, args)` invocations recorded so far.

## Related

- parent: `kei-net-ipsec/Cargo.toml`
- imports: anyhow, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
