---
title: runner.rs
path: kei-net-wireguard/src/runner.rs
dna_hash: sha256:5d680024ec038d47
language: rust
size_loc: 81
generated: by-keidocs
---

# kei-net-wireguard/src/runner.rs

Runner trait — the seam every `wg`/`wg-quick` shell-out goes through.

Constructor Pattern: ALL subprocess invocation lives here. `network.rs`
takes `Arc<dyn Runner + Send + Sync>` so unit/smoke tests substitute a
fixture-backed mock without touching the host system. Mirrors the
`kei-llm-mlx::runner` pattern (sync trait; the async glue lives at the
call site via `tokio::task::spawn_blocking`).

## Public API

- Captured one-shot subprocess result. `code = None` means the child was
- `pub trait Runner` — The seam. Implementors: [`SystemRunner`] (real host) or test mocks.
- `pub struct SystemRunner` — Real-host runner — the only production user of `Command::new` in the
- `pub fn check_success` — Helper for callers that want a "must succeed or anyhow!" wrapper. Kept

## Related

- parent: `kei-net-wireguard/Cargo.toml`
- imports: anyhow, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
