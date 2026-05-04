---
title: dispatcher.rs
path: kei-decompose/src/dispatcher.rs
dna_hash: sha256:c69081eb95ba0c30
language: rust
size_loc: 195
generated: by-keidocs
---

# kei-decompose/src/dispatcher.rs

kei-spawn / kei-ledger CLI wrapper.

Same shell-out pattern as kei-decision: spawn a child process, capture
stdout JSON, parse into `SpawnRecord`. No tokio, no async.

Binary lookup order (kei-spawn):
1. `KEI_SPAWN_BIN` env var (absolute path)
2. `kei-spawn` on PATH
3. fallback `~/Projects/KeiSeiKit/_primitives/_rust/target/release/kei-spawn`

Same logic for kei-ledger via `KEI_LEDGER_BIN`.

## Public API

- `pub fn dispatch_all` — Dispatch each emitted task.toml to kei-spawn (and optionally kei-ledger).

## Related

- parent: `kei-decompose/Cargo.toml`
- imports: anyhow, crate, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
