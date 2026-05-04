---
title: runner.rs
path: kei-llm-mlx/src/runner.rs
dna_hash: sha256:10d70aa32d31d6a7
language: rust
size_loc: 144
generated: by-keidocs
---

# kei-llm-mlx/src/runner.rs

Runner trait — the seam every shell-out goes through.

Constructor Pattern: ALL subprocess invocation lives here. Every other
cube (`generate`, `stream`, `server`, `discovery`) accepts a `&dyn Runner`
so unit tests substitute `MockRunner` without touching the host system
and without invoking real `mlx_lm`.

Mirrors the W56 `kei-machine-probe` pattern (sync trait, sanitized
fixture stems). Tokio is held as a workspace dep for future streaming
transport but the trait surface stays sync — every mlx_lm shell-out is
whole-output capture, not interactive PTY.

## Public API

- Captured one-shot subprocess result. `code = None` means the child was
- `pub trait Runner` — Single seam. Implementors: `SystemRunner` (real host) or `MockRunner`
- `pub struct SystemRunner` — Real-host runner. ONLY production user of `std::process::Command::new`
- `pub fn fixture_stem` — Sanitize `(cmd, args)` into a fixture filename stem. Bytes outside
- `pub struct MockRunner` — In-memory fixture-backed runner for tests. Two layers:

## Related

- parent: `kei-llm-mlx/Cargo.toml`
- imports: anyhow, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
