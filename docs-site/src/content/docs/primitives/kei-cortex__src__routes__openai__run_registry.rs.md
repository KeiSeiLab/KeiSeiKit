---
title: run_registry.rs
path: kei-cortex/src/routes/openai/run_registry.rs
dna_hash: sha256:c28870e36cf97320
language: rust
size_loc: 131
generated: by-keidocs
---

# kei-cortex/src/routes/openai/run_registry.rs

`RunRegistry` + `RunSlot` — process-wide store of in-flight `/v1/runs`.

Lifted out of `runs.rs` so that file stays under the 200-LOC
Constructor-Pattern ceiling. The handlers in `runs.rs` mutate this
registry; the registry knows nothing about HTTP.

P1.1.d (2026-04-28): cancel migrated from `Arc<Notify>` to
`tokio_util::sync::CancellationToken`. Reason: `agent_runner::
stream_events` already takes a `CancellationToken`, so a direct
field avoids the Notify→Token bridge spawn. `cancel()` calls
`token.cancel()` which is fire-once, observable via
`is_cancelled()` and `cancelled().await` — same semantics the
agent loop wants.

## Public API

- Per-process registry of in-flight runs.
- One in-flight run. `rx` is held inside an `Arc<Mutex<Option<>>>`
- `pub fn take_receiver` — Take the receiver out of the slot — first subscriber wins.
- `pub fn global` — Process-singleton accessor. Same pattern as `session::global()` —

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: dashmap, once_cell, std, tokio, tokio_util

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
